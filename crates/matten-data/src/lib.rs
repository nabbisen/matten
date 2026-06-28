//! `matten-data` — a tiny table-to-Tensor preparation companion for small PoC
//! datasets.
//!
//! # Status
//!
//! **Production-ready candidate.** This is a scope-locked companion (RFC-033) for the boring
//! step between table-like input and a numeric [`matten::Tensor`]. The API is
//! mostly stable but pre-1.0; pin the minor version. Under lock-step family versioning
//! (RFC-030) the crate shares the workspace family version; maturity is the Status
//! label, not the version number.
//!
//! # The workflow
//!
//! ```text
//! small CSV / table-like data
//!   -> inspect schema
//!   -> select columns by name
//!   -> clean missing values explicitly
//!   -> convert to numeric explicitly
//!   -> matten::Tensor
//! ```
//!
//! ```
//! # #[cfg(not(feature = "csv"))] fn main() {}
//! # #[cfg(feature = "csv")] fn main() -> Result<(), matten_data::MattenDataError> {
//! use matten_data::Table;
//!
//! let csv = "sales,cost,note\n10,2,a\n20,,b\n30,4,c";
//! let table = Table::from_csv_str(csv)?;
//!
//! // Inspect, select, clean, convert — every step explicit.
//! let tensor = table
//!     .select_columns(["sales", "cost"])?
//!     .fill_missing(0.0)?
//!     .try_numeric()?
//!     .to_tensor()?;
//!
//! assert_eq!(tensor.shape(), &[3, 2]);
//! assert_eq!(tensor.as_slice(), &[10.0, 2.0, 20.0, 0.0, 30.0, 4.0]);
//! # Ok(())
//! # }
//! ```
//!
//! # What it is not
//!
//! `matten-data` is **not a dataframe library**. It has no joins, group-by, pivot,
//! query DSL, lazy execution, indexing/`loc`/`iloc`, rolling/window operations,
//! datetime engine, categorical dtype system, or large-data streaming. For those
//! workloads use [Polars](https://pola.rs), [DataFusion](https://datafusion.apache.org),
//! Pandas, or another dataframe/query tool. It is a small conversion helper for
//! application-validated or trusted data, not a CSV firewall or input sandbox.
//!
//! # Relationship to core `dynamic`
//!
//! Core `matten`'s `dynamic` feature is *value-level* ingestion (mixed values
//! inside a `Tensor`, with explicit `try_numeric()`). `matten-data` is *table-level*
//! preparation (headers, named columns, schema summary, table-shaped missing-value
//! policy) whose end goal is a numeric `Tensor`. It does not expose a second
//! computation engine.
//!
//! # Conversion rules
//!
//! Numeric conversion is strict and explicit (`try_numeric` then `to_tensor`):
//! integers and floats become `f64`; booleans and non-numeric text are rejected;
//! a remaining missing cell is rejected (fill it first). Missing values never
//! silently become zero, and booleans never silently become `1`/`0`.

#![forbid(unsafe_code)]

#[cfg(feature = "csv")]
mod csv;
mod error;
mod numeric;
mod schema;
mod table;

#[cfg(all(test, feature = "csv"))]
mod tests;

pub use error::MattenDataError;
pub use numeric::NumericTable;
pub use schema::{ColumnKind, ColumnSummary, SchemaSummary};
pub use table::{CellValue, Table};
