# RFC-124: A Simpler, Softer Mark

**Status:** **Accepted** 2026-08-28 by the owner. Performed by the high-capability model — asset
production and mechanical regeneration, **no Developer Handoff** (see `accepted/README.md`).
No version, no tag, no publish.
**Target:** `assets/`, `docs/theme/`
**Theme:** Replace the mark with one that matches the positioning it is meant to carry
**Related:** RFC-122 (the mark being replaced), RFC-094 §4.3 (no release), RFC-121 / RFC-123
(sequencing — no collision)

---

## 1. Summary

```text
Replace the 3×3×3 cube-in-a-palm with a 2×2×2 cube in a cradle: fewer elements,
softer shapes, one SVG as the source of truth.

FIVE FILES CHANGE. NO MARKUP CHANGES AT ALL — the filenames are unchanged, so
RFC-122's README hero, favicon wiring and book.toml all keep working untouched.
```

**No `crates/` change, so no release.** `assets/` and `docs/theme/` reach zero published packages.

## 2. Why — the owner's judgement, and the evidence that already supported it

The owner's assessment: the current mark is *"too complicated and intimidating"* against a stated goal
of a **family-like atmosphere**.

**That is a positioning call and it is the owner's to make.** But it was not unsupported, and this RFC
records where the supporting evidence already sat:

```text
RFC-122 E5 measured the mark FAILING below 64px — mush at 32, unidentifiable
at 16 — and identified the cause as 27 facelets separated by thin gaps.

That was read at the time as a RENDERING problem, to be solved with a second
simplified mark. It was equally a DESIGN signal: the density that defeats a
32px grid is the same density that reads as complicated at any size.
```

A reviewer concern about the mark's *semantics* — that a subdivided cube reads as *a puzzle:
difficult, to be solved*, against a library calling itself a family car — was raised at RFC-122's
review, challenged by the owner, and withdrawn. **The trademark half of that concern was correctly
withdrawn and stays withdrawn.** The semantic half is what this RFC acts on, and it was under-weighted
rather than wrong.

## 3. The mark

```text
2×2×2 cube, three faces, soft palette          the object
one round-capped arc cradling it from below    the warmth
nothing else                                   no sun, no swoosh, no detailed palm
```

**`[2,2,2]` is not a simplification of the idea — it is the smallest non-trivial rank-3 tensor**, the
first shape in which all three axes exist. It is the canonical teaching example and it happens to be
the gentler drawing. A plain undivided cube was considered and rejected: it is softer still but stops
meaning *array*, and plain cubes are generic in software.

The cradle carries the "held, cared for" sense the original palm carried, reduced to a single stroke
that survives at 32px — which the original palm did not.

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | The mark being replaced fails below 64px | RFC-122 E5, measured by rendering |
| E2 | The replacement reads at 32px, cradle included | rendered at 32px and inspected during design |
| E3 | `[2,2,2]` is the smallest rank-3 shape with all three axes present | definitional |
| E4 | Filenames are unchanged, so no markup edit is required | `README.md:3` references `assets/matten-logo.png`; mdBook theme overrides are by filename |
| E5 | The README's `alt` is already `"matten"`, not a description | RFC-122's review correction, `README.md:3` |
| E6 | `assets/` and `docs/theme/` reach **zero** published packages | `cargo package --list` per crate |
| E7 | `raw.githubusercontent.com` serves `.svg` as `image/svg+xml` | live `curl -sI` against the pushed asset |

### 4.1 E5 is worth dwelling on

RFC-122's review required changing the hero's `alt` from a description of the artwork
(*"A solved 3×3×3 cube… held in an open palm"*) to `alt="matten"`, on the argument that **a
description is a statement that goes false when the artwork changes.**

**The artwork is changing three days later.** Had that correction not been made, this RFC would be
fixing a false statement in the README as well. It is a small vindication of a small rule, and it is
recorded because the rule is the transferable part.

## 5. What changes

```text
assets/matten-mark.svg      REPLACED  — the new mark; source of truth
assets/matten-logo.png      REPLACED  — rendered from the SVG, transparent
assets/matten-social.png    REPLACED  — recomposed, wordmark unchanged, no tagline
docs/theme/favicon.svg      REPLACED  — copy of the mark
docs/theme/favicon.png      REPLACED  — 32×32, rendered from the mark
```

```text
UNCHANGED, and this is the point:
  README.md          same filename, same URL, alt already correct (E4, E5)
  docs/book.toml     theme overrides are by filename
  any crates/** file
```

## 6. On the SVG, and what it is for

The SVG is the **source of truth**; every PNG is derived from it. That removes the two-mark split
RFC-122 created — a detailed raster for large surfaces and a simple SVG for small — which existed
only because the detailed mark could not survive 32px. One mark now serves every size.

The README hero **stays a PNG**. E7 shows GitHub would serve the SVG correctly, but crates.io renders
that same README through its own HTML sanitiser, and a PNG is guaranteed on both. The SVG is
available to anyone who wants it, which is what was asked for.

## 7. Sequencing — no collision, may proceed in parallel

```text
RFC-121 (0.46.1)  touches README.md's version pins    — this touches NO markup
RFC-123 (badges)  touches README.md's table + a guard — this touches NO markup
```

**This RFC edits only binary/SVG asset files and changes no line of any document**, so it cannot
collide with either. It does not need to wait, and it does not block them.

## 8. Execution

Performed by the high-capability model, as with RFC-122's Changes A and A.1: this is asset production
and mechanical regeneration, with no placement work left to do. **No Developer Handoff** (see
`accepted/README.md`).

## 9. Scope

### Out of scope — a diff touching these is a defect

```text
README.md, docs/book.toml, any markup      — filenames unchanged (§5)
any crates/** file                          — no release
the wordmark                                — unchanged; only the mark changes
the tagline                                 — remains absent (RFC-122 §10 Q1)
a brand or style guide                      — one mark, same surfaces
the candidate artwork in .git-exclude/tmp/  — untracked; left as the owner's original
```

## 10. Risks

```text
R1  Renaming a file. Every filename must stay identical or RFC-122's placement
    silently breaks — a 404 hero and a stale favicon.
R2  Regenerating the social image with the tagline. It stays absent (Q1).
R3  Leaving docs/theme/favicon.* on the old mark. mdBook caches by content
    hash; verify the emitted names CHANGE, as RFC-122 did.
R4  Losing the transparent background, which is what makes one file work on
    both GitHub themes.
R5  Treating this as a release. It is not, and no crate content changes.
```

## 11. Acceptance criteria

```text
[ ] all five files replaced; every filename byte-identical to before
[ ] README.md, book.toml and all markup UNCHANGED — asserted by diff
[ ] the mark renders as a cube with its cradle at 32px — verified by looking
[ ] the logo PNG is transparent and checked on both #ffffff and #0d1117
[ ] the social image carries the wordmark and NO tagline
[ ] the built book's emitted favicon filenames CHANGE
[ ] git diff touches no crates/** path; cargo package --list unchanged
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 12. What this does not do

```text
- give docs.rs a logo. Still 0.47.0 (RFC-122 §9), and it now gets the better
  mark on its first appearance rather than one that would need replacing.
- set the social preview. Still an owner action in repository settings.
- change the wordmark, which was never the problem.
```
