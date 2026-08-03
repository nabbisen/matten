# RFC-095: Two-Dimensional Matrix Rendering in the Playground

**Status:** **Implemented** 2026-08-02 in commit *"Render matrices as a grid in the playground
(RFC-095)"*, reviewed and approved after one correction. Rank ≤ 2 renders as an aligned grid;
rank > 2 is byte-identical to the pre-RFC-095 flat form, proven at the source rather than by
sampled output. RFC-093 §6 was amended at acceptance. C1 was the nav still repeating —
`Playground › Playground` rather than the old `Playground › Shape playground` — fixed by folding
the page into `New users` and deleting the one-item section. Unreleased and unversioned, but the
page deploys from a push to `main`, and the `README.md` link reaches crates.io at the next release
**Target:** Playground crate only; no published-crate change, no version, no release
**Theme:** Show a matrix as a matrix, so reshape and broadcasting stop being invisible
**Amends:** RFC-093 §6 — argued in §3, not assumed
**Related:** RFC-002, RFC-020, RFC-070, RFC-093 §8, RFC-069/071/073 (report tool conventions)

---

## 1. Summary

Render rank-2 tensors as aligned rows and columns in the shape playground, instead of a flat value
list.

**This RFC argues against RFC-093 §6 by name, as that section requires.** It does not claim the
existing rule already permits a grid. It proposes replacing the rule's *wording* with a sharper line
that admits this and still refuses everything §6 was written to refuse.

## 2. The defect this fixes, in the page's own output

Reported by the owner from the deployed page. Reshape currently prints:

```text
input            shape=[2, 3] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
reshaped         shape=[3, 2] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
```

**Two identical value lists.** The one thing reshape teaches — that the arrangement changes while the
row-major order does not — is precisely what this output hides. A reader could reasonably conclude
reshape does nothing.

The same flaw affects broadcasting, where *"right repeats along axis 0"* is a claim the reader must
take on trust rather than a fact they can see.

Proposed instead:

```text
input     shape=[2, 3]        reshaped  shape=[3, 2]
  1  2  3                       1  2
  4  5  6                       3  4
                                5  6
```

The row-major rule becomes self-evident. Nothing is asserted that the numbers do not already show.

## 3. Arguing RFC-093 §6

§6 says, in full force:

> the playground renders text, and only text. Numbers, shapes, and prose glosses. No pixels that
> represent data. A change that draws data — bars, axes, lines, colour scales, an SVG element — is
> out of scope regardless of how small.

**The rule's purpose is right and is not in question.** The named path — *"shapes → a little bar for
each value → a chart → a chart library → a rendering API"* — is real, and RFC-070 declined the
endpoint. Nothing here reopens that.

**But "text only" does not draw the line the rule intends.** A bar chart made of `#` characters is
text. Under a literal reading it is permitted, while a grid of plain numbers is arguably forbidden as
"drawing". The wording admits the thing §6 exists to stop and excludes something harmless — so it is
the wording that is wrong, not the intent.

**Proposed replacement line — representation versus visualization:**

```text
REPRESENTATION (permitted): showing the tensor's own structure. Rows as rows, columns as
  columns, numbers as numbers. This is how mathematics writes a matrix and how NumPy prints
  one. It adds no information that is not already in the shape and the values.

VISUALIZATION (forbidden): encoding values as visual MAGNITUDE or colour. Bars, sparklines,
  heat maps, axes, lines, colour scales, SVG, canvas. Forbidden regardless of the medium —
  a bar chart drawn in `#` characters is visualization and stays out of scope.

THE TEST: does the rendering encode a value as something other than that value?
  A grid does not. A bar does, however it is drawn.
```

This is a **narrower** rule than "text only", not a wider one: it newly forbids ASCII charts, which
the current wording permits. The `matten-viz` path is cut at the same place §6 cut it, and cut more
cleanly.

## 4. Conventions are inherited, not invented

`tools/matten-report` already answered three of the four open questions. A second set of answers would
put two surfaces on the same site — RFC-093 §8's phase 2 — disagreeing about what a tensor looks like.

```text
MAX_DISPLAY_COLUMNS       = 12      truncate columns beyond this
MAX_TENSOR_PREVIEW_VALUES = 12
format_fixed_value                  {:.3}, with |v| < 0.0005 clamped to 0.0
```

That clamp is load-bearing and easy to drop: it exists so a tiny negative does not render as `-0.000`.

The playground adopts all three verbatim. If they are wrong, they are wrong in both places and should
change in both — which is the point of sharing them.

## 5. Scope

### In scope

```text
rank 2  -> aligned grid, right-aligned columns, one space of padding
rank 1  -> single row, same formatter
rank 0  -> the scalar
rank >2 -> UNCHANGED: the existing flat `values=[...]` list, with the shape line
truncation past 12 columns, marked, per §4
```

### Out of scope — a diff touching these is a defect

```text
core matten's Debug or Display. RFC-020 lists "Debug output for tensor" as a governed
  diagnostic; changing it is that RFC's territory and every user's debug output
a public formatting API in any published crate — RFC-002 minimalism applies
the visual_* examples. They have the same flaw and are a separate decision (§8)
bars, charts, colour, SVG, canvas — forbidden by §3's replacement rule
the report tool, its HTML, and phase 2
```

## 5.1 Page presentation and discoverability (added 2026-08-02)

Added at the owner's request after using the deployed page. Presentation only — no new capability,
no change to what is computed.

**The page buries its own purpose.** It opens with 28 lines before the first input: a four-line
intro, a **nineteen-line blockquote** on building the WebAssembly module locally, and a five-line
paragraph restating the scope rule. Both blocks are meta-information for people working *on* the
page, placed above the thing the page is *for*. A reader arrives and sees contributor instructions.

```text
now                              wanted
  title                            title
  intro                            one orienting sentence
  19-line build note               THE FOUR FORMS
  scope-rule paragraph             ---
  the four forms                   Notes for contributors
                                     build note, scope rule
```

**There is no CSS.** `book.toml` declares `additional-js` only, so the controls are unstyled: labels
do not align, inputs vary in width, and each Run button sits flush against the left margin.

**Nothing links to it.** `README.md`, `introduction.md` and `quick-start.md` do not mention the page;
the book's navigation is its only route in. The nav also reads `Playground › Shape playground`, which
is redundant.

### Decisions

```text
ORDER      title, one orienting sentence, the four forms, then a "Notes for contributors"
           section at the bottom holding the build note and the scope-rule restatement

NAME       "Shape playground" -> "Playground", and the nav section stops repeating itself.
           "Shape" was a scope signal, but RFC-093 §6 now states the boundary explicitly and
           a title is a weak place to keep a lock. The opening sentence says what it covers.

STYLE      docs/theme/playground.css via [output.html] additional-css. Aligned label/input
           rows, consistent input widths, the Run button not flush left, styled output blocks.

REACH      link from README.md, introduction.md and quick-start.md; move the Playground nav
           section above Tutorial.
```

### The one constraint on styling

RFC-093 §6, as amended by §3, still applies to CSS. **Styling chrome is permitted; styling that
encodes a value is not.** A colour scale keyed to magnitude, a bar whose width tracks a number, a
cell shaded by sign — all are visualization however they are produced, and CSS is a medium like any
other. Layout, spacing, borders, and monospacing are chrome.

### Theme compatibility is a requirement, not a nicety

mdBook ships **light, rust, coal, navy and ayu**. Hardcoded colours look broken in at least three of
them. The stylesheet must use mdBook's own CSS variables so the page follows the reader's theme.

### Note on README.md

`README.md` ships inside the published `matten` crate, so this link becomes crates.io front-page
content at the next release. That is intended — it is the one route that reaches people who never
open the book — but it is the only part of this section that leaves the documentation site.

## 6. Why rank > 2 is left alone

A three-dimensional tensor has no honest two-dimensional arrangement. Rendering it as stacked blocks
invents a reading order the data does not have, and the flat list at least does not mislead. The
existing output stays.

This is a boundary, not a deferral: it is not "grids for higher ranks later", it is "the grid is the
right answer for the case where a grid is what the data is".

## 7. Risks

```text
1. PRECEDENT. The §6 wording change is what future work will cite. §3 is written to be
   cited accurately: it narrows the rule. Anyone quoting it to justify a chart is
   quoting it against its plain text.
2. ALIGNMENT BUGS ARE SILENT. A mis-padded column still renders; it just misleads.
   Tests must assert exact strings, not "contains the values".
3. TWO SURFACES, ONE CONVENTION. If the report tool later changes its constants and the
   playground does not, they diverge. §4 makes them shared by intent; nothing enforces
   it, and that is recorded rather than solved.
```

## 8. Recorded, not authorized

```text
the visual_* examples in crates/*/examples print the same flat format and have the same
  defect. Whether they should share this rendering is a separate decision — they are
  shipped teaching artifacts in published crates, which is a different bar
core Debug/Display pretty-printing, e.g. honouring {:#?} — RFC-020 territory
```

## 9. Acceptance criteria

```text
[ ] rank-2 output is an aligned grid; rank-1 a row; rank-0 the scalar
[ ] rank >2 output is UNCHANGED from today
[ ] the three report-tool constants are used, including the -0.000 clamp
[ ] tests assert exact rendered strings, including padding
[ ] RFC-093 §6 is amended with §3's rule, and docs/src/playground.md's restatement of
    it is updated to match — the page currently states the old wording
[ ] the four forms appear BEFORE the build note and the scope paragraph (§5.1)
[ ] the page and nav are renamed to "Playground", without repetition
[ ] playground.css exists, is wired via additional-css, and uses mdBook theme
    variables — verified by eye in a dark theme, not only the default
[ ] no CSS encodes a value: no magnitude-keyed colour, width, or shading (§5.1)
[ ] README.md, introduction.md and quick-start.md link to the page
[ ] no published crate touched: git diff --name-only -- crates/ is empty
[ ] all seven guards pass; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish, no version change
```

## 10. Non-goals

```text
reopening RFC-070's refusal of a public visualization crate
any chart, in any medium, including ASCII
making the playground a data-inspection tool rather than a teaching page
```
