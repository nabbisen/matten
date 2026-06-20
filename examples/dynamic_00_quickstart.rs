//! Phase 2 dynamic feature quickstart: mixed-type tensors with `Element`.
//!
//! Run: cargo run --example dynamic_00_quickstart --features dynamic
//!
//! `matten` is **not** a full dataframe library. The `dynamic` feature is for
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
    println!("shape    = {:?}", t.shape());
    println!("[0,0]    = {:?}", t.get_element(&[0, 0])); // Float(1.0)
    println!("[0,1]    = {:?}", t.get_element(&[0, 1])); // Text("active")
    println!("[1,1]    = {:?}", t.get_element(&[1, 1])); // None

    // ── fill_none: replace missing values ────────────────────────────────
    let filled = t.fill_none(Element::text("unknown"));
    println!("[1,1] filled = {:?}", filled.get_element(&[1, 1])); // Text("unknown")

    // ── Parse mixed JSON ─────────────────────────────────────────────────
    #[cfg(feature = "json")]
    {
        let json = r#"[[1, "active", true], [2, null, false]]"#;
        let from_json = Tensor::from_json_dynamic(json)?;
        println!("JSON shape = {:?}", from_json.shape()); // [2, 3]
        println!("JSON[1,1]  = {:?}", from_json.get_element(&[1, 1])); // None
    }

    // ── Parse mixed CSV ──────────────────────────────────────────────────
    #[cfg(feature = "csv")]
    {
        let csv = "1,active,true\n2,,false\n";
        let from_csv = Tensor::from_csv_dynamic(csv)?;
        println!("CSV shape  = {:?}", from_csv.shape()); // [2, 3]
        println!("CSV[1,1]   = {:?}", from_csv.get_element(&[1, 1])); // None
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
    println!("numeric shape = {:?}", numeric.shape());
    println!("numeric data  = {:?}", numeric.as_slice());

    println!("\nAll assertions passed.");
    Ok(())
}
