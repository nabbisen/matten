//! Explicit numeric coercion policy demonstration.
//!
//! Run: cargo run --example dynamic_04_numeric_coercion --features dynamic
//!
//! matten does NOT silently coerce Bool, Text, or None to numbers.
//! Coercion must be explicit. This example shows the policy and how to
//! work within it.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, Tensor};

    // ── Allowed: Float and Int coerce to f64 ─────────────────────────────
    assert_eq!(Element::Float(1.5).try_as_f64(), Some(1.5));
    assert_eq!(Element::Int(42).try_as_f64(), Some(42.0));
    println!("Float/Int -> f64: OK");

    // ── Not allowed silently: Bool, Text, None ───────────────────────────
    assert_eq!(Element::Bool(true).try_as_f64(), None);
    assert_eq!(Element::text("3.14").try_as_f64(), None);
    assert_eq!(Element::None.try_as_f64(), None);
    println!("Bool/Text/None -> f64: None (no silent coercion)");

    // ── Mixed tensor with Int and Float: try_numeric succeeds ────────────
    let mixed_numeric = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::Int(2),
            Element::Int(3),
            Element::Float(4.0),
        ],
        &[2, 2],
    );
    let n = mixed_numeric.try_numeric()?;
    println!("Int+Float -> numeric: {:?}", n.as_slice()); // [1.0, 2.0, 3.0, 4.0]

    // ── Mixed with Bool: try_numeric fails, fill needed ──────────────────
    let with_bool = Tensor::from_elements(vec![Element::Float(1.0), Element::Bool(true)], &[2]);
    match with_bool.try_numeric() {
        Err(e) => println!("Bool blocks try_numeric: {} (expected)", e),
        Ok(_) => panic!("should have failed"),
    }

    // ── Explicit coercion pattern: map Bool manually ──────────────────────
    let data: Vec<Element> = with_bool
        .to_elements()
        .into_iter()
        .map(|e| match e {
            Element::Bool(b) => Element::Float(if b { 1.0 } else { 0.0 }),
            other => other,
        })
        .collect();
    let shape = with_bool.shape().to_vec();
    let coerced = Tensor::from_elements(data, &shape);
    let n2 = coerced.try_numeric()?;
    println!("After explicit Bool->Float: {:?}", n2.as_slice()); // [1.0, 1.0]

    println!("All assertions passed.");
    Ok(())
}
