//! End-to-end messy CSV ingestion, cleaning, and numeric conversion.
//!
//! Run: cargo run --example dynamic_05_dirty_csv_cleanup --features dynamic,csv

#[cfg(not(all(feature = "dynamic", feature = "csv")))]
fn main() {
    println!("Requires --features dynamic,csv");
}

#[cfg(all(feature = "dynamic", feature = "csv"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, Tensor};

    // CSV without header row (dynamic parser reads all rows as data)
    let csv = "85.0,active,1\n92.0,,2\n,inactive,1\n78.0,active,\n";

    // 1. Ingest as dynamic tensor
    let raw = Tensor::from_csv_dynamic(csv)?;
    assert_eq!(raw.shape(), &[4, 3]);
    assert_eq!(raw.count_none(), 3);
    println!(
        "raw shape={:?}, none_count={}",
        raw.shape(),
        raw.count_none()
    );

    // 2. Inspect
    assert_eq!(raw.get_element(&[0, 0]), Some(Element::Float(85.0)));
    assert_eq!(raw.get_element(&[0, 1]), Some(Element::text("active")));
    assert_eq!(raw.get_element(&[1, 1]), Some(Element::None));
    println!("element access OK");

    // 3. Extract numeric columns by get_element
    let n_rows = raw.shape()[0];
    let scores_data: Vec<Element> = (0..n_rows)
        .map(|r| raw.get_element(&[r, 0]).unwrap_or(Element::None))
        .collect();
    let weights_data: Vec<Element> = (0..n_rows)
        .map(|r| raw.get_element(&[r, 2]).unwrap_or(Element::None))
        .collect();
    let scores_raw = Tensor::from_elements(scores_data, &[n_rows]);
    let weights_raw = Tensor::from_elements(weights_data, &[n_rows]);
    assert_eq!(scores_raw.len(), 4);
    assert_eq!(weights_raw.len(), 4);

    // 4. Fill missing values
    let scores = scores_raw.fill_none(Element::Float(0.0));
    let weights = weights_raw.fill_none(Element::Float(1.0));
    assert_eq!(scores.count_none(), 0);
    assert_eq!(weights.count_none(), 0);
    println!("fill_none OK");

    // 5. Convert to numeric Tensor values
    let s = scores.try_numeric()?;
    let w = weights.try_numeric()?;
    assert_eq!(s.as_slice(), &[85.0, 92.0, 0.0, 78.0]);
    assert_eq!(w.as_slice(), &[1.0, 2.0, 1.0, 1.0]);
    println!("try_numeric OK: scores={:?}", s.as_slice());

    // 6. numeric Tensor arithmetic: weighted average
    let weighted_sum = (&s * &w).sum();
    let weight_sum = w.sum();
    let weighted_avg = weighted_sum / weight_sum;
    assert!((weighted_avg - (85.0 + 92.0 * 2.0 + 0.0 + 78.0) / 5.0).abs() < 1e-9);
    println!("weighted average score = {:.2}", weighted_avg);

    println!("Done — messy CSV cleaned and processed. All assertions passed.");
    Ok(())
}
