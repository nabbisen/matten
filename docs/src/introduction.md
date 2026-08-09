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

> This documentation tracks the current 0.46 release family, carrying RFC-110, RFC-111,
> and RFC-112 — no new API, only behaviour changes and a restriction removed. Zero-sized
> dimensions are now **constructible directly** (`Tensor::try_new(vec![], &[0, 3])`
> succeeds), not merely reachable by slicing as before: every constructor, `reshape`,
> the shape-composition family, `linspace`/`eye`, serde, and the `matten-ndarray` bridge
> accept them. `Display` on an empty tensor now shows its **shape** instead of an empty
> string; `Debug` is unchanged. `mean_axis`/`min_axis`/`max_axis`/`var_axis`/`std_axis`
> now **error** when the **reduced** axis has length 0, instead of leaking `NaN`/`inf`/
> `-inf` — a zero-length *surviving* axis is a different case and was and remains `Ok`
> with an empty result; `sum`/`sum_axis` are unchanged, their additive identity was
> already correct. `matten-ndarray`'s `ZeroSizedAxis` error variant is deprecated and
> never constructed, kept only so existing code matching on it still compiles.
> `matten-mlprep`'s `standardize_columns`/`minmax_scale_columns` return a different
> error — not a different outcome — for a zero-row input: previously a shape-rejection
> error from tensor construction, now an axis-reduction error, both `Err`, never a
> panic in any released version — see the `[0.46.0]` CHANGELOG entry for the complete
> list.
