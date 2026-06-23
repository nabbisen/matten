//! The `Table` model and `CellValue` (RFC-034 §4–5).
//!
//! `Table` is a small, owned, rectangular table-like data set whose end goal is a
//! numeric [`matten::Tensor`]. It is **not** a dataframe (RFC-033, RFC-042): there
//! are no joins, group-by, pivot, query, lazy execution, or indexing APIs.

use std::collections::HashSet;

use crate::error::MattenDataError;
use crate::schema::SchemaSummary;
use crate::{numeric, schema};

/// A single table cell value (RFC-034 §4.2).
///
/// `CellValue` is **intentionally crate-local** (architect ruling, RFC-033–042
/// review Q4): it models table-ingestion cells, not core `Tensor` dynamic values,
/// and is distinct from `matten::Element` (it is not an alias). `Text` holds an
/// owned `String` (core `Element::Text` uses `Arc<str>`); the representation is a
/// local, practical choice that can change without affecting the public model.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CellValue {
    /// Free text that did not parse as a number or boolean.
    Text(String),
    /// A floating-point value.
    Float(f64),
    /// An integer value.
    Int(i64),
    /// A boolean (`true` / `false`).
    Bool(bool),
    /// A missing / empty cell.
    Missing,
}

impl From<f64> for CellValue {
    fn from(v: f64) -> Self {
        CellValue::Float(v)
    }
}
impl From<i64> for CellValue {
    fn from(v: i64) -> Self {
        CellValue::Int(v)
    }
}
impl From<bool> for CellValue {
    fn from(v: bool) -> Self {
        CellValue::Bool(v)
    }
}
impl From<&str> for CellValue {
    fn from(v: &str) -> Self {
        CellValue::Text(v.to_string())
    }
}
impl From<String> for CellValue {
    fn from(v: String) -> Self {
        CellValue::Text(v)
    }
}

/// A small, owned, rectangular table-like data set.
///
/// External guarantees: row order is preserved; column order is preserved; column
/// names are stable after loading. Operations return new owned `Table` values; no
/// borrowed view lifetimes appear in normal use.
#[derive(Debug, Clone)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<CellValue>>,
}

impl Table {
    /// Construct a `Table` from validated parts. Internal: callers (CSV ingestion,
    /// selection, fill) guarantee that every row has `headers.len()` cells.
    pub(crate) fn from_parts(headers: Vec<String>, rows: Vec<Vec<CellValue>>) -> Self {
        Table { headers, rows }
    }

    /// Number of data rows.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Number of columns.
    pub fn column_count(&self) -> usize {
        self.headers.len()
    }

    /// Column names, in column order.
    pub fn column_names(&self) -> &[String] {
        &self.headers
    }

    /// A small, displayable schema summary (row/column counts, per-column missing
    /// counts and inferred kinds). Does not perform expensive analysis.
    pub fn schema_summary(&self) -> SchemaSummary {
        schema::summarize(self)
    }

    /// Select columns by name, returning a new `Table`.
    ///
    /// Behavior (RFC-034 §5.3): preserves the requested column order; errors with
    /// [`MattenDataError::MissingColumn`] if a requested column does not exist;
    /// rejects duplicate selections with [`MattenDataError::DuplicateSelection`];
    /// an empty selection is [`MattenDataError::EmptySelection`].
    pub fn select_columns<I, S>(&self, columns: I) -> Result<Table, MattenDataError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let requested: Vec<String> = columns
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        if requested.is_empty() {
            return Err(MattenDataError::EmptySelection);
        }

        let mut seen = HashSet::with_capacity(requested.len());
        for name in &requested {
            if !seen.insert(name.as_str()) {
                return Err(MattenDataError::DuplicateSelection { name: name.clone() });
            }
        }

        let mut indices = Vec::with_capacity(requested.len());
        for name in &requested {
            match self.headers.iter().position(|h| h == name) {
                Some(idx) => indices.push(idx),
                None => return Err(MattenDataError::MissingColumn { name: name.clone() }),
            }
        }

        let rows = self
            .rows
            .iter()
            .map(|row| indices.iter().map(|&i| row[i].clone()).collect())
            .collect();

        Ok(Table::from_parts(requested, rows))
    }

    /// Fill every missing cell with `value`, returning a new `Table`.
    ///
    /// Missing values are never silently turned into zero; filling is always
    /// explicit (RFC-035 §6). Non-missing cells and the shape are unchanged.
    pub fn fill_missing(&self, value: impl Into<CellValue>) -> Result<Table, MattenDataError> {
        let fill = value.into();
        let rows = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| {
                        if matches!(cell, CellValue::Missing) {
                            fill.clone()
                        } else {
                            cell.clone()
                        }
                    })
                    .collect()
            })
            .collect();
        Ok(Table::from_parts(self.headers.clone(), rows))
    }

    /// Convert the table to an explicit numeric table (RFC-035 §7).
    ///
    /// Strict conversion: `Int`/`Float` become `f64`; `Bool` and `Text` are
    /// rejected ([`MattenDataError::NonNumericValue`]); a remaining `Missing` cell
    /// is rejected ([`MattenDataError::MissingValue`]). Text is never parsed as a
    /// number by default.
    pub fn try_numeric(&self) -> Result<numeric::NumericTable, MattenDataError> {
        numeric::try_numeric(self)
    }

    // --- crate-internal accessors used by sibling modules ---

    pub(crate) fn headers(&self) -> &[String] {
        &self.headers
    }

    pub(crate) fn rows(&self) -> &[Vec<CellValue>] {
        &self.rows
    }
}
