//! CSV ingestion (RFC-035 §3–4). Behind the default-on `csv` feature.
//!
//! CSV input is external input: every constructor returns `Result` and malformed
//! data never panics (RFC-035 §1). The first row is the header (required); rows
//! must be rectangular; only empty cells are missing by default.
//!
//! Note: this module is named `csv`, so the external `csv` crate is referenced via
//! the absolute path `::csv` to disambiguate it from the module.

use std::path::Path;

use crate::error::MattenDataError;
use crate::table::{CellValue, Table};

/// Reads `path` into a `String`, refusing to read more than
/// `matten::MattenLimits::default().max_parse_bytes` bytes (RFC-132 §12.1) —
/// mirrors core `matten`'s `read_bounded_file` (`tensor/ops.rs`). A metadata
/// check rejects an obviously oversized file before opening it; `Read::take`
/// bounds the actual read too, since a size check alone is a TOCTOU race.
fn read_bounded_file(path: &Path) -> Result<String, MattenDataError> {
    use std::io::Read;

    let max_bytes = matten::MattenLimits::default().max_parse_bytes;
    let metadata = std::fs::metadata(path).map_err(|source| MattenDataError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    if metadata.len() > max_bytes as u64 {
        return Err(MattenDataError::Matten(matten::MattenError::Parse {
            format: matten::DataFormat::Csv,
            message: format!(
                "file is {} bytes, exceeding the maximum parse size of {max_bytes} bytes \
                 (MattenLimits::max_parse_bytes)",
                metadata.len()
            ),
        }));
    }

    let file = std::fs::File::open(path).map_err(|source| MattenDataError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut content = String::new();
    file.take(max_bytes as u64 + 1)
        .read_to_string(&mut content)
        .map_err(|source| MattenDataError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    if content.len() > max_bytes {
        return Err(MattenDataError::Matten(matten::MattenError::Parse {
            format: matten::DataFormat::Csv,
            message: format!(
                "file exceeds the maximum parse size of {max_bytes} bytes \
                 (MattenLimits::max_parse_bytes)"
            ),
        }));
    }
    Ok(content)
}

impl Table {
    /// Parse a `Table` from a CSV string.
    ///
    /// The first row is the header. Empty input, empty or duplicate header names,
    /// and ragged rows are reported as [`MattenDataError`] (never a panic).
    ///
    /// ```
    /// use matten_data::Table;
    /// let table = Table::from_csv_str("a,b\n1,2\n3,4").unwrap();
    /// assert_eq!(table.row_count(), 2);
    /// assert_eq!(table.column_names(), &["a".to_string(), "b".to_string()]);
    /// ```
    pub fn from_csv_str(input: &str) -> Result<Table, MattenDataError> {
        if input.trim().is_empty() {
            return Err(MattenDataError::EmptyInput);
        }

        let mut reader = ::csv::ReaderBuilder::new()
            .has_headers(true)
            // Allow varying record lengths so ragged rows can be reported with a
            // precise RaggedRow error rather than the parser's generic message.
            .flexible(true)
            .from_reader(input.as_bytes());

        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| MattenDataError::Csv {
                message: e.to_string(),
            })?
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

        // Reject duplicate header names (named selection requires unambiguity).
        for i in 0..headers.len() {
            for j in (i + 1)..headers.len() {
                if headers[i] == headers[j] {
                    return Err(MattenDataError::DuplicateColumn {
                        name: headers[i].clone(),
                    });
                }
            }
        }

        let n = headers.len();
        let mut rows: Vec<Vec<CellValue>> = Vec::new();
        for (idx, record) in reader.records().enumerate() {
            let record = record.map_err(|e| MattenDataError::Csv {
                message: e.to_string(),
            })?;

            // Skip a stray fully-empty record (e.g. a blank trailing line).
            if record.is_empty() {
                continue;
            }

            // Header is CSV line 1, so the first data record is line 2.
            let line = idx + 2;
            if record.len() != n {
                return Err(MattenDataError::RaggedRow {
                    row: line,
                    expected: n,
                    actual: record.len(),
                });
            }

            rows.push(record.iter().map(parse_cell).collect());
        }

        Ok(Table::from_parts(headers, rows))
    }

    /// Parse a `Table` from a CSV file at `path`.
    ///
    /// I/O failures (for example a missing file) are reported as
    /// [`MattenDataError::Io`] with the path and underlying error preserved.
    ///
    /// # Errors
    ///
    /// Also returns [`MattenDataError::Matten`] if the file exceeds
    /// `matten::MattenLimits::default().max_parse_bytes` (RFC-132 §12.1) — the
    /// same boundary control core `matten`'s `load_csv` enforces, applied here
    /// too since this is the same kind of untrusted-file entry point. Checked
    /// via file metadata before the file is opened, and enforced again during
    /// the read itself (a size check alone is a TOCTOU race).
    pub fn from_csv_path<P: AsRef<Path>>(path: P) -> Result<Table, MattenDataError> {
        let path = path.as_ref();
        let content = read_bounded_file(path)?;
        Table::from_csv_str(&content)
    }
}

/// Infer a [`CellValue`] from a raw CSV field (RFC-035 §4.1–4.2).
///
/// Surrounding whitespace is trimmed. An empty field is `Missing`; otherwise the
/// value is inferred as `Int`, then `Float`, then `Bool` (`true`/`false`), and
/// finally `Text`. Booleans are not numbers, and text is not parsed as numeric
/// here — numeric conversion is strict and explicit in `try_numeric`.
pub(crate) fn parse_cell(raw: &str) -> CellValue {
    let s = raw.trim();
    if s.is_empty() {
        return CellValue::Missing;
    }
    if let Ok(i) = s.parse::<i64>() {
        return CellValue::Int(i);
    }
    if let Ok(fl) = s.parse::<f64>() {
        return CellValue::Float(fl);
    }
    match s {
        "true" => CellValue::Bool(true),
        "false" => CellValue::Bool(false),
        _ => CellValue::Text(s.to_string()),
    }
}
