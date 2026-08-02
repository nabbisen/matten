# RFC-095 Matrix Grid Rendering: Implementation Handoff

**Status:** Issued 2026-08-02. Implementation authorized under RFC-095, accepted the same day.
**Scope widened 2026-08-02** (RFC-095 §5.1): the page's presentation and discoverability are now
part of this work — see §5a below. Widened before implementation began.
**Design authority:** `rfcs/accepted/095-matrix-grid-rendering.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Render rank-2 tensors in the shape playground as aligned rows and columns instead of a flat value
list, so reshape stops printing two identical lists and broadcasting becomes visible rather than
asserted.

**The scope rule has already changed — you are working under the new one.** RFC-093 §6 was amended at
acceptance and now reads representation-permitted / visualization-forbidden. Read that amendment
before writing rendering code; it is the boundary your work sits directly against.

**One thing you must not infer from it:** the new rule is *narrower*, not a relaxation. It newly
forbids ASCII charts, which the old "text only" wording permitted. If any part of this work starts
encoding a value as a length, a repeat count, or a symbol density, it is out of scope.

## 2. Inherit the display constants — do not choose new ones

`tools/matten-report/src/render/common.rs` already fixed these:

```rust
MAX_DISPLAY_COLUMNS       = 12    // truncate columns past this, and mark it
MAX_TENSOR_PREVIEW_VALUES = 12
// format_fixed_value: {:.3}, with |v| < 0.0005 clamped to 0.0
```

Use all three. The clamp is not cosmetic — it exists so a tiny negative renders as `0.000` rather
than `-0.000`, and it is the easiest of the three to drop by accident.

Reason this matters beyond tidiness: RFC-093 §8's phase 2 would put the report tool's HTML on the same
site as this page. Two surfaces there must not disagree about what a tensor looks like. The playground
crate is workspace-excluded and cannot import from `tools/matten-report`, so this is a deliberate
duplication of *values* — copy them with a comment naming the source, so the next person finds both.

## 3. What to render

```text
rank 2   aligned grid, right-aligned columns
rank 1   a single row, same formatter
rank 0   the scalar alone
rank >2  UNCHANGED — the existing flat `values=[...]` list plus the shape line
```

Rank >2 is a boundary, not a deferral (RFC-095 §6). Do not add stacked-block rendering for rank 3
"while you are in there" — a 3-D tensor has no honest 2-D arrangement, and inventing a reading order
is worse than the flat list.

Column width is the widest rendered cell in that column, right-aligned, so signs and decimals line up:

```text
input     shape=[2, 3]
   1.000   2.000   3.000
   4.000  -5.000   6.000
```

Keep the existing `shape=[...]` line and the plain-language `meaning` gloss. The grid replaces the
`values=[...]` list for rank ≤ 2; it does not replace the surrounding explanation.

## 4. Tests must assert exact strings

RFC-095 §7 names this: **a mis-padded column still renders, it just misleads.** A test asserting
"output contains 1.000" passes on broken alignment.

```text
assert_eq! on the FULL rendered block, padding included
cover: rank 0, rank 1, rank 2 square, rank 2 non-square, negative values,
       a value that triggers the -0.000 clamp, >12 columns (truncation marked),
       and rank 3 asserting the flat form is UNCHANGED
```

That last one is a regression test for the boundary, not a formality.

## 5. The page must be updated to match

`docs/src/playground.md` currently restates the **old** rule verbatim:

> The output is text only: shapes, values, and a plain-language gloss. **No charts, no pixels that
> represent data** … A change that draws anything needs its own RFC that argues against RFC-093 §6 by
> name.

Rewrite that paragraph to state the amended rule — representation permitted, visualization forbidden
in any medium, with the "does it encode a value as something other than that value" test. Keep the
final sentence's force: a change that crosses it still needs its own RFC arguing §6 by name.

Leaving the page on the old wording would put the deployed site in direct contradiction with the RFC
it cites, which is the defect class found on `introduction.md` during the `0.42.0` release.

## 5a. Page presentation (RFC-095 §5.1)

Same commit as the rendering work — the two touch the same file and must not be split across two
sessions.

**Reorder first; it is the change with the most effect.** The page currently puts 28 lines of
contributor prose above the first input: the nineteen-line WebAssembly build blockquote and the
scope-rule paragraph. Move both to a `## Notes for contributors` section at the **bottom**. What
remains at the top is the title, one sentence saying what the page covers, and then the four forms.

Do not delete either block. The build note is the only place the manual `wasm-bindgen` step is
written down, and the scope paragraph is required by §5 of this handoff to be rewritten — it just
belongs after the forms, not before them.

**Rename** `# Shape playground` to `# Playground`, and fix `SUMMARY.md` so the nav stops reading
`Playground › Shape playground`. Keep the filename `playground.md` — the deployed URL
`playground.html` must not change; it has been given out.

**Style** via a new `docs/theme/playground.css`, wired with `additional-css` in `docs/book.toml`
alongside the existing `additional-js`:

```text
label/input rows aligned, inputs of consistent width
the Run button no longer flush against the left margin
output <pre> visually distinct from the inputs
```

Two hard constraints:

```text
1. USE MDBOOK'S CSS VARIABLES. The book ships light, rust, coal, navy and ayu. Hardcoded
   colours look broken in at least three. Check one dark theme by eye before reporting.

2. NO CSS MAY ENCODE A VALUE. RFC-093 §6 as amended applies to stylesheets too: a colour
   scale keyed to magnitude, a bar whose width tracks a number, a cell shaded by sign are
   all visualization, whatever produces them. Layout, spacing, borders, monospacing are
   chrome and are fine.
```

**Discoverability:** add a link from `README.md`, `docs/src/introduction.md` and
`docs/src/quick-start.md`, and move the `# Playground` nav section above `# Tutorial`.

`README.md` ships in the published crate, so that line reaches crates.io at the next release. Keep it
to one sentence and a URL; it is the only part of this work that leaves the documentation site.

## 6. Verification

```bash
cargo test --manifest-path tools/matten-playground/Cargo.toml
cargo clippy --manifest-path tools/matten-playground/Cargo.toml -- -D warnings
cargo fmt --manifest-path tools/matten-playground/Cargo.toml --check
cargo build --manifest-path tools/matten-playground/Cargo.toml --target wasm32-unknown-unknown --release
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh     # rule 002 §8
mdbook build docs
git diff --name-only -- crates/    # expect EMPTY
```

`RUSTFLAGS="-D warnings"` is not optional — omitting it is what broke CI on 2026-08-01.

## 7. Known pitfalls

```text
- encoding a value as length/repetition/density — forbidden by the amended §6
- adding rank-3 block rendering because it seems natural (RFC-095 §6)
- choosing your own column cap or float format instead of §2's
- dropping the |v| < 0.0005 clamp, so -0.000 appears
- tests that check "contains" rather than the exact padded block
- leaving docs/src/playground.md quoting the superseded rule (§5)
- touching core matten's Debug — RFC-020 governs it, explicitly out of scope
- renaming the FILE, which would break the deployed playground.html URL
- hardcoded colours that break the coal/navy/ayu themes
- deleting the build note instead of moving it — it is the only record of the step
```

## 8. What the review request must report

```text
- the rendered output for each tested rank, verbatim
- confirmation the three report-tool constants are used, and where they are cited
- the rank-3 regression test showing the flat form is unchanged
- the rewritten playground.md paragraph, quoted
- the new page order, as a heading list, showing forms above the contributor notes
- a screenshot or description of the styled forms in a DARK mdBook theme
- the three links added, quoted
- full gate output, seven guards, check-doc-code.sh under -D warnings
- git diff --name-only -- crates/ showing EMPTY
- confirmation that no tag was created and nothing was published
```

## 9. Review stop

Stop after committing. Report, and the high-capability model reviews before anything deploys.
**Pushing to `main` publishes the page** — as with RFC-093, landing the commit is the release. Raise
anything uncertain before committing rather than after.
