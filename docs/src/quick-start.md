# Quick start

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
assert_eq!(a.shape(), &[2, 2]);
assert_eq!(a.ndim(), 2);
```

Install the lean core only:

```toml
matten = { version = "0.43.0", default-features = false }
```

Want to try shape reasoning first, with nothing to install? See the [Playground](./playground.md).
