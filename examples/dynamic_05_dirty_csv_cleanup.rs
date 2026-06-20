//! End-to-end messy CSV ingestion, cleaning, and numeric conversion.
//!
//! Run: cargo run --example dynamic_05_dirty_csv_cleanup --features dynamic,csv
//!
//! This is the primary use case for the `dynamic` feature: ingest messy
//! business data, inspect it, clean it, then hand off to Phase 1 arithmetic.
//! `matten` is not a dataframe — no joins, group-by, or SQL queries.

#[cfg(not(all(feature = "dynamic", feature = "csv")))]
fn main() {
    println!("Requires --features dynamic,csv");
}

#[cfg(all(feature = "dynamic", feature = "csv"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, Tensor};

    // Messy CSV: mixed types, missing values, non-numeric labels
    // CSV without header row (dynamic parser reads all rows as data)
    let csv = "\
85.0,active,1
92.0,,2
,inactive,1
78.0,active,
";

    // 1. Ingest as dynamic tensor
    let raw = Tensor::from_csv_dynamic(csv)?;
    println!("raw shape      = {:?}", raw.shape()); // [4, 3]
    println!("none count     = {}", raw.count_none());

    // 2. Inspect what we got
    println!("[0,0] = {:?}", raw.get_element(&[0, 0])); // Float(85.0)
    println!("[0,1] = {:?}", raw.get_element(&[0, 1])); // Text("active")
    println!("[1,1] = {:?}", raw.get_element(&[1, 1])); // None (empty field)

    // 3. Extract columns by collecting elements row by row
    //    (dynamic slicing via builder is Phase 1 only; use get_element for dynamic)
    let n_rows = raw.shape()[0];
    let scores_data: Vec<Element> = (0..n_rows)
        .map(|r| raw.get_element(&[r, 0]).unwrap_or(Element::None))
        .collect();
    let weights_data: Vec<Element> = (0..n_rows)
        .map(|r| raw.get_element(&[r, 2]).unwrap_or(Element::None))
        .collect();
    let scores_raw = Tensor::from_elements(scores_data, &[n_rows]);
    let weights_raw = Tensor::from_elements(weights_data, &[n_rows]);

    println!("scores:  {:?}", scores_raw.to_elements());
    println!("weights: {:?}", weights_raw.to_elements());

    // 4. Fill missing numeric values with a fallback
    let scores = scores_raw.fill_none(Element::Float(0.0));
    let weights = weights_raw.fill_none(Element::Float(1.0));

    // 5. Convert to Phase 1 numeric tensors
    let s = scores.try_numeric()?;
    let w = weights.try_numeric()?;
    println!("scores  = {:?}", s.as_slice()); // [85.0, 92.0, 0.0, 78.0]
    println!("weights = {:?}", w.as_slice()); // [1.0, 2.0, 1.0, 1.0]

    // 6. Phase 1 arithmetic: weighted average
    let weighted_sum = (&s * &w).sum();
    let weight_sum = w.sum();
    let weighted_avg = weighted_sum / weight_sum;
    println!("weighted average score = {:.2}", weighted_avg);

    println!("Done — messy CSV cleaned and processed.");
    Ok(())
}
