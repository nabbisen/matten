# Migration to specialised libraries

`matten` is a **starting point**, not an endpoint. When a PoC graduates to
production or numerical performance becomes critical, migrate the data to a
specialised crate. This page shows how.

## When to migrate

| Signal | Recommended path |
|---|---|
| Matrix operations on > 1 000 × 1 000 data | `ndarray` + BLAS, or `nalgebra` |
| Machine learning / automatic differentiation | `candle`, `burn`, or `tch` |
| Large sparse data | `sprs` or domain-specific crates |
| Web API payloads needing serde but no math | stay with `matten` |
| Mixed messy data → clean numeric → arithmetic | stay with `matten` dynamic |

## Exporting data from `matten`

Every `matten` tensor exposes its flat row-major data. Migration is always
one line:

```rust
let flat: Vec<f64> = tensor.into_vec();  // consuming, no copy
// or
let flat: Vec<f64> = tensor.to_vec();    // borrowing clone
```

The shape is available as:

```rust
let shape: &[usize] = tensor.shape();
```

## To `ndarray`

```rust
use matten::Tensor;
use ndarray::ArrayD;

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
let shape: Vec<usize> = t.shape().to_vec();
let flat: Vec<f64>    = t.into_vec();

// ndarray from flat Vec<f64> + shape
let arr = ArrayD::from_shape_vec(shape, flat).unwrap();
println!("{arr}");
```

`ndarray` supports BLAS-backed matrix multiplication, advanced indexing,
views, and strided arrays.

## To `nalgebra`

```rust
use matten::Tensor;
use nalgebra::DMatrix;

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let flat: Vec<f64> = t.into_vec();

// DMatrix is column-major; transpose if needed
let mat = DMatrix::from_row_slice(2, 2, &flat);
println!("{mat}");
```

`nalgebra` provides static and dynamic matrices, LU/QR/SVD decomposition,
and linear algebra operations.

## To `candle` (ML tensors)

```rust
use matten::Tensor;
// candle_core = { version = "0.x", features = ["..."] }

let t = Tensor::new(vec![1.0f32, 2.0, 3.0, 4.0], &[2, 2]);
// matten uses f64; convert to f32 if needed
let flat_f32: Vec<f32> = t.as_slice().iter().map(|&v| v as f32).collect();
let shape = t.shape().to_vec();
// let candle_t = candle_core::Tensor::from_vec(flat_f32, shape, &device)?;
println!("data ready for candle: {flat_f32:?}, shape: {shape:?}");
```

`candle` targets GPU-accelerated ML workflows (transformers, training loops).

## Dynamic tensors: clean then migrate

If your data went through `matten`'s `dynamic` feature, convert to Phase 1
numeric first:

```rust
use matten::{Element, Tensor};

let raw = Tensor::from_csv_dynamic("1.0,2.0\n3.0,4.0\n")?;
let filled  = raw.fill_none(Element::Float(0.0));
let numeric: Tensor = filled.try_numeric()?; // MattenError if non-numeric
let flat: Vec<f64>  = numeric.into_vec();    // hand off
```

## Phase 1 allocation warning

`matten` Phase 1 clones on every reshape and slice. For large datasets,
migrate before performing many transformations:

```rust
// Prefer this pattern for large data:
let result = compute_in_matten(&small_data);
let flat   = result.into_vec();
// then pass `flat` to ndarray/nalgebra for the heavy lifting
```

## Compatibility promise (v0.x)

During `v0.x`, API changes are allowed but minimised after a release. The
core `Tensor` type name, the four exports (`Tensor`, `MattenError`,
`DataFormat`, `Element`), and the panic-vs-Result split are stable design
decisions that will not change without a documented breaking change.

`v1.0.0` requires explicit maintainer confirmation and a full public API
review. See the project CHANGELOG for migration notes on any breaking changes.
