# Quick start

Add `matten` to your `Cargo.toml`:

```toml
matten = "0.46.2"
```

Then:

```rust
use matten::Tensor;

let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let b = Tensor::ones(&[2, 2]);
let c = &a + &b;
println!("{c}");
// 2.0 3.0
// 4.0 5.0
```

The default profile includes `serde`, JSON (`Tensor::from_json`), and CSV (`Tensor::from_csv`) —
see [Boundary integration](./reference/boundary.md) for those.

## Lean core only

If you don't need JSON/CSV/serde, opt into the smaller dependency footprint:

```toml
matten = { version = "0.46.2", default-features = false }
```

On this profile, `Tensor::from_json` / `Tensor::from_csv` are not available. See
[Cargo features](./reference/boundary.md#cargo-features) for the full feature list.

Want to try shape reasoning first, with nothing to install? See the [Playground](./playground.md).
