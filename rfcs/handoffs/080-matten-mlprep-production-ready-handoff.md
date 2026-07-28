# RFC-080 Promote `matten-mlprep` to Production-Ready: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-080 (design authority)
**Document kind:** Maturity-label implementation handoff
**Status:** Accepted and implemented (GO, conditional on three corrections, all applied): the
sites list corrected from a self-authored, over-broad seven-file table to a verified six-site
list across four files (three sites the original table missed entirely) — implementation then
found a seventh real site (`crates/matten-mlprep/src/lib.rs`'s crate-level Status doc comment)
neither list caught; a new maturity guard for `matten-mlprep` was added and proven to fail on a
reintroduced violation before being reverted; RFC-076's resulting staleness recorded rather than
fixed
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

## 4. Sites to change — verified six-site list (corrected after review)

The original draft of this section listed seven files by raw occurrence count without checking which
crate each occurrence described. Review found that four of those seven (`docs/src/examples/companions.md`,
`docs/src/examples/data.md`, `docs/src/examples/index.md`, `docs/src/contributing/release-checklist.md`)
contain **zero** `matten-mlprep` maturity claims — every hit in them is `matten-data`'s, or (for
`release-checklist.md`) a generic v1.0-readiness conditional that names no specific crate. Review also
found **three current-status sites the original table missed entirely**, outside that seven-file set.

The verified list — six sites across four files:

| # | Site | What it says now | Edit |
|---|---|---|---|
| 1 | `README.md:29` (root) | crate table row | candidate → production-ready. **Line 30 is `matten-data`'s own row — do not touch it** |
| 2 | `crates/matten-mlprep/README.md:7` | banner *"Production-ready candidate (`0.39.x` family)"* | reword; phrased differently from a table row, so a pattern tuned to the others misses it |
| 3 | `docs/src/reference/compatibility.md:180-181` | *"the ladder now reads … `matten-mlprep` and `matten-data` production-ready candidates"* | update the `matten-mlprep` half only. **Leave lines 94 (generic v1.0 conditional), 160, 167, 175 (RFC-057/058/059 promotion history) untouched** |
| 4 | `scripts/check-release-docs.sh:392-403` | comment + error string both assert `matten-mlprep` is currently candidate | update both to say production-ready (see §5 for the accompanying new guard) |
| 5 | `scripts/check-release-docs.sh:77-80` | error string says *"should be beta"* — **already stale since RFC-058** promoted the crate to candidate in v0.26.0; a pre-existing defect this RFC did not create, but the natural place to fix it | update to production-ready |
| 6 | `rfcs/README.md:129` | remaining-themes row: *"`matten-mlprep` and `matten-data` are production-ready candidates"* | `matten-mlprep` resolved; `matten-data` stays candidate; `matten-stats` stays Experimental |

**Explicitly checked and confirmed to need NO edit** — record this in the review request so a later
reader knows they were considered, not overlooked:

```text
docs/src/examples/companions.md              both hits are matten-data's, in its own section
docs/src/examples/data.md                    matten-data's own description
docs/src/examples/index.md                   matten-data's csv_to_tensor row
docs/src/contributing/release-checklist.md   generic "if any crate remains candidate" conditional
```

**Do not run a global find-and-replace.** Verification after editing:

```bash
grep -rn "production-ready candidate" README.md crates/*/README.md docs/src/ rfcs/README.md scripts/
# every remaining hit must be matten-data's, matten-stats-adjacent, generic policy, or dated
# RFC-057/058/059 history -- account for each explicitly in the review request
grep -rn "production-ready" README.md crates/matten-mlprep/README.md | grep -i mlprep
# every hit must now say production-ready WITHOUT "candidate"
```

## 5. New guard — `matten-mlprep` must not say candidate (required)

`scripts/check-release-docs.sh` has a "must not say candidate" guard for `matten-ndarray` (added when
*it* was promoted, RFC-057) but not for `matten-mlprep` — today's block only asserts absence of `Beta`
(site 4/5 above). Every prior companion promotion added a guard mirroring the new claim; skipping it here
would leave `matten-mlprep` the only companion whose current label nothing enforces.

```text
KEEP    the existing "no Beta" assertion — a crate should not regress two rungs either
ADD     a new assertion that matten-mlprep's status files (README.md, src/lib.rs, Cargo.toml)
        must not say "production-ready candidate" -- mirroring matten-ndarray's guard exactly
UPDATE  the block's comment and both error strings (sites 4 and 5 above) to name production-ready
```

**Prove the new assertion actually fires** before considering this done: temporarily reintroduce
"production-ready candidate" in `crates/matten-mlprep/README.md`, run the guard, confirm it fails, revert.
Report that evidence in the review request — a guard that has never demonstrably failed on a real
violation is not a guard, it is prose that looks like one.

## 6. Tracking

```text
ROADMAP.md    Status sentence; a history row recording the promotion AND its basis
              (RFC-058 §5.1 Option B, satisfied by RFC-077) — not just the label change.
              Also record RFC-076's resulting staleness (its RFC-067 family maturity
              table still lists matten-mlprep as production-ready candidate) --
              recorded, not fixed; RFC-076 is not edited by this slice.
rfcs/README.md  move RFC-080 proposed -> done on acceptance; update the
              "Companion full-production decisions" remaining-themes row: matten-mlprep
              resolved, matten-data still open, matten-stats Experimental
```

The history row must record **why** the deferral ended. A future reader should see that RFC-058's own exit
criteria were met, not that a label drifted upward.

## 7. What must NOT change

```text
matten-data's label     still production-ready candidate, for its own separate reasons
matten-stats's label    still Experimental
any code, test, example, doc-comment, dependency, MSRV, or feature
CHANGELOG.md            no entry — this is not a release (RFC-080 §11)
version                 0.39.0 unchanged
```

## 8. Verification

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

Run `check-release-docs.sh` **before and after** editing it (§5) and confirm the new assertion fails on a
reintroduced "production-ready candidate," per §5's instruction.

## 9. What the review request must report

```text
[ ] every changed site, with the crate each one refers to identified (the six-site list, §4)
[ ] confirmation companions.md/data.md/index.md/release-checklist.md were checked and needed no edit
[ ] grep output proving no matten-data or matten-stats label moved
[ ] git diff --name-only -- 'crates/*/src/' 'crates/*/tests/' 'crates/*/examples/'  (expect empty)
[ ] the new "no production-ready candidate" guard for matten-mlprep, with evidence it fails on
    a reintroduced violation (§5)
[ ] ROADMAP history row recording the RFC-058 §5.1 Option B basis AND RFC-076's resulting staleness
[ ] confirmation: no CHANGELOG entry, no version change, no release action
[ ] full gate set results
```

## 10. Known pitfalls

1. **Trusting the original seven-file table without re-verifying.** Review found it over-broad (four files
   needed no edit) and incomplete (three current-status sites outside it were missed). The six-site list in
   §4 is the corrected version — verify it again anyway; a second miss is easy after the first correction.
2. **Missing the crate README's banner**, which is phrased *"Production-ready candidate (`0.39.x` family)"* —
   different from the table rows, so a pattern tuned to the others skips it.
3. **Skipping the guard addition (§5).** Every prior companion promotion added a mirroring guard;
   `matten-mlprep` would otherwise be the only companion whose current label nothing enforces.
4. **Editing RFC-076 to fix its now-stale maturity table.** Record the staleness (§6); do not touch RFC-076
   itself in this slice.
5. **Adding a CHANGELOG entry.** No release here; the label ships with the next one, whose notes must state
   it (RFC-080 §11).
6. **Touching code "while here."** The promotion's evidence is zero churn; churning the crate contradicts it.
7. **Reading the promotion as a feature promise.** It does not add stratified/grouped/time-series splits —
   those remain declared scope exclusions (RFC-080 §7).

## 11. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, publish, or any other
crate's maturity change.
