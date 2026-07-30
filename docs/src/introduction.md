# Introduction

`matten` is a developer-experience-first multidimensional array (tensor) library
for Rust — the *family car* for learning, teaching, small numerical workflows,
data exploration, and early prototypes.

> Maturity labels in this book — such as *production-ready* — describe stability
> **within that scope**, not performance or scale. `matten` optimizes for time to
> first understanding and a runnable PoC, not benchmark leadership.

This book is organized by reader:

- **New users** — philosophy and a quick start.
- **Reference** — the rules that shape the public API.
- **Contributors** — project layout, milestones, and process.

> This documentation tracks the current 0.41 release family, an RFC-089 release
> publishing two core `matten` additions: `repeat`/`repeat_axis`/`tile`/`meshgrid`
> (RFC-087) and negative indices in `slice_str` (RFC-088, `"-1"` for the last
> element, out of range errors rather than clamping) — without any other public
> API, dependency, or runtime behavior change.
