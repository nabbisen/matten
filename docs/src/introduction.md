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

> This documentation tracks the current 0.43 release family, an RFC-099 and
> RFC-100 release adding two things to core `matten`'s public surface:
> `try_dot`/`try_matmul` (Result forms for the last two panic-only operations;
> the panicking `dot`/`matmul` are unchanged, including every message) and
> `Display` for `Tensor` (an aligned grid for rank ≤ 2, the existing flat form
> above it; `Debug` is unchanged). No existing behavior changed — see the
> `[0.43.0]` CHANGELOG entry.
