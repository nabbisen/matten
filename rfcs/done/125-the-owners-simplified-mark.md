# RFC-125: The Owner's Simplified Mark

**Status:** **Implemented** 2026-08-31. Five asset files replaced, every filename unchanged, so
`README.md` and `docs/book.toml` have empty diffs. mdBook's emitted favicon hashes changed
(`905a5a24`/`c7cc3629` → `d896f3e4`/`5baa6546`), verified against the built HTML. **Two deviations
from §7, both recorded in §12.1.** No version, no tag, no publish.
**Target:** `assets/`, `docs/theme/`
**Theme:** Adopt the owner's regenerated artwork — the original composition, cube simplified
**Related:** RFC-122 (first mark), RFC-124 (second, superseded), RFC-094 §4.3 (no release)

---

## 1. Summary

```text
Replace the mark with the owner's regenerated artwork: the ORIGINAL composition
— cream plate, two tan sweeps, warm sun, sage palm — with the cube simplified
from 3×3×3 (27 facelets) to 2×2×2 (8 cells).

FIVE FILES. NO MARKUP CHANGES — filenames unchanged, so RFC-122's README hero,
favicon wiring and book.toml keep working untouched, exactly as in RFC-124.
```

**No `crates/` change, so no release.**

## 2. Why there is a third logo RFC, stated plainly

```text
RFC-122   3×3×3 cube in a palm, full composition.
          Owner: "too complicated and intimidating" against a family positioning.

RFC-124   2×2×2 cube in a bare cradle. The cube was simplified — and the sun,
          the sweeps and the plate were STRIPPED, on the reviewer's reading that
          they were "visual noise without meaning".
          Owner: "the warm light ball and the soft wave surroundings should be
          preserved with the cube simplified."

RFC-125   what should have been done at RFC-124: simplify ONLY the cube.
```

**The error is worth recording because it was an error of scope, not of taste.** The measurement in
RFC-122 §E5 justified simplifying *the cube* — 27 facelets could not survive a 32px grid. It never
justified removing the surroundings, which carry the warmth the project wants and cost nothing at any
size. A reviewer generalised a narrow finding into a broad one.

A hand-authored SVG reproduction of the full composition was also attempted and **is not proposed**:
it reproduces the elements but not the character, because the sweeps and palm are organic forms that
flat vector fills approximate poorly. The owner's regenerated raster is better than any reproduction
of it.

## 3. Which candidate, and why

Two candidates were supplied. **`-1` is chosen: the subdivided 2×2×2 cube.** `-2` is a plain
undivided cube.

```text
MEANING          -1 shows EIGHT CELLS — an array. -2 shows a box.
                 And for a RUST library specifically, a plain cube reads as a
                 CRATE: the package unit, and crates.io's own iconography.
                 That is the single most likely misreading of -2, and it points
                 at the wrong concept entirely.

LEGIBILITY       measured at 16/32/64px: -1 holds BETTER, despite more detail.
                 Its warmer, more saturated faces keep separation where -2's
                 muted pastels merge. Detail lost at RFC-122 because 27 facelets
                 overwhelmed the grid; 8 cells with strong colour contrast do not.

WARMTH           -1's orange-and-yellow top is warmer than -2's muted amber,
                 and its palm is more present — which is the stated goal.

SIMPLIFICATION   8 cells against 27. The change the owner asked for is delivered
                 either way; -2 goes further than was asked.
```

**Two honest differences from the RFC-122 artwork**, recorded so they are not discovered later: `-1`'s
top face is more saturated than the rest of the palette, so it reads slightly more energetic; and it
carries soft drop-shadows and bevels the flat original did not.

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `-1` is 1254×1254, sRGB, **no alpha**, 1.32 MB | `magick identify` |
| E2 | `-1` reads at 16, 32 and 64px; `-2` is weaker at 16–32 | rendered at each size, point-filtered, and compared |
| E3 | Transparency extraction works: 512px RGBA, **27,844 bytes**, 128 colours | flood-fill from the corner, re-encoded, inspected on `#ffffff` and `#0d1117` |
| E4 | The extraction leaves a **faint plate outline along the bottom** | visible against `#0d1117` — anti-aliased edge not caught at 8% fuzz (§6) |
| E5 | Face colours ≈ `#FCAA6C` / `#FABF5B` top, `#94B0A3` left, lilac right; sun `#F8926F`; palm `#92B0A4`; plate `#FDF3E7` | sampled by averaging inside each face — **re-derive before authoring the SVG** |
| E6 | mdBook emits **both** `favicon.png` and `favicon.svg`; overriding only the PNG leaves Rust's default served | RFC-122 §C, verified then |
| E7 | `assets/` and `docs/theme/` reach **zero** published packages | `cargo package --list` per crate |

## 5. What changes

```text
assets/matten-logo.png      REPLACED  — from -1, transparent, 512px, 128 colours
assets/matten-social.png    REPLACED  — recomposed, wordmark unchanged, no tagline
assets/matten-mark.svg      REPLACED  — the favicon lockup (§6)
docs/theme/favicon.svg      REPLACED  — copy of the mark
docs/theme/favicon.png      REPLACED  — 32×32, from -1

UNCHANGED:  README.md, docs/book.toml, every crates/** file
```

## 6. The favicon, and why it is a different drawing

`favicon.svg` must exist or mdBook serves Rust's icon (E6). The full composition **cannot be
faithfully reproduced in SVG** — that was attempted and rejected in §2.

**So `matten-mark.svg` is a deliberate favicon lockup: the 2×2×2 cube alone**, in `-1`'s face colours,
no plate, no sun, no palm.

```text
This is not a compromise. At 16-32px the plate, sun and palm do not register
as distinct forms anyway — they become a coloured surround. Dropping them for
the favicon gives MORE legible pixels to the element that identifies the mark.
A favicon lockup differing from the full mark is ordinary practice.
```

**Re-derive E5's colours from `-1` before authoring** — do not copy them from this RFC.

## 7. The edge artifact must be fixed

E4: the corner flood-fill at 8% fuzz leaves a faint cream outline tracing the plate's bottom, visible
on dark backgrounds. **Widen the fuzz or cut the edge differently until it is gone, and verify against
`#0d1117` by looking** — not by assuming a higher number worked.

## 8. Scope

### Out of scope — a diff touching these is a defect

```text
README.md, docs/book.toml, any markup   — filenames unchanged (§5)
any crates/** file                       — no release
the wordmark                             — unchanged
the tagline                              — remains absent (RFC-122 §10 Q1)
redrawing the owner's artwork             — it is adopted as supplied
an SVG reproduction of the full mark      — attempted, rejected (§2)
```

## 9. Risks

```text
R1  Renaming a file. Every filename must stay identical or RFC-122's placement
    silently breaks — a 404 hero and a stale favicon.
R2  Shipping the bottom-edge artifact (§7). It is invisible on white and
    obvious on dark, so checking only the light theme will miss it.
R3  Copying E5's colours instead of re-deriving them from -1.
R4  Reproducing the full composition in the SVG. Rejected in §2; the favicon
    is the cube alone (§6).
R5  Regenerating the social card with the tagline. It stays absent.
R6  Leaving docs/theme/favicon.* on the old mark — verify the emitted hashes
    CHANGE, as RFC-122 and RFC-124 both did.
R7  Treating this as a release. No crate content changes.
```

## 10. Acceptance criteria

```text
[ ] all five files replaced; every filename byte-identical to before
[ ] README.md, book.toml and all markup UNCHANGED — asserted by diff
[ ] the logo PNG is transparent, and the §7 edge artifact is GONE, verified
    by looking at it against #0d1117
[ ] matten-mark.svg is the cube alone, in colours re-derived from -1
[ ] the mark reads at 32px — verified by rendering and looking
[ ] the social card carries the wordmark and NO tagline
[ ] the built book's emitted favicon filenames CHANGE
[ ] git diff touches no crates/** path; cargo package --list unchanged
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 11. What this does not do

```text
- give docs.rs a logo. Still 0.47.0 — and it now receives a settled mark on
  its first appearance rather than the third revision of one.
- set the social preview. Still an owner action in repository settings.
- change the wordmark, which was never in question.
```

**RFC-124 is superseded three days after it closed.** That is the cost of having read a narrow
measurement as a broad mandate, and it is cheap only because every logo change so far has stayed
outside the published packages.

---

## 12.1 Deviations found during implementation

**§7's prescribed fix did not work, and could not have.** It asked for a wider flood-fill fuzz until
the plate's bottom edge artifact disappeared. Measured:

```text
plate cream   #FCF3E7
white surround #FDFEFE      -> they differ by ~2%
```

No fuzz value separates them: below ~6% the artifact survives; above it, the flood-fill reaches
*through* the plate and starts eating the tan sweeps. Rendered at 8%, 14% and 20% against `#0d1117`
to confirm — 14% already showed notches bitten out of the sweeps.

**What was done instead:** the plate boundary was located *geometrically* rather than by colour. Its
bounding box was measured by scanning for the cream signature (`r - b >= 12`) along a row and a
column — `x = 18..1233`, `y = 17..1237`, i.e. **the plate nearly fills the frame and the white sits
only in the corners**, which is why colour-trim could not find it. The mark is now that crop with a
rounded-rectangle alpha mask at radius 118. No colour matching, so no erosion, and the artifact is
gone — verified by looking at `#0d1117`, not by assuming.

**Consequence, and a visible design difference worth stating:** this **retains the cream plate**,
where RFC-122 and RFC-124 removed it and let the mark float. Removal is not available for this
artwork — plate and surround cannot be told apart by colour — and the artwork is drawn as an app
icon, so it is kept as one: a rounded tile with transparent corners. It reads correctly on both
`#ffffff` and `#0d1117`.

