//! `matten-data` — a tiny table-to-Tensor preparation companion for small PoC
//! datasets.
//!
//! # Status
//!
//! **Experimental (scaffold).** This crate is an approved, scope-locked companion
//! (RFC-033) but currently contains no public API. Table ingestion and conversion
//! land in later releases once RFC-034 (table model) and RFC-035 (CSV ingestion and
//! numeric conversion) are accepted and implemented. Maturity is expressed by this
//! Status label, not by the crate version: under lock-step family versioning
//! (RFC-030) the crate shares the workspace family version.
//!
//! # What it will be
//!
//! A small helper for the boring step between table-like input and a numeric
//! [`matten::Tensor`]:
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
//! # What it is not
//!
//! `matten-data` is **not a dataframe library**. It has no joins, group-by, pivot,
//! query DSL, lazy execution, indexing/`loc`/`iloc`, rolling/window operations,
//! datetime engine, categorical dtype system, or large-data streaming. For those
//! workloads use [Polars](https://pola.rs), [DataFusion](https://datafusion.apache.org),
//! Pandas, or another dataframe/query tool.
//!
//! # Relationship to core `dynamic`
//!
//! Core `matten`'s `dynamic` feature is *value-level* ingestion (mixed values inside
//! a `Tensor`, with explicit `try_numeric()`). `matten-data` is *table-level*
//! preparation (headers, named columns, schema summary, table-shaped missing-value
//! policy) whose end goal is a numeric `Tensor`. `matten-data` may use core
//! `dynamic` internally, but it does not expose a second computation engine.
//!
//! # Dependency direction
//!
//! `matten-data` depends on core `matten`; core `matten` never depends on
//! `matten-data`. The dependency-boundary CI check enforces this.

#![forbid(unsafe_code)]

// Reserved module boundaries for later RFCs. These are intentionally private and
// empty in the scaffold; they are filled and selectively exposed only when their
// RFCs are accepted and implemented. Do not add public API here in the scaffold.
mod csv; // RFC-035: CSV ingestion
mod error; // RFC-034 / RFC-035: MattenDataError
mod numeric; // RFC-035: numeric conversion -> NumericTable -> Tensor
mod schema; // RFC-034 / RFC-035: SchemaSummary / ColumnKind
mod table; // RFC-034: Table model
