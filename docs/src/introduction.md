# Introduction

`matten` is a developer-experience-first multidimensional array (tensor) library
for Rust — the *family car* for learning, teaching, small numerical workflows,
data exploration, and early prototypes.

> Maturity labels in this book — such as *production-ready* — describe stability
> **within that scope**, not performance or scale. `matten` optimizes for time to
> first understanding and a runnable PoC, not benchmark leadership.

This book is organized by reader:

- **New users** — philosophy and a quick start.
- **Playground** — try broadcasting, reshape, axis reductions, and `matmul` in the browser,
  no install required: [Playground](./playground.md).
- **Reference** — the rules that shape the public API.
- **Contributors** — project layout, milestones, and process.

> This documentation tracks the current 0.48 release family. `matmul` and the axis
> reductions (`sum_axis`, `mean_axis`, `min_axis`, `max_axis`) are substantially faster on
> larger tensors. **No numeric output changes** — both are exact restructurings, verified
> bit-identical to the previous implementations rather than merely close. The published
> crates now carry a `homepage` link to the project book. See the `[0.48.0]` CHANGELOG
> entry for the complete list.
