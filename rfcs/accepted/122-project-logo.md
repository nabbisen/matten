# RFC-122: Project Logo — Assets and the Unpublished Surfaces

**Status:** **Accepted** 2026-08-28 by the owner, with a **split execution**: Changes A and A.1 (the
assets and the small mark) are performed by the high-capability model; Changes B, C and D are handed
over with the assets already in place. Handoff: `rfcs/handoffs/122-project-logo-handoff.md`.
Both §10 questions are answered. No version, no tag, no publish.
**Target:** `assets/` (new, workspace root), `README.md`, `docs/theme/`, `docs/book.toml`
**Theme:** Give the project a face on the surfaces that need no release, and keep it out of the
published packages
**Related:** RFC-094 §4.1 (as amended by RFC-120), RFC-116 (repository weight), RFC-121

---

## 1. Summary

```text
A  add assets/ at the WORKSPACE ROOT — outside every published package
B  README.md gains a hero mark, light/dark, by absolute URL
C  mdBook gets a real favicon — BOTH png and svg, or the default leaks
D  a GitHub social-preview image (repository setting, not a file)

OUT: docs.rs. `html_logo_url` is published crate content and a logo is not a
     correctness fix, so it cannot ride a patch. It waits for 0.47.0 (§9).
```

**No `crates/` change, so no release** — verified by the same `cargo package --list` test that governs
this project's release decisions (§3).

## 2. Why the staging is forced, not chosen

The obvious instinct is "add the logo everywhere in one change." The packaging boundary forbids it,
and RFC-094 forbids the shortcut around it.

```text
root README.md, docs/**, assets/    in 0 of 5 published packages -> no release
crates/*/src/lib.rs                 published -> a release, and (RFC-094 §4.1
                                    as amended) a patch carries "correctness
                                    fixes ... and NOTHING ELSE".
                                    A logo is not a correctness fix.
```

So the docs.rs line is not merely *later* — it is **ineligible for `0.46.1`** and must ride a minor.
Splitting here is what the policy says, not a preference. Everything else lands the moment it is
pushed, and is visible on GitHub and the docs site immediately.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Candidates are 1254×1254 sRGB, **no alpha**, ~1.15 MB each | `magick identify`; `channels=srgb 3.0, alpha=Undefined` |
| E2 | The art contains **14,065 unique colours** despite being a flat illustration | `magick -format %k` — generation noise, not design |
| E3 | 512px / 128 colours / no dither is **69,932 bytes** and visually identical | re-encoded and viewed; 16× smaller than the original |
| E4 | 32 colours is **too few** — visible dithering in the sun and the top face | re-encoded and viewed; rejected on inspection, not on size |
| E5 | The full mark **fails below 64px**: legible at 128, marginal at 64, mush at 32, unidentifiable at 16 | rendered at each size, point-filtered so true pixels are visible |
| E6 | **Cropping to the cube alone rescues it** — clearly a cube at 32px and 48px, marginal at 16px | same method, applied to a cube-only crop |
| E7 | Root-level files reach **zero** of the five published packages | `cargo package --list` across all five; `crates/matten/Cargo.toml` states the rule |
| E8 | crates.io renders each crate README but does **not** resolve relative image paths | the existing shields.io badges already use absolute URLs for this reason |
| E9 | mdBook emits **both** `favicon.png` and `favicon.svg` | `docs/book/favicon-8114d1fc.png`, `favicon-de23e50b.svg` |
| E10 | No crate sets `html_logo_url` today | `grep -rn html_logo_url crates/*/src/lib.rs` → none |
| E11 | The repository has never stated the origin of the name "matten" | searched `README.md`, `docs/src/introduction.md`, `docs/src/philosophy.md` |

**Re-derive E3 and E5 before exporting.** They are the two that decide the asset set, and both were
measured by me — this project's record says that is reason enough to check.

## 4. Change A — the assets, and where they live

```text
assets/                              NEW, at the workspace root
  matten-logo.png                    512px, RGBA, 128 colours, no dither
  matten-mark.svg                    the small mark — see §5
  matten-social.png                  1280×640 wide lockup (§8)
```

> **Implemented 2026-08-28 (Changes A and A.1). One design change, found by measuring.**
>
> **No dark variant is needed, and E1's premise was wrong in a useful way.** Removing the white
> surround also lifts the cream plate — the two are close enough in value that one flood-fill takes
> both. The result is the mark on transparency, and it was composited against GitHub light
> (`#ffffff`) and GitHub dark (`#0d1117`) and inspected: **the single file works on both.** On dark,
> the facelet gaps become dark and the mark reads *better* than on light. Checked for halo against
> magenta — none.
>
> ```text
> assets/matten-logo.png    512×512 RGBA    26,580 bytes   (was 1,153,513 — 43× smaller)
> assets/matten-mark.svg    hand-authored    2,504 bytes
> assets/matten-social.png  1280×640       124,043 bytes
>                                          -------------
>                                          153,127 bytes total
> ```
>
> This **simplifies Change B**: one `<img>`, no `<picture>`, no dark asset to keep in sync. §6 is
> amended accordingly.

```text
DO NOT place any of these under crates/*/. Anything inside a package directory
ships to crates.io, five times over across the family, on every download.
This is E7, and crates/matten/Cargo.toml already states the rule in prose.
```

**Export at 512px / 128 colours / no dither (E3), with alpha (E1).** Not 32 colours — I checked and
it speckles (E4). Not the 1.15 MB original: RFC-116 already had this conversation when
`mermaid.min.js` added 2.67 MB, and the outcome was that cost gets stated plainly. Here the cost is
avoidable, so avoid it.

## 5. Change A.1 — the small mark, and why an SVG is the right answer

E5 is a structural failure, not a resolution one: 27 facelets separated by thin near-white gaps
cannot survive a 32px grid. E6 shows the cube alone *does* survive.

**Author `assets/matten-mark.svg` as a hand-drawn isometric cube** — three faces, the candidate's
three face colours, no sun, no swoosh, no palm, transparent background.

```text
WHY SVG rather than another PNG export
  - resolution-independent: one file serves 16px and 512px
  - a few hundred bytes instead of tens of kilobytes
  - E9 REQUIRES an svg anyway, or mdBook keeps serving Rust's default
  - the geometry is three rhombus grids; this is authorable directly,
    with no design tooling
```

**Simplify the face grid if 3×3 does not hold at 16px.** A 2×2 face is a legitimate small-size
lockup and still reads as a cube. Verify by rendering at 16px and looking — do not assume.

## 6. Change B — the README hero

Above the existing badge block. Light and dark, via the element GitHub supports:

**Amended after implementation (§4): one image, not a `<picture>` pair.** The asset is transparent
and verified on both themes, so a dark variant would be a second file to keep in sync for no gain.

```text
<img alt="matten" src="<absolute raw.githubusercontent URL>" width="200">
```

```text
ABSOLUTE URLs, raw.githubusercontent.com — relative paths do not render on
crates.io (E8), and this README is the only logo surface all five crates get.
The existing shields.io badges are the pattern to follow.
WIDTH ~200px. The mark is legible well above 64px (E5); do not inline it small.
ALT text is required and must say what it is, not "logo".
```

## 7. Change C — the mdBook favicon

**Both files, or this does not work (E9):**

```text
docs/theme/favicon.png    32×32, from the small mark
docs/theme/favicon.svg    the small mark itself
```

Overriding only the PNG leaves mdBook serving its default `favicon.svg`, and a browser preferring SVG
shows Rust's icon on this project's documentation. **Verify by grepping the built HTML and by
checking the emitted filenames change** — RFC-116's lesson was that a green `mdbook build` proves
nothing about what was rendered.

`book.toml` needs no new key for favicons; theme overrides are by filename. **Do not add one.**

## 8. Change D — the GitHub social preview

1280×640, from the candidate sheet's top band (mark plus wordmark). This is set in **repository
settings**, not committed — but `assets/matten-social.png` should exist so the image is versioned and
reproducible rather than living only in a settings form.

**This step is the owner's to perform**; the implementer produces the file only.

## 9. Out of scope — docs.rs, and why

```text
#![doc(html_logo_url = "...")]  in crates/*/src/lib.rs
```

This is the one surface that needs a release, and it is **ineligible for `0.46.1`** (§2). It rides
`0.47.0`, folded into that release's RFC or as its own small one.

Also out of scope:

```text
RFC-121 and the 0.46.1 preparation   — untouched. A logo would breach its
                                       scope lock and RFC-094 §4.1 alike.
any crates/** file                   — none, at all
replacing the candidate art          — this RFC ships what the owner made
a brand/style guide                  — one mark, four surfaces, no system
```

## 10. Open questions — both the owner's, both blocking

**Both questions are answered** (2026-08-28) and implementation may proceed on them. They are recorded
with their reasoning because neither was an implementer decision and neither should be re-opened
without new information.

**Q1 — ANSWERED by the owner, 2026-08-28. Drop the tagline.**

```text
The name comes from a Japanese dialect phrase meaning "I waited for", chosen
because it sounds pleasant. "Math • Tensor • Rust" is a coincidence the owner
enjoys but does not wish to assert:

    "I enjoy the misleading but it is not necessary to intentionally show."

DECISION: remove "Math • Tensor • Rust" from every asset — the wordmark
lockup and the social preview (§8).

The coincidence still works. It simply stops being CLAIMED, which is the
whole difference: a reader who notices mat(rix)+ten(sor) is delighted; a
reader told that is what it means has been told something untrue. E11 stands
— the repository has never stated the origin, and this RFC does not add one.

If a descriptor is wanted under the wordmark, it must describe the LIBRARY,
not the name. The coloured-dot device carries any three words equally well.
Recommended: nothing at all, or the crate's own description.
```

```te**Q2 — ANSWERED 2026-08-28. Keep the 3×3 cube. The concern does not survive scrutiny.**

```text
Raised at review, then withdrawn on the owner's challenge. Recorded here with
its reasoning so it is not re-raised from scratch by the next reader.

WHY IT IS NOT A PROBLEM
  - a 3x3x3 cube is the canonical picture of a RANK-3 TENSOR. That is what
    this library holds. The puzzle occupies the same geometry; the mark is
    not borrowing it.
  - trademark is class-specific and confusion-based. A Rust numerical library
    is not a puzzle toy, and nobody installs `matten` expecting one.
  - the EU 3-D shape mark was invalidated on FUNCTIONALITY grounds — the
    rotating mechanism. Name marks persist; the shape is not the asset.
  - the palette is pastel yellow/green/purple, not the classic six, with no
    black frame and softened facelets.
  - the semantic worry ("a puzzle means difficulty, matten claims ease") is
    answered by the artwork itself: the cube is SOLVED — three faces, three
    uniform colours — and it is HELD IN A PALM. Scrambled says problem;
    solved and held says order and manageable.

AND THE 2x2 ALTERNATIVE IS WITHDRAWN TOO. It was offered partly on
legibility grounds, and E6 does not support that: the cube-only crop reads
clearly at 32px and 48px as 3x3. Only 16px is marginal, and the SVG (§5) can
be tuned for that one case. 2x2 would have bought almost nothing and cost the
correct tensor picture.

DECISION: 3x3, unchanged, in both the full mark and the small mark.
```

Q1's resolution reaches Change D directly: the social preview must not carry the tagline either.

## 11. Risks

```text
R1  Placing assets under crates/*/ — ships to crates.io on every download,
    five times over (§4, E7). The defect this RFC most exists to prevent.
R2  Relative image URLs — renders on GitHub, silently blank on crates.io (E8).
    It will look correct where the author checks and broken where users look.
R3  Overriding favicon.png but not favicon.svg (§7, E9).
R4  Shipping the 1.15 MB originals because they are what was handed over.
R5  Taking 32 colours because it is smallest. It speckles (E4).
R6  Scaling the full mark down for the favicon instead of using the small
    mark. It is unidentifiable at 16px (E5) and no export setting fixes it.
R7  Adding html_logo_url "while we are here". It is a release, and not one
    0.46.1 can carry (§9).
R8  Re-raising §10's questions from scratch. Both are settled, with reasons.
```

## 12. Acceptance criteria

```text
[ ] assets/ exists at the WORKSPACE ROOT; git diff touches no crates/** path
[ ] cargo package --list for all five crates is UNCHANGED — asserted, not assumed
[ ] assets/ carries matten-logo.png, matten-mark.svg, matten-social.png (A, A.1 — done)
[ ] assets/matten-mark.svg renders as a cube at 16px — DONE and verified by
    rendering at 16/32/48/64 and looking; 3×3 holds at 16px
[ ] README hero present, ABSOLUTE URL, alt text (one image — see §4's note)
[ ] docs/theme/favicon.png AND favicon.svg both overridden
[ ] the built book serves the new favicons — verified by grepping built HTML
    and confirming the emitted hashed filenames changed
[ ] book.toml unchanged
[ ] the tagline is absent from every asset (§10 Q1); the cube stays 3×3 (Q2)
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, no tag, no publish, no crates/** change
```

## 13. What this does not do

```text
- give docs.rs a logo. That is 0.47.0 (§9).
- establish a brand system. One mark, four surfaces.
- change the candidate artwork. Re-encoding and cropping are not redesign;
  if the owner wants a different mark, that is a different RFC.
- address the audit's open items: the ROADMAP Status block, the v1.0
  readiness audit, SECURITY.md, the tools' unsafe policy, or a guard that
  can read a published claim.
```
