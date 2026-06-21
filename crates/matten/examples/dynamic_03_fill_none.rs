//! Filling missing values before numeric processing.
//!
//! Run: cargo run --example dynamic_03_fill_none --features dynamic
//!
//! Always clean None values before arithmetic. Use fill_none for constant
//! fill or forward_fill_none to carry the last known value forward.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, Tensor};

    let raw = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::None,
            Element::Float(3.0),
            Element::None,
            Element::Float(5.0),
            Element::None,
        ],
        &[2, 3],
    );

    println!("raw none count = {}", raw.count_none()); // 3

    // ── Constant fill ────────────────────────────────────────────────────
    let filled = raw.fill_none(Element::Float(0.0));
    assert_eq!(filled.count_none(), 0);
    println!(
        "fill_none(0.0): {:?}",
        filled
            .to_elements()
            .iter()
            .map(|e| e.try_as_f64().unwrap())
            .collect::<Vec<_>>()
    );

    // ── Forward fill ─────────────────────────────────────────────────────
    let fwd = raw.forward_fill_none(Element::Float(-1.0)); // -1.0 for leading Nones
    println!(
        "forward_fill:  {:?}",
        fwd.to_elements()
            .iter()
            .map(|e| e.try_as_f64().unwrap())
            .collect::<Vec<_>>()
    );
    // [1.0, 1.0, 3.0, 3.0, 5.0, 5.0]
    assert_eq!(fwd.get_element(&[0, 1]), Some(Element::Float(1.0)));
    assert_eq!(fwd.get_element(&[1, 0]), Some(Element::Float(3.0)));

    // ── Convert to numeric tensor for arithmetic ─────────────────────────
    let numeric = filled.try_numeric()?;
    println!("numeric sum = {}", numeric.sum());

    println!("All assertions passed.");
    Ok(())
}
