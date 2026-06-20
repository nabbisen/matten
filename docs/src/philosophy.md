# Philosophy

`matten` optimizes for *time to a runnable PoC*, not benchmark leadership.

- **One primary type.** You work through `Tensor`; no generic dtype parameters
  and no visible lifetimes in ordinary code.
- **Predictable, readable failures.** Convenience APIs panic with actionable
  messages; boundaries return `Result`.
- **Start now, optimize later.** When a prototype becomes performance-critical,
  hand `matten`'s flat data to a specialized crate such as `ndarray`,
  `nalgebra`, or `candle`.

`matten` is intentionally **not** a full dataframe engine, an ML framework, or a
GPU/sparse/distributed array library.
