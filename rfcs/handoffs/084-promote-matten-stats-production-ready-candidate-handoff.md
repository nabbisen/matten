# RFC-084 `matten-stats` Promotion: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/084-promote-matten-stats-production-ready-candidate.md`
**Document kind:** Detailed implementation handoff
**Status:** Proposed; inherits RFC-084's state. Not authorized for implementation until accepted
**Date:** 2026-07-29

---

## 1. Purpose

Promote `matten-stats` **Experimental → production-ready candidate**, discharging RFC-081 §3 Exit A.

**Two ordered parts. Part 1 is real work, not paperwork.** `matten-stats` is the only published crate
with no CI job and no example smoke runs, and the candidate bar includes "examples in CI." Close that
first; move the label second. **If Part 1 does not land, the label does not move.**

No release, no version bump, no API change, no code change inside `matten-stats`.

## 2. Preconditions

```text
RFC-084 and this handoff accepted
working tree clean; version stays 0.39.0
matten-stats currently declares Experimental in README.md and src/lib.rs, and
  scripts/check-release-docs.sh (lines 113-127) ASSERTS that. Expect it to fail
  mid-change — that is the guard working. It gets inverted in Part 2, not disabled.
```

## 3. PART 1 — close the CI gap

### 3.1 New job in `.github/workflows/test.yaml`

Mirror the `matten-mlprep` job's shape (currently at lines 75-82):

```yaml
  matten-stats:
    name: matten-stats
    runs-on: ubuntu-latest
    steps:
      # ... same checkout/toolchain steps as the matten-mlprep job
      - run: cargo test -p matten-stats
      - run: cargo test -p matten-stats --features dynamic
      - run: cargo test -p matten-stats --doc
```

The `--features dynamic` line matters and is not decorative. The workspace test step is
`cargo test --workspace --all-targets` (line 42) — **`--all-targets`, not `--all-features`** — so
`matten-stats`'s one `dynamic`-gated test (`tests/statistics.rs:455`) currently runs *only* in the
MSRV job. Without this line the promotion would claim CI coverage the crate does not have.

### 3.2 Four example smoke runs

Add to the `example smoke runs` job, after the `matten-data` block (ends line 178):

```yaml
      - run: cargo run -p matten-stats --example stats_covariance
      - run: cargo run -p matten-stats --example stats_correlation
      - run: cargo run -p matten-stats --example stats_quantile
      - run: cargo run -p matten-stats --example stats_expansion
```

Verify the names against `crates/matten-stats/Cargo.toml`'s `[[example]]` entries rather than
guessing from filenames — the file is `examples/expansion.rs` but the example name is
`stats_expansion`.

### 3.3 Release checklist

`docs/src/contributing/release-checklist.md` does not mention `matten-stats` at all. Add it wherever
the other four published crates are enumerated, matching the surrounding style.

### 3.4 Prove Part 1 independently

Run all four commands locally and confirm each exits 0 before touching any label:

```bash
cargo test -p matten-stats
cargo test -p matten-stats --features dynamic
cargo test -p matten-stats --doc
for e in stats_covariance stats_correlation stats_quantile stats_expansion; do
  cargo run -p matten-stats --example $e || echo "FAILED: $e"
done
```

## 4. PART 2 — move the label

### 4.1 The guard must be inverted, not disabled

`scripts/check-release-docs.sh` lines 113-127 currently **assert** `matten-stats` says `Experimental`.
Replace that block with the shape already used for `matten-data` directly above it (lines 95-111):

```text
1. a NEGATIVE check — fail if a stale Experimental/Beta label survives in the crate's
   status files (README banner line, lib.rs //! line, Cargo.toml)
2. a POSITIVE check — fail if README.md does not declare "production-ready candidate"
3. a SHARED-DOCS check — fail if a current-status shared doc still marks it Experimental
```

Model 1 and 2 on the `$DATA` block verbatim, substituting `$STATS`. For 3, note that
`docs/src/examples/companions.md` and `index.md` currently do **not** mention `matten-stats` at all —
so that check will have nothing to match today. Add it anyway, mirroring the `matten-data` pattern,
so the invariant exists when those pages do gain a row.

**Do not delete the guard.** A promotion that removes its own check leaves the next label change
unguarded.

### 4.2 Label sites — LIVE only

```text
crates/matten-stats/src/lib.rs               # Status section
crates/matten-stats/README.md                status banner, line 7
README.md                                    root crate table row (line ~31)
docs/src/reference/compatibility.md          two sites (~75, ~188)
docs/src/reference/stats.md                  the RFC-083 section's "Experimental" mention (~106)
rfcs/proposed/076-v1-release-preparation.md  FOUR sites (~64, ~207, ~228, ~239) — this RFC is
                                             still in proposed/, so its inventory is live and
                                             its "may not enter while Experimental" precondition
                                             is now discharged
ROADMAP.md                                   Status prose (line 7) + a new history row + header bump
rfcs/README.md                               remaining-themes row (~133) + RFC-084's own entry
```

### 4.3 Sites that MUST NOT be touched

```text
rfcs/done/**          incl. 080-...md:133's "not near promotion" line — that is a correct
                      record of RFC-080's position at the time. RFC-084 §3 answers it in
                      argument; it does not get edited away
rfcs/handoffs/**      historical handoff records
CHANGELOG.md
docs/design/history/**
ROADMAP.md history rows (append a new one; do not rewrite 3.24.0/3.26.0-3.31.0)
```

**Derive the live list yourself; do not trust §4.2 as complete.** Sweep and classify:

```bash
grep -rniE "matten-stats.{0,90}experimental|experimental.{0,90}matten-stats" \
  --include="*.md" --include="*.rs" --include="*.sh" . | grep -v "^./.git-exclude"
```

Then classify every hit as **live current claim** (change) or **historical record** (leave). This
project has been burned three times by enumerated site lists that were incomplete — RFC-079 missed
two, RFC-080 went 7→3→6→7, RFC-081 went 5→16→17. The sweep is the method; §4.2 is a cross-check.

## 5. Required verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
bash scripts/check-release-docs.sh          # MODIFIED in this slice — must pass with the new label
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-matten-data-scope.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

**Prove the inverted guard actually guards.** A check that passes but cannot fail is worthless — this
project shipped exactly that mistake once already (RFC-082's grep tested the wrong pattern and
reported success). So:

```text
1. temporarily revert ONE label site to "Experimental"
2. run check-release-docs.sh -> it MUST fail, naming that site
3. restore the site; re-run -> passes
4. record both outcomes in the review request
```

Scope confirmation:

```bash
git diff --name-only | grep -E '^(rfcs/done|rfcs/handoffs|docs/design/history)/|^CHANGELOG.md'
# expect EMPTY
git diff --name-only -- crates/matten crates/matten-data crates/matten-mlprep crates/matten-ndarray
# expect EMPTY
git diff -- crates/matten-stats/src crates/matten-stats/tests
# expect EMPTY except the lib.rs Status section — NO code or test change
grep -m1 '^version' Cargo.toml     # still 0.39.0
```

## 6. Known pitfalls

1. **Moving the label before Part 1 lands.** The bar's "examples in CI" signal is the one thing
   currently unmet; promoting first would assert a bar that is not met.
2. **Deleting the Experimental assertion instead of inverting it** (§4.1).
3. **Omitting `--features dynamic`** from the new job, leaving the dynamic test MSRV-only (§3.1).
4. **Editing `rfcs/done/080-...md:133`** to remove the "not near promotion" line. It is history.
5. **Trusting §4.2's site list** instead of running the sweep (§4.4). Assume it is incomplete.
6. **Promoting to full production-ready.** Candidate is the rung Exit A requires; RFC-084 §8 is
   explicit that full production-ready is not claimed.
7. **Touching `matten-stats`'s code or tests.** This slice changes labels, CI, docs and one guard.
8. **Bumping the version or treating Exit A as authorizing v1.0 work.** RFC-076 stays deferred; v1.0
   is not currently wanted.

## 7. What the review request must report

```text
[ ] Part 1 evidence: the four commands' output, each exiting 0
[ ] the new CI job and the four smoke-run lines, as a diff
[ ] the inverted guard's diff, plus the deliberate-failure proof from §5 (both outcomes)
[ ] the full sweep output, with every hit classified live vs historical
[ ] confirmation rfcs/done/, rfcs/handoffs/, CHANGELOG.md, docs/design/history/ are absent
    from git diff --name-only
[ ] confirmation no matten-stats source or test file changed
[ ] RFC-081 §3 Exit A recorded as discharged, and RFC-076 still marked deferred
[ ] full gate set incl. MSRV and mdbook; version still 0.39.0
```

## 8. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, publish, further
promotion, or any RFC-076 execution.
