# RFC-116: Render the Diagrams, and Repair Two Unmaintained Records

**Status:** Proposed
**Target:** `docs/` + one build dependency + one CI step; `ROADMAP.md`, `rfcs/handoffs/README.md`
**Theme:** Make the diagrams diagrams, and stop two indexes lying by omission
**Related:** RFC-093, RFC-094, RFC-107

---

## 1. Summary

```text
A. mdbook-mermaid, so the two diagrams RFC-107 wrote actually render.
B. Backfill ROADMAP §4's release table — FIVE shipped releases have no row.
C. Backfill or retire rfcs/handoffs/README.md — 79 rows for 97 files.
```

## 2. Part A — the diagrams are code blocks

RFC-107 deliberately wrote two ```` ```mermaid ```` blocks and deliberately did **not** add a renderer,
on the reasoning that Mermaid source degrades to readable text and enabling it later needs zero
content rewrite. That reasoning holds and this RFC is the "later".

Measured: mdBook renders the blocks as `class="language-mermaid"` — a code block, not a diagram.

### 2.1 What it takes

```text
docs/book.toml       a [preprocessor.mermaid] section, plus mermaid.min.js in
                     additional-js — the file already lists theme/playground.js,
                     so the mechanism exists (book.toml:14)
.github/workflows/docs.yaml   an `mdbook-mermaid` install step beside the existing
                     `cargo install mdbook` (docs.yaml:34)
docs/theme/          the mermaid JS/CSS assets mdbook-mermaid installs
```

**This is a new build dependency in CI.** That is the cost RFC-107 declined to pay without a reason;
the reason is that two diagrams now exist and read as source.

### 2.2 The constraint that decides the approach

**The assets must be vendored, not fetched at page load.** The book is built in CI and served as
static files; a CDN reference would make the page depend on a third party at read time and would fail
offline. `mdbook-mermaid install` writes the assets into `docs/theme/`; commit them.

Check what that adds to the repository and **state the number** — if it is large, say so and let the
owner weigh it before it lands.

## 3. Part B — ROADMAP §4's release table

It stops after `v0.41.0`. Five shipped releases have no row:

```text
v0.42.0  v0.43.0  v0.44.0  v0.45.0  v0.46.0
```

Each *is* recorded — in the document-history rows, which are maintained. So the table is not the only
record, and that is exactly why it rotted: nothing depended on it.

**Two honest options, and the RFC does not pick:**

```text
BACKFILL   five rows, matching the existing columns. Keeps a scannable
           release-by-release view that the history rows do not give.
RETIRE     delete the table, or freeze it with a dated note saying the history
           rows are the record from v0.42.0 on.
```

**Recommendation: backfill**, but only if someone will maintain it. A table that stops again in three
releases is worse than no table, because the gap looks like "nothing shipped" rather than "nobody
updated this".

## 4. Part C — the handoffs index

`rfcs/handoffs/README.md` has **79 rows** against **97 files**. Every recent handoff is listed; the
18 missing are historical, from the RFC-033–042 era.

Same choice as Part B, same recommendation, same condition.

## 5. Scope

### In scope

```text
docs/book.toml                     the preprocessor section
docs/theme/                        vendored mermaid assets
.github/workflows/docs.yaml        the install step
ROADMAP.md §4                      five rows (Part B)
rfcs/handoffs/README.md            the missing rows (Part C)
```

### Out of scope — a diff touching these is a defect

```text
any crate, any .rs file
the CONTENT of the two mermaid blocks — they are correct; only rendering changes
ROADMAP's document-history rows, or §3.1
adding diagrams to any other page
CHANGELOG.md — no crate changes, so no release (§7)
```

## 6. Risks

```text
R1  A CDN reference instead of vendored assets (§2.2). Breaks offline reading and
    adds a third-party dependency at page-load time.
R2  The mermaid blocks rendering differently than intended — verify BOTH pages in
    the built HTML, not just that the build succeeds.
R3  Backfilling a table nobody will maintain (§3). If that is the honest answer,
    retire it instead and say so.
R4  A CI step that works locally and not in the workflow. The docs job pins
    mdBook with `--locked`; pin mdbook-mermaid the same way.
R5  Scope creep into adding diagrams elsewhere.
```

## 7. Acceptance criteria

```text
[ ] both mermaid blocks render as DIAGRAMS in the built HTML — verified by
    inspecting docs/book/, not by the build exiting 0
[ ] assets are vendored under docs/theme/, not fetched from a CDN
[ ] the added repository size is stated
[ ] mdbook-mermaid pinned with --locked, matching the mdBook step
[ ] the docs workflow succeeds
[ ] Part B: five rows added, or the table retired with a dated note
[ ] Part C: the index completed, or retired with a dated note
[ ] no crate, no .rs file touched — assert via git diff --stat
[ ] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 8. This produces no release

No `crates/` change, so under RFC-094 no release is triggered — the same as RFC-115. Both were
described as release themes; neither will produce one.

## 9. Non-goals

```text
diagrams on any page other than the two RFC-107 wrote
a general docs-tooling upgrade
§3.1, the document-history rows, or any other ROADMAP section
```
