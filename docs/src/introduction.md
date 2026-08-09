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

> This documentation tracks the current 0.45 release family, carrying RFC-104, RFC-105,
> and RFC-108. `get_mut`/`get_flat_mut` (numeric) and `get_element_mut` (dynamic) add
> in-place mutation, mirroring the existing getters; on a dynamic tensor,
> `get_element_mut` **shares storage** with whatever the tensor was sliced from, and the
> first write through it **materializes** a fresh, uniquely owned copy — releasing the
> parent's allocation as a side effect, not a feature built for that purpose — see
> [Dynamic feature](./reference/dynamic.md). Two behaviour changes ship alongside the
> additions, not only fixes: `mean`/`min`/`max`/`argmin`/`argmax` now **error** on an
> empty tensor instead of panicking with a raw index error or silently returning
> `NaN`/`inf`/`-inf` — `sum` is unchanged and still returns a zero, its identity under
> addition; and `dot`/`matmul`/`try_dot`/`try_matmul` no longer panic on a zero-column
> product, returning the empty `[m, 0]` result instead — the fix removes a panic from an
> operation that already existed, not a new capability. `Tensor::is_empty()` is new, but
> the state it reports was always reachable via slicing to a zero-sized shape; no
> constructor accepts one directly, and that has not changed — see the `[0.45.0]`
> CHANGELOG entry for the complete list.
