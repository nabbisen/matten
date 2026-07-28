//! Batched CSV reading (RFC-082). Behind the off-by-default `streaming` feature.
//!
//! `CsvBatchReader` reads a CSV file incrementally, yielding [`Table`] batches of a
//! caller-chosen row count, so a file larger than available memory can still be
//! processed as long as the *work* is batch-friendly (column statistics, chunked
//! standardization, a train/test split). It is synchronous, single-pass, and
//! reuses the same header validation, ragged-row detection, and cell-parsing rules
//! as [`Table::from_csv_path`] — a batch is indistinguishable from the
//! corresponding slice of a fully-loaded `Table`.
//!
//! This module introduces no schema evolution, no lenient/skip-malformed mode, and
//! no streaming numeric conversion — see RFC-082 §5 for the full non-goal list.

use std::fs::File;
use std::path::{Path, PathBuf};

use crate::csv::parse_cell;
use crate::error::MattenDataError;
use crate::table::{CellValue, Table};

/// Reads a CSV file in row-count-bounded batches (RFC-082).
///
/// Only available behind the `streaming` feature. A batch is an ordinary [`Table`];
/// every existing `Table` operation works on it unchanged.
///
/// ```
/// # fn main() -> Result<(), matten_data::MattenDataError> {
/// use matten_data::CsvBatchReader;
///
/// # let path = std::env::temp_dir().join("matten_data_stream_doctest.csv");
/// # std::fs::write(&path, "a,b\n1,2\n3,4\n5,6").unwrap();
/// let mut reader = CsvBatchReader::open(&path, 2)?;
/// let first = reader.next_batch()?.expect("first batch");
/// assert_eq!(first.row_count(), 2);
/// let second = reader.next_batch()?.expect("second batch");
/// assert_eq!(second.row_count(), 1);
/// assert!(reader.next_batch()?.is_none());
/// # std::fs::remove_file(&path).ok();
/// # Ok(())
/// # }
/// ```
pub struct CsvBatchReader {
    reader: ::csv::Reader<File>,
    path: PathBuf,
    headers: Vec<String>,
    batch_rows: usize,
    line: usize,
    done: bool,
}

impl CsvBatchReader {
    /// Opens `path` for batched reading, `batch_rows` data rows at a time.
    ///
    /// The header row is read once, here, and applied to every batch. Header
    /// validation (non-empty, no duplicates) matches [`Table::from_csv_path`]
    /// exactly.
    ///
    /// # Errors
    ///
    /// - [`MattenDataError::InvalidBatchSize`] if `batch_rows == 0`.
    /// - [`MattenDataError::Io`] if the file cannot be opened, or an I/O error
    ///   occurs while reading the header.
    /// - [`MattenDataError::EmptyInput`] if the file has no header row.
    /// - [`MattenDataError::Csv`] for a malformed header (empty column name, or a
    ///   parser-level problem).
    /// - [`MattenDataError::DuplicateColumn`] if the header repeats a column name.
    pub fn open(path: &Path, batch_rows: usize) -> Result<Self, MattenDataError> {
        if batch_rows == 0 {
            return Err(MattenDataError::InvalidBatchSize);
        }

        let file = File::open(path).map_err(|source| MattenDataError::Io {
            path: path.to_path_buf(),
            source,
        })?;

        let mut reader = ::csv::ReaderBuilder::new()
            .has_headers(true)
            // Match Table::from_csv_str: allow varying record lengths so a ragged
            // row is reported as a precise RaggedRow error, not the parser's own
            // generic "wrong number of fields" message.
            .flexible(true)
            .from_reader(file);

        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| map_csv_error(e, path))?
            .iter()
            .map(|h| h.trim().to_string())
            .collect();

        if headers.is_empty() {
            return Err(MattenDataError::EmptyInput);
        }
        for (i, name) in headers.iter().enumerate() {
            if name.is_empty() {
                return Err(MattenDataError::Csv {
                    message: format!("header column {} is empty", i + 1),
                });
            }
        }
        for i in 0..headers.len() {
            for j in (i + 1)..headers.len() {
                if headers[i] == headers[j] {
                    return Err(MattenDataError::DuplicateColumn {
                        name: headers[i].clone(),
                    });
                }
            }
        }

        Ok(CsvBatchReader {
            reader,
            path: path.to_path_buf(),
            headers,
            batch_rows,
            line: 1, // the header is CSV line 1
            done: false,
        })
    }

    /// Reads the next batch of up to `batch_rows` data rows.
    ///
    /// Returns `Ok(None)` once the file is exhausted; every subsequent call also
    /// returns `Ok(None)`. On error, the reader becomes unusable: `done` is set
    /// before the error is returned, so every subsequent call also returns
    /// `Ok(None)` rather than resuming or repeating the error.
    ///
    /// # Errors
    ///
    /// - [`MattenDataError::RaggedRow`] if a data row has a different field count
    ///   than the header — the same variant and one-based line number
    ///   [`Table::from_csv_path`] reports for the same file.
    /// - [`MattenDataError::Csv`] for a parser-level problem (e.g. invalid UTF-8).
    /// - [`MattenDataError::Io`] for an I/O error while reading.
    pub fn next_batch(&mut self) -> Result<Option<Table>, MattenDataError> {
        if self.done {
            return Ok(None);
        }

        let n = self.headers.len();
        let mut rows: Vec<Vec<CellValue>> = Vec::with_capacity(self.batch_rows);
        let mut record = ::csv::StringRecord::new();

        while rows.len() < self.batch_rows {
            let read = match self.reader.read_record(&mut record) {
                Ok(read) => read,
                Err(e) => {
                    self.done = true;
                    return Err(map_csv_error(e, &self.path));
                }
            };

            if !read {
                break; // EOF
            }

            self.line += 1;

            // Skip a stray fully-empty record (e.g. a blank trailing line),
            // matching Table::from_csv_str.
            if record.is_empty() {
                continue;
            }

            if record.len() != n {
                self.done = true;
                return Err(MattenDataError::RaggedRow {
                    row: self.line,
                    expected: n,
                    actual: record.len(),
                });
            }

            rows.push(record.iter().map(parse_cell).collect());
        }

        if rows.is_empty() {
            self.done = true;
            return Ok(None);
        }

        Ok(Some(Table::from_parts(self.headers.clone(), rows)))
    }
}

/// Maps a `csv` crate error to `MattenDataError`, distinguishing a genuine I/O
/// failure from a parser-level problem — the two categories `Table::from_csv_path`
/// already distinguishes (`Io` at its upfront file read, `Csv` thereafter).
fn map_csv_error(e: ::csv::Error, path: &Path) -> MattenDataError {
    let message = e.to_string();
    match e.into_kind() {
        ::csv::ErrorKind::Io(source) => MattenDataError::Io {
            path: path.to_path_buf(),
            source,
        },
        _ => MattenDataError::Csv { message },
    }
}
