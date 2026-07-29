# RFC-084: Promote `matten-stats` to Production-Ready Candidate

**Status:** `proposed/` by folder (not yet implemented); **reviewed and accepted 2026-07-29** —
implementation authorized under the handoff. Maturity promotion, **not** label-only (§5). No release,
no version bump. Part of §5's PART 1 landed early, before this acceptance, in commit
*"Draft RFC-084: promote matten-stats to production-ready candidate"*; the acceptance authorizes it
retroactively (owner decision, 2026-07-29)
**Target:** Post-`0.39.0`, on the `0.x` line
**Theme:** Discharge RFC-081 §3's **Exit A** for `matten-stats`, audited against the RFC-057 bar as
RFC-058, RFC-059 and RFC-080 were
**Depends on:** RFC-030, RFC-040, RFC-057, RFC-067, RFC-078, RFC-081, RFC-083
**Related:** RFC-058, RFC-059, RFC-076, RFC-079, RFC-080

---

## 1. Summary

Promote `matten-stats` from **Experimental** to **production-ready candidate**, discharging RFC-081
§3's Exit A.

**This is not the label-only change RFC-080 was.** The audit in §4 finds one candidate-bar signal
genuinely unmet — `matten-stats` is the only published crate in the family with **no CI job and no
example smoke runs**. That must be fixed *before* the label moves, not alongside it. §5 sequences it.

No version bump, no release, no publish, no API change.

## 2. Why now

RFC-081 §3 requires every `Experimental` crate to take Exit A (promote) or Exit B (leave lock-step)
before the family can reach `1.0.0`, and records that Exit A is the expected path because Exit B —
for a crate already published under lock-step — means either a divergent version line or a
withdrawal that crates.io keeps in its history forever.

The owner decided **Exit A** on 2026-07-29, and separately that **v1.0 is not currently wanted**.
Those two facts together are what make this RFC honest rather than expedient: the promotion is not
being done to unblock a release, because there is no release to unblock. RFC-081 §10 asked for
exactly that — the exit *"should be made on the crate's own evidence … rather than because a v1.0
release is wanted."*

RFC-083 then settled the crate's surface: six functions, estimator conventions stated explicitly and
verified against SciPy and pandas by execution, and the expansion axis closed with each deferral
given a reason.

## 3. The objection this RFC must answer

RFC-080 §7 said, in its own words:

> it does not imply `matten-stats` is near promotion — that crate is Experimental with three-week-old
> APIs and an unreviewed policy divergence

That was written days ago, by this project, about this crate. Promoting now without addressing it
would be exactly the quiet drift this project has repeatedly caught. Taking both halves:

**"An unreviewed policy divergence" — substantially resolved.** The `ddof = 1` choice was the
divergence. RFC-083 (a) verified by execution that the crate's conventions match the ecosystem
defaults its function names imply, closing the question of whether the choice was *correct for its
name*, and (b) added `covariance_population`, so a caller who wants the other estimator no longer has
to reimplement it. The divergence is now documented, empirically anchored, and escapable. What was
never obtained is an *external* read; that remains true and is recorded in §7.

**"Three-week-old APIs" — unchanged, and no RFC can change it.** `matten-stats` still has
essentially no usage history. This RFC does not claim otherwise, and it is the strongest argument
against promoting.

The reason it does not block a *candidate* label is what that rung means here. RFC-081 §5 records
that a candidate label denotes *"an explicit scope or workflow caveat, not hidden API churn — the
crate's surface is settled; only its recommendation breadth is qualified."* Candidate does not assert
"battle-tested"; it asserts "settled surface, recommended narrowly." `matten-stats`'s surface is now
settled. Its usage breadth is exactly what the candidate caveat is *for*.

That reasoning would **not** support promoting it to full production-ready, which is why this RFC
does not (§8).

## 4. Audit against the bar

Candidate signals, per RFC-057 §3: strong tests · examples in CI · clear error types · documented
compatibility policy · no known P0/P1 issues · release checklist complete.

| Signal | Evidence | Verdict |
|---|---|---|
| Strong tests | `tests/statistics.rs`: 33 tests (32 + 1 `dynamic`-gated) plus 7 doctests, for six functions. Covers exact-value pins (`kurtosis([1,2,3,4,5]) == -1.3` bit-exact), the `cov_pop*n == cov_sample*(n-1)` identity, every error path per function, `ZeroVariance` instead of silent `NaN`, and dynamic rejection for all six | ✅ |
| Clear error types | `MattenStatsError`: 6 documented variants, `#[non_exhaustive]`, `Display`, `std::error::Error`. No variant wraps an inner error, so the default `source() -> None` is correct rather than an omission | ✅ |
| Documented compatibility policy | `crates/matten-stats/README.md` §Compatibility; lock-step family versioning (RFC-030); MSRV 1.85 | ✅ |
| No known P0/P1 issues | None recorded. The RFC-083 implementation review approved with no corrections | ✅ |
| No hidden dependency surprises | Sole dependency is `matten`. No feature gates on the API. `check-published-dependency-isolation.sh` passes | ✅ |
| **Examples in CI** | **`matten-stats` has no CI job at all, and none of its four examples are executed.** `grep -rn "matten-stats" .github/` returns nothing | ❌ |
| Release checklist complete | `docs/src/contributing/release-checklist.md` does not mention `matten-stats` | ⚠️ |

### 4.1 The CI gap, precisely

`matten-stats` is covered only incidentally, by workspace-wide steps:

```text
cargo test --workspace --all-targets          .github/workflows/test.yaml:42
cargo test --workspace --doc                  :44
cargo check --workspace --examples --all-features  :46
```

Every other published companion has more:

```text
matten-ndarray   dedicated job (test, --features dynamic, --doc) + 2 example smoke runs
matten-mlprep    dedicated job (test, --features dynamic, --doc) + 5 example smoke runs
matten-data      dedicated job (check --examples, test, --doc, RFC-042 guard) + 8 example smoke runs
matten-stats     none
```

Two concrete consequences, not merely cosmetic asymmetry:

1. **The four examples are compiled but never run.** RFC-057 treated exactly this state for
   `matten-ndarray` as the single ⚠️ on its board and gave it a dedicated section. A compiled example
   can still panic, divide by zero, or print nonsense.
2. **The `dynamic`-gated test runs only in the MSRV job.** Line 42 is `--all-targets`, *not*
   `--all-features`; only the MSRV job (line 104) passes `--all-features`. So the dynamic-rejection
   test for all six functions is exercised once, on the oldest toolchain, and not on stable.

A crate cannot honestly hold a maturity label whose own bar includes "examples in CI" while being the
one crate with neither a job nor a smoke run.

## 5. Decision — fix the bar, then move the label

**The promotion is conditional on the CI gap being closed first, in the same change.** Two ordered
parts; the label must not move if part 1 does not land.

```text
PART 1  Close the CI gap
        - add a `matten-stats` job mirroring matten-mlprep's shape:
            cargo test -p matten-stats
            cargo test -p matten-stats --features dynamic
            cargo test -p matten-stats --doc
        - add the four examples to the smoke-run job:
            stats_covariance, stats_correlation, stats_quantile, stats_expansion
        - add matten-stats to docs/src/contributing/release-checklist.md

PART 2  Move the label: Experimental -> production-ready candidate, at every live site,
        and update the guard that asserts the old value
```

This is the difference from RFC-080, which was genuinely label-only because `matten-mlprep` already
met every signal. Recording it explicitly so this RFC is not read as the same shape of change.

## 6. Scope

### In scope

```text
.github/workflows/test.yaml            new matten-stats job + 4 example smoke runs (PART 1)
docs/src/contributing/release-checklist.md   matten-stats entry (PART 1)

scripts/check-release-docs.sh          the assertion at lines 113/120/124 currently REQUIRES the
                                       string "Experimental" in matten-stats's README and lib.rs.
                                       It must be inverted to assert the new label, exactly as the
                                       matten-data candidate check above it does
crates/matten-stats/src/lib.rs         # Status section
crates/matten-stats/README.md          status banner (line 7)
README.md                              root crate table row
docs/src/reference/compatibility.md    two sites
docs/src/reference/stats.md            the RFC-083 section's "Experimental" mention
rfcs/proposed/076-v1-release-preparation.md   its live inventory: four sites, incl. the
                                       "may not enter while Experimental" precondition
ROADMAP.md                             Status prose + a history row
rfcs/README.md                         remaining-themes row + this RFC's entry
```

### Out of scope

```text
promotion to full production-ready — candidate is the rung RFC-081 Exit A requires (§8)
any code, API, test-logic, or behaviour change in matten-stats itself
the external ddof read — still not obtained, recorded as residual risk (§7)
matten-data's label — separate crate, separate caveat
any v1.0 activity; v1.0 is not currently wanted
version bump, CHANGELOG, release prep, tag, publish
```

### Must not be touched

```text
rfcs/done/**, rfcs/handoffs/**, CHANGELOG.md, docs/design/history/**, ROADMAP history rows
```

Those record what was true when written. `rfcs/done/080-...md:133`'s "not near promotion" line in
particular is a correct record of RFC-080's position and must survive intact — §3 answers it, it does
not get edited away.

## 7. Residual risk

```text
No usage history. The candidate caveat is what carries this, not evidence of field use (§3).
No external read of the ddof policy. RFC-083 verified the conventions match their ecosystem
  names, which is narrower than an independent statistical review. Unchanged by this RFC.
Six functions is a small surface. Candidate is a claim about settledness, not breadth.
```

None of these is hidden by the promotion; the candidate label is precisely the mechanism for
shipping a crate whose recommendation breadth is qualified.

## 8. What this does NOT claim

```text
it does not promote matten-stats to full production-ready
it does not assert field-tested maturity or usage history
it does not resolve the ddof external read
it does not make the family v1.0-ready, and authorizes no release action
it does not change any API, behaviour, or numeric result
it does not affect matten-data, which remains candidate for its own reasons
```

## 9. Compatibility

Label, documentation, CI and guard changes only. No API, behaviour, dependency, feature, MSRV, or
version change. Lock-step family version stays `0.39.0`.

## 10. Acceptance criteria

```text
[ ] PART 1 lands first and is verifiable independently of the label change
[ ] the new matten-stats CI job runs test, --features dynamic, and --doc
[ ] all four examples execute in the smoke job and exit 0
[ ] check-release-docs.sh asserts the NEW label and fails on the old one — proven by
    temporarily reverting one site and observing the guard fail, then restoring
[ ] no site anywhere still calls matten-stats Experimental as a CURRENT claim
[ ] rfcs/done/**, rfcs/handoffs/**, CHANGELOG.md, docs/design/history/** unchanged —
    proven by git diff --name-only
[ ] RFC-081 §3 Exit A recorded as discharged
[ ] full gate set: fmt, clippy, workspace tests, doctests, MSRV, mdbook, all guards
[ ] version still 0.39.0; no CHANGELOG entry, tag, or publish
```

## 11. Non-goals

```text
full production-ready promotion
the external ddof read
any expansion of the six-function surface
v1.0 preparation or execution
```

## 12. Follow-up

Discharging Exit A removes RFC-081's precondition from RFC-076. **RFC-076 remains deferred
regardless** — v1.0 is not currently wanted, and this RFC authorizes nothing about it. When and
whether v1.0 happens stays entirely the owner's decision (RFC-081 §3.1, org policy §2.5/§6.7).
