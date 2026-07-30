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

> This documentation tracks the current 0.40 release family, an RFC-086 release
> publishing `matten-data`'s `CsvBatchReader` (RFC-082), `matten-stats`'s
> `covariance_population`/`skewness`/`kurtosis` (RFC-083), and three maturity
> promotions (`matten-mlprep` production-ready, `matten-stats` production-ready
> candidate, `matten-data` production-ready) without any other public API,
> dependency, or runtime behavior change.
