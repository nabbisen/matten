# RFC-097 Report Demos on the Site: Implementation Handoff

**Status:** Issued 2026-08-02. Implementation authorized under RFC-097, accepted the same day.
**Design authority:** `rfcs/accepted/097-report-demos-on-the-site.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Publish `matten-report`'s five demos as generated Markdown pages in the book, and fix the rank-2
rendering they would otherwise publish.

**Order matters: fix the rendering first, generate second.** Generating pages from the current
renderer would commit the defect, and the fix would then show up as a diff in files that are supposed
to be mechanically derived — which is exactly the state §6's guard exists to make impossible.

This is a larger change than the last two rounds. Five renderer modules, five generated pages, a new
guard, and nav. Take it in the order below.

## 2. Fix the rank-2 rendering — but only where it is a defect

RFC-097 §5.1 is the part most likely to be over-applied. A grep for `values:` finds **13** across the
five demos. **Most of them are correct and must not change.**

```text
DEFECT — a rank-2 tensor flattened; the arrangement IS the information
  shape-flow    broadcast result [2,3]
  shape-flow    reshape result   [3,2]
  shape-flow    matmul result    [2,2]
  educational-path / mlprep-standardization: any rank-2 tensor values

CORRECT — leave exactly as they are
  shape-flow    mean_axis(0): [2.5, 3.5, 4.5]   a rank-1 result IS a list
  shape-flow    mean_axis(1): [2.0, 5.0]
  data-readiness / dynamic-readiness: column statistics — lists of per-column
                                       numbers, not matrices
```

If you cannot tell from the renderer whether a value is rank 2, that is the question to answer before
writing the change — not something to infer from how the list looks.

**Markdown renderers only.** `tools/matten-report/src/render/markdown/` — five modules, 460 lines.
The HTML and JSON renderers keep the flat form (RFC-097 §5.2). That inconsistency is deliberate and
recorded; do not "finish the job" by fixing them too.

Reuse the alignment approach from `tools/matten-playground/src/render.rs` — but note that crate
cannot be imported from here either, so this is a third local implementation. That is accepted; the
alternative was a public API nobody wants.

## 3. Generate, and COMMIT, the five pages

```text
docs/src/reports/<demo>.md   generated, COMMITTED, one per demo
docs/src/reports/index.md    hand-written, NOT generated — see §4
docs/src/SUMMARY.md          a Reports section listing all six
```

**Do not git-ignore the generated pages.** RFC-097 §6 has the reason and it was tested: with a
`SUMMARY.md` entry pointing at a missing file, **mdBook creates it empty and exits 0**. Git-ignoring
them means a failed generation step deploys five blank pages with nothing reporting a problem.

Generate with the tool's existing Markdown output:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo <kind> --output docs/src/reports/<kind>.md
```

## 4. The index page is hand-written, and says what these are not

`docs/src/reports/index.md` must not be generated, because it has to state things the tool does not
know:

```text
- these are FIXED DEMOS, not a live tool and not expression tracing
- matten-report is a local development tool, publish = false, not a published crate
- the JSON and HTML renderers exist but are not published (RFC-070 stands)
- how to run the tool yourself against your own CSV
```

Without that, a reader reasonably concludes `matten` ships a reporting product. RFC-070 declined
exactly that, and this page is the only place the distinction gets made to a reader.

## 5. The freshness guard

A new `scripts/check-report-demos.sh`. It is what makes committing generated content safe:

```text
regenerate each demo into a TEMP directory
diff against the committed docs/src/reports/<demo>.md
fail, naming the demo, if any differs
```

**Prove it can fail** (rule 002 §4, §6, §7): edit one committed page by a single character, confirm
the guard fails and names that demo, restore, confirm it passes. Report both.

Do not have the guard regenerate in place — a guard that silently fixes drift hides the drift.

Wire it into `.github/workflows/test.yaml` beside the other guards, and add it to
`docs/src/contributing/release-checklist.md`'s list.

## 6. Verification

```bash
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
bash tools/matten-report/tests/process-boundary.sh
bash tools/matten-report/tests/module-boundaries.sh
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh
mdbook build docs
wc -c docs/src/reports/*.md      # no page may be 0 bytes — that is the silent failure
git diff --name-only -- crates/  # expect EMPTY
```

The `wc -c` line is not a formality. An empty generated page is the exact failure §6 of the RFC is
built around, and it looks like success everywhere else.

The report tool's two harnesses must still pass. `module-boundaries.sh` runs deliberate-failure
self-tests; if your renderer change breaks a module boundary it will say so.

## 7. Known pitfalls

```text
- converting rank-1 lists to grids — most of the 13 are correct as they are (§2)
- fixing the HTML/JSON renderers too (§5.2 — deliberate inconsistency)
- git-ignoring the generated pages (§3 — silent blank deploy)
- generating the index page instead of writing it (§4)
- a guard that regenerates in place instead of failing (§5)
- generating pages BEFORE fixing the renderer, so the fix lands as a diff in
  derived files
- touching crates/** — nothing here does
```

## 8. What the review request must report

```text
- for each of the five demos: the rendered Markdown, verbatim, before and after
- an explicit list of which flat lists you changed and which you left, with the rank
  of each — this is the judgement call most likely to be wrong
- the guard's deliberate-failure proof, both directions
- wc -c for all six pages, showing none is empty
- full gate output, including the report tool's two harnesses
- git diff --name-only -- crates/ showing EMPTY
- confirmation that no tag was created and nothing was published
```

## 9. Review stop

Stop after committing. Report, and the high-capability model reviews before anything deploys.
**Pushing publishes these pages** — as with RFC-093 and RFC-095, landing the commit is the release.
Raise anything uncertain before committing rather than after.
