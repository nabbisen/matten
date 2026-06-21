//! JSON serialisation and deserialisation round-trip.
//!
//! Run: cargo run --example 10_json_roundtrip
//!
//! Two accepted forms:
//!   • canonical object  {"shape":[…],"data":[…]}  (preferred)
//!   • convenience nested arrays  [[1,2],[3,4]]  (rank 1 and 2 only)

use matten::Tensor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── serde round-trip ────────────────────────────────────────────────
    let original = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let json = serde_json::to_string(&original)?;
    println!("serialised: {json}");

    let restored: Tensor = serde_json::from_str(&json)?;
    assert_eq!(original, restored);
    println!("round-trip: OK  shape={:?}", restored.shape());

    // ── from_json: canonical object form ────────────────────────────────
    let t = Tensor::from_json(r#"{"shape":[2,2],"data":[1.0,2.0,3.0,4.0]}"#)?;
    println!("object form:  {t:?}");

    // ── from_json: nested-array convenience form ─────────────────────────
    let t2 = Tensor::from_json("[[1.0,2.0],[3.0,4.0]]")?;
    println!("nested form:  {t2:?}");

    // ── load_json from file ──────────────────────────────────────────────
    let t3 = Tensor::load_json("examples/data/tensor_2x2.json")?;
    println!("from file:    {t3:?}");

    Ok(())
}
