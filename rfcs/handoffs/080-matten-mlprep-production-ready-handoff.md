# RFC-080 Promote `matten-mlprep` to Production-Ready: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-080 (design authority)
**Document kind:** Maturity-label implementation handoff
**Status:** Drafted for review; implementation unauthorized until RFC-080 and this handoff are accepted
**Date:** 2026-07-28

---

## 1. Purpose

Apply the `matten-mlprep` maturity promotion as one reviewable slice. RFC-080 is the design authority.

**Documentation only.** No code, no version, no release.

## 2. Preconditions

```text
RFC-080 and this handoff accepted
the 0.39.0 post-release alignment committed first (rfcs/handoffs/0390-post-release-alignment-handoff.md)
  — the repository must correctly describe the released state before a maturity
  claim is layered on top of it
working tree clean; version stays 0.39.0
```

## 3. This is a label change — that is the whole slice

The temptation in a maturity slice is to "improve things a bit while here." Resist it. A diff containing a
code change, a test change, or an example change is a defect, not a bonus: the promotion's evidence base is
RFC-074's *zero-churn* measurement, and churning the crate in the same commit undermines the claim being
made.

```bash
git diff --name-only -- 'crates/*/src/' 'crates/*/tests/' 'crates/*/examples/'
# expect: EMPTY
```

## 4. Sites to change

`matten-mlprep`'s maturity label appears in these files. Counts are of "production-ready candidate"
occurrences at the time of writing; **verify each is about `matten-mlprep`** before editing, because three
of these files also describe `matten-data` and `matten-stats`.

| File | Occurrences | Note |
|---|---:|---|
| `README.md` (root) | 2 | Crate table row + surrounding prose. **`matten-data` also appears here — do not touch it** |
| `crates/matten-mlprep/README.md` | 1 | Status banner, line 7: *"**Production-ready candidate (`0.39.x` family).**"* — phrased differently from the others, so a naive grep-replace misses it |
| `docs/src/reference/compatibility.md` | 5 | **Highest risk.** Several are about `matten-data` or the family generally |
| `docs/src/examples/companions.md` | 2 | Check which crate each describes |
| `docs/src/examples/data.md` | 1 | Likely `matten-data` — **verify before changing** |
| `docs/src/examples/index.md` | 1 | Check |
| `docs/src/contributing/release-checklist.md` | 1 | Check |

**Do not run a global find-and-replace.** `docs/src/examples/data.md` in particular is a `matten-data`
document; its candidate label is probably not `matten-mlprep`'s and must stay.

Verification after editing:

```bash
grep -rn "production-ready candidate" README.md crates/*/README.md docs/src/
# every remaining hit must be about matten-data (or the family policy generally)
grep -rn "production-ready" README.md crates/matten-mlprep/README.md | grep -i mlprep
# every hit must now say production-ready WITHOUT "candidate"
```

## 5. Tracking

```text
ROADMAP.md    Status sentence; a history row recording the promotion AND its basis
              (RFC-058 §5.1 Option B, satisfied by RFC-077) — not just the label change
rfcs/README.md  move RFC-080 proposed -> done on acceptance; update the
              "Companion full-production decisions" remaining-themes row: matten-mlprep
              resolved, matten-data still open, matten-stats Experimental
```

The history row must record **why** the deferral ended. A future reader should see that RFC-058's own exit
criteria were met, not that a label drifted upward.

## 6. What must NOT change

```text
matten-data's label     still production-ready candidate, for its own separate reasons
matten-stats's label    still Experimental
any code, test, example, doc-comment, dependency, MSRV, or feature
CHANGELOG.md            no entry — this is not a release (RFC-080 §11)
version                 0.39.0 unchanged
```

## 7. Verification

```bash
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-matten-data-scope.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

`check-release-docs.sh` has maturity-label assertions (it currently enforces that `matten-mlprep` does not
claim Experimental, and similar for the other crates). **Read it before editing docs** — if it hardcodes the
candidate label for `matten-mlprep`, the guard itself needs updating in this same slice, and leaving it
would either fail the build or silently keep enforcing a stale claim.

## 8. What the review request must report

```text
[ ] every changed site, with the crate each one refers to identified
[ ] grep output proving no matten-data or matten-stats label moved
[ ] git diff --name-only -- 'crates/*/src/' 'crates/*/tests/' 'crates/*/examples/'  (expect empty)
[ ] whether check-release-docs.sh needed a maturity-assertion update, and if so what
[ ] ROADMAP history row recording the RFC-058 §5.1 Option B basis
[ ] confirmation: no CHANGELOG entry, no version change, no release action
[ ] full gate set results
```

## 9. Known pitfalls

1. **Global find-and-replace on "production-ready candidate."** Three of the seven files describe other
   crates. This is the single most likely way to silently promote `matten-data`, which RFC-067 forbids as an
   implied promotion.
2. **Missing the crate README's banner**, which is phrased *"Production-ready candidate (`0.39.x` family)"* —
   different from the table rows, so a pattern tuned to the others skips it.
3. **`docs/src/examples/data.md`** — almost certainly about `matten-data`. Verify; do not assume.
4. **Not checking `check-release-docs.sh`** for a hardcoded maturity assertion.
5. **Adding a CHANGELOG entry.** No release here; the label ships with the next one, whose notes must state
   it (RFC-080 §11).
6. **Touching code "while here."** The promotion's evidence is zero churn; churning the crate contradicts it.
7. **Reading the promotion as a feature promise.** It does not add stratified/grouped/time-series splits —
   those remain declared scope exclusions (RFC-080 §7).

## 10. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, publish, or any other
crate's maturity change.
