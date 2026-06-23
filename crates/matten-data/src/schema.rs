//! Schema summary and column-kind inference (RFC-034 §4.3, RFC-035 §5).
//!
//! `schema_summary()` helps a user decide what to select and convert. It reports
//! row/column counts, per-column names, missing counts, and a simple inferred
//! [`ColumnKind`]. It does not perform expensive dataframe analysis.

use std::fmt;

use crate::table::{CellValue, Table};

/// A simple, inferred kind for a column (RFC-035 §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ColumnKind {
    /// All non-missing cells are integers.
    Integer,
    /// All non-missing cells are numeric (integers and/or floats).
    Float,
    /// All non-missing cells are booleans.
    Boolean,
    /// All non-missing cells are text.
    Text,
    /// A mix of incompatible kinds (for example text and numbers).
    Mixed,
    /// Every cell in the column is missing.
    MissingOnly,
}

impl fmt::Display for ColumnKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ColumnKind::Integer => "integer",
            ColumnKind::Float => "float",
            ColumnKind::Boolean => "boolean",
            ColumnKind::Text => "text",
            ColumnKind::Mixed => "mixed",
            ColumnKind::MissingOnly => "missing-only",
        };
        f.write_str(s)
    }
}

/// Per-column entry in a [`SchemaSummary`].
#[derive(Debug, Clone)]
pub struct ColumnSummary {
    /// Column name.
    pub name: String,
    /// Inferred simple kind.
    pub kind: ColumnKind,
    /// Number of missing cells in the column.
    pub missing: usize,
}

/// A small, displayable description of a [`Table`]'s columns.
#[derive(Debug, Clone)]
pub struct SchemaSummary {
    /// Number of data rows.
    pub rows: usize,
    /// Number of columns.
    pub columns: usize,
    per_column: Vec<ColumnSummary>,
}

impl SchemaSummary {
    /// Per-column summaries, in column order.
    pub fn column_summaries(&self) -> &[ColumnSummary] {
        &self.per_column
    }
}

impl fmt::Display for SchemaSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Table: {} rows x {} columns", self.rows, self.columns)?;
        for col in &self.per_column {
            writeln!(
                f,
                "  - {} ({}, {} missing)",
                col.name, col.kind, col.missing
            )?;
        }
        Ok(())
    }
}

/// Compute a [`SchemaSummary`] for a table.
pub(crate) fn summarize(table: &Table) -> SchemaSummary {
    let headers = table.headers();
    let rows = table.rows();

    let per_column = headers
        .iter()
        .enumerate()
        .map(|(c, name)| {
            let mut missing = 0usize;
            let mut has_int = false;
            let mut has_float = false;
            let mut has_bool = false;
            let mut has_text = false;

            for row in rows {
                match &row[c] {
                    CellValue::Missing => missing += 1,
                    CellValue::Int(_) => has_int = true,
                    CellValue::Float(_) => has_float = true,
                    CellValue::Bool(_) => has_bool = true,
                    CellValue::Text(_) => has_text = true,
                }
            }

            let kind = infer_kind(has_int, has_float, has_bool, has_text);
            ColumnSummary {
                name: name.clone(),
                kind,
                missing,
            }
        })
        .collect();

    SchemaSummary {
        rows: rows.len(),
        columns: headers.len(),
        per_column,
    }
}

fn infer_kind(has_int: bool, has_float: bool, has_bool: bool, has_text: bool) -> ColumnKind {
    let numeric = has_int || has_float;
    let categories = [numeric, has_bool, has_text].iter().filter(|&&b| b).count();

    match categories {
        0 => ColumnKind::MissingOnly,
        1 => {
            if has_bool {
                ColumnKind::Boolean
            } else if has_text {
                ColumnKind::Text
            } else if has_float {
                ColumnKind::Float
            } else {
                ColumnKind::Integer
            }
        }
        _ => ColumnKind::Mixed,
    }
}
