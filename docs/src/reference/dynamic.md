# Dynamic feature (`Element` model)

The `dynamic` feature enables heterogeneous dynamic tensors. Enable it in
`Cargo.toml`:

```toml
matten = { version = "0.24", features = ["dynamic"] }
```

`matten` is **not** a dataframe library. The `dynamic` feature is for ingesting
and cleaning messy PoC data before converting to numeric tensors or handing off
to a specialised crate.

## `Element` variants

```rust
use matten::Element;

Element::Float(1.5)            // IEEE 754 f64
Element::Int(42)               // i64
Element::text("active")        // UTF-8 text (Arc<str> internally)
Element::Bool(true)            // boolean
Element::None                  // missing / null
```

`size_of::<Element>() == 24` bytes on 64-bit targets (all text representations
give the same size; `Arc<str>` was chosen for cheap clone in CoW slices).

## Constructing dynamic tensors

```rust
use matten::{Element, Tensor};

let t = Tensor::from_elements(
    vec![
        Element::Float(1.0), Element::text("ok"), Element::Bool(true),
        Element::Int(2),     Element::None,        Element::Bool(false),
    ],
    &[2, 3],
);

// Boundary-safe variant:
let t = Tensor::try_from_elements(data, &[2, 3])?;
```

## Element predicates and coercion

```rust
Element::None.is_none()         // true
Element::Float(1.0).is_numeric() // true
Element::Int(42).is_numeric()   // true
Element::Bool(true).is_numeric() // false — no silent bool coercion

Element::Float(1.5).try_as_f64()  // Some(1.5)
Element::Int(7).try_as_f64()      // Some(7.0)
Element::text("3").try_as_f64()   // None — no silent text coercion
Element::None.try_as_f64()        // None
```

## Coercion policy (RFC-011 §11)

| From | To `f64` | Allowed? |
|---|---|---|
| `Float(f64)` | itself | yes |
| `Int(i64)` | cast | yes |
| `Bool` | — | **no** |
| `Text` | — | **no** |
| `None` | — | **no** |

Use `fill_none` or explicit conversion helpers to clean data before arithmetic.

## Accessing elements

```rust
t.get_element(&[0, 1])  // Option<Element> — None if out of bounds
t.is_dynamic()          // true for dynamic tensors
t.to_elements()         // Vec<Element> in row-major order
```

## Missing-value utilities

```rust
use matten::{Element, Tensor};

let t = Tensor::from_elements(
    vec![Element::Float(1.0), Element::None, Element::Float(3.0), Element::None],
    &[4],
);

// Count None values
t.count_none()          // 2

// Boolean-like mask: 1.0 where None, 0.0 elsewhere (numeric f64 tensor)
let mask = t.none_mask();   // [0.0, 1.0, 0.0, 1.0]
// RFC-011 named alias:
let mask = t.is_none_mask(); // identical result

// Constant fill
let filled = t.fill_none(Element::Float(0.0)); // [1.0, 0.0, 3.0, 0.0]

// Forward-fill: carry last non-None value forward (fallback for leading None)
let t2 = Tensor::from_elements(
    vec![Element::None, Element::Float(1.0), Element::None, Element::Float(4.0)],
    &[4],
);
let fwd = t2.forward_fill_none(Element::Float(-1.0));
// [-1.0, 1.0, 1.0, 4.0]  (leading None takes fallback)

// Sum skipping None (panics on non-numeric non-None elements)
t.sum_skip_none()  // 4.0  (1.0 + 3.0, None values skipped)
```

## Parsing mixed data

```rust
// JSON: null→None, booleans→Bool, strings→Text, integers→Int, floats→Float
#[cfg(feature = "json")]
let t = Tensor::from_json_dynamic(r#"[[1, "active", true], [2, null, false]]"#)?;

// CSV: empty field→None, "true"/"false"→Bool, integers→Int, floats→Float, rest→Text
#[cfg(feature = "csv")]
let t = Tensor::from_csv_dynamic("1,active,true\n2,,false\n")?;
```

## Current limitations (guard model)

In the current release, many numeric operations **reject** dynamic
tensors with a clear `matten unsupported error` message. You must convert
to a numeric tensor first using `try_numeric()`.

Guarded (will panic or return `Err`):
- `reshape`, `flatten`, `transpose`, `swap_axes`
- `slice()` builder and `slice_str()` → `MattenError::Unsupported`
- all arithmetic operators and reductions
- `dot` / `matmul`
- `as_slice`, `to_vec`, `into_vec`, `get`, `get_flat`
- `Serialize` / serde

The underlying `Arc`-based CoW storage (`DynamicTensor`) is implemented
internally and will back future public dynamic slicing and reshape in a later
release.

```rust
// Correct pattern: ingest → clean → convert → arithmetic
let raw = Tensor::from_csv_dynamic("1.0,2.0\n3.0,4.0\n")?;
let filled  = raw.fill_none(Element::Float(0.0));
let numeric: Tensor = filled.try_numeric()?; // convert to numeric
let result = &numeric * 2.0;                 // numeric arithmetic
```

## Workflow pattern

```rust
use matten::{Element, Tensor};

fn process_messy_csv(input: &str) -> Result<Tensor, Box<dyn std::error::Error>> {
    // 1. Ingest as dynamic
    let raw = Tensor::from_csv_dynamic(input)?;

    // 2. Fill missing values
    let clean = raw.fill_none(Element::Float(0.0));

    // 3. Convert to numeric tensor for arithmetic
    let numeric = clean.try_numeric()?;

    // 4. Use numeric arithmetic, reductions, matmul...
    Ok(numeric)
}
```

## Limitations

- No dataframe joins, group-by, pivot, or query operations.
- No date/time dtype.
- No categorical dtype.
- No silent text-to-number or bool-to-number coercion.
- Batched matmul on dynamic tensors requires `try_numeric` first.
- For large datasets, consider specialised crates (`polars`, `ndarray`).
