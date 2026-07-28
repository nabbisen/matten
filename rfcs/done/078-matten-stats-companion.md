# RFC-078: `matten-stats` Companion Crate

**Status:** Implemented — reviewed (GO, no conditions, `matten-rfc078-implementation-review-v0.1.md`), committed `3ab3864`; fifth published crate, Experimental maturity, pre-v1 on the `0.38.x` line; no release authorized
**Target:** Pre-`1.0.0` feature work on the `0.x` line; release family undecided
**Theme:** Create the `matten-stats` companion RFC-040 §9 anticipated, with the three statistics APIs
core deliberately does not carry
**Depends on:** RFC-022, RFC-030, RFC-032, RFC-040
**Related:** RFC-024, RFC-042, RFC-057, RFC-058, RFC-059, RFC-067, RFC-074, RFC-076

---

## 1. Summary

Create a fifth published crate, `matten-stats`, at maturity **Experimental**, providing three statistics
APIs that RFC-040 placed outside core:

```rust
pub fn covariance(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError>;
pub fn correlation(x: &Tensor, y: &Tensor) -> Result<f64, MattenStatsError>;
pub fn quantile(x: &Tensor, q: f64) -> Result<f64, MattenStatsError>;
```

RFC-040 §9 set the gate for this crate's existence: *"Create `matten-stats` only if: at least three APIs
are clearly useful; policy choices are documented; examples are small; it does not overlap confusingly with
`matten-mlprep`; it does not introduce heavy dependencies."* §3 below addresses each condition.

**Deliberately pre-v1.** A new crate's API shape benefits most from being changeable, and `0.x` allows
that. Introducing a companion after `1.0.0` would either freeze an unproven surface under SemVer or ship a
crate whose version number implies a stability it has not earned.

## 2. Motivation

Core `matten` ships `sum`, `mean`, `var`, `std`, `var_axis`, `std_axis` — all population statistics
(`ddof = 0`). RFC-040 §8 deliberately kept quantile and histogram out of core, citing bin-selection,
interpolation, edge-inclusion, sorting cost, and statistical-expectation policy, and directed them to
`matten-stats` "if accepted."

The result is a real gap: users comparing two columns, or wanting a median, currently have nowhere in the
family to go, and the alternatives (hand-rolling, or pulling a full stats crate) are both worse than a
small, explicit companion. This RFC accepts the crate RFC-040 anticipated, with the smallest set of APIs
that clears its own gate.

## 3. Clearing the RFC-040 §9 gate

| Condition | How this RFC satisfies it |
|---|---|
| ≥3 clearly useful APIs | `covariance`, `correlation`, `quantile` — three, each independently useful |
| Policy choices documented | §4 decides sample-vs-population and interpolation explicitly, in the RFC, not in code comments |
| Examples small | One example per API, in the established `stats_*` namespaced style |
| No confusing overlap with `matten-mlprep` | §5 — `mlprep` transforms tensors for ML pipelines; `stats` computes scalar summaries. Disjoint |
| No heavy dependencies | **Zero dependencies beyond `matten`.** All three are implementable with `std` |

## 4. Policy decisions

These are the decisions RFC-040 §8 said must be made before these APIs can exist. They are made here, in
the RFC, so implementation does not have to invent them.

### 4.1 Sample vs population — `ddof = 1` for covariance and correlation

Core uses **population** (`ddof = 0`) for `var`/`std`, documented as such. This RFC uses **sample**
(`ddof = 1`, i.e. `n-1`) for `covariance` and `correlation`.

That is a deliberate divergence and needs justifying, because an inconsistent family is worse than a
consistent one:

- Covariance and correlation are used overwhelmingly in inferential contexts, where the sample estimator is
  the near-universal default (NumPy's `cov`/`corrcoef`, R, pandas all default to `ddof = 1`). Matching core's
  `ddof = 0` here would surprise more users than it would satisfy.
- `correlation` is **unaffected by the choice** — the `n-1` factors cancel in the ratio — so only
  `covariance` is genuinely a decision.
- Core's population choice is correct for *its* purpose (describing a tensor you have, not estimating a
  population you sampled).

**Requirement:** this divergence must be stated explicitly in the crate README, in each function's doc
comment, and in `compatibility.md` — never left for a user to discover empirically. A future
`covariance_population` may be added if demand appears; it is not in this RFC.

### 4.2 Quantile interpolation — linear, on the sorted sample

`quantile(x, q)` uses **linear interpolation between the two nearest ranks**, the method NumPy calls
`linear` and the most widely expected default:

```text
sort the values ascending
h = (n - 1) * q
lo = floor(h), hi = ceil(h)
result = v[lo] + (h - lo) * (v[hi] - v[lo])
```

`q = 0.0` → minimum, `q = 1.0` → maximum, `q = 0.5` → median. Alternative methods (nearest, lower, higher,
midpoint) are **not** provided; adding a method parameter later is additive.

### 4.3 Shape and NaN policy

```text
covariance/correlation: both inputs must have equal element counts and be
  non-empty; shape beyond that is not constrained — values are read in row-major
  order, matching how core reductions treat data
quantile: any non-empty tensor; q must be finite and in [0.0, 1.0]
non-finite input values: rejected with an explicit error, never silently
  propagated — consistent with the finite-value discipline the project already
  applies to JSON report output
correlation with zero variance in either input: explicit error, not NaN
```

Rejecting non-finite input rather than propagating `NaN` is the deliberate choice: a `NaN` correlation
tells the user nothing about *why*.

## 5. Boundary against `matten-mlprep`

RFC-040 §9 requires no confusing overlap. The split is clean and should be stated in both crates' READMEs:

```text
matten-mlprep  transforms tensors for ML pipelines
               (standardize, min-max scale, bias column, train/test split)
               input: Tensor -> output: Tensor

matten-stats   computes scalar statistical summaries
               (covariance, correlation, quantile)
               input: Tensor -> output: f64
```

No function appears in both. `standardize_columns` uses population statistics internally but exposes a
transformed tensor, not a statistic.

## 6. Crate boundary and dependency direction

Follows the established companion pattern exactly (RFC-022, RFC-032):

```text
new workspace member          crates/matten-stats
version                       version.workspace = true (RFC-030 lock-step)
dependencies                  matten = { workspace = true }   — and nothing else
features                      dynamic = ["matten/dynamic"], forwarded for the
                              dynamic-tensor rejection path only
core matten                   must NEVER depend on matten-stats
no companion-to-companion dependency (no matten-mlprep, no matten-data)
publish                       yes — this is a published crate, not a tool
initial maturity              Experimental (RFC-040 §9)
```

**Experimental is the honest label** for a crate with no usage history, and it is a label the family has not
used since the early companions matured. It must appear in the crate README, the root README table, and
`compatibility.md`.

## 7. Scope

### In scope

```text
crates/matten-stats/ — Cargo.toml, src/lib.rs, src/error.rs, the three APIs, README
workspace membership in the root Cargo.toml
guard-script updates for the three scripts that enumerate published crates by name
  (a fourth, check-streaming-scope.sh, globs crates/* and auto-covers the new crate)
root README crate table row (Experimental)
compatibility.md: new-crate entry, maturity, and the ddof divergence (§4.1)
three small examples, stats_* namespaced
```

### Out of scope

```text
histogram (RFC-040 §8 lists bin-selection policy as unresolved; not decided here)
z-score, percentile aliases, mode, skew, kurtosis
covariance/correlation matrices over many columns (scalar pair APIs only)
axis-wise variants
any core matten change
any matten-mlprep change
promoting matten-stats above Experimental
version bump, release prep, tag, publish
```

Histogram is deliberately excluded: RFC-040 §8 named bin-selection policy as an open question, and this RFC
does not resolve it. Three APIs already clear the §9 gate.

## 8. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | **Additive** — a new crate; no existing item changes |
| Runtime behavior | None for existing crates |
| Feature flags | None existing; the new crate forwards `dynamic` |
| Dependencies | **None new** — `matten-stats` depends only on `matten` |
| MSRV | None (`1.85`) |
| Maturity labels | Adds `matten-stats` at **Experimental**; no existing label changes |
| Lock-step family | The family grows from four crates to five; all share the version (RFC-030) |

**The family size change is the notable consequence:** every future release now publishes five crates, and
`cargo package --workspace` plus the publish order both grow by one. RFC-076's release-prep specification
would need updating before it is executed.

## 9. Acceptance criteria

```text
[ ] three APIs implemented with the §4 policies, exactly as specified
[ ] ddof = 1 divergence from core documented in README, doc comments, and compatibility.md
[ ] quantile linear interpolation documented, with q=0/0.5/1 behaviour stated
[ ] non-finite input and zero-variance correlation rejected with explicit errors
[ ] zero dependencies beyond matten; core matten does not depend on matten-stats
[ ] Experimental maturity stated in crate README, root README, and compatibility.md
[ ] the three name-enumerating guard scripts updated and passing, and
    check-streaming-scope.sh confirmed to auto-cover the new crate with no false positive
[ ] no matten or matten-mlprep source change
[ ] no version bump, release prep, tag, or publish
```

## 10. Non-goals

```text
[ ] histogram, z-score, skew, kurtosis, mode
[ ] matrix-wide covariance/correlation
[ ] axis-wise variants
[ ] promoting above Experimental
[ ] any core matten API change
[ ] adding a method parameter to quantile
[ ] any 1.0 release activity
```

## 11. Follow-up

If accepted and implemented, a release containing a **new published crate** is a more consequential
decision than a normal minor bump — a fifth crate appears on crates.io permanently. That release, and
RFC-076's corresponding update to account for five crates, are separate decisions.
