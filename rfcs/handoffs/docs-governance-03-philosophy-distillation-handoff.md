# Docs-Governance Handoff 03 — Philosophy Distillation

**Project:** `matten`
**Document kind:** Documentation-governance handoff (design team)
**Status:** Draft for execution; produced by the reviewer, to be implemented then re-reviewed
**Authority / source:** `.git-exclude/reviewed/matten-specs-supersession-map-v0.1.md` §6, §7 (item 4)
**Sequence:** Runs after Handoff 01 and after Handoff 02 has created tracked archived specs under
`docs/design/history/`. Do not rely on `.git-exclude` originals for this slice; they are not
available in a fresh clone.

---

## 1. Summary

`docs/src/philosophy.md` is currently ~18 lines — far too thin for the load-bearing role of
stating matten's concept, direction, and principles. This slice distills a proper, evergreen
**principles page** from the specs' design-goals material, published in the mdBook and guarded
against drift/overclaim.

The specs are the *source*, not the authority (map §0): quarry them, do not copy them wholesale.
Docs/examples only. No API/dependency/version change.

---

## 2. What to distill, and from where

Draw the durable, evergreen content from the tracked archival copies created by Handoff 02:

```text
external-design §1  Design Goals      (mission, "family car" positioning, primary user stories, non-goals)
external-design §2  Public API Principles (single Tensor type, no visible lifetimes, concrete-over-generic, minimal surface)
requirements  §1.3  DX-over-benchmarks comparison table
requirements  §1.1  mission / developer-speed-not-benchmark-leadership
```

Target the result at a reader deciding whether and how to use matten — *why it is shaped this way*,
not a spec dump. Keep it concise (a readable page, not 50 KB).

Suggested shape for `docs/src/philosophy.md`:

```text
# Philosophy
- What matten is (developer-experience-first multidimensional arrays for Rust)
- The "family car" positioning: comfortable, predictable, for small/learning/PoC work; not a
  Formula-1 replacement for ndarray/nalgebra/candle
- Core principles: one concrete Tensor, no visible lifetimes, concrete-over-generic, small surface,
  panic-local / Result-at-boundaries
- What matten is deliberately not (the non-goals)
- Where to go when you outgrow it (pointer to the migration guide)
```

## 3. Hard constraints (evergreen + guard-safe)

```text
[ ] no version pins or family numbers (must not need retargeting each release; the version guard
    should have nothing to catch here)
[ ] no retired vocabulary: "Phase 1/Phase 2/Phase-1/Phase-2/Sedan/SUV" are banned in user docs
[ ] no overclaim verbs: must migrate, production-scale, business-critical, "faster than",
    "scales to", drop-in, guaranteed, best
[ ] keep the RFC-065 positioning signal present: learning/teaching + honest boundary language
[ ] no new public API, capability, or dependency implied
[ ] distill and rephrase — do not paste large spec passages verbatim
```

## 4. Bring it under the positioning guard

`scripts/check-release-docs.sh` enforces RFC-065 positioning across a set of `POSITIONING_DOCS`
(learning/teaching presence + an overclaim denylist).

Do:
1. Confirm `docs/src/philosophy.md` remains in the guard's `POSITIONING_DOCS` / high-visibility
   set. If it is ever missing, add it so the strengthened page cannot drift or overclaim later.
2. Ensure the new content satisfies the guard (required positioning tokens present, denylist clear).

## 5. Non-goals

```text
[ ] not a requirements/design dump into the book
[ ] no benchmark numbers or performance guarantees (that is Handoff 01 §2.1 territory, and non-binding there)
[ ] no change to SUMMARY.md structure beyond what already lists Philosophy
[ ] no companion-crate or migration content beyond a one-line pointer
```

## 6. Files

```text
docs/src/philosophy.md                 # expand from stub to principles page
scripts/check-release-docs.sh          # only if philosophy.md must be added to POSITIONING_DOCS
docs/src/introduction.md               # optional: one-line link to Philosophy if not already present
ROADMAP.md                             # record this slice after the tracked archival step
```

## 7. Acceptance criteria

```text
[ ] philosophy.md is a coherent principles page distilled from the specs, evergreen (no version pins)
[ ] source material comes from tracked docs/design/history snapshots, not .git-exclude originals
[ ] positioning is honest: "family car" / learning / PoC framing, explicit non-goals, migration pointer
[ ] no retired vocabulary and no overclaim verbs present
[ ] philosophy.md is covered by the RFC-065 positioning guard and passes it
[ ] ROADMAP.md records philosophy distillation as the third docs-governance step
[ ] check-release-docs.sh passes; mdbook build docs succeeds; git diff --check clean
[ ] no public API/dependency/version change
```

## 8. Verification

```bash
bash scripts/check-release-docs.sh
mdbook build docs   # remove generated docs/book afterward
git diff --check
```

## 9. Review focus (for the returning reviewer)

```text
whether the page reads as distilled principles for a reader, not a spec transcript
whether every claim is evergreen and free of version pins / retired vocabulary / overclaim verbs
whether philosophy.md is actually inside the positioning guard now (not just passing incidentally)
whether the "family car" positioning stays consistent with README/introduction (no new contradiction)
whether anything implies a capability, guarantee, or dependency matten does not have
```
