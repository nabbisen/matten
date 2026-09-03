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

> This documentation tracks the current 0.47 release family, carrying RFC-128 through
> RFC-132. The four arithmetic operators gain recoverable twins — `try_add`, `try_sub`,
> `try_mul`, `try_div` — returning `Result` where `+`/`-`/`*`/`/` panic; the operators
> themselves are unchanged and now delegate to them. The element budget **no longer applies**
> to arithmetic, reductions, slicing, or concatenation on data already in memory, so
> `&big + &big` succeeds where it previously panicked. It still applies at every boundary
> where a size arrives from outside, and to any operation whose output can exceed its inputs
> combined — `matmul`, `outer`, broadcast expansion, and `repeat`/`tile`. `max_parse_bytes`
> is now enforced at every file and string parser, including `matten-data`'s
> `Table::from_csv_path`, where it had been documented but inert. Five rustdoc statements
> that had gone false are corrected — see the `[0.47.0]` CHANGELOG entry for the complete
> list.
