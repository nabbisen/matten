# RFC-097: Report Demos as Generated Book Pages

**Status:** **Accepted** 2026-08-02 — implementation authorized under
[the handoff](../handoffs/097-report-demos-on-the-site-handoff.md). Supersedes RFC-093 §8's HTML
sketch on both counts: Markdown rather than HTML, and the tool's own rendering fixed before
anything is published
**Target:** `docs/`, `tools/matten-report`; no published-crate change, no version, no release
**Theme:** RFC-093 §8's phase 2 — make the report tool's demos readable without a checkout
**Related:** RFC-069, RFC-070, RFC-071, RFC-073, RFC-093 §8, RFC-095, RFC-096, RFC-094

---

## 1. Summary

Publish `matten-report`'s five demos as **generated Markdown pages in the book**, and fix the tensor
rendering they would otherwise publish.

Two things here differ from what RFC-093 §8 sketched, both because investigating changed the answer:
§8 assumed **HTML**, and §8 did not know the tool carries the **same defect** RFC-095 and RFC-096
just removed from two other surfaces.

## 2. Markdown, not HTML

§8 said *"route the report tool's HTML output through the same site"*. The HTML is genuinely good —
self-contained, one inline `<style>`, no external links or scripts, 2.5–4.6 KB per demo. But it emits
**complete `<html>` documents**, and an mdBook page is a fragment. Publishing HTML therefore means
standalone files served beside the book, with their own styling, outside its nav, theme and search.

The tool also emits **Markdown containing zero raw HTML**, which mdBook renders as an ordinary page.

```text
HTML      standalone file, own <style>, outside the nav, own theme, not searchable
Markdown  a real book page: site nav, the reader's chosen theme, in the search index
```

Markdown is the better integration, and it also weakens the objection §3 has to answer: a generated
page inside the book is a smaller commitment than a separately-styled document deployed beside it.

## 3. Does this make the report tool a product surface? — the question §8 required

RFC-070 audited public visualization and reporting and authorized **none** of: a public
`matten-report` crate, a `matten-viz` crate, a reusable renderer API, a public report model API,
public JSON/SVG/Vega-Lite output, notebook or browser integration, or core `Tensor` visualization
methods.

Every one of those is a commitment to an **interface** — something a downstream user writes code
against and expects to keep working. This RFC commits to none:

| RFC-070 declined | This RFC |
|---|---|
| public `matten-report` crate | still `publish = false`, workspace-excluded |
| reusable renderer API, public report model | no library surface; only rendered text is published |
| public schemas | none — the JSON renderer is untouched and unpublished |
| public JSON / SVG / Vega-Lite output | none |
| browser integration | none — these are static pages, no tool runs |

**What is published is output, not an interface.** A reader can read a demo; nobody can build against
it. That distinction is the whole argument, and it is narrower than RFC-093's, which had to argue an
overlap with "browser integration" — here there is no overlap to argue.

**The honest cost:** publishing creates an expectation that these pages exist and stay accurate. That
is a maintenance commitment, and §6's guard is what makes it a cheap one rather than a promise
nobody checks.

## 4. Static demos only — the second §8 question, answered

§8 asked: *"static pre-generated demos only, or the tool running in the browser too?"*

**Static only, and the alternative is not close.** The tool is a CLI whose non-demo mode reads a CSV
from disk. Running it in the browser would require a second WebAssembly build **and file upload** —
which RFC-093 §5 forbids outright. That is a settled boundary, not a cost comparison.

## 5. The defect this must fix first

The tool prints rank-2 tensors as flat lists — the same defect RFC-095 fixed in the playground and
RFC-096 fixed in the example. Publishing without fixing it puts a third copy on a public page, on a
demo named `shape-flow`:

```text
## Reshape
input: shape [2, 3]
operation: reshape([3, 2])
shape flow: [2, 3] -> [3, 2]
result values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
```

**This is worse than the two already fixed.** Those printed input and output as identical lists, so a
reader could at least see something had not changed. This prints only the *result*, so the
rearrangement is not merely unclear — it is absent.

### 5.1 Not every flat list is a defect

The distinction matters, and getting it wrong would mean over-fixing:

```text
DEFECT   a rank-2 tensor as a flat list — the arrangement IS the information
  shape-flow: broadcast result [2,3], reshape result [3,2], matmul result [2,2]

CORRECT  a rank-1 tensor as a flat list — it IS a list
  shape-flow: mean_axis(0) [2.5, 3.5, 4.5], mean_axis(1) [2.0, 5.0]
  data-readiness / dynamic-readiness: column statistics
```

A grep for `values:` finds 13 across the five demos. Only the rank-2 ones are in scope, and the same
rank ≤ 2 rule RFC-096 used applies here.

### 5.2 Markdown renderer only

Only the Markdown output is published, so only the Markdown renderers are in scope — five modules,
460 lines total. The HTML and JSON renderers keep the flat form.

That leaves the tool internally inconsistent, which is a real cost and is accepted deliberately: HTML
and JSON are local-only output with no reader to mislead, and widening the fix to all three renderers
triples the diff and the test churn for no published benefit. Recorded here so a later reader sees a
decision rather than an oversight.

## 6. Generated pages must be committed, with a freshness guard

The playground precedent says git-ignore generated artifacts and build them in CI. **That is wrong
here, and the reason is a silent failure.**

Tested rather than assumed: with a `SUMMARY.md` entry pointing at a file that does not exist, mdBook
**creates it empty and exits 0**. So if generation failed or was skipped, the deploy would succeed
with five blank pages in the nav and nothing reporting a problem. A missing wasm asset degrades one
page visibly; a missing page is invisible.

So:

```text
COMMIT the five generated .md files, so the book always builds what it will deploy
GUARD  regenerate into a temp dir and diff against the committed copies; fail on drift
```

The guard is what makes committing generated content safe — without it, the pages rot the first time
someone changes the tool and forgets. Precedent: `check-benchmark-dependency-sync.sh` exists because
a hand-maintained cross-file value drifted once.

## 7. Scope

### In scope

```text
rank-2 grid rendering in the five Markdown demo renderers (§5)
five generated pages under docs/src/, committed, plus SUMMARY.md entries
a guard that regenerates and diffs them
a short hand-written index page explaining what the demos are and are not
```

### Out of scope — a diff touching these is a defect

```text
the HTML and JSON renderers (§5.2)
publishing HTML, SVG, JSON, or schemas — RFC-070 stands
any tool running in the browser, and any file upload (RFC-093 §5)
making matten-report publishable, or adding any public API anywhere
crates/** — nothing here touches a published crate
a version bump, tag, or publish
```

## 8. Risks

```text
1. GENERATED CONTENT IN GIT. Mitigated by §6's guard, which is the only thing standing
   between committed output and silent rot.
2. A LARGER DIFF THAN IT LOOKS. Five renderers plus five pages plus a guard plus nav.
   The rendering change is small per site but touches five modules with their own tests.
3. EXPECTATION. Published pages imply currency (§3). The guard covers accuracy against
   the tool; nothing covers whether the demos remain the right five.
```

## 9. Acceptance criteria

```text
[ ] rank-2 values render as an aligned grid in the five Markdown renderers; rank-1
    values are UNCHANGED flat lists (§5.1)
[ ] five demo pages generated, committed, listed in SUMMARY.md, and rendering in the book
[ ] a guard regenerates and diffs them, and is proven to FAIL on a deliberate edit
[ ] mdbook build produces no empty page (check the byte size of each generated page)
[ ] the HTML and JSON renderers are untouched, and their tests still pass
[ ] no published crate touched: git diff --name-only -- crates/ is empty
[ ] all guards pass, including the new one; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish, no version change
```

## 10. Non-goals

```text
reopening RFC-070's refusal of a public reporting or visualization crate
the report tool reading user data in the browser
fixing the HTML/JSON renderers' flat lists (§5.2 — deliberate)
retrofitting the demos themselves; this publishes the five that exist
```
