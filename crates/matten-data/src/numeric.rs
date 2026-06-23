//! Explicit numeric conversion to a [`matten::Tensor`] (RFC-035 §7–8).
//!
//! Conversion is always explicit (`try_numeric()` then `to_tensor()`); there is no
//! silent coercion. Missing values never become zero, booleans never become 1/0,
//! and non-numeric text is rejected.

use crate::error::MattenDataError;
use crate::table::{CellValue, Table};

/// A table whose cells have all been validated as numeric (RFC-034 §4.4).
///
/// Produced by [`Table::try_numeric`]. Holds row-major `f64` data ready to become
/// a [`matten::Tensor`] via [`NumericTable::to_tensor`].
#[derive(Debug, Clone)]
pub struct NumericTable {
    headers: Vec<String>,
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl NumericTable {
    /// Number of rows.
    pub fn row_count(&self) -> usize {
        self.rows
    }

    /// Number of columns.
    pub fn column_count(&self) -> usize {
        self.cols
    }

    /// Column names, in column order.
    pub fn column_names(&self) -> &[String] {
        &self.headers
    }

    /// Build a numeric [`matten::Tensor`] with shape `[rows, columns]`, row-major.
    ///
    /// Errors with [`MattenDataError::EmptySelection`] if there are no columns, and
    /// wraps any core construction failure (for example a zero-length dimension
    /// when there are no rows) as [`MattenDataError::Matten`].
    pub fn to_tensor(&self) -> Result<matten::Tensor, MattenDataError> {
        if self.cols == 0 {
            return Err(MattenDataError::EmptySelection);
        }
        matten::Tensor::try_new(self.data.clone(), &[self.rows, self.cols])
            .map_err(MattenDataError::Matten)
    }
}

/// Strictly convert a table's cells to `f64` (RFC-035 §7.1–7.4).
pub(crate) fn try_numeric(table: &Table) -> Result<NumericTable, MattenDataError> {
    let headers: Vec<String> = table.headers().to_vec();
    let cols = headers.len();
    let table_rows = table.rows();
    let rows = table_rows.len();

    let mut data = Vec::with_capacity(rows * cols);
    for (r, row) in table_rows.iter().enumerate() {
        // One-based CSV line number: header is line 1, first data row is line 2.
        let line = r + 2;
        for (c, cell) in row.iter().enumerate() {
            let value = match cell {
                CellValue::Int(i) => *i as f64,
                CellValue::Float(f) => *f,
                CellValue::Missing => {
                    return Err(MattenDataError::MissingValue {
                        column: headers[c].clone(),
                        row: line,
                    });
                }
                CellValue::Bool(b) => {
                    return Err(MattenDataError::NonNumericValue {
                        column: headers[c].clone(),
                        row: line,
                        value: b.to_string(),
                    });
                }
                CellValue::Text(s) => {
                    return Err(MattenDataError::NonNumericValue {
                        column: headers[c].clone(),
                        row: line,
                        value: s.clone(),
                    });
                }
            };
            data.push(value);
        }
    }

    Ok(NumericTable {
        headers,
        data,
        rows,
        cols,
    })
}
