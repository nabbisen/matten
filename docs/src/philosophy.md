# Philosophy

`matten` is a developer-experience-first tensor library for Rust. It is shaped for learning,
teaching, early prototypes, small numerical workflows, and data exploration where clear tensor code
matters more than exposing every specialized engine concern up front.

The project optimizes for time to first understanding: create a tensor, see its shape, transform it,
and keep moving. That does not mean hiding Rust. It means using Rust's packaging, explicit errors,
and predictable ownership while keeping the public tensor surface narrow enough to learn.

## What matten is

`matten` gives Rust users a small, concrete `Tensor`-centered path for vectors, matrices, axes,
broadcasting, reductions, simple statistics, dynamic ingestion, and small table-to-tensor workflows.
It is useful when the goal is to explore an idea, explain an operation, teach tensor shape, or build a
readable proof of concept before choosing heavier tools.

The intended feel is the family car: practical and predictable, comfortable to start, explicit about
boundaries, and honest about when another library is a better next step.

## Core principles

**One primary type.** Ordinary numeric work starts with `Tensor`. The public API avoids generic dtype
parameters and lifetime-bearing tensor views in common examples so a new reader can focus on the
operation and its shape.

**Concrete before abstract.** Core `Tensor` computation is numeric and `f64`-based by default.
Mixed external data enters through the dynamic ingestion path, where cleanup and numeric conversion
are explicit steps instead of hidden coercion.

**Small surface, visible meaning.** Shape, axis, and data movement should be inspectable. Examples and
visual explanations are part of the learning path, not decoration.

**Panic locally, return `Result` at boundaries.** Trusted local math conveniences may panic with
actionable messages. Anything that reads files, parses JSON/CSV, accepts user-provided shapes, or
crosses an external boundary returns `Result`.

**Evidence without ranking.** Benchmarks and reports exist to explain tradeoffs and catch regressions,
not to claim universal speed leadership.

## What matten is not

`matten` is not a dataframe engine, an ML framework, a GPU backend, a sparse tensor library, an
automatic differentiation system, or a broad wrapper around external numeric crates. Companion crates
and migration docs can help users connect to other ecosystems, but core `matten` stays small and
tensor-centered.

## When to move on

If a workflow grows beyond the small, readable, educational, or prototype-oriented scope, move the hot
path to the tool that owns that domain. Use `ndarray` for broader Rust N-D array work, `nalgebra` for
linear algebra structures, Polars or Pandas for dataframe workflows, and Candle for ML tensor/model
workflows. The [migration guide](./migration/index.md) explains those paths.

The goal is not to keep every project inside `matten`; the goal is to make the first model clear
enough that the next decision is informed.
