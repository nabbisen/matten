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

> This documentation tracks the current 0.44 release family, an RFC-102 release
> changing how `slice()` and `slice_str()` behave on dynamic tensors: they no
> longer return `MattenError::Unsupported` and instead return a dynamic tensor
> that shares storage with its source (`Arc::clone`, not a copy). The slice
> grammar, rank rules, and error messages are unchanged, and every numeric
> slicing result is unchanged. Sharing has a cost — a slice retains its
> source's entire allocation for as long as the slice lives, even after the
> source is dropped; see
> [Slicing](./reference/slicing.md#slicing-dynamic-tensors-rfc-102-cfgfeature--dynamic).
> No public item was added: an error was removed from methods that already
> existed — see the `[0.44.0]` CHANGELOG entry.
