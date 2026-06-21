//! Demonstrating Result-zone APIs: malformed input returns Err, never panics.
//!
//! Run: cargo run --example 12_boundary_error_handling
//!
//! All external-input APIs in `matten` are Result-zone. This example shows
//! the kinds of errors you can expect and how to handle them.

use matten::{MattenError, Tensor};

fn main() {
    // ── shape/data mismatch ──────────────────────────────────────────────
    match Tensor::try_new(vec![1.0, 2.0], &[3]) {
        Err(MattenError::Shape { operation, message }) => {
            println!("[Shape]  {operation}: {message}")
        }
        other => println!("{other:?}"),
    }

    // ── malformed JSON ───────────────────────────────────────────────────
    match Tensor::from_json(r#"[[1.0,"text"]]"#) {
        Err(MattenError::Parse { format, message }) => println!("[Parse/{format}]  {message}"),
        other => println!("{other:?}"),
    }

    // ── ragged JSON array ────────────────────────────────────────────────
    match Tensor::from_json("[[1.0,2.0],[3.0]]") {
        Err(MattenError::Parse { format, message }) => println!("[Parse/{format}]  {message}"),
        other => println!("{other:?}"),
    }

    // ── non-numeric CSV field ────────────────────────────────────────────
    match Tensor::load_csv("examples/data/malformed_numeric.csv") {
        Err(MattenError::Parse { format, message }) => println!("[Parse/{format}]  {message}"),
        other => println!("{other:?}"),
    }

    // ── missing file ─────────────────────────────────────────────────────
    match Tensor::load_json("/no/such/file.json") {
        Err(MattenError::Io { path, source }) => println!("[Io]  {}: {source}", path.display()),
        other => println!("{other:?}"),
    }

    // ── slice out of range ───────────────────────────────────────────────
    let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
    match t.slice().index(5).all().build() {
        Err(MattenError::Slice { message, .. }) => println!("[Slice]  {message}"),
        other => println!("{other:?}"),
    }

    println!("All boundary errors handled gracefully — no panics.");
}
