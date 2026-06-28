# Introduction

`matten` is a developer-experience-first multidimensional array (tensor) library
for Rust — the *family car* for small numerical and data-exploration
proof-of-concept work.

> Maturity labels in this book — such as *production-ready* — describe stability
> **within that scope**, not performance or scale. `matten` optimizes for time to
> a runnable PoC, not benchmark leadership.

This book is organized by reader:

- **New users** — philosophy and a quick start.
- **Reference** — the rules that shape the public API.
- **Contributors** — project layout, milestones, and process.

> This documentation tracks the current 0.28 family, which moves the `matten-ndarray` bridge to `ndarray` 0.17 (RFC-062), on top of the completed companion-maturity line: `matten-ndarray` is production-ready and `matten-mlprep` and `matten-data` are production-ready candidates.
