# Developer Handoff — RFC-116: Render the Diagrams, Repair Two Records

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/116-docs-infrastructure-and-stale-records.md`
**Base:** `main` @ `b3200e2`, clean tree.
**Sequencing:** independent of RFC-115; they touch different files. Either order.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Add `mdbook-mermaid` so RFC-107's two diagrams render; backfill or retire ROADMAP §4's release table
and the handoffs index.

## 2. Part A — the constraint that decides everything

**Vendor the assets. Do not reference a CDN.**

The book is built in CI and served as static files. A CDN reference makes the page depend on a third
party *at read time* and fails offline — a different product from the one this project ships.
`mdbook-mermaid install` writes the JS/CSS into `docs/theme/`; commit them.

**State the added repository size in the review request.** If it is large, say so plainly rather than
letting it land unremarked — the owner should weigh it.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | The two blocks render as `class="language-mermaid"` — code, not diagrams | `mdbook build docs`, then grep `docs/book/architecture.html` and `docs/book/reference/data-model.html` |
| E2 | `book.toml` already carries `additional-js` | `docs/book.toml:14` — `theme/playground.js`, so the mechanism exists |
| E3 | CI installs mdBook with `--locked` | `.github/workflows/docs.yaml:34` |
| E4 | The docs job already installs extra tooling beside mdBook | same workflow — `wasm32` target and `wasm-bindgen-cli` |
| E5 | ROADMAP §4's table stops after `v0.41.0` | `grep -oE "^\| \*\*v0\.4[0-9]\.0\*\*" ROADMAP.md` → only `v0.40.0`, `v0.41.0` |
| E6 | Five shipped releases have no row | `v0.42.0` `v0.43.0` `v0.44.0` `v0.45.0` `v0.46.0` |
| E7 | The handoffs index has **79 rows** for **97 files** | `grep -c 'handoff.md\`' rfcs/handoffs/README.md`; `ls rfcs/handoffs/*.md \| grep -vc README` |

Re-derive E5–E7 before editing; my counts have been wrong twice this week.

## 4. Part A — implementation

```text
docs/book.toml               [preprocessor.mermaid], plus the mermaid JS in
                             additional-js beside theme/playground.js (E2)
docs/theme/                  the vendored assets `mdbook-mermaid install` writes
.github/workflows/docs.yaml  an install step beside the mdBook one (E3), pinned
                             with --locked the same way
```

**Verify by inspecting the built HTML, not by the build exiting 0.** Both pages must show a rendered
diagram, and the mermaid *source* in each must be unchanged — RFC-107 wrote those blocks to survive
exactly this change without a rewrite.

## 5. Parts B and C — the RFC does not pick, and you should not either

```text
BACKFILL   §4 gains five rows matching its existing columns; the handoffs index
           gains its 18 missing entries.
RETIRE     each is deleted, or frozen with a dated note naming the record that
           IS maintained — the document-history rows, and the directory itself.
```

**Recommendation: backfill, but only if it will be maintained.** §4's table already stopped once,
silently, for five releases — a gap reads as *"nothing shipped"* rather than *"nobody updated this"*,
which is worse than no table.

**If you form a view while doing it, say so in the review request.** You will have just written five
rows and eighteen entries; you are better placed than I am to judge whether either is worth keeping.

## 6. Out of scope

```text
any crate, any .rs file
the CONTENT of the two mermaid blocks — only rendering changes
diagrams on any other page
ROADMAP's document-history rows or §3.1
CHANGELOG.md — no crate change, so no release (RFC §8)
```

## 7. Acceptance criteria

```text
[ ] both diagrams render as DIAGRAMS — verified by inspecting docs/book/
[ ] the mermaid source in both pages is UNCHANGED
[ ] assets vendored under docs/theme/; no CDN reference anywhere
[ ] the added repository size stated
[ ] mdbook-mermaid pinned with --locked, matching the mdBook step (E3)
[ ] the docs workflow succeeds
[ ] Part B and Part C each backfilled, or retired with a dated note
[ ] no crate, no .rs file touched — assert via git diff --stat
[ ] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 8. Risks

```text
R1  A CDN reference (§2). Breaks offline reading; changes what the book is.
R2  "The build succeeded" mistaken for "the diagram rendered" (§4).
R3  Editing the mermaid source to make it render. If a block does not render,
    the renderer is misconfigured — the source is correct.
R4  An unpinned mdbook-mermaid, so CI drifts from local (E3).
R5  Backfilling a record nobody will maintain (§5). If that is the honest
    answer, retire it and say so.
R6  Scope creep: diagrams elsewhere, or a general docs-tooling upgrade.
```

## 9. Required review-request format

Write to:
`.git-exclude/review-request/RFC-116/matten-rfc116-docs-infrastructure-and-stale-records-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, the added
repository size, evidence that both diagrams render, your view on backfill-versus-retire, guard
output, deviations with reasoning, and anything you want answered at review.
