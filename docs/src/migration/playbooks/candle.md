# Migrating to Candle (ML tensors)

[Candle](https://github.com/huggingface/candle) is a Rust ML tensor framework: autograd,
neural-network layers, model loading, and CPU/GPU execution. Move here when your workflow
becomes **machine learning** — training loops, autodiff, or device acceleration.

`matten` is **not** an ML framework and does not aim to become one. It has no autograd, no
layers, no optimizers, and no device backend. When you need those, that capability lives in
Candle (or another ML framework), not in a future `matten` feature.

## Choose this target when

- You need **automatic differentiation** / backprop.
- You are building or running a **model** (layers, training loop, inference).
- You need **GPU/device** execution.

## Do not choose this target when

- You are doing plain numeric array math with no learning → `ndarray` or stay with `matten`.
- You need classical linear-algebra results (decompositions/solvers) → `nalgebra`.
- The "ML" is actually a small hand-written numeric step (e.g. a single gradient-descent
  update) that `matten` already expresses clearly — it may not be worth a framework yet.

## Concept mapping

| `matten` | Candle |
|---|---|
| `Tensor` (`f64`, CPU, no grad) | `candle_core::Tensor` (often `f32`, CPU/GPU, autograd) |
| manual update step (e.g. `35_linear_regression_gradient_descent`) | optimizer + `loss.backward()` |
| `.matmul(&b)` | `a.matmul(&b)?` on a device |
| *(not available)* autodiff / layers / optimizers | `candle_nn` modules, `Var`, optimizers |

## Example migrations

- `35_linear_regression_gradient_descent` → Candle once you want autodiff and an optimizer
  instead of a hand-written gradient step.
- `37_kmeans_small` / `38_nearest_neighbor_classification` → Candle (or a dedicated ML
  crate) if these grow into trained models on real data; for small teaching versions,
  `matten` is fine.

## Conversion path

`matten` is `f64`; Candle workflows are commonly `f32`, so the boundary involves a precision
conversion as well as a copy. The shape carries over directly. Illustratively (Candle is not
a `matten` dependency):

```rust
use matten::Tensor;
// candle_core = { version = "0.x", features = ["..."] }

let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
let shape = t.shape().to_vec();
let flat_f32: Vec<f32> = t.as_slice().iter().map(|&v| v as f32).collect();
// let device = candle_core::Device::Cpu;
// let candle_t = candle_core::Tensor::from_vec(flat_f32, shape, &device)?;
```

## Common pitfalls

- **`f64` → `f32` is a precision change**, not just a copy. Do it once at the boundary and be
  aware of the loss.
- **Don't expect `matten` to provide autograd or layers.** If you reach for those, you have
  already crossed into ML-framework territory.
- **A single update step is not a model.** If your "training" is one hand-written step,
  consider whether you actually need a framework yet.

## Performance / positioning notes

There is **no `matten`-vs-Candle benchmark**. They occupy different layers (a plain numeric
tensor vs. an autodiff/device ML framework), and a cross-framework comparison would be
RFC-049 Phase 3, which is **not authorized**. Choose Candle for **ML capability and device
support**, not on the basis of a measured speed comparison.

## Minimal checklist

- [ ] You actually need autodiff, layers/optimizers, or device execution.
- [ ] You handle the `f64` → `f32` precision change once, at the boundary.
- [ ] You are not treating `matten` as an ML framework it never claimed to be.
