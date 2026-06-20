//! Explicit numeric conversion policy (RFC-017): `NumericPolicy` and
//! `try_numeric_with`.
//!
//! Run: cargo run --example dynamic_06_numeric_policy --features dynamic
//!
//! The default `try_numeric()` is strict: only `Float` and `Int` elements
//! are accepted. For messier data, `try_numeric_with(policy)` lets you
//! choose how `Bool`, `Text`, and `None` values are handled.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use matten::{Element, NumericPolicy, Tensor};

    let raw = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::None,
            Element::Bool(true),
            Element::text("3.5"),
            Element::Int(4),
        ],
        &[5],
    );

    println!("Schema: {}", raw.schema_summary());

    // ── Strict default: only Float and Int ───────────────────────────────
    assert!(raw.try_numeric().is_err(), "strict rejects None/Bool/Text");
    println!("strict try_numeric(): Err (expected)");

    // ── none_as: treat None as 0.0 ───────────────────────────────────────
    let with_none = Tensor::from_elements(
        vec![Element::Float(1.0), Element::None, Element::Float(3.0)],
        &[3],
    );
    let x = with_none.try_numeric_with(NumericPolicy::default().none_as(0.0))?;
    assert_eq!(x.as_slice(), &[1.0, 0.0, 3.0]);
    println!("none_as(0.0):  {:?}", x.as_slice());

    // ── allow_bool: true → 1.0, false → 0.0 ─────────────────────────────
    let with_bool = Tensor::from_elements(vec![Element::Bool(true), Element::Bool(false)], &[2]);
    let x = with_bool.try_numeric_with(NumericPolicy::default().allow_bool())?;
    assert_eq!(x.as_slice(), &[1.0, 0.0]);
    println!("allow_bool():  {:?}", x.as_slice());

    // ── allow_text_parse: parse numeric strings ───────────────────────────
    let with_text = Tensor::from_elements(vec![Element::text("42.5")], &[1]);
    let x = with_text.try_numeric_with(NumericPolicy::default().allow_text_parse())?;
    assert!((x.as_slice()[0] - 42.5_f64).abs() < 1e-10);
    println!("allow_text_parse(): {:?}", x.as_slice());

    // ── permissive: all variants accepted ────────────────────────────────
    let x = raw.try_numeric_with(NumericPolicy::permissive())?;
    assert_eq!(x.shape(), &[5]);
    println!("permissive: {:?}", x.as_slice());

    // ── Chained: none_as_nan + allow_bool ────────────────────────────────
    let mixed = Tensor::from_elements(
        vec![Element::Float(1.0), Element::None, Element::Bool(false)],
        &[3],
    );
    let x = mixed.try_numeric_with(NumericPolicy::default().none_as_nan().allow_bool())?;
    assert_eq!(x.as_slice()[0], 1.0);
    assert!(x.as_slice()[1].is_nan());
    assert_eq!(x.as_slice()[2], 0.0);
    println!(
        "none_as_nan + allow_bool: first={}, second=NaN, third={}",
        x.as_slice()[0],
        x.as_slice()[2]
    );

    println!("done.");
    Ok(())
}
