# RFC-083 `matten-stats` Expansion: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/083-matten-stats-expansion.md` — the design authority
**Document kind:** Detailed implementation handoff
**Status:** Inherits RFC-083's state — **accepted 2026-07-29; implementation authorized**. Ends at a
reviewed commit: no release, no version bump, no tag, no publish, no maturity change
**Date:** 2026-07-29

---

## 1. Purpose

Add `covariance_population`, `skewness`, and `kurtosis` to `matten-stats`, taking the crate from 3 public
functions to 6. One reviewable slice.

**No release.** Ends at a reviewed implementation commit on the `0.x` line. Version stays `0.39.0`.

## 2. Preconditions

```text
RFC-083 and this handoff accepted
working tree clean; version stays 0.39.0
matten-stats stays Experimental — this slice changes no maturity label
```

**Before writing code, confirm §9's three ecosystem-default claims by execution.** RFC-083 §4.1 flags them
as unverified — `numpy`/`scipy`/`pandas` were not installed in the authoring environment. They are the
justification for the estimator choice, so if any differs, **stop and escalate**; do not adjust the
documentation to match whatever you find.

## 3. Files

```text
crates/matten-stats/src/covariance.rs    + covariance_population; generalize validate_pair's min-n
crates/matten-stats/src/moments.rs       NEW — skewness, kurtosis, shared central-moment helper
crates/matten-stats/src/lib.rs           + mod moments; + pub use; + estimator-convention doc section
crates/matten-stats/src/error.rs         Empty variant DOC COMMENT ONLY (§5.4)
crates/matten-stats/Cargo.toml           + one [[example]] entry, stats_* prefixed
crates/matten-stats/examples/            + one example
crates/matten-stats/tests/statistics.rs  + tests for all three functions
crates/matten-stats/README.md            public-API list + estimator convention table
docs/src/reference/stats.md              same table, for the book
```

**No new dependency. No feature gate.** The three functions are unconditional, like the existing three.

## 4. Formulas — implement exactly these

For `n` elements with mean `x̄`, central moments `m_k = Σ (xᵢ - x̄)^k / n`:

```text
covariance_population(x, y) = Σ (xᵢ - x̄)(yᵢ - ȳ) / n
skewness(x)                 = m3 / m2^(3/2)
kurtosis(x)                 = m4 / m2^2 - 3.0          <- EXCESS (Fisher). The -3.0 is not optional.
```

## 5. Implementation notes

### 5.1 `covariance_population` — reuse the existing validation

`covariance` and `correlation` share `validate_pair` (`crates/matten-stats/src/covariance.rs:14-37`), which
hard-codes `if left < 2 { return Err(Empty) }`. `covariance_population` needs `n >= 1`, not `n >= 2`
(RFC-083 §4.3) — its divisor is `n`, so a single observation is well-defined and returns `0.0`.

Generalize `validate_pair` to take a minimum, or add a sibling. Either is fine; **what must not happen is a
second copy of the dynamic/length/non-finite checks.** A divergence between the two validation paths is the
defect most likely to slip through here.

### 5.2 `moments.rs` — one helper, three consumers

`skewness` and `kurtosis` both need `m2` plus one higher moment. Compute the mean once and the moments in a
single pass; do not call a public function from another public function to get them.

**Do not use core `matten`'s `var()`/`std()`.** They are population (`ddof = 0`) — which happens to match
what `m2` needs — but `covariance.rs:76-84` already records the standing rule that this crate computes its
own moments locally rather than depending on core's estimator choice, precisely so a future change to core
cannot silently alter a statistic here. Follow it.

### 5.3 Zero variance

`m2 == 0.0` (a constant input) makes both ratios `0/0`. Verified: the naive computation yields `NaN`.
Return `MattenStatsError::ZeroVariance` instead — consistent with `correlation`
(`covariance.rs:121-123`) and with RFC-078 §4.3's "explicit error rather than a silent `NaN`."

### 5.4 `error.rs` — doc comment only

The `Empty` variant currently reads *"...or fewer than 2 elements for `covariance`/`correlation` (their
`n - 1` divisor would be zero)."* That is now incomplete: `covariance_population` requires only 1, and
`skewness`/`kurtosis` require 2. Update the text.

**Add no variant.** Every failure maps to an existing one (RFC-083 §4.5).

## 6. Required tests — `crates/matten-stats/tests/statistics.rs`

```text
[ ] covariance_population against covariance on the same input:
    cov_pop * n == cov_sample * (n - 1), within f64 tolerance.
    VERIFIED to hold: with x=[1,2,3,7], y=[2,4,5,11] both sides are 30.5 exactly
[ ] covariance_population with n = 1 returns 0.0 (not an error)
[ ] covariance with n = 1 still returns Empty — the two minimums differ on purpose
[ ] skewness([1,2,3,4,5]) == 0.0 exactly (symmetric)
[ ] skewness of an asymmetric fixture vs a hand-computed value, arithmetic in the comment
[ ] kurtosis([1,2,3,4,5]) == -1.3 exactly.  m2 = 2, m4 = 6.8, raw ratio 1.7,
    excess = 1.7 - 3 = -1.3. THIS is the test that pins the excess convention
[ ] ZeroVariance for skewness AND kurtosis on a constant input
[ ] Empty for skewness/kurtosis on a 1-element and 0-element input
[ ] DynamicTensor for all three
[ ] NonFiniteValue (NaN and infinity) for all three
[ ] LengthMismatch for covariance_population
[ ] existing 3 functions' results unchanged — the pre-existing tests must still pass untouched
```

**Do not write a test asserting a "normal-ish" sample has excess kurtosis near 0.** Small discrete samples
do not: a 9-point symmetric fixture gives −0.75, not 0. Such a test either fails or gets its tolerance
widened until it stops meaning anything. The exact −1.3 assertion above is the real guard.

## 7. Documentation

The estimator convention table goes in three places — `lib.rs`, `crates/matten-stats/README.md`, and
`docs/src/reference/stats.md`:

```text
covariance             sample,     ddof = 1
covariance_population  population, ddof = 0
skewness               g1, uncorrected
kurtosis               g2, uncorrected, EXCESS (normal = 0.0, not 3.0)
```

Each function's doc comment must write out its formula and name its convention. "Skewness" and "kurtosis"
alone are ambiguous, and the pandas-vs-SciPy divergence (RFC-083 §4.1) must be stated where a user will see
it, not only in the RFC.

Follow the existing house style in `covariance.rs`: `# Errors` bullets naming each variant, plus a runnable
doctest.

## 8. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test -p matten-stats
cargo run -p matten-stats --example <the new stats_* example>
bash scripts/check-release-docs.sh                    # must pass UNMODIFIED — see below
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-matten-data-scope.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

`check-release-docs.sh` **positively asserts** that `matten-stats` declares `Experimental` — line 120 for
`README.md`, line 124 for `src/lib.rs`. If it fails, you have changed a maturity label this slice does not
authorize. Do not edit the guard.

Scope confirmation:

```bash
git diff --name-only -- crates/matten crates/matten-data crates/matten-mlprep crates/matten-ndarray
# expect EMPTY
grep -m1 '^version' Cargo.toml     # still 0.39.0
```

## 9. Escalate before coding if

```text
[ ] scipy's skew/kurtosis defaults are NOT uncorrected-g1 / excess-uncorrected
[ ] pandas' .skew()/.kurt() are NOT bias-corrected
```

Either finding invalidates RFC-083 §4.1's justification. Report it; do not pick a different estimator
locally and do not reword the RFC to fit. This is the one place in this slice where an upstream claim is
explicitly marked unverified, so it is the one most worth checking first.

## 10. Known pitfalls

1. **Forgetting the `- 3.0`.** Raw kurtosis and excess kurtosis differ by exactly 3; the `-1.3` test is
   what catches it.
2. **Duplicating the validation logic** instead of generalizing `validate_pair` (§5.1).
3. **Calling core's `var()`/`std()`** for `m2` (§5.2).
4. **Returning `NaN` on constant input** instead of `ZeroVariance` (§5.3).
5. **Adding an error variant** — none is needed (§5.4).
6. **Touching the maturity label**, or the guard that asserts it (§8).
7. **Adding `correlation_population`** — correlation is ddof-invariant, so it would be a second name for the
   same numbers (RFC-083 §3).
8. **Implementing anything from RFC-083 §6's out-of-scope list** — histogram, z-score, percentile aliases,
   matrix or axis-wise forms. Each was excluded for a stated reason.
9. **Bumping the version.** No release in this slice.

## 11. What the review request must report

```text
[ ] the §9 ecosystem-default check, with the actual command output
[ ] the cov_pop/cov_sample identity test result
[ ] the exact kurtosis == -1.3 assertion
[ ] validate_pair's diff, showing one validation path and not two
[ ] confirmation error.rs gained no variant (diff is the Empty doc comment only)
[ ] check-release-docs.sh passing UNMODIFIED, and that no scripts/ path is in the diff
[ ] the estimator table present in all three doc locations
[ ] the other four crates untouched; version still 0.39.0
[ ] full gate set incl. MSRV and mdbook
```

## 12. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, publish, or maturity
promotion. The `matten-stats` promotion to production-ready candidate is a **separate, later RFC** that
audits the resulting six-function surface (RFC-083 §10).
