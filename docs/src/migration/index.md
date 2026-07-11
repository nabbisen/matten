# Production migration guide

`matten` is the family car: small, approachable, `Tensor`-centered, and good for
proof-of-concept work, learning, and small serious workflows. It stays deliberately
dependency-light and does **not** try to become a dataframe engine, an ML framework, or a
high-performance linear-algebra backend.

This guide is about the other half of that promise: helping you **know when and how to
leave `matten`** when a workflow outgrows it. Moving a hot path to a production-oriented
ecosystem is not a failure — **outgrowing `matten` is a successful PoC outcome.** It means
the idea earned the move.

## What this guide is — and is not

This guide helps you migrate *intentionally*. It is:

- a way to decide **when to stay** with `matten` and **when to migrate**;
- a **target-selection** matrix from your workload to the right ecosystem;
- a set of **playbooks** for specific targets (`ndarray`, `nalgebra`, Polars/Pandas, Candle,
  and NumPy);
- guidance on the **bridge crates** that own dependency-specific conversion.

It is explicitly **not**:

- a claim that `matten` is faster, or a promise that you can swap `matten` out unchanged;
- a claim that any target is universally "better" — it depends entirely on the workload;
- a tool that rewrites your code for you. `matten` helps you understand and plan a
  migration. (An assisted tool, `matten-migrate`, is a deferred future possibility, not part
  of this guide.)

`tools/matten-migrate` is now available as a local, unpublished, advisory helper
for generating a first migration-readiness report. It is a heuristic
text/dependency scan, not a source rewriter and not a correctness oracle: it may
miss or over-report usage, has not been validated against real downstream
projects yet, and should be treated as a starting point for manual review.

## The layered idea

```text
core matten   →  owns Tensor; stays small; no heavy target-library dependencies
bridge crates →  own dependency-specific conversion (e.g. matten-ndarray)
docs (here)   →  when to stay, when to migrate, and how
```

Core `matten` gains **no** new heavy dependency from any of this. The conversion to a
specific ecosystem lives in a dedicated bridge crate (such as `matten-ndarray`) or in your
own code — never inside core `matten`.

## Where to go next

- [When to migrate](./when-to-migrate.md) — signals that you have outgrown `matten`, and
  the equally important signals that you should *stay*.
- [Choosing a target](./target-selection.md) — a matrix from workload shape to ecosystem.
- [Common pitfalls](./common-pitfalls.md) — mistakes to avoid when moving data out.
- [Readiness checklist](./readiness-checklist.md) and
  [report template](./readiness-report.md) — turn the signals into an explicit, advisory
  decision you can record and review.
- [Target playbooks](./playbooks/index.md) — step-by-step, per-target migration guides.

For quick, copy-paste data-export snippets, the reference page
[Migration to specialised libraries](../reference/migration.md) is the companion to this
narrative guide.
