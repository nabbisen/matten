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

> This documentation tracks the current 0.42 release family, an RFC-090 release
> publishing one `matten-stats` addition: `histogram` (`bins` is a required
> argument; there is no automatic bin-count rule) — without any other public
> API or dependency change. Core `matten`'s public surface is untouched by
> this release. Two `matten-stats` error message strings also changed
> (`ZeroVariance`, `NonFiniteValue`) — see the `[0.42.0]` CHANGELOG entry.
