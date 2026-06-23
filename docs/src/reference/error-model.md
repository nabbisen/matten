# Error model

`matten` uses a single public error type, `MattenError`, and splits every API
into one of two zones. Understanding the split is the key to writing correct
code with `matten`.

## Panic zone vs Result zone

| Zone | When | How |
|---|---|---|
| **Panic zone** | Local, developer-authored PoC code where shapes are known | API panics with an actionable `matten <category> error in <operation>: ...` message |
| **Result zone** | Any external boundary — parsing, file I/O, user-supplied shapes | API returns `Result<Tensor, MattenError>` and never panics on ordinary invalid input |

Rule of thumb: if the shape or data comes from *outside* your code (a file,
a web request, user input), use the `try_*` form.

```rust
use matten::{MattenError, Tensor};

// Panic zone: shape is a trusted literal
let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);

// Result zone: shape comes from somewhere external
let result = Tensor::try_new(data, &user_shape);
match result {
    Ok(t) => println!("{t:?}"),
    Err(e) => eprintln!("bad input: {e}"),
}
```

## `MattenError` variants

```rust
#[derive(Debug)]
#[non_exhaustive]
pub enum MattenError {
    Shape     { operation: &'static str, message: String },
    Broadcast { left: Vec<usize>, right: Vec<usize> },
    Allocation { requested_elements: usize, message: String },
    Slice     { input: Option<String>, message: String },
    Parse     { format: DataFormat, message: String },
    Io        { path: std::path::PathBuf, source: std::io::Error },
    Unsupported { operation: &'static str, message: String },
    InvalidArgument { operation: &'static str, argument: &'static str, message: String },
}
```

`MattenError` is `#[non_exhaustive]`, so match it with a wildcard arm to stay
forward-compatible.

`DataFormat` identifies which parser produced a `Parse` error:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DataFormat { Json, Csv }
```

## Variant guide

| Variant | Produced by |
|---|---|
| `Shape` | construction mismatch, reshape, invalid `arange` arguments |
| `Broadcast` | incompatible operand shapes in arithmetic |
| `Allocation` | shape product overflow or `arange` element-count limit |
| `Slice` | slice builder bounds errors, `slice_str` parse/bounds errors |
| `Parse` | `from_json`, `from_csv`, and their file-loading variants |
| `Io` | `load_json`, `load_csv` file I/O errors |
| `Unsupported` | disabled-feature or not-yet-implemented operation, or a numeric-only API called on a dynamic tensor |
| `InvalidArgument` | a supported operation given an out-of-range/ill-defined argument (e.g. `clip` with `min > max`); distinct from `Unsupported` |

## Matching errors

`MattenError` embeds `std::io::Error` in `Io`, which is neither `Clone` nor
`PartialEq`. **Never compare with `==`; always match by variant.**

```rust
let err = Tensor::try_new(vec![1.0], &[2, 2]).unwrap_err();

// correct
assert!(matches!(err, MattenError::Shape { .. }));

// correct
if let MattenError::Shape { operation, message } = &err {
    println!("{operation}: {message}");
}

// will not compile — MattenError does not implement PartialEq
// assert_eq!(err, MattenError::Shape { .. });
```

## Panic message format

Panic-zone APIs always begin with `"matten"`:

```text
matten shape error in reshape: cannot reshape tensor with 6 elements
    from shape [2, 3] into shape [4, 2] requiring 8 elements
```

The format is `matten <category> error in <operation>: <detail>`. When
something panics unexpectedly, this prefix makes it easy to grep.

## Using `?` in application code

`MattenError` implements `std::error::Error`, so it works with `?` and
`Box<dyn Error>`:

```rust
fn load_and_process(path: &str) -> Result<Tensor, Box<dyn std::error::Error>> {
    let t = Tensor::load_json(path)?;  // Io or Parse on failure
    let flat = t.try_reshape(&[t.len()])?;  // Shape on mismatch
    Ok(flat)
}
```
