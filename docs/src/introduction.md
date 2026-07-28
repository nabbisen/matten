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

> This documentation tracks the current 0.39 release family, an RFC-079 release
> adding `matten_mlprep::train_test_split_seeded` (a reproducible, seeded
> shuffled train/test split) without changing existing public APIs, published
> dependencies, core runtime behavior, or maturity labels.
