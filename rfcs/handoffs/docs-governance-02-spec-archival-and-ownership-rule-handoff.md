# Docs-Governance Handoff 02 — Spec Archival + Ownership Rule

**Project:** `matten`
**Document kind:** Documentation-governance handoff (design team)
**Status:** Draft for execution; produced by the reviewer, to be implemented then re-reviewed
**Authority / source:** `.git-exclude/reviewed/matten-specs-supersession-map-v0.1.md` §0, §1, §5, §6, §7
**Sequence:** Runs **after Handoff 01** (coverage-gap closure). Do not archive until the three §4
gaps are resolved, or normative content is lost to history.

---

## 1. Summary

The v0.19.0 specs already self-declare RFCs/`ROADMAP.md` as canonical and label themselves
historical, but they (a) sit in `.git-exclude/` so they are invisible to anyone cloning the repo,
(b) carry no "superseded" banner, and (c) freeze at RFCs 000–030 (~35 RFCs stale). This slice
closes priorities #1 (single source of truth) and #2 (discoverability) by **archiving the specs as
tracked, banner-marked history** and **writing down the three-plane ownership rule**.

This slice does not reconcile spec content into the codebase — the codebase already superseded it
(map §2–§3). It relocates + labels, and declares the rule. No API/dependency/version change.

---

## 2. Create the tracked history location

Create a **tracked** directory that is **outside the mdBook** (sibling of `docs/src`, so `mdbook
build` never touches it and no `SUMMARY.md` entry is needed):

```text
docs/design/history/
```

Copy the three snapshot files there (content unchanged except the banner in §3):

```text
docs/design/history/matten-v0.19.0-requirements-v1.md
docs/design/history/matten-v0.19.0-external-design-v1.md
docs/design/history/matten-v0.19.0-roadmap-milestones-v1.md
```

(The roadmap snapshot is included for consistency; its canonical successor is `ROADMAP.md`.)

After the tracked copies exist and are verified, the `.git-exclude/specs/` originals **may be
deleted** to avoid a duplicate untracked shadow copy (keep only if you want a personal scratch
copy — but the tracked copy is now the history-of-record).

## 3. Required banner (top of each archived file)

Insert immediately under the existing title, before the metadata block:

```text
> **HISTORICAL SNAPSHOT — DO NOT CITE AS CURRENT.**
> As-built through v0.19.0 (RFCs 000–030). Superseded by the current RFC corpus
> (`rfcs/`) and user documentation (`docs/src/`); forward schedule lives in
> `ROADMAP.md`. This document froze ~35 RFCs ago and predates matten-data, the
> 0.20–0.30 API/companion work, and the visual/educational/migration programs.
> Retained for design traceability only. Section-by-section canonical owners:
> see the supersession map. Terminology note: the "Phase 1/Phase 2/Sedan/SUV"
> vocabulary here is retired and is banned from user docs.
```

Optionally add a one-line pointer beside external-design §8.1's pre-correction `MattenError` enum
("superseded — see the as-built block below"), the one internal drift trap (map §5); the banner
otherwise covers it.

## 4. Declare the three-plane ownership rule

Create `docs/design/README.md` recording the rule (map §1) so the model is written down and
findable:

```text
RFCs (rfcs/)        = canonical normative decisions (data model, error model,
                      boundaries, feature scope, threat model, versioning, companion contracts)
User docs (docs/src)= the distilled, evergreen contract-in-use + philosophy/positioning
ROADMAP.md          = forward schedule / milestone history
docs/design/history = dated design snapshots; historical only, never cited as current
```

Include the one-line editing heuristic: *"If it's a decision, it's an RFC; if it's what a user must
know to use matten, it's `docs/src`; the specs are never cited as current."*

Add a pointer from `rfcs/README.md` to `docs/design/README.md` so the plane model is reachable from
the RFC index.

**Challenge note (decide, don't skip):** the ownership rule is itself a *governance decision*. A
tracked `docs/design/README.md` note is the minimum. If the maintainer wants it to carry normative
weight equal to other policy (e.g. RFC-000 lifecycle, RFC-030 versioning), **promote it to a short
RFC** (next free id is **066**) and have `docs/design/README.md` point at it. Record which path was
chosen.

## 5. Non-goals

```text
[ ] do not edit the substance of the archived specs (banner + optional §8.1 pointer only)
[ ] do not add any docs/design/** file to docs/src/SUMMARY.md (must stay out of the published book)
[ ] no public API / dependency / version / CHANGELOG change
[ ] do not copy retired "Phase/Sedan/SUV" vocabulary into any docs/src page
[ ] do not delete the .git-exclude originals until tracked copies are verified in place
```

## 6. Files

```text
docs/design/history/matten-v0.19.0-requirements-v1.md      # new (tracked copy + banner)
docs/design/history/matten-v0.19.0-external-design-v1.md    # new (tracked copy + banner)
docs/design/history/matten-v0.19.0-roadmap-milestones-v1.md # new (tracked copy + banner)
docs/design/README.md                                       # new (ownership rule)
rfcs/README.md                                              # pointer to docs/design/README.md
ROADMAP.md                                                  # track the ownership rule / archived-spec status
rfcs/handoffs/README.md                                     # mark this handoff done, if desired
# optional: rfcs/proposed/066-*.md if the rule is promoted to an RFC (§4)
```

## 7. Acceptance criteria

```text
[ ] all three snapshots exist under docs/design/history/, tracked, each with the §3 banner
[ ] no docs/design/** file appears in docs/src/SUMMARY.md; mdbook build is unaffected
[ ] docs/design/README.md states the three-plane rule + editing heuristic
[ ] rfcs/README.md links to docs/design/README.md
[ ] ROADMAP.md points to the ownership rule location and records that archived specs are historical only
[ ] ownership-rule disposition recorded (README note vs RFC-066) with rationale
[ ] no duplicate authoritative-looking copy remains (.git-exclude originals removed or clearly scratch)
[ ] check-release-docs.sh passes; mdbook build docs succeeds; git diff --check clean
```

## 8. Verification

```bash
bash scripts/check-release-docs.sh
mdbook build docs   # confirm docs/design/** is NOT pulled into the book; remove generated docs/book after
git diff --check
grep -rn "docs/design" docs/src/SUMMARY.md || echo "correctly absent from the book nav"
```

## 9. Review focus (for the returning reviewer)

```text
whether the banner is present and unambiguous on all three snapshots
whether docs/design/** is genuinely outside the published book (no SUMMARY entry, mdbook clean)
whether the ownership rule is written down and reachable, and the RFC-vs-note choice is recorded
whether the .git-exclude shadow copy is resolved (no two live copies)
whether Handoff 01's gap resolutions landed before this archival (nothing normative lost)
```
