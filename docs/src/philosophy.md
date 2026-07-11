# Philosophy

`matten` optimizes for *time to first understanding* and *time to a runnable
PoC*, not benchmark leadership.

- **One primary type.** You work through `Tensor`; no generic dtype parameters
  and no visible lifetimes in ordinary code.
- **Predictable, readable failures.** Convenience APIs panic with actionable
  messages; boundaries return `Result`.
- **Start now, optimize later.** When a prototype becomes performance-critical,
  hand `matten`'s flat data to a specialized crate such as `ndarray`,
  `nalgebra`, or `candle`.
- **Learn the shape before the engine.** Use the examples and visual
  explanations to understand tensors, axes, broadcasting, and small data
  transformations before reaching for heavier ecosystems.

`matten` is intentionally **not** a full dataframe engine, an ML framework, or a
GPU/sparse/distributed array library.
