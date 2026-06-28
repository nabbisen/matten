//! The complete dynamic-to-numeric on-ramp workflow.
//!
//! Run: cargo run --example dynamic_07_on_ramp_summary --features dynamic
//!
//! This example shows the recommended lifecycle for messy data:
//!
//!   messy input
//!     → ingest as dynamic tensor
//!     → inspect: schema_summary, numeric_mask, is_numeric_convertible
//!     → clean: fill_none, forward_fill_none
//!     → convert: try_numeric (or try_numeric_with for policy)
//!     → numeric tensor computation

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, NumericPolicy, Tensor};

    // ── Step 1: Ingest messy data ─────────────────────────────────────────
    // Simulate a row that arrived from a messy source (CSV or JSON) with some
    // missing values. The on-ramp is the same whichever format it came from.
    let raw = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::None,
            Element::Float(3.0),
            Element::Int(4),
            Element::None,
        ],
        &[5],
    );

    // ── Step 2: Inspect ───────────────────────────────────────────────────
    println!("Schema:               {}", raw.schema_summary());
    println!("None count:           {}", raw.count_none());
    println!("Numeric convertible:  {}", raw.is_numeric_convertible());

    let nmask = raw.numeric_mask();
    println!("Numeric mask:         {:?}", nmask.as_slice());
    // → [1.0, 0.0, 1.0, 1.0, 0.0]  (None slots are 0.0)

    assert!(!raw.is_numeric_convertible());

    // ── Step 3: Clean ─────────────────────────────────────────────────────
    let filled = raw.fill_none(0.0);
    println!(
        "After fill_none(0.0): {:?}",
        filled
            .to_elements()
            .iter()
            .map(|e| format!("{e:?}"))
            .collect::<Vec<_>>()
    );

    assert!(filled.is_numeric_convertible());

    // ── Step 4: Convert ───────────────────────────────────────────────────
    let numeric = filled.try_numeric()?;
    assert_eq!(numeric.shape(), &[5]);
    println!("Numeric tensor:       {:?}", numeric.as_slice());

    // ── Step 5: Numeric computation ───────────────────────────────────────
    let scaled = &numeric * 2.0;
    println!("Scaled (×2):          {:?}", scaled.as_slice());
    assert_eq!(scaled.as_slice(), &[2.0, 0.0, 6.0, 8.0, 0.0]);

    // ── Alternative: policy-based conversion ─────────────────────────────
    let numeric2 = raw.try_numeric_with(NumericPolicy::default().none_as(0.0))?;
    assert_eq!(numeric.as_slice(), numeric2.as_slice());
    println!("Policy shortcut gives same result: yes");

    println!("done.");
    Ok(())
}
