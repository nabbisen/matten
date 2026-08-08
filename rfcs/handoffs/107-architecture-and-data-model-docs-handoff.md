# Developer Handoff — RFC-107: Architecture Overview and Data-Model Lifecycle Pages

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/107-architecture-and-data-model-docs.md`
**Base:** `main` @ `0df4c06`, family at `0.44.0`.
**Sequencing:** independent of RFC-106. Either order; they touch different files.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Add two reader-facing pages — an architecture overview and a data-model/lifecycle page — and link
them into `SUMMARY.md`. **Documentation only. No `.rs` file may change.**

## 2. What makes this task different

Every other page in this book documents an API. **These two document a *shape*** — how the crates
relate, and what states a tensor's storage moves through. That has one consequence worth stating
up front:

> **Every structural claim must cite what establishes it**, so a future reader can re-derive it. This
> book already contains claims that were true when written and silently went false — `is_empty()`'s
> rationale and `stats.rs`'s "not constructible in practice" both did exactly that, and both were
> found this week. A diagram nobody can check is a diagram that will go wrong quietly.

## 3. Evidence — verified, re-derive before using

| # | Claim | Established by |
|---|---|---|
| E1 | Core `matten` depends on **no** companion | `crates/matten/Cargo.toml` `[dependencies]` — no `matten-*` entry |
| E2 | All four companions depend on core, and on **no other companion** | each `crates/matten-*/Cargo.toml` lists `matten` only |
| E3 | The core→companion direction is guarded | `scripts/check-core-dependency-boundary.sh` (RFC-022 §10) |
| E4 | Companion→companion is forbidden by policy | RFC-078 §6 |
| E5 | `Tensor { data: Vec<f64>, shape: Vec<usize>, dynamic: Option<Box<DynamicTensor>> }`, the third field `#[cfg(feature = "dynamic")]` | `crates/matten/src/tensor.rs:39-44` |
| E6 | `DynamicTensor { storage: Arc<Vec<Element>>, shape, len, view: ViewKind }` | `dynamic/storage.rs:47-52` |
| E7 | `ViewKind::{Contiguous{offset}, Indexed(Vec<usize>)}` | `dynamic/storage.rs:34-40` |
| E8 | `Element::{Float(f64), Int(i64), Text(Arc<str>), Bool(bool), None}` | `dynamic/element.rs:35-46` |
| E9 | `Table { headers: Vec<String>, rows: Vec<Vec<CellValue>> }`, reaching core via `to_tensor()` | `matten-data/src/table.rs:67-70`; `matten-data/src/numeric.rs:43` |
| E10 | mdBook renders a ```` ```mermaid ```` block as `class="language-mermaid"` — a code block, **not** a diagram | `mdbook build docs` then grep the built HTML |
| E11 | Three tools and `benchmarks/` are workspace-excluded, `publish = false` | root `Cargo.toml` |

## 4. The two pages

```text
docs/src/architecture.md          NEW — reader-facing overview
docs/src/reference/data-model.md  NEW — types, lifecycle, storage states
docs/src/SUMMARY.md               both linked into the READER's path, not Contributing
```

`docs/src/contributing/architecture.md` **stays as it is.** It is the contributor reference — source
layout, re-exports, milestone sequence. At most add a cross-link. **Do not move its content, and do
not restate any of it**; a fact on both pages will rot on one of them (RFC §8 risk 1).

## 5. Architecture page — lead with the shape

The single most useful fact is that the dependency graph is a **star, not a stack**, and that it is
enforced from both directions (E3 policy-side, E4 guard-side). Say that plainly before any detail.

```text
one mermaid block: the star (core at centre, four companions pointing in)
a table:           each crate, one line on what it is for, its maturity label
a short section:   the three publish=false tools and benchmarks/ are NOT part of
                   the published surface (E11)
feature matrix:    LINK to the contributor page, do not duplicate (RFC §7)
```

## 6. Data-model page — three things, in this order

**6.1 One type, two modes.** State it as a plain sentence before any structure. `Tensor` holds
`Vec<f64>` *and* an optional dynamic box (E5). **Name the `cfg` gate** — without the `dynamic`
feature that field does not exist, and a default-feature reader must not be misled (RFC §8 risk 4).

**6.2 The lifecycle**, as a table, four stages:

```text
ingest   CSV/JSON -> Table (matten-data), or from_csv_dynamic / from_json_dynamic (core)
clean    fill_none, selection      -- still dynamic, still Element
convert  try_numeric()             -- THE single gate; fails on Text/None
compute  arithmetic, reductions, matmul -- numeric only
```

`try_numeric()` being the one gate is the load-bearing idea. `Table` is a **companion type reaching
core through `to_tensor()`** (E9) — not a `Tensor` variant, and not in core.

**6.3 The storage state machine**, the second mermaid block:

```text
Contiguous, unique   --slice()-->           Indexed, SHARED
Indexed, shared      --get_element_mut()--> Contiguous, unique   (materialize)
Contiguous, unique   --get_element_mut()--> unchanged            (no-op)
```

Both consequences go on this page, together — they currently live in two method doc comments written
hours apart:

```text
- a slice RETAINS its source's entire allocation while it lives          (RFC-102 §8.1)
- mutating a slice RELEASES that allocation, as a side effect            (RFC-104 §6.1)
```

## 7. Mermaid — two blocks, and no toolchain change

```text
WRITE   two ```mermaid blocks: §5's star, §6.3's state machine.
        They render as CODE BLOCKS today (E10) and become diagrams if a renderer is
        ever enabled — with zero content rewrite. Keep the source readable as text,
        because today that IS how a reader sees it.

DO NOT  add mdbook-mermaid, touch book.toml, or touch .github/workflows/**.
        That decision is deliberately deferred (RFC §6) and is not yours to take here.

EVERYTHING ELSE is a table or prose. The type list, the crate table, the lifecycle
stages, the feature matrix — all worse as diagrams. Two blocks, no more.
```

## 8. Acceptance criteria

```text
[ ] both pages exist, reachable from SUMMARY.md in the reader's path
[ ] every structural claim cites its source (Cargo.toml, guard script, file:line)
[ ] the star shape stated, with BOTH directions of enforcement (E3, E4)
[ ] "one type, two modes" as a plain sentence, with the cfg gate named
[ ] four lifecycle stages, try_numeric named as the single gate
[ ] Table described as a companion type reaching core via to_tensor(), not a variant
[ ] the state machine, with BOTH the retention cost and the materialize release
[ ] exactly two mermaid blocks; everything else table or prose
[ ] contributing/architecture.md's substance unchanged; no fact duplicated
[ ] git diff shows NO .rs, no book.toml, no workflow, no Cargo.*, no CHANGELOG.md
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
```

## 9. Risks

```text
R1  DUPLICATION with the contributor page. If you find yourself restating source
    layout or re-exports, stop — that page owns them.
R2  ```rust fences must COMPILE AND RUN under check-doc-code.sh. §6.1's struct
    sketches are illustrative: use ```text. Only write a ```rust fence if you
    intend it to be a real, running example.
R3  Overstating the dynamic mode. It is cfg-gated (E5). A reader on default
    features has no such field.
R4  Drawing a value. RFC-093 §6 forbids encoding a tensor VALUE as visual
    magnitude. These diagrams show crates and states — no tensor values — so they
    are outside it. Do not add anything that renders data.
R5  A diagram that cannot be re-derived. Every node and edge must trace to E1-E11.
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-107/matten-rfc107-architecture-and-data-model-docs-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, the two mermaid
blocks quoted in full, `git diff --stat` showing no code file, guard and `mdbook build` output,
deviations with reasoning, and anything you want answered at review.
