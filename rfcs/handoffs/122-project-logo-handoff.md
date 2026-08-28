# Developer Handoff — RFC-122: Project Logo, Changes B / C / D

**From:** High-capability model. **Date:** 2026-08-28.
**Design authority:** `rfcs/accepted/122-project-logo.md`
**Base:** `main` @ the RFC-122 asset commit, family at `0.46.0`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Place the logo on three surfaces: the README, the mdBook favicon, and the GitHub social preview.
**The assets already exist — you are not making them.**

## 2. What is already done, and what is yours

```text
DONE (Changes A, A.1 — high-capability model)
  assets/matten-logo.png     512×512 RGBA, transparent      26,580 bytes
  assets/matten-mark.svg     hand-authored isometric cube    2,504 bytes
  assets/matten-social.png   1280×640, no tagline          124,043 bytes

YOURS (Changes B, C, D)
  B  README.md hero
  C  docs/theme/favicon.png + favicon.svg
  D  hand the social image to the owner — a repository SETTING, not a commit
```

**No `crates/` change. No release. No version bump, tag, or publish.** If your diff touches anything
under `crates/`, stop — the docs.rs logo is deliberately deferred to `0.47.0` (RFC §9).

## 3. Two facts that will save you a wrong turn

```text
THE ASSET IS TRANSPARENT AND WORKS ON BOTH THEMES.
  It was composited against #ffffff and #0d1117 and inspected. On dark the
  facelet gaps go dark and it reads BETTER than on light. So Change B is ONE
  <img>, not a <picture> pair. The RFC's original <picture> instruction was
  amended after implementation — follow §6 as it now reads, and do NOT
  reintroduce a dark variant.

THERE IS NO DARK ASSET TO REFERENCE.
  If you find yourself needing one, something has gone wrong — raise it.
```

## 4. Change B — the README hero

Above the existing badge block, below the `# matten` heading.

```text
<img alt="..." src="<ABSOLUTE raw.githubusercontent URL>" width="200">
```

```text
ABSOLUTE URL, raw.githubusercontent.com. NOT a relative path.
  crates.io renders this README for all five crates and does NOT resolve
  relative image paths — a relative src renders on GitHub and is silently
  blank on crates.io. That is the failure mode: correct where you check,
  broken where users look. The existing shields.io badges are the pattern.

WIDTH ~200. The mark is legible well above 64px; do not inline it small.

ALT TEXT must say what it is. Not "logo". Something a screen-reader user
  gets meaning from.
```

## 5. Change C — the mdBook favicon, BOTH files

```text
docs/theme/favicon.svg    <- assets/matten-mark.svg
docs/theme/favicon.png    <- 32×32 PNG rendered from that SVG
```

**Both, or this silently fails.** mdBook emits `favicon.png` *and* `favicon.svg`; override only the
PNG and a browser preferring SVG serves **Rust's default icon on matten's documentation.**

A working derivation for the PNG:

```bash
magick -background none assets/matten-mark.svg -resize 32x32 docs/theme/favicon.png
```

```text
VERIFY BY LOOKING AT THE BUILT OUTPUT, not by a green build. RFC-116's lesson
was exactly this. Confirm the emitted hashed filenames under docs/book/
CHANGED from favicon-8114d1fc.png / favicon-de23e50b.svg.

book.toml needs NO new key — theme overrides work by filename. Do not add one.
```

## 6. Change D — the social preview

`assets/matten-social.png` is 1280×640 and ready. **Setting it is the owner's action** (repository
Settings → Social preview); it is not a commit. Your job is only to confirm the file is committed and
tell the owner it is ready, with the path.

## 7. Out of scope

```text
any crates/** file                  — docs.rs waits for 0.47.0 (RFC §9)
regenerating or editing the assets  — they are done and verified (§2)
a dark logo variant                 — none exists and none is needed (§3)
RFC-121 / the 0.46.1 preparation    — untouched
book.toml                           — no change needed for favicons
a brand or style guide              — one mark, three surfaces
```

## 8. Acceptance criteria

```text
[ ] README hero present, ABSOLUTE raw.githubusercontent URL, meaningful alt text
[ ] docs/theme/favicon.png AND docs/theme/favicon.svg both present
[ ] the built book's favicon filenames CHANGED from the defaults — verified by
    inspecting docs/book/, not by a green build
[ ] book.toml unchanged
[ ] git diff touches NO crates/** path — asserted
[ ] cargo package --list unchanged for all five crates — asserted
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook build clean
[ ] no version bump, tag, or publish
```

## 9. Risks

```text
R1  A relative image URL in the README. Renders on GitHub, blank on crates.io.
R2  Overriding favicon.png but not favicon.svg — Rust's icon stays served.
R3  Reintroducing a <picture>/dark variant. There is no dark asset and none
    is needed; the RFC was amended after implementation (§3).
R4  Touching crates/** "while we are here". That is a release, and not one
    0.46.1 can carry.
R5  Trusting `mdbook build` exit 0 as proof the favicon changed.
R6  Regenerating the assets. They are measured and verified; if you believe
    one is wrong, REPORT it rather than replacing it.
```

## 10. Required evidence

```text
- the README hero's exact src URL, and confirmation it is absolute
- the built book's new favicon filenames, before and after
- git diff --stat proving no crates/** path is touched
- guard and mdbook output
```

## 11. Required review-request format

Write to:
`.git-exclude/review-request/RFC-122/matten-rfc122-project-logo-implementation-review-request-v0.1.md`

Include files changed with line counts, §10's evidence, deviations with reasoning, and anything you
want answered at review.
