# RFC-107: Architecture Overview and Data-Model Lifecycle Pages

**Status:** **Implemented** 2026-08-08 in commit *"Add architecture overview and data-model/lifecycle
docs pages (RFC-107)"*, reviewed and approved after two corrections — a pair of section references a
reader could not follow, and a maturity label that contradicted the README. Docs only; no release
trigger (RFC-094). Handoff:
`rfcs/handoffs/107-architecture-and-data-model-docs-handoff.md`.
**Target:** `docs/` only — no code, no API, no version change
**Theme:** Two reader-facing pages the book does not have
**Related:** RFC-011, RFC-012, RFC-022 §10, RFC-078 §6, RFC-093 §6, RFC-102, RFC-104

---

## 1. Summary

Add two pages: an **Architecture overview** and a **Data model and lifecycle** page. Both are
reader-facing and go in the reader's path, not under Contributing.

## 2. What exists, measured

```text
docs/src/contributing/architecture.md   115 lines. Source layout, public re-exports, the
                                        Cargo feature matrix, design invariants, milestone
                                        sequence. A contributor REFERENCE LIST, no diagrams,
                                        and filed at SUMMARY.md:83 under Contributing — a
                                        reader orienting themselves never reaches it.

data model / lifecycle                  NOTHING. Fragments in reference/dynamic.md and
                                        tutorial/start-here.md; no page draws them together.
```

The second gap widened **today**. RFC-012's copy-on-write, RFC-102's storage-sharing slices, and
RFC-104's materialize-on-write together form a real state machine with transitions and a retention
consequence — currently documented only in scattered method doc comments, where no reader will
assemble it.

## 3. Not a scope-lock question

RFC-093 §6's test is *"does the rendering encode a value as something other than that value?"* It
governs how **tensor data** is displayed — bars, sparklines, heat maps, in any medium.

A crate-dependency graph or a storage state machine encodes **no tensor value at all**. These pages
are outside that lock and do not need an RFC arguing against §6 by name. Recorded because §6 requires
a crossing change to say so explicitly, and this one does not cross.

## 4. Content — architecture overview

Verified against `Cargo.toml`, not assumed:

```text
matten (core)      depends on NO companion
matten-ndarray  -> matten
matten-mlprep   -> matten
matten-data     -> matten
matten-stats    -> matten
```

**No companion depends on another companion** (RFC-078 §6), and core depending on a companion is
blocked by `scripts/check-core-dependency-boundary.sh` (RFC-022 §10). That is the whole shape: a star,
not a stack — and it is the single most useful thing to show a reader deciding what to depend on.

Also in scope: the three workspace-excluded `publish = false` tools and `benchmarks/`, named as *not*
part of the published surface; and the feature matrix, moved from the contributor page or referenced
rather than duplicated.

**The existing contributor page stays** as the detailed reference. This page does not replace it and
must not duplicate the source layout.

## 5. Content — data model and lifecycle

### 5.1 The types, verified

```text
Tensor { data: Vec<f64>, shape: Vec<usize>, dynamic: Option<Box<DynamicTensor>> }
    ONE type, TWO modes. The single most misunderstood thing in the library, and it is
    currently stated nowhere as a plain sentence.

DynamicTensor { storage: Arc<Vec<Element>>, shape, len, view: ViewKind }
ViewKind      { Contiguous { offset }, Indexed(Vec<usize>) }
Element       { Float(f64), Int(i64), Text(Arc<str>), Bool(bool), None }

Table (matten-data) { headers: Vec<String>, rows: Vec<Vec<CellValue>> }
    a SEPARATE type in a companion crate, reaching core through
    `to_tensor() -> Result<matten::Tensor, _>` — not a Tensor variant.
```

### 5.2 The lifecycle — the part that exists nowhere

```text
ingest    CSV/JSON -> Table (matten-data), or from_csv_dynamic/from_json_dynamic (core)
clean     fill_none, selection            -- still dynamic, still Element
convert   try_numeric()                   -- the ONE gate; fails on Text/None
compute   arithmetic, reductions, matmul  -- numeric only
```

### 5.3 The storage state machine

```text
Contiguous, unique          --slice()-->        Indexed, SHARED
Indexed, shared             --get_element_mut-->  Contiguous, unique   (materialize)
Contiguous, unique          --get_element_mut-->  unchanged            (no-op)
```

Two consequences that belong here and are currently only in method docs:

- a slice **retains its source's entire allocation** while it lives (RFC-102 §8.1)
- **mutating a slice releases that allocation** as a side effect (RFC-104 §6.1) — the retention
  escape hatch, arriving from an unrelated operation

## 6. Rendering — the one decision, and it is smaller than it looks

Measured: mdBook renders a ```` ```mermaid ```` block as `class="language-mermaid"` — **a plain code
block, not a diagram.** Rendering needs `mdbook-mermaid` (preprocessor plus a JS asset), a new build
dependency and a change to `.github/workflows/docs.yaml:35`.

**Recommendation: write the two graph-shaped items as ```` ```mermaid ```` blocks anyway, and defer
the renderer decision.**

```text
Mermaid source is READABLE AS TEXT. `graph LR  A[Tensor] --> B[DynamicTensor]` degrades to a
legible code block today, and becomes a diagram the moment a renderer is enabled — with ZERO
content rewrite. The rendering decision therefore does not block the content, and can be made
later on its own merits.
```

Everything else — the type list, the feature matrix, the lifecycle stages — is **better as a table
than as a diagram** regardless, and should be one. Only two things are genuinely graph-shaped: the
crate star (§4) and the storage state machine (§5.3).

**Not recommended now:** adding `mdbook-mermaid`. For one star graph and a three-state machine, a
build dependency and a CI change is a poor trade in a project that has kept its toolchain
deliberately small. Revisit if the pages prove they need it.

## 7. Scope

### In scope

```text
docs/src/architecture.md            NEW, reader-facing
docs/src/reference/data-model.md    NEW
docs/src/SUMMARY.md                 both linked into the reader's path
```

### Out of scope — a diff touching these is a defect

```text
any .rs file, any API, any version
contributing/architecture.md's SUBSTANCE — it stays as the contributor reference;
  at most a cross-link is added
mdbook-mermaid, book.toml, or .github/workflows/** (§6)
CHANGELOG.md — the release RFC writes it, and docs never trigger a release (RFC-094)
```

## 8. Risks

```text
1. DUPLICATION. Two architecture pages that restate each other rot independently.
   The new page is an OVERVIEW; the contributor page keeps source layout and
   re-exports. If a fact appears on both, it belongs on one and is linked from
   the other.
2. STALENESS BY CONSTRUCTION. Every structural claim must cite what establishes it
   (Cargo.toml, a guard script) so a future reader can re-derive it. A diagram that
   cannot be checked is a diagram that will silently go wrong.
3. ANY ```rust FENCE MUST COMPILE under check-doc-code.sh. Prefer ```text for
   struct sketches — §5.1's field lists are illustrative, not compilable.
4. OVERSTATING THE MODEL. Tensor's two modes are cfg-gated: without the `dynamic`
   feature the field does not exist. Say so, or the page misleads default-feature
   readers.
```

## 9. Acceptance criteria

```text
[x] both pages exist and are reachable from SUMMARY.md in the reader's path
[x] the crate graph matches Cargo.toml, and cites it
[x] the star shape is stated: companions depend on core, never on each other
    (RFC-078 §6), and core never on a companion (guard, RFC-022 §10)
[x] "one type, two modes" stated plainly, with the cfg-gating named (risk 4)
[x] the four lifecycle stages, with try_numeric named as the single gate
[x] the storage state machine, with BOTH the retention cost and the
    materialize-on-write release
[x] Table described as a companion type reaching core via to_tensor(), not a variant
[x] two mermaid blocks only (§6); everything else a table or text
[x] contributing/architecture.md's substance unchanged; no fact duplicated
[x] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[x] no .rs, no book.toml, no workflow, no version change
```

## 10. Non-goals

```text
mdbook-mermaid, or any rendering-toolchain change (§6)
replacing or rewriting the contributor architecture page
API documentation — that is rustdoc's job, and the reference pages'
```
