//! Phase 2 dynamic feature quickstart: mixed-type tensors with `Element`.
//!
//! Run: cargo run --example dynamic_00_quickstart --features dynamic
//!
//! `matten` is not a full dataframe library. The `dynamic` feature is for
//! ingesting and cleaning messy PoC data before converting to numeric tensors
//! or handing off to a specialised crate.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("This example requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, Tensor};

    // ── Build a mixed tensor directly ────────────────────────────────────
    let data = vec![
        Element::Float(1.0),
        Element::text("active"),
        Element::Bool(true),
        Element::Int(2),
        Element::None,
        Element::Bool(false),
    ];
    let t = Tensor::from_elements(data, &[2, 3]);
    assert_eq!(t.shape(), &[2, 3]);
    assert_eq!(t.len(), 6);
    assert!(t.is_dynamic());
    assert_eq!(t.get_element(&[0, 0]), Some(Element::Float(1.0)));
    assert_eq!(t.get_element(&[0, 1]), Some(Element::text("active")));
    assert_eq!(t.get_element(&[1, 1]), Some(Element::None));
    println!("construction OK: shape={:?}, len={}", t.shape(), t.len());

    // ── fill_none: replace missing values ────────────────────────────────
    let filled = t.fill_none(Element::text("unknown"));
    assert_eq!(filled.get_element(&[1, 1]), Some(Element::text("unknown")));
    assert_eq!(t.get_element(&[1, 1]), Some(Element::None)); // original unchanged
    println!("fill_none OK");

    // ── Parse mixed JSON ─────────────────────────────────────────────────
    #[cfg(feature = "json")]
    {
        let from_json = Tensor::from_json_dynamic(r#"[[1, "active", true], [2, null, false]]"#)?;
        assert_eq!(from_json.shape(), &[2, 3]);
        assert_eq!(from_json.get_element(&[1, 1]), Some(Element::None));
        println!("from_json_dynamic OK: shape={:?}", from_json.shape());
    }

    // ── Parse mixed CSV ──────────────────────────────────────────────────
    #[cfg(feature = "csv")]
    {
        let from_csv = Tensor::from_csv_dynamic("1,active,true\n2,,false\n")?;
        assert_eq!(from_csv.shape(), &[2, 3]);
        assert_eq!(from_csv.get_element(&[1, 1]), Some(Element::None));
        println!("from_csv_dynamic OK: shape={:?}", from_csv.shape());
    }

    // ── Convert to numeric tensor after cleanup ───────────────────────────
    let numeric_data = vec![
        Element::Float(1.0),
        Element::Int(2),
        Element::Float(3.0),
        Element::Int(4),
    ];
    let dyn_t = Tensor::from_elements(numeric_data, &[2, 2]);
    let numeric = dyn_t.try_numeric()?;
    assert_eq!(numeric.shape(), &[2, 2]);
    assert_eq!(numeric.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
    println!("try_numeric OK: {:?}", numeric.as_slice());

    println!("\nAll assertions passed.");
    Ok(())
}
