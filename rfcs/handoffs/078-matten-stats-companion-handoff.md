# RFC-078 `matten-stats` Companion: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-078 (design authority)
**Document kind:** Detailed implementation handoff
**Status:** Accepted; implemented on the `0.38.x` line; no version bump or release
**Date:** 2026-07-28

---

## 1. Purpose

Create the `matten-stats` companion crate as one reviewable slice. RFC-078 is the design authority.

**No release.** Ends at a reviewed implementation commit on the `0.x` line.

## 2. Preconditions

```text
RFC-078 and this handoff accepted
RFC-077 implemented, reviewed, and committed FIRST — see §3
working tree clean; version stays 0.38.0
RFC-076 (v1.0 prep) remains proposed and unexecuted
```

## 3. Sequencing — do RFC-077 first

Both slices touch the published family, and RFC-077 is far smaller. Landing it first means:

- the first published-crate change since `0.31.0` is a **single additive function** in an existing crate,
  not a whole new crate;
- if anything about resuming published-crate work is rusty (gates, feature matrix, MSRV), it surfaces on
  the cheap slice;
- the two reviews do not compete for attention on the same files.

Do not start this handoff until RFC-077 is committed.

## 4. New crate layout

```text
crates/matten-stats/
  Cargo.toml
  README.md
  src/lib.rs           crate-level //! docs, pub use re-exports, dynamic-rejection helper
  src/error.rs         MattenStatsError
  src/covariance.rs    covariance + correlation
  src/quantile.rs      quantile
  tests/statistics.rs  ALL tests for the crate (see §6)
  examples/covariance.rs
  examples/correlation.rs
  examples/quantile.rs
```

Split `covariance`/`correlation` into one module since they share validation and mean computation. Keep
every file well under the project's 500-line norm.

### `src/lib.rs` — two things that are easy to under-specify

Both were omitted from the RFC-077 handoff's equivalent list and had to be worked out during
implementation. They are stated explicitly here so that does not repeat.

**1. The re-exports are what make this a public API.** Without them the crate compiles and exports
nothing:

```rust
pub use crate::covariance::{correlation, covariance};
pub use crate::error::MattenStatsError;
pub use crate::quantile::quantile;
```

Mirror `matten-mlprep/src/lib.rs`'s re-export style. Do **not** re-export anything from `matten` —
RFC-032 forbids companions re-exporting core.

**2. Crate-level `//!` documentation is required**, not optional, and must state:

```text
what the crate is: small, explicit scalar statistics over matten::Tensor
the matten-mlprep boundary (RFC-078 §5): mlprep transforms Tensor -> Tensor,
  stats computes Tensor -> f64; no function appears in both
the ddof = 1 divergence from core (RFC-078 §4.1): covariance and correlation
  use the sample estimator, while core matten's var/std are population
  (ddof = 0) — a reader must not have to discover this empirically
the quantile interpolation method (RFC-078 §4.2): linear
Status: Experimental (RFC-040 §9)
```

The `ddof` sentence is the single most important line in the crate's documentation. It is the one place a
user's expectation can silently diverge from behaviour.

### `Cargo.toml` — copy the `matten-mlprep` pattern exactly

```toml
[package]
name        = "matten-stats"
description = "Small, explicit scalar statistics for matten::Tensor (covariance, correlation, quantile)."
readme      = "README.md"
categories  = ["science", "mathematics"]
keywords    = ["tensor", "statistics", "covariance", "correlation", "matten"]
version.workspace      = true
edition.workspace      = true
rust-version.workspace = true
authors.workspace      = true
license.workspace      = true
repository.workspace   = true

[dependencies]
matten = { workspace = true }

[features]
dynamic = ["matten/dynamic"]

[[example]]
name = "stats_covariance"
path = "examples/covariance.rs"
# ... stats_correlation, stats_quantile likewise
```

**Nothing beyond `matten` in `[dependencies]`.** The `stats_*` example-name prefix is mandatory — bare
names collide across the workspace.

Add `"crates/matten-stats"` to the root `Cargo.toml` `[workspace] members` list, after
`"crates/matten-data"`.

## 5. Implementation

### 5.1 Errors

```rust
pub enum MattenStatsError {
    DynamicTensor,                                   // mirrors mlprep's variant
    Empty,                                           // empty input
    LengthMismatch { left: usize, right: usize },    // covariance/correlation
    NonFiniteValue,                                  // RFC-078 §4.3
    ZeroVariance,                                    // correlation only
    InvalidQuantile(f64),                            // q not finite or outside [0,1]
}
```

Match `matten-mlprep`'s error style: `Display`, `std::error::Error`, and doc comments per variant. Do not
re-export `matten`'s error type — RFC-032 forbids companions re-exporting core.

### 5.2 `covariance` — sample, `ddof = 1`

```text
validate: not dynamic; equal element counts; non-empty; n >= 2; all finite
mean_x = sum(x)/n ; mean_y = sum(y)/n
cov = Σ (xi - mean_x)(yi - mean_y) / (n - 1)
```

`n < 2` must error (`Empty` or a dedicated case) — `n - 1` would divide by zero. Read values in row-major
order via `as_slice()`; shape beyond element count is not constrained (RFC-078 §4.3).

### 5.3 `correlation`

```text
cov(x, y) / (std_sample(x) * std_sample(y))
```

Both standard deviations use `ddof = 1`. **Compute them locally — do not call core's `std()`,** which is
population (`ddof = 0`); mixing them silently produces a wrong result. If either is zero, return
`ZeroVariance` rather than `NaN` (RFC-078 §4.3).

Note in the doc comment that the `n-1` factors cancel, so correlation is identical under either convention —
this is why only `covariance` is a real policy decision.

### 5.4 `quantile` — linear interpolation

```text
validate: not dynamic; non-empty; q finite and in [0.0, 1.0]; all values finite
sort a copy ascending with f64::total_cmp (values are already finite-checked)
h  = (n - 1) as f64 * q
lo = h.floor() as usize ; hi = h.ceil() as usize
if lo == hi { v[lo] } else { v[lo] + (h - lo as f64) * (v[hi] - v[lo]) }
```

Use `total_cmp`, not `partial_cmp().unwrap()`. Sort a copy; never mutate the caller's data.

### 5.5 Dynamic rejection

Every public function must reject dynamic tensors under `feature = "dynamic"`, mirroring how
`matten-mlprep` does it. Follow that crate's existing helper rather than inventing a new mechanism.

## 6. Tests — required

### Placement — decided, not left to convention

`matten-stats` is a new crate with no existing convention to follow, so this is specified rather than
inferred:

```text
ALL tests go in a single integration test file: crates/matten-stats/tests/statistics.rs
```

Rationale: this matches `matten-mlprep`, which keeps its 27 test fns in one `tests/preprocessing.rs`, and
integration tests exercise the crate through its **public** surface — which is exactly what should be
verified for a crate whose entire purpose is three public functions. Do not add `#[cfg(test)]` unit modules
inside `src/*.rs` unless a test genuinely cannot be written through the public API; if that happens, say so
in the review request rather than splitting placement silently.

Doctests on the three public functions are separate from this and are still required (§7).

```text
covariance
  [ ] known-value case verified against a hand-computed expectation
  [ ] ddof = 1 explicitly: for a small vector, assert the n-1 result and show it
      differs from the n result — this locks the §4.1 policy
  [ ] symmetry: cov(x,y) == cov(y,x)
  [ ] cov(x,x) equals the sample variance of x
  [ ] length mismatch -> LengthMismatch ; n < 2 -> error ; non-finite -> NonFiniteValue

correlation
  [ ] perfect positive (y = 2x + 1) -> 1.0 within tolerance
  [ ] perfect negative -> -1.0
  [ ] known intermediate value against a hand-computed expectation
  [ ] bounded in [-1, 1] for several inputs
  [ ] zero variance in either input -> ZeroVariance (NOT NaN)
  [ ] invariance: correlation is identical whether computed with ddof 0 or 1
      (documents why §4.1 only affects covariance)

quantile
  [ ] q = 0.0 -> min ; q = 1.0 -> max ; q = 0.5 on odd n -> exact middle
  [ ] q = 0.5 on even n -> interpolated midpoint
  [ ] a known non-midpoint case, hand-computed, locking the linear method
  [ ] unsorted input yields the same result as sorted (input order irrelevant)
  [ ] input tensor is NOT mutated
  [ ] q outside [0,1], NaN, or infinite -> InvalidQuantile
  [ ] non-finite value in input -> NonFiniteValue

shared
  [ ] empty tensor -> Empty for all three
  [ ] dynamic tensor (feature = "dynamic") -> DynamicTensor for all three
  [ ] doctests on all three public functions
```

Use an explicit epsilon for float comparisons; match the tolerance style already used in `matten-mlprep`
tests rather than introducing a new one.

## 7. Documentation

```text
crates/matten-stats/README.md
  overview; the mlprep boundary (RFC-078 §5, stated explicitly);
  ## Public API block (the NF-1 convention — all companions have one);
  Status: Experimental;
  the ddof = 1 divergence from core, called out prominently;
  the quantile interpolation method
README.md (root)
  new crate-table row: matten-stats | 0.38.x family | Experimental | <description>
docs/src/reference/compatibility.md
  new-crate entry, Experimental maturity, and the ddof divergence
```

Do **not** touch `CHANGELOG.md`, versions, or `public-api-snapshot.md` (that documents core `matten`).

**Before writing the `ddof = 1` rationale into the README and `compatibility.md`**, confirm the NumPy /
pandas / R defaults RFC-078 §4.1 cites against current documentation for each — this is a third-party
claim the RFC could not verify from this repository. If any of them has since changed its default, report
that in the review request rather than propagating §4.1's claim unchecked.

## 8. Guard scripts — three need the new crate, one auto-covers it

These enumerate published crates by name and will not know about `matten-stats` until edited:

```text
scripts/check-core-dependency-boundary.sh
scripts/check-published-dependency-isolation.sh
scripts/check-release-docs.sh
```

Read each before editing. The critical additions:

- **core-dependency-boundary** — assert core `matten` does not depend on `matten-stats`.
- **published-dependency-isolation** — assert `matten-stats` pulls in no third-party dependency.
- **release-docs** — include `matten-stats` in whatever crate-table / maturity checks it performs.

Run each before *and* after your edit and confirm it still fails on a real violation — a guard that no
longer detects anything is worse than no guard.

**`scripts/check-streaming-scope.sh` needs no edit.** It globs `crates/*/src` (not an explicit crate list),
so it picks up `matten-stats` automatically the moment the crate exists. Its patterns (`stream_csv`,
`CsvStream`, `BatchReader`, `AsyncCsvReader`, and the matching example names) do not match any planned
`matten-stats` symbol, so no false positive is expected. Do **not** add the crate to it — there is nothing
to add.

**`scripts/check-benchmark-dependency-sync.sh` and `scripts/check-matten-data-scope.sh` are unaffected.**
The former only syncs the `ndarray` peer-dependency pin between the root manifest and the out-of-workspace
benchmark harness; it has nothing to do with crate enumeration. The latter is specific to `matten-data`'s
dataframe scope lock. Neither needs a look, let alone an edit.

## 9. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --no-default-features --features dynamic -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo test -p matten-stats --all-features
cargo test -p matten-stats --no-default-features
cargo run -p matten-stats --example stats_covariance
cargo run -p matten-stats --example stats_correlation
cargo run -p matten-stats --example stats_quantile
cargo +1.85.0 build && cargo +1.85.0 test --all-features
cargo package --workspace          # must now package FIVE crates
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-matten-data-scope.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-release-docs.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

Scope confirmation:

```bash
cargo metadata --format-version 1 --no-deps    # five crates, all at 0.38.0
git diff -- crates/matten/ crates/matten-mlprep/ crates/matten-data/ crates/matten-ndarray/
                                                # expect EMPTY
bash tools/matten-report/tests/process-boundary.sh   # anchors untouched
```

## 10. What the review request must report

```text
[ ] cargo metadata showing five crates at 0.38.0
[ ] confirmation matten-stats has exactly one dependency (matten)
[ ] the src/lib.rs re-exports, shown — all three functions and MattenStatsError
    reachable as matten_stats::<name> from outside the crate
[ ] the crate-level //! docs, confirming they state the mlprep boundary, the
    ddof = 1 divergence, the quantile method, and Experimental status
[ ] the ddof = 1 test output, showing n-1 vs n differ
[ ] the correlation ddof-invariance test result
[ ] the locked quantile non-midpoint case
[ ] zero-variance -> ZeroVariance (not NaN) confirmed
[ ] all tests in tests/statistics.rs; if any #[cfg(test)] unit module was added
    inside src/, the reason it could not be written through the public API
[ ] the three guard scripts updated, and evidence each still fails on a violation
[ ] confirmation that check-streaming-scope.sh needed no edit and shows no false positive
[ ] the NumPy/pandas/R ddof=1 defaults reconfirmed against current docs (or any change reported)
[ ] cargo package --workspace packaging five crates
[ ] git diff showing the four existing crates untouched
[ ] full gate set incl. MSRV
[ ] confirmation of no version bump, CHANGELOG, or release metadata change
```

## 11. Known pitfalls

1. **Calling core `std()` inside `correlation`** — it is population (`ddof=0`); the result would be subtly
   wrong. Compute sample deviation locally.
2. **`partial_cmp().unwrap()` when sorting** — use `total_cmp`.
3. **Mutating the caller's data while sorting** — sort a copy.
4. **Returning `NaN`** for zero variance instead of an explicit error.
5. **Forgetting a guard script** — the crate would silently escape dependency-boundary enforcement.
6. **Adding a dependency** — even a tiny one defeats RFC-040 §9's "no heavy dependencies" and RFC-078's
   zero-dependency claim.
7. **Bumping the version or writing CHANGELOG** — no release in this slice.
8. **Labelling it above Experimental** — RFC-040 §9 fixes the initial maturity.
9. **Adding histogram** — explicitly out of scope; its bin policy is unresolved.

## 12. Review stop

Acceptance makes this a commit point. It does not authorize a release, a version bump, publishing a fifth
crate, or any maturity promotion.

**Note for whoever next touches RFC-076:** the family becomes five crates. RFC-076's release-prep
specification, publish order, and `cargo package --workspace` expectations all assume four and must be
updated before that RFC is executed.
