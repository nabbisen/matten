//! Working with mixed-type Element tensors.
//!
//! Run: cargo run --example dynamic_01_mixed_elements --features dynamic
//!
//! `matten` is not a dataframe. Use the dynamic feature to ingest and inspect
//! mixed data before converting to a numeric Tensor.

#[cfg(not(feature = "dynamic"))]
fn main() {
    println!("Requires --features dynamic");
}

#[cfg(feature = "dynamic")]
fn main() {
    use matten::{Element, Tensor};

    // Build directly from a Vec<Element>
    let t = Tensor::from_elements(
        vec![
            Element::Int(1),
            Element::text("alpha"),
            Element::Bool(true),
            Element::Float(2.5),
            Element::text("beta"),
            Element::Bool(false),
            Element::Int(3),
            Element::None,
            Element::Bool(true),
        ],
        &[3, 3],
    );

    println!("shape = {:?}", t.shape());
    println!("is_dynamic = {}", t.is_dynamic());

    // Inspect individual elements
    println!("[0,0] = {:?}", t.get_element(&[0, 0])); // Int(1)
    println!("[0,1] = {:?}", t.get_element(&[0, 1])); // Text("alpha")
    println!("[2,1] = {:?}", t.get_element(&[2, 1])); // None

    // Element type predicates
    let e_float = Element::Float(1.5);
    let e_text = Element::text("hi");
    let e_none = Element::None;
    println!("Float.is_numeric = {}", e_float.is_numeric()); // true
    println!("Text.is_numeric  = {}", e_text.is_numeric()); // false
    println!("None.is_none     = {}", e_none.is_none()); // true

    // Count and locate missing values
    println!("none count = {}", t.count_none()); // 1

    // none_mask: 1.0 where None, 0.0 elsewhere
    let mask = t.none_mask();
    println!("none_mask shape = {:?}", mask.shape()); // [3,3]
    assert_eq!(mask.get(&[2, 1]), Some(1.0));
    assert_eq!(mask.get(&[0, 0]), Some(0.0));

    println!("All assertions passed.");
}
