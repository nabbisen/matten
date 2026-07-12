# Docs-Governance Handoff 01 — Spec Coverage-Gap Closure

**Project:** `matten`
**Document kind:** Documentation-governance handoff (design team)
**Status:** Implemented; prepared for review
**Authority / source:** `.git-exclude/reviewed/matten-specs-supersession-map-v0.1.md` §4
**Sequence:** **This handoff runs FIRST** — before Handoff 02 (archival). Nothing normative may be
lost to "history" until the three gaps below are resolved.

---

## 1. Summary

The supersession map showed the two v0.19.0 specs are ~95% already owned by the RFC + `docs/src`
corpus, and self-declare RFCs as canonical. Only **three fragments have no current owner**. This
slice resolves each — by **promotion** (into an RFC or a `docs/src` page) or **explicit
retirement** (recorded as a decision) — so that archiving the specs (Handoff 02) loses nothing.

This slice is analysis + small docs edits only. It adds no public API, dependency, or version bump.

---

## 2. The three gaps (from map §4)

### 2.1 Quantified non-functional targets

Source: requirements §9.1/§9.2 — "minimal examples compile < 15 s", "incremental rebuild < 3 s",
memory tiers (<1 MiB clone-ok / 1–100 MiB warn / >100 MiB use specialised crates).

Do:
1. Search the repo for any place these numbers are enforced or stated
   (`docs/src/benchmarks/*`, RFCs, CI). Confirmed status: **not found by the reviewer.**
2. Decide, explicitly, one of:
   - **Adopt (non-binding):** add a short "Performance expectations (non-binding)" note under
     `docs/src/benchmarks/` (or `reference/compatibility.md`), phrased as *approximate guidance,
     not a guarantee*, with no overclaim verbs; **or**
   - **Retire:** record that these 0.19-era targets were never enforced and are not maintained.
3. Record the decision (see §4 output).

### 2.2 Golden NumPy fixtures + fuzz/property tests

Source: external §11.3 (golden NumPy comparison fixtures) and §11.4 (fuzz/property tests).

Do:
1. Inventory what actually exists: check `crates/*/tests`, `benchmarks/`, and RFC-013 / RFC-049
   for golden-fixture, fuzz, and property coverage.
2. Reconcile the contract to reality: if the suites exist, point the testing contract (RFC-013 or a
   short doc note) at them; if a category does not exist, record it as *aspirational / not
   implemented* rather than leaving a silent "SHOULD" in a frozen spec.
3. Record the inventory + decision.

### 2.3 `Display` formatting contract

Source: requirements §5.9 / external §10.2 — a user-facing `Display` (matrix-like) format ("MAY").

Do:
1. Verify the shipped state of `Display` for `Tensor` in `crates/matten/src`.
2. If implemented: document it briefly in `docs/src/reference/` (near the `Debug` description).
   If not implemented: record that `Display` is not part of the current contract.

---

## 3. Non-goals

```text
[ ] no public API change
[ ] no new dependency
[ ] no version bump
[ ] no change to the specs themselves (archival is Handoff 02)
[ ] no new performance guarantee or benchmark claim (guidance must stay non-binding, no overclaim verbs)
[ ] do not add "Phase 1/Phase 2/Sedan/SUV" vocabulary to any docs/src page (retired + guard-banned)
```

## 4. Output / files

Produce a short, tracked resolution note capturing all three decisions so the harvest is durable:

```text
docs/design/coverage-gap-resolution.md      # new; or fold into docs/design/README.md from Handoff 02
```

Plus any of these only if a gap is *adopted*:

```text
docs/src/benchmarks/index.md  (or reference/compatibility.md)   # §2.1 if adopted
docs/src/reference/*.md                                          # §2.3 if Display documented
rfcs/done/013-... or a short note                               # §2.2 if the testing contract is re-anchored
ROADMAP.md                                                       # track the docs-governance sequence/status
```

If any decision is normative enough to be a rule (e.g. "matten publishes no perf guarantees"),
prefer recording it where it will be found — a doc note is fine; an RFC is optional.

Update `ROADMAP.md` so the docs-governance track remains visible outside the
handoff index:

```text
01 coverage-gap closure -> before archival
02 archival + ownership rule -> after 01
03 philosophy distillation -> after tracked archives exist
```

## 5. Acceptance criteria

```text
[ ] each of the three gaps has an explicit, recorded decision (adopt-with-location OR retire-with-reason)
[ ] no unowned numeric target remains implied as a live requirement anywhere tracked
[ ] any adopted performance guidance is explicitly non-binding and passes the overclaim guard
[ ] Display contract status is verified against source and recorded
[ ] testing-contract (golden/fuzz/property) status is inventoried against real tests and recorded
[ ] ROADMAP.md records the docs-governance sequence and marks this slice as the first prerequisite
[ ] check-release-docs.sh passes; mdbook build docs succeeds; git diff --check clean
```

## 6. Verification

```bash
bash scripts/check-release-docs.sh
mdbook build docs   # remove generated docs/book afterward
git diff --check
```

## 7. Review focus (for the returning reviewer)

```text
whether each gap decision is explicit and durable, not hand-waved
whether adopted perf guidance stays non-binding and clears the overclaim denylist
whether the Display and testing-contract claims match the actual code/tests (spot-check)
whether anything normative from the specs was dropped without a recorded decision
```
