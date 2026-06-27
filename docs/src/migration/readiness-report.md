# Migration readiness report

When you have worked through the [readiness checklist](./readiness-checklist.md) and want to
record a decision — for a code review, a design doc, or just your own notes — fill in the
report template below. It is a **manual** template: you write it, drawing on what you know
about your own workload. There is no generator and no source-scanner.

> **This report is advisory. It does not prove production readiness, does not guarantee a
> target library is better, and does not perform automatic conversion.**

Keep that framing in mind: the report's job is to make a migration *decision* explicit and
reviewable, not to certify anything.

## How to use the template

Copy the skeleton below into your own doc and fill each section. Sections you have nothing to
say about can be marked "none" — that is itself useful information (e.g. "Manual redesign
areas: none" means the move is a near-direct port).

## Template

```markdown
# matten Migration Readiness Report

## Summary
One or two sentences: what is being assessed, and the headline recommendation
(stay, or migrate which part to which target).

## Current matten usage
What the code does in matten today — the shapes, the operations (matmul, reductions,
slicing, dynamic ingestion), and which examples it resembles.

## Production pressure signals
Which checklist signals are present, and the evidence (a profile, a data size, a
required capability). Be concrete; "runtime pressure: the per-step matmul dominates
at N samples" beats "it feels slow".

## Recommended target(s)
The target(s) the signals point to, and why. It is fine to recommend "stay with
matten" if the signals are weak.

## Direct conversion candidates
The operations that map cleanly onto the target (e.g. matmul → ndarray `.dot()`),
including which bridge function carries the data across.

## Manual redesign areas
The parts that do not port mechanically and need rethinking (e.g. an iterative loop,
or switching an algorithm to a decomposition-based form). "none" is a valid answer.

## Bridge crates / tools
Which bridge crate applies (e.g. matten-ndarray) or that the conversion is manual
(e.g. nalgebra). Note copy/precision boundaries.

## Risks
What could go wrong: precision changes (f64 → f32), memory-order traps (row- vs
column-major), converting inside a hot loop, or scope creep into over-migration.

## Next steps
The concrete plan: profile to confirm the hot path, convert once at the boundary,
move the kernel, keep matten for setup/glue, and a checkpoint to reassess.
```

## A filled-in example

See [Linear regression (GD) readiness](./examples/linear-regression-gd-readiness.md) for the
template applied to the `35_linear_regression_gradient_descent` example.

