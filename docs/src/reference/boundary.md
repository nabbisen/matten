# Boundary integration

All external-input APIs in `matten` are Result-zone: they never panic on
malformed data and always return `Result<Tensor, MattenError>`.

## JSON

### Canonical object form

The preferred form for programmatic use — unambiguous for any rank:

```rust
use matten::Tensor;

let t = Tensor::from_json(
    r#"{"shape":[2,2],"data":[1.0,2.0,3.0,4.0]}"#
).unwrap();
assert_eq!(t.shape(), &[2, 2]);
```

### Convenience nested-array form

Rank 1 and rank 2 nested arrays are also accepted:

```rust
let t = Tensor::from_json("[[1.0,2.0],[3.0,4.0]]").unwrap();
assert_eq!(t.shape(), &[2, 2]);

let v = Tensor::from_json("[1.0,2.0,3.0]").unwrap();
assert!(v.is_vector());
```

Ragged arrays and non-numeric values return `MattenError::Parse`:

```rust
assert!(Tensor::from_json("[[1.0,2.0],[3.0]]").is_err()); // ragged
assert!(Tensor::from_json(r#"[[1.0,"text"]]"#).is_err()); // non-numeric
```

### Serde integration

`Tensor` implements `Serialize` and `Deserialize` using the canonical object
form (requires the `serde` or `json` feature, both on by default):

```rust
use matten::Tensor;

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let json = serde_json::to_string(&t).unwrap();
let t2: Tensor = serde_json::from_str(&json).unwrap();
assert_eq!(t, t2);
```

### File loading

```rust
let t = Tensor::load_json("examples/data/tensor_2x2.json")?;
```

File errors map to `MattenError::Io`; parse errors to `MattenError::Parse`.

## CSV

Numeric CSV ingestion accepts rectangular numeric-only CSV. Shape is inferred as
`[rows, cols]`.

```rust
let t = Tensor::from_csv("1.0,2.0,3.0\n4.0,5.0,6.0\n")?;
assert_eq!(t.shape(), &[2, 3]);
assert_eq!(t.as_slice(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
```

Errors include row and column context:

```text
matten csv parse error: at row 1, column 1: expected f64, got "active"
```

```rust
let t = Tensor::load_csv("examples/data/numeric_2x3.csv")?;
```

## Cargo features

| Feature | Default | What it enables |
|---|---|---|
| `serde` | yes | `Serialize`/`Deserialize` for `Tensor` |
| `json` | yes (implies `serde`) | `from_json`, `load_json` |
| `csv` | yes | `from_csv`, `load_csv` |

Lean install (no I/O dependencies):

```toml
matten = { version = "0.36.0", default-features = false }
```

## Error mapping

| Situation | Error variant |
|---|---|
| Malformed JSON, wrong type, ragged array | `MattenError::Parse { format: DataFormat::Json, .. }` |
| Non-numeric CSV field, ragged rows | `MattenError::Parse { format: DataFormat::Csv, .. }` |
| File not found, permission error | `MattenError::Io { path, source }` |
| Shape/data length mismatch in JSON payload | `MattenError::Parse` (wraps the shape error message) |
