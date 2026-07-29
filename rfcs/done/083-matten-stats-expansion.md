# RFC-083: `matten-stats` Expansion — Population Covariance, Skewness, Kurtosis

**Status:** Implemented — commit *"Expand matten-stats with covariance_population, skewness, kurtosis"*;
implementation reviewed and approved 2026-07-29. Additive API only; no release, no version bump, no
maturity change (`matten-stats` remains `Experimental`). `matten-stats` now exposes six functions
**Target:** Post-`0.39.0` feature work on the `0.x` line
**Theme:** Grow `matten-stats` from three functions to six, along the axis RFC-078 §7 deferred, and
settle the estimator question RFC-078 §4.1 left partially open
**Depends on:** RFC-030, RFC-032, RFC-040, RFC-078, RFC-081
**Related:** RFC-057, RFC-058, RFC-067, RFC-076, RFC-079, RFC-080

---

## 1. Summary

Add three functions to `matten-stats`:

```rust
pub fn covariance_population(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError>;
pub fn skewness(x: &Tensor)                          -> Result<f64, MattenStatsError>;
pub fn kurtosis(x: &Tensor)                          -> Result<f64, MattenStatsError>;
```

All three are scalar summaries of tensor input — `Tensor -> f64` — so the crate's shape and its boundary
against `matten-mlprep` (RFC-078 §5) are unchanged. No new error variant. No new dependency. No version
bump, no release, no maturity change.

The crate goes from 3 public functions to 6.

## 2. Motivation

Two distinct reasons, and they should not be conflated.

**2.1 The `ddof` question is currently unanswerable by a user.** RFC-078 §4.1 chose the sample estimator
(`ddof = 1`) for `covariance`, deliberately diverging from core `matten`'s population `var`/`std`
(`ddof = 0`). That divergence is documented, but a caller who wants the population estimator has no way to
get it from this crate — they must reimplement it. `covariance_population` closes that, and converts the
open question from *"did we pick the right default?"* into *"we picked a default and both are available."*

This does **not** dissolve the default question — `covariance` still means the sample estimator, and that
choice still has to be right. It reduces the cost of the choice being wrong from "users must reimplement"
to "users call the other function."

**2.2 The crate is thin.** Three functions is a small surface for a published crate. `skewness` and
`kurtosis` are the two most commonly reached-for scalar summaries not already present, they fit the
`Tensor -> f64` shape exactly, and they need no policy that this RFC cannot settle (§4).

**What this RFC is not for.** It is not motivated by a v1.0 release; v1.0 is not currently wanted. It is
not a promotion — `matten-stats` stays `Experimental` here (§7). Sequencing note in §10.

## 3. Why there is no `correlation_population`

`crates/matten-stats/src/covariance.rs:76-84` already records this, and it is worth restating because a
reader will otherwise expect symmetry:

> The `n - 1` factors in the sample covariance and the two sample standard deviations cancel
> algebraically, so `correlation` is identical whether computed with `ddof = 0` or `ddof = 1` — only
> `covariance` is a genuine policy decision.

So the ddof axis has exactly one function on it, and one addition completes it. Adding a
`correlation_population` would add a second name for a function that returns the same numbers.

## 4. Policy decisions

### 4.1 Estimator convention — match the ecosystem default per function

RFC-078 §4.1 justified `ddof = 1` for covariance not on statistical principle but on convention: *"the
near-universal default (NumPy's `cov`/`corrcoef`, R, pandas all default to `ddof = 1`). Matching core's
`ddof = 0` here would surprise more users than it would satisfy."*

This RFC keeps that principle and applies it consistently — which means the chosen estimator **differs
per function**, because the ecosystem's defaults differ per function:

```text
covariance             sample,     ddof = 1        (RFC-078 §4.1; NumPy/R/pandas default)
covariance_population  population, ddof = 0        (explicit in the name; no default to choose)
skewness               g1,  uncorrected            (SciPy skew(bias=True) default; NumPy, R e1071 type 1)
kurtosis               g2,  uncorrected, EXCESS    (SciPy kurtosis(fisher=True, bias=True) default)
```

**This asymmetry is deliberate and is the most likely thing to be challenged, so it is stated plainly:**
`covariance` uses the bias-corrected estimator while `skewness`/`kurtosis` use uncorrected ones. That is
not an inconsistency in the underlying principle — the principle is *"match what a user coming from the
Python/R ecosystem will expect from a function of this name"*, and it is applied identically in all four
cases. It would become an inconsistency only under a different principle ("always bias-correct"), which
this project has not adopted and which RFC-078 §4.1 already declined.

pandas' `.skew()`/`.kurt()` **do** bias-correct, so a pandas user will see a different number than a
SciPy/NumPy user. Both conventions must therefore be named explicitly in the doc comments, with the
formula written out — not merely "skewness."

> **Unverified in this environment.** The ecosystem-default claims in this section — SciPy's
> `skew(bias=True)` / `kurtosis(fisher=True, bias=True)` defaults, and pandas' bias-corrected
> `.skew()`/`.kurt()` — are stated from documentation knowledge. `numpy`, `scipy`, and `pandas` are not
> installed here, so they were **not** confirmed by execution. They are the load-bearing justification for
> §4.1, so confirm them before implementation: install the three packages and run `stats.skew([1,2,3,10])`,
> `stats.skew([1,2,3,10], bias=False)`, `stats.kurtosis([1,2,3,10])`, and `pd.Series([1,2,3,10]).skew()`.
> If any default differs from what is claimed here, §4.1's choice must be revisited, not the
> documentation adjusted to match. What §4.2's formulas compute *is* verified — see §9.

**Alternative considered:** offer both forms (`skewness` / `skewness_sample`). Rejected for now — it
doubles the surface for a distinction most callers do not need, on a crate whose surface we want to settle,
and the uncorrected form is the ecosystem default. If a bias-corrected form is later wanted it is a purely
additive follow-up.

### 4.2 Formulas, written out

For `n` elements with mean `x̄`, define the central moments `m_k = Σ (xᵢ - x̄)^k / n`:

```text
covariance_population(x, y) = Σ (xᵢ - x̄)(yᵢ - ȳ) / n

skewness(x)                 = m3 / m2^(3/2)

kurtosis(x)                 = m4 / m2^2  -  3.0       (excess; Fisher's definition)
```

The `- 3.0` makes a normal distribution's kurtosis `0.0` rather than `3.0`. This must be stated in the doc
comment, since "kurtosis" alone is ambiguous between the two.

### 4.3 Minimum element counts

```text
covariance_population   n >= 1     divisor is n, so n = 1 is well-defined and returns 0.0
skewness                n >= 2     needs m2 > 0; see below
kurtosis                n >= 2     needs m2 > 0; see below
```

`covariance_population` accepting `n = 1` is a deliberate difference from `covariance`, which requires
`n >= 2` because its `n - 1` divisor would be zero. Returning `0.0` for a single observation matches
NumPy's `cov(..., ddof=0)`. It is documented rather than rejected.

For `skewness`/`kurtosis` the binding constraint is not `n` but `m2 > 0`: a constant input has zero
variance and the ratio is `0/0`. That is reported as `ZeroVariance`, consistent with `correlation`'s
existing treatment (RFC-078 §4.3: an explicit error rather than a silent `NaN`).

Note that `n = 2` yields a mathematically defined but statistically meaningless skewness. This RFC does
**not** raise the minimum to 3 or 4: the crate's existing policy is to reject only what is undefined, and
to document rather than paternalise. Stated in the doc comments.

### 4.4 Shape, dynamic, and non-finite policy — unchanged

All three functions inherit RFC-078 §4.3 exactly, via the same validation the existing functions use:

```text
dynamic tensor      -> DynamicTensor
non-finite value    -> NonFiniteValue
empty / too few     -> Empty
length mismatch     -> LengthMismatch   (covariance_population only)
zero variance       -> ZeroVariance     (skewness/kurtosis only)
```

Values are read in row-major order; shape beyond the element count is not constrained.

### 4.5 No new error variant

Every failure above maps to an existing `MattenStatsError` variant. This is deliberate: RFC-078 §4.3
established the policy, and a caller moving between functions should not meet a parallel error story. The
`Empty` variant's doc comment must be updated, since its current text names only `covariance`/`correlation`
as the "fewer than 2" cases.

## 5. Boundary checks

**Against `matten-mlprep` (RFC-078 §5).** The split is `matten-mlprep` transforms tensors
(`Tensor -> Tensor`); `matten-stats` computes scalar summaries (`Tensor -> f64`). All three additions
return `f64`. No function appears in both crates.

**Against core `matten` (RFC-040 §9, "no confusing overlap").** Core has population `var`/`std`. This RFC
adds no variance or standard-deviation function, so no overlap is created. `covariance_population` has no
core counterpart.

**Dependency direction (RFC-078 §6).** Unchanged: `matten-stats` depends on `matten` only. No
companion-to-companion dependency.

## 6. Scope

### In scope

```text
crates/matten-stats/src/covariance.rs   covariance_population
crates/matten-stats/src/moments.rs      new module: skewness, kurtosis
crates/matten-stats/src/lib.rs          mod + pub use + crate-doc section for the estimator conventions
crates/matten-stats/src/error.rs        Empty variant doc-comment update only (§4.5)
crates/matten-stats/tests/statistics.rs tests for all three
crates/matten-stats/examples/           one new example, plus its [[example]] entry in Cargo.toml
                                        using the existing stats_* name prefix (a convention set by
                                        RFC-078 §7 and carried in Cargo.toml; no guard enforces it)
crates/matten-stats/README.md           public-API list + the estimator convention table
docs/src/reference/stats.md             the same, for the book
```

### Out of scope — and why

```text
histogram                    RFC-040 §8 named bin-selection policy unresolved; RFC-078 §7 deferred it
                             on that basis; this RFC does not resolve it either
covariance/correlation       returns a matrix, not f64 — would change the crate's stated Tensor -> f64
  matrices, axis-wise        shape (RFC-078 §5). That is a boundary decision deserving its own RFC
z-score                      returns a Tensor, so it belongs to matten-mlprep by RFC-078 §5 — and
                             matten_mlprep::standardize_columns already does it. Adding it here would
                             create exactly the "confusing overlap" RFC-040 §9 forbids
percentile aliases           a second name for quantile() with a 0-100 argument. Pure sugar; it grows
                             a surface we want to settle without adding capability
mode                         ill-defined for f64 without a binning or tolerance policy — the same class
                             of unresolved question as histogram
bias-corrected skew/kurt     §4.1; purely additive later if wanted
any core matten change       —
any matten-mlprep change     —
promotion above Experimental §7
version bump, release, tag, publish
```

## 7. Maturity — explicitly unchanged

`matten-stats` remains **Experimental** throughout this RFC. Two consequences:

- `scripts/check-release-docs.sh` positively asserts that `matten-stats` declares `Experimental` — line
  120 checks `README.md`, line 124 checks `src/lib.rs`. That guard must keep passing **unmodified**. If it
  fails, the implementation has changed a maturity label it was not asked to change.
- RFC-081's rule is untouched. This RFC neither takes nor forecloses Exit A or Exit B.

Adding surface to an `Experimental` crate is precisely what the `0.x` line and that label exist for.

## 8. Compatibility

```text
additive only; no existing signature, behavior, or error mapping changes
MattenStatsError gains no variant (§4.5); it is #[non_exhaustive] regardless
no new dependency
no feature gate — these are unconditional, like the existing three
lock-step family version unchanged at 0.39.0 (RFC-030)
MSRV unchanged (1.85)
```

`correlation`'s documented ddof-invariance (§3) means no existing numeric result changes.

## 9. Acceptance criteria

```text
[ ] three functions implemented, matching §4.2's formulas exactly
[ ] covariance_population verified against covariance on the same input:
    cov_pop * n == cov_sample * (n - 1), to within f64 tolerance
[ ] skewness of a symmetric input is exactly 0.0 (e.g. [1,2,3,4,5])
[ ] skewness of an asymmetric fixture matches a hand-computed value, with the
    arithmetic written out in the test comment
[ ] EXCESS CONVENTION, pinned by an exact value, not an approximation:
      kurtosis([1.0, 2.0, 3.0, 4.0, 5.0]) == -1.3
    m2 = 2, m4 = 6.8, so the raw (Pearson) ratio is 6.8/4 = 1.7 and the excess
    (Fisher) value is 1.7 - 3 = -1.3. Asserting -1.3 distinguishes the two
    conventions unambiguously. Do NOT write a test asserting that some
    "normal-ish" small sample has excess kurtosis near 0 — small discrete
    samples do not, and such a test either fails or gets a meaningless
    tolerance widened until it passes
[ ] ZeroVariance on constant input, for both skewness and kurtosis
[ ] covariance_population accepts n = 1 and returns 0.0
[ ] every RFC-078 §4.3 error path exercised for each new function
[ ] Empty variant doc comment updated (§4.5)
[ ] estimator convention table present in lib.rs, README.md, and docs/src/reference/stats.md,
    naming the SciPy/pandas divergence explicitly (§4.1)
[ ] check-release-docs.sh passes UNMODIFIED, still asserting Experimental (§7)
[ ] full gate set: fmt, clippy (all-targets), workspace tests, doctests, MSRV 1.85
[ ] no version bump, CHANGELOG entry, tag, or publish
[ ] git diff touches no crate other than matten-stats (plus docs/src/reference/stats.md)
```

## 10. Follow-up and sequencing

This RFC is the first of two. The second is the `matten-stats` promotion to production-ready candidate
(Exit A under RFC-081 §3), decided by the owner on 2026-07-29.

**Expansion precedes promotion, deliberately.** RFC-081 §5 defines a candidate as a crate whose surface is
settled. Promoting the three-function crate and then adding three more would leave the additions outside
the RFC-057 audit that the promotion performs. RFC-080 set the precedent in the other direction:
`matten-mlprep` was promoted *after* RFC-077 added the feature that closed its exit criterion. The
promotion RFC should audit the six-function surface, once.

Nothing in this RFC authorizes that promotion, a release, or a version bump.

## 11. Non-goals

```text
resolving whether ddof = 1 is the right default for covariance — it stays the default (§2.1)
resolving histogram's bin-selection policy (RFC-040 §8)
changing the crate's Tensor -> f64 shape
any v1.0 activity; v1.0 is not currently wanted
promoting or demoting any maturity label
```
