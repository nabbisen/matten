# RFC-126: The Favicon Should Be the Mark

**Status:** **Implemented** 2026-08-31. `docs/theme/favicon.svg` now carries the artwork embedded at
a 128px source (6,726 bytes); `favicon.png` is 32×32 downscaled from `assets/matten-logo.png`;
`assets/matten-mark.svg` is deleted after re-deriving that nothing consumes it. The emitted favicon
rendered at 32px was compared against `assets/matten-logo.png` at 32px and **matches**. Hashes moved
`d896f3e4`/`5baa6546` → `cb2a668f`/`0458a13a`. No version, no tag, no publish.
**Target:** `docs/theme/`, `assets/`
**Theme:** Serve the actual artwork in the browser tab, and delete the file that displaced it
**Related:** RFC-125 §6 (the decision being reversed), RFC-122 §C, RFC-094 §4.3 (no release)

---

## 1. Summary

```text
docs/theme/favicon.svg   REPLACED  — the artwork, embedded; currently a vector
                                     drawing that is not the mark
docs/theme/favicon.png   REPLACED  — 32×32 from assets/matten-logo.png
assets/matten-mark.svg   DELETED   — referenced by nothing

No markup changes. No crates/ change, so no release.
```

## 2. Why — the icon browsers actually serve is not the logo

mdBook emits two declarations:

```html
<link rel="icon"          href="favicon-….svg">   <- modern browsers use THIS
<link rel="shortcut icon" href="favicon-….png">   <- legacy fallback
```

The `.svg` is RFC-125's hand-authored lockup: a bare 2×2 cube, no plate, no sun, no palm. **So the
browser tab shows a drawing that is not the project's mark**, on the one surface whose entire job is
identification.

`assets/matten-mark.svg` compounds it: **nothing references it.** Not `README.md`, not `book.toml`,
not any workflow — only RFC records, which describe it rather than consume it. It exists solely as the
source the favicons were derived from. Once they come from the artwork, it has no purpose, and a
logo-shaped file in `assets/` that is not the logo is a trap for the next reader.

## 3. The reasoning being reversed, stated plainly

RFC-125 §6 justified the simplified lockup:

> *"At 16-32px the plate, sun and palm do not register as distinct forms anyway … Dropping them for
> the favicon gives MORE legible pixels to the element that identifies the mark."*

**That was asserted, not measured, and it is false.** Rendered side by side at 32px, the full artwork
is *more* legible than the lockup, not less — the plate supplies a silhouette and the sun a
distinguishing accent, and both survive the downscale (E3).

This is the third time in the logo sequence that a claim about the artwork was reasoned rather than
rendered — the sun and sweeps called "noise" (RFC-124), the 27-facelet measurement generalised beyond
the cube (RFC-125 §2), and now this. The transferable rule is the same each time: **render it and
look, before writing the justification.**

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | The `.svg` is served under `rel="icon"`; the `.png` only as `rel="shortcut icon"` | `grep '<link[^>]*favicon' docs/book/index.html` |
| E2 | `assets/matten-mark.svg` is referenced by **no** document, config, or workflow | `grep -rn 'matten-mark'` across `*.md/*.toml/*.yaml/*.rs`, excluding `rfcs/` — zero consumers |
| E3 | The artwork at 32px reads **better** than the lockup at 32px | both rendered at 32px, magnified, compared side by side |
| E4 | An SVG embedding the artwork costs `4,559` / `6,561` / `10,633` bytes at a 96 / 128 / 192px source | built and measured |
| E5 | That SVG renders faithfully at both 200px and 32px | rendered and inspected |
| E6 | `assets/` and `docs/theme/` reach **zero** published packages | `cargo package --list` per crate |

## 5. What changes

```text
docs/theme/favicon.svg   the artwork embedded as a data URI, 128px source (~6.5 KB)
docs/theme/favicon.png   32×32, downscaled from assets/matten-logo.png
assets/matten-mark.svg   deleted

UNCHANGED:  assets/matten-logo.png, assets/matten-social.png, README.md,
            docs/book.toml, every crates/** file
```

## 6. Why an embedded raster, and the tradeoff stated honestly

**There is no faithful vector of this artwork, because the artwork is raster.** A vector version can
only ever be an approximation, and an approximation is exactly what is being removed. The real choice:

```text
faithful but raster-backed   <- chosen. Correct at every size a favicon is used.
scalable but wrong           <- what is there now.
```

A 128px source covers 16, 32, 48 and 64px with room to spare; favicons are not used larger. The file
is an SVG in form — which satisfies mdBook's `.svg` slot — and carries the true mark in content.

**This does not deliver a scalable vector logo**, and the RFC should not be read as claiming one.
If a genuine vector mark is ever wanted, it needs to be drawn as vector art from the start, not
traced from a raster after the fact.

## 7. Scope

### Out of scope — a diff touching these is a defect

```text
assets/matten-logo.png, assets/matten-social.png  — correct as they are
README.md, docs/book.toml, any markup             — nothing references the favicon by name
any crates/** file                                 — no release
redrawing or re-approximating the mark             — §6
RFC-121 / 0.46.1, RFC-123                          — untouched; this collides with neither
```

## 8. Risks

```text
R1  Deleting assets/matten-mark.svg while something still references it.
    E2 says nothing does — RE-DERIVE that before deleting, not from this RFC.
R2  Leaving docs/theme/favicon.svg on the old drawing. It is the one browsers
    use; getting the .png right and the .svg wrong fixes nothing visible.
R3  Verifying by file existence instead of by looking. Render the emitted
    favicon and compare it to assets/matten-logo.png.
R4  Embedding at too low a source resolution. 128px minimum (E4).
R5  Treating this as a release. No crate content changes.
```

## 9. Acceptance criteria

```text
[ ] docs/theme/favicon.svg carries the artwork; rendered at 32px it matches
    assets/matten-logo.png at 32px — verified by looking, not by size
[ ] docs/theme/favicon.png is 32×32, downscaled from assets/matten-logo.png
[ ] assets/matten-mark.svg deleted, after re-deriving that nothing references it
[ ] the built book's emitted favicon filenames CHANGE
[ ] README.md, book.toml and all markup UNCHANGED — asserted by diff
[ ] git diff touches no crates/** path; cargo package --list unchanged
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 10. What this does not do

```text
- change the mark itself. RFC-125's artwork is correct and stays.
- deliver a scalable vector logo (§6).
- give docs.rs a logo. Still 0.47.0.
- set the social preview. Still an owner action.
```
