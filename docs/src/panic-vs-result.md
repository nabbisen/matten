# Panic vs Result

`matten` has two error zones:

- **Panic zone** — local convenience APIs (e.g. `Tensor::new`) panic with an
  actionable `matten <category> error in <operation>: ...` message.
- **Result zone** — every external boundary (parsing, file I/O, user-driven
  construction such as `Tensor::try_new`) returns `Result<_, MattenError>` and
  does not panic on ordinary invalid input.

`MattenError` embeds `std::io::Error`, so it derives only `Debug`. Match it by
variant, never by `==`:

```rust
use matten::{MattenError, Tensor};

let err = Tensor::try_new(vec![1.0], &[2, 2]).unwrap_err();
assert!(matches!(err, MattenError::Shape { .. }));
```
