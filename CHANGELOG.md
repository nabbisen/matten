# Changelog — matten

All notable changes to the **matten** workspace are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the project
follows [Semantic Versioning](https://semver.org/).

Entries are ordered by release milestone (which is also the tarball version).
The workspace uses **lock-step family versioning** (RFC-030): every crate shares
one version, so each entry applies to the whole family — core `matten`,
`matten-ndarray`, `matten-mlprep`, and `matten-data`. Maturity differences between crates are
expressed by per-crate status labels, not by separate version numbers. Through
`0.16.0` the project was the single `matten` crate.

> **Convention (resolved in v0.19.0, RFC-022 §12).** While the crates ship
> together as milestone tarballs, the project keeps a *single* root `CHANGELOG.md`
> and *root-only* `LICENSE`/`NOTICE`. Each crate is licensed via its SPDX
> `license` field, so no per-crate license file is required. Per-crate changelogs
> and license files are reintroduced if and when crates begin publishing to
> crates.io on independent cadences.

## [0.20.9] - 2026-06-23

**Core numeric comfort APIs — elementwise band (RFC-038, first sub-band). Additive,
non-breaking public API under lock-step family versioning.**

This begins RFC-038 (core comfort APIs), delivered as small sub-bands rather than one
drop. Subsequent bands (selection `argmin`/`argmax`, shape `squeeze`/`expand_dims`,
creation `linspace`/`eye`) follow as separate releases.

### Added

- **Elementwise comfort math on `Tensor`** (RFC-038 §4.3), in a new
  `ops/elementwise.rs` module (placed there per RFC-038 §5.3 so the near-threshold
  `math.rs` is not pushed over the ELOC limit):
  - `abs()`, `sqrt()`, `exp()`, `ln()` — elementwise, shape-preserving, ordinary
    `f64` NaN/Inf behavior (e.g. `sqrt` of a negative is `NaN`, `ln(0.0)` is `-inf`);
  - `clip(min, max)` — clamp into a range (panics if `min > max`);
  - `try_clip(min, max)` — non-panicking form returning `Result`.
- **`MattenError::InvalidArgument { operation, argument, message }`** (RFC-038 §5.2) —
  for supported operations given an out-of-range argument (e.g. `clip` with
  `min > max`), distinct from `Unsupported`. Added under the existing
  `#[non_exhaustive]` enum, so it is **non-breaking**.

### Changed

- Reference docs updated to match: `error-model.md` (new variant + guide row),
  `public-api-snapshot.md` (new elementwise section), and `operators.md` (user-facing
  comfort-math section).

### Notes

- All new methods are numeric-only: on a dynamic tensor the convenience forms panic
  with an `Unsupported` message and `try_clip` returns `MattenError::Unsupported`;
  call `try_numeric()` first. This follows the established dynamic-rejection pattern.
- No new data flows, external integrations, or auth: these are pure in-memory numeric
  transforms and local argument validation. `#![forbid(unsafe_code)]`, the
  core→companion dependency boundary, and the release-doc guards remain valid; the
  threat model is unchanged.

## [0.20.8] - 2026-06-23

**Examples program — ML-like band (RFC-047). Additive examples and documentation
under lock-step family versioning; no API, behavior, or breaking change to any crate.
This completes the implementable example bands of RFC-043–048.**

### Added

- **ML-like examples** (RFC-047), default Phase-1 numeric API, small hard-coded
  inputs, deterministic self-checking output, with an explicit "algorithm demo, not
  an ML framework" boundary:
  - `examples/37_kmeans_small.rs` — Lloyd's k-means on a `[points, features]` `Tensor`
    with fixed (deterministic) initial centroids; converges to two clusters;
  - `examples/38_nearest_neighbor_classification.rs` — 1-nearest-neighbor
    classification over a labeled data matrix.
  - Both use a small local `argmin` helper (core `matten` has no `argmin` yet; a
    future RFC-038 candidate), so they stay on current APIs.
- **Examples documentation**: new `docs/src/examples/ml-like.md`, the two examples
  added to the "Applied problems" table in `docs/src/examples/index.md`, and a
  `SUMMARY.md` entry.
- **CI smoke runs** for both new examples in `test.yaml`.

### Notes

- This finishes the additive `30+` famous-problem band that current APIs support:
  `30_`–`38_` are now implemented across RFC-044 (`30_`–`32_`), RFC-045 (`33_`–`34_`),
  RFC-046 (`35_`–`36_`), and RFC-047 (`37_`–`38_`); RFC-048 audited the existing
  companion examples in place. Still deferred: `39_finite_difference_derivative` and
  `40_trapezoidal_integration` (await RFC-038), and the optional, not-reserved
  `41_adjacency_walks_extended`.
- No new data flows, external integrations, or auth: examples use only in-memory
  hard-coded data. `#![forbid(unsafe_code)]`, the core→companion dependency boundary,
  and the release-doc guards remain valid and unchanged.

## [0.20.7] - 2026-06-23

**Examples program — numerical-methods band (RFC-046). Additive examples and
documentation under lock-step family versioning; no API, behavior, or breaking
change to any crate.**

### Added

- **Numerical-methods examples** (RFC-046), default Phase-1 numeric API, small
  hard-coded inputs, deterministic self-checking output:
  - `examples/35_linear_regression_gradient_descent.rs` — fits `y = 2x + 1` by batch
    gradient descent, using `Tensor::matmul` for predictions and `Tensor::transpose`
    + `matmul` for the MSE gradient;
  - `examples/36_heat_equation_1d.rs` — explicit finite-difference heat equation on a
    rod, encoding the stencil as a tridiagonal matrix so each time step is one
    `Tensor::matmul`; converges to the steady-state linear profile.
- **Examples documentation**: new `docs/src/examples/numerical-methods.md`, the two
  examples added to the "Applied problems" table in `docs/src/examples/index.md`, and
  a `SUMMARY.md` entry.
- **CI smoke runs** for both new examples in `test.yaml`.

### Notes

- Examples continue in the additive `30+` band; the existing `00_`–`28_` suite and
  the named/`dynamic_*` examples are unchanged. The two further numerical examples
  (`39_finite_difference_derivative`, `40_trapezoidal_integration`) remain deferred
  until the RFC-038 comfort APIs land.
- No new data flows, external integrations, or auth: examples use only in-memory
  hard-coded data. `#![forbid(unsafe_code)]`, the core→companion dependency boundary,
  and the release-doc guards remain valid and unchanged.

## [0.20.6] - 2026-06-23

**Examples program — companion-crate examples audited and improved in place
(RFC-048). Additive documentation and example hardening under lock-step family
versioning; no API, behavior, or breaking change to any crate, and no new or renamed
example files.**

### Changed

- **Audited and improved the existing companion examples in place** (RFC-048, per
  architect Q2/Q3 — no duplicate or renamed files):
  - `matten-ndarray`: `from_arrayd`, `to_arrayd` — richer teaching headers (copies
    data / no zero-copy claim, shape preserved, numeric-only conversion, one-way
    dependency direction); added a shape-preservation assertion to `to_arrayd`.
  - `matten-mlprep`: `standardize_columns`, `minmax_scale`, `add_bias_column`,
    `train_test_split` — teaching headers (rows = samples, columns = features,
    deterministic, no model training) and **deterministic correctness assertions**
    (previously print-only).
  - `matten-data`: `csv_to_tensor` — header now states Experimental status and the
    "not a dataframe" boundary; added a full data assertion on the converted tensor.

### Added

- **Examples documentation**: new `docs/src/examples/companions.md`, a "Companion
  crate examples" section in `docs/src/examples/index.md`, and a `SUMMARY.md` entry.
- **CI smoke run** for `matten-data csv_to_tensor` in `test.yaml` (the `matten-ndarray`
  and `matten-mlprep` examples were already covered).

### Notes

- All companion examples keep their filenames, `[[example]]` names, and printed
  output stable; they remain RFC-032-compliant (import `Tensor` from `matten`, no
  companion re-export) — the release-doc guard confirms this. No new files were
  added; vector distance, cosine similarity, and the companion examples are
  cross-referenced, not duplicated.
- No new data flows, external integrations, or auth: examples use only in-memory
  hard-coded data. `#![forbid(unsafe_code)]`, the core→companion dependency
  boundary, and the release-doc guards remain valid and unchanged.

## [0.20.5] - 2026-06-23

**Benchmarking & positioning program planning (RFC-049). Documentation/planning
patch under lock-step family versioning — no source, API, or behavior change to any
crate.**

### Added

- **RFC-049 — Benchmarking, Complexity Metrics, and Positioning Report** added to
  `rfcs/proposed/`: a reproducible, anti-marketing measurement program (execution
  time, memory where practical, example ELOC, dependency footprint, regression
  visibility) with peer (`ndarray`/`nalgebra`) vs reference (NumPy/Pandas)
  separation and SciPy/Candle deferred.
- **Benchmarking developer handoff** at
  `rfcs/handoffs/049-benchmarking-developer-handoff.md` (PR plan, QA checklist,
  isolated `publish = false` benchmark package).

### Changed

- **ROADMAP** bumped to Document Version `1.5.0`: added **Track D — benchmarking &
  positioning** (goal, posture/sequencing, four phases, binding hard constraints,
  acceptance gate), added RFC-049 to the v0.20+ RFC table, and recorded the shipped
  v0.20.3 / v0.20.4 example bands plus this v0.20.5 benchmarking-planning row in the
  release-theme table.
- **`rfcs/README.md`** index lists RFC-049 under Proposed and points to its handoff.

### Notes

- RFC-049 is a non-API, measurement-only RFC. Binding constraints recorded in the
  ROADMAP: benchmark tooling stays in an isolated `publish = false` package (never a
  core/companion dependency), no Python in ordinary Rust CI, no network or external
  datasets, no hard speed-fail gate initially, and reports use tradeoff language —
  never replacement/marketing claims.
- No new data flows, external integrations, or auth in this release; it changes only
  planning/design documents. `#![forbid(unsafe_code)]`, the core→companion
  dependency boundary, and the release-doc guards remain valid and unchanged.

## [0.20.4] - 2026-06-23

**Examples program — matrix-iteration band (RFC-045). Additive examples and
documentation under lock-step family versioning; no API, behavior, or breaking
change to any crate.**

### Added

- **Matrix-iteration examples** (RFC-045), default Phase-1 numeric API, small
  hard-coded inputs, deterministic self-checking output:
  - `examples/33_markov_chain_weather.rs` — a two-state weather process whose
    distribution evolves by vector × matrix `Tensor::matmul` and converges to the
    stationary distribution `[5/6, 1/6]`;
  - `examples/34_tiny_pagerank.rs` — PageRank by power iteration on a tiny directed
    graph, using matrix × vector `Tensor::matmul` with damping/teleport in plain
    Rust.
- **Examples documentation**: new `docs/src/examples/matrix-iteration.md`, the two
  examples added to the "Applied problems" table in `docs/src/examples/index.md`,
  and a `SUMMARY.md` entry.
- **CI smoke runs** for both new examples in `test.yaml`.

### Notes

- Examples continue in the additive `30+` band; the existing `00_`–`28_` suite,
  the `dynamic_*` set, and the named examples are unchanged. The optional
  `41_adjacency_walks_extended` remains a conditional candidate (not added).
- No new data flows, external integrations, or auth: examples use only in-memory
  hard-coded data. `#![forbid(unsafe_code)]`, the core→companion dependency
  boundary, and the release-doc guards remain valid and unchanged.

## [0.20.3] - 2026-06-23

**Examples program — structure + beginner applied-math band (RFC-043 + RFC-044).
Additive examples and documentation under lock-step family versioning; no API,
behavior, or breaking change to any crate.**

### Added

- **Beginner applied-math examples** (RFC-044), using only the default Phase-1
  numeric API, small hard-coded inputs, and deterministic self-checking output:
  - `examples/30_magic_square_checker.rs` — row/column/diagonal sums via
    `Tensor::get`;
  - `examples/31_fibonacci_matrix_power.rs` — Fibonacci via the `Q^n` identity and
    repeated `Tensor::matmul`;
  - `examples/32_graph_path_counting.rs` — walk counting via adjacency-matrix
    powers.
- **Examples documentation** (RFC-043): new `docs/src/examples/beginner-math.md`,
  an "Applied problems (famous small math)" section in
  `docs/src/examples/index.md`, and a `SUMMARY.md` entry.
- **CI smoke runs** for the three new examples in `test.yaml`.

### Changed

- Finalized the optional adjacency-walks wording across RFC-045, the examples
  handoff, and the ROADMAP: conditional candidate `41_adjacency_walks_extended.rs`,
  **not reserved**, to be added only if the Phase 0 inventory shows it teaches a
  distinct concept beyond `32_graph_path_counting.rs` (architect follow-up ruling).

### Notes

- New famous-problem examples use the additive `30+` band; the existing `00_`–`28_`
  suite, the `dynamic_*` set, and the named examples are unchanged. Vector distance
  and cosine similarity are cross-referenced to the existing `pairwise_distance` /
  `25_normalize_vector` / `26_cosine_similarity`, not duplicated (RFC-043–048
  review Q2).
- No new data flows, external integrations, or auth: examples use only in-memory
  hard-coded data. `#![forbid(unsafe_code)]`, the core→companion dependency
  boundary, and the release-doc guards remain valid and unchanged.

## [0.20.2] - 2026-06-23

**Examples-program planning. Documentation/planning patch under lock-step family
versioning — no source, API, or behavior changes to any crate.**

### Added

- **Examples program RFC set (RFC-043–048)** added to `rfcs/proposed/`:
  - RFC-043 example program structure, quality gate, and documentation policy;
  - RFC-044 beginner core math examples; RFC-045 matrix-iteration / graph /
    probability examples; RFC-046 numerical-methods examples; RFC-047 small
    ML-like examples; RFC-048 companion-crate examples.
- **Compact examples implementation handoff** at
  `rfcs/handoffs/043-048-examples-implementation-handoff.md`, opening with a
  Phase 0 inventory of the existing example suite.

### Changed

- **Reconciled the examples program to architect rulings** (RFC-043–048 review)
  and to shipped reality before any implementation:
  - new famous-problem examples use an **additive 30+ band**
    (`30_magic_square_checker` … `40_trapezoidal_integration`); the existing
    `00_`–`28_` core suite, `dynamic_*` set, and named examples are **not**
    renumbered;
  - cosine similarity, vector/pairwise distance, and the companion examples are
    **cross-referenced / audited in place**, not duplicated (the existing
    `26_cosine_similarity`, `pairwise_distance`, `from_arrayd`/`to_arrayd`,
    `standardize_columns`/`train_test_split`, and `csv_to_tensor` already cover
    them);
  - `matten-data` `csv_to_tensor` is recorded as **shipped in v0.20.1**, not
    future-only;
  - docs use the existing `docs/src/examples/index.md`, not a parallel
    `examples.md`;
  - the `test.yaml` smoke list is to be extended deliberately as runnable
    examples land.
- **ROADMAP** bumped to Document Version `1.4.0`: applied the 30+ band, dedup
  cross-references, shipped-`matten-data` wording, and the CI smoke-list note;
  **fixed a version regression** by replacing the erroneous `v0.19.4` planning
  row with accurate `v0.20.0` / `v0.20.1` / `v0.20.2` release-theme rows.
- **`rfcs/README.md`** index lists RFC-043–048 under Proposed and points to the
  examples handoff.

### Notes

- No new data flows, external integrations, or auth logic; this release changes
  only planning/design documents. Existing security controls
  (`#![forbid(unsafe_code)]`, the core→companion dependency boundary, release-doc
  guards) remain valid and unchanged.

## [0.20.1] - 2026-06-23

**`matten-data` becomes functional: the experimental table-to-Tensor API ships
(RFC-034 + RFC-035). Additive minor work under lock-step family versioning; no
breaking change to existing crates.**

### Added

- **`matten-data` public API** (RFC-034, RFC-035), still **Experimental**:
  - `Table` with CSV constructors `from_csv_str` / `from_csv_path` (behind the
    default-on `csv` feature), inspection (`row_count`, `column_count`,
    `column_names`, `schema_summary`), `select_columns` (by name, order-preserving),
    and explicit `fill_missing`.
  - Strict, explicit numeric conversion: `try_numeric` → `NumericTable` → `to_tensor`
    (shape `[rows, selected_columns]`, row-major `f64`). Integers/floats convert;
    booleans, non-numeric text, and unfilled missing cells are rejected — never
    silently coerced or zero-filled.
  - `CellValue` (crate-local, distinct from core `Element` — architect Q4),
    `SchemaSummary` / `ColumnSummary` / `ColumnKind`, and a crate-local
    `MattenDataError` with row/column context (one-based CSV line numbers).
  - CSV policy: header required; empty/duplicate headers and ragged rows rejected
    with precise errors; only empty cells are missing by default; surrounding
    whitespace trimmed.
  - `csv` feature (default-on) gating the `csv`-crate dependency and constructors.
  - Runnable example `examples/csv_to_tensor.rs`; 33 unit tests + doctests.
- RFC-034 and RFC-035 moved to `rfcs/done/`.

### Notes

- No source-logic or public-API change to `matten`, `matten-ndarray`, or
  `matten-mlprep`; the `matten` dependency pins (`0.20`) already cover `0.20.1`.
- `matten-data` produces a numeric `Tensor`; it does not use or expose core
  `dynamic`, and remains scope-locked (RFC-033, RFC-042) — no dataframe/query/lazy
  APIs. The beta decision remains deferred to v0.21+.
- Remaining RFC-036 work (broader example suite, the RFC-042 example scope-guard in
  the release-docs script) is a follow-up, not included here.

## [0.20.0] - 2026-06-23

**v0.20+ materialization start. Introduces the experimental `matten-data` companion
crate as a scope-locked scaffold (RFC-033). Minor bump under lock-step family
versioning: a new workspace member is added; no breaking change to existing crates.**

### Added

- **`matten-data`** companion crate (RFC-033) — an **experimental scaffold** for
  table-to-Tensor preparation of small PoC datasets. It currently has **no public
  API**; table ingestion and conversion land in later releases (RFC-034, RFC-035).
  The crate is `#![forbid(unsafe_code)]`, depends only on core `matten`, reserves
  private module boundaries (`table`, `csv`, `schema`, `numeric`, `error`) for the
  follow-up RFCs, and documents its scope lock (not a dataframe library; use
  Polars/DataFusion/Pandas for dataframe or large-data workloads).
- Dedicated `matten-data` CI job (`cargo check` / `clippy -D warnings` / doctests).
- `matten-data` added to the `pub use matten` companion guard list in
  `scripts/check-release-docs.sh` (RFC-032 / RFC-033 architect Q1 tooling note).

### Changed

- **Family version → 0.20.0** (lock-step, RFC-030). The `matten` dependency pins in
  the workspace and in every companion manifest are bumped `0.19` → `0.20` so the
  family resolves at the new version. Root README install snippets and crate-table
  versions updated to `0.20`.
- **RFC-033 decision recorded** and moved to `rfcs/done/`: `matten-data` proceeds as
  a scope-locked **experimental** companion — **not** beta. The beta / keep-experimental
  / freeze decision is deferred to v0.21+ pending real implementation evidence. RFC
  index updated.

### Notes

- No source-logic or public-API change to `matten`, `matten-ndarray`, or
  `matten-mlprep`. `matten-data` adds no public surface in this release.
- Scope guardrails for `matten-data` (RFC-033, RFC-042) are in force; table behavior
  (RFC-034/RFC-035) and core comfort APIs (RFC-038) remain gated and unstarted.

## [0.19.3] - 2026-06-22

**Planning and documentation patch. Adds the v0.20+ proposed RFC set (RFC-033–042)
and their implementation handoffs, reconciles the ROADMAP to lock-step versioning
and RFC-032, and applies the architect's review rulings. No source-logic or public
API changes.**

### Added

- **RFC-033–042** added as a proposed v0.20+ design set in `rfcs/proposed/`:
  `matten-data` decision/scope-lock, table model, CSV ingestion, examples/release
  gate, deferred streaming; core numeric comfort APIs; and the shape-composition,
  statistics, linear-algebra, and Pandas-scope-guard boundary RFCs. All remain
  **proposed** (design only) — no implementation is authorized by their presence.
- **Implementation handoffs** for RFC-033–042 in `rfcs/handoffs/` (with a directory
  `README.md`), translating each RFC into PR boundaries and acceptance criteria.

### Changed

- **ROADMAP reconciled** (now v1.2.0) to shipped reality and architect rulings:
  §13 corrected so the companion `pub use matten;` convenience re-export is
  **deferred** per RFC-032 (the release-doc guard forbids it); the planning baseline
  corrected to lock-step family versions (no per-crate `0.1.x`); v0.19.2 and v0.19.3
  release-theme rows added.
- **Architect rulings applied to the RFC set**: RFC-034 states `CellValue` is
  intentionally crate-local (not a `matten::Element` alias); RFC-035 locks the
  inference + strict `try_numeric` model; RFC-038 pins the NaN house policy, the
  `MattenError::InvalidArgument` variant for invalid numeric arguments, and new-module
  placement; RFC-039 uses non-square stack examples; RFC-040 documents the NaN house
  policy; RFC-042 replaces the broad term-scan with a precise filename + public-API
  guard; the 033/038/042 handoffs updated to match.
- **RFC index (`rfcs/README.md`) corrected**: RFC-024 and RFC-025 moved to the
  Implemented list (they were shipped/relabeled in v0.19.1 but still listed as
  proposed); RFC-031 and RFC-032 added to the Implemented list; stale per-crate
  `0.1.x` annotations removed; RFC-033–042 listed under Proposed.

## [0.19.2] - 2026-06-21

**Documentation and tooling patch (RFC-032). Records the companion dependency and
import convention, adds release-doc guardrails, and corrects stale companion README
wording. No source-logic or public API changes.**

### Added

- **RFC-032** (companion dependency and import convention) implemented and moved to
  `rfcs/done/`. The canonical user-facing style is explicit dependencies — declare
  `matten` *and* each companion, and import `Tensor` (and other core types) from
  `matten`. Broad core-type re-exports from companions are forbidden; the
  single-dependency `pub use matten;` convenience is deliberately deferred.
- A short **Dependency style** note added to both companion READMEs
  (`matten-ndarray`, `matten-mlprep`).
- `scripts/check-release-docs.sh` extended with two RFC-032 guardrails: companions
  must not re-export core `matten` (`pub use matten…`), and `Tensor` must not be
  imported from a companion in examples/README/mdBook.

### Fixed

- **Stale companion README wording (doc accuracy vs. shipped RFC-031).** Both
  companion READMEs described dynamic-tensor rejection as conditional on the
  `dynamic` feature; rejection has been unconditional since v0.19.1 (RFC-031). The
  wording now states rejection happens regardless of the feature.
- `matten-ndarray` README design note referred to the crate as "an experimental
  bridge"; updated to "a copy-based bridge" to match its production-ready-candidate
  status (RFC-029).

## [0.19.1] - 2026-06-21

**Companion maturity hardening patch (RFC-031). Feature-robust dynamic rejection,
maturity-label alignment, RFC lifecycle cleanup, and release-doc script extension.
No breaking changes.**

### Added

- `Tensor::is_dynamic()` is now **unconditionally available** in all builds
  (RFC-031). Previously it was defined only inside the `#[cfg(feature = "dynamic")]`
  module; it now lives in the main `Tensor` impl and returns `false` when `dynamic`
  is off, the true storage state when it is on.
- Regression fixture `tests/fixtures/dynamic_rejection_unification` — a standalone
  crate (excluded from the workspace) that reproduces the Cargo feature-unification
  panic scenario and asserts `Err(DynamicTensor)` from both companions under the
  fixed behaviour (RFC-031 §6.2).
- `scripts/check-release-docs.sh` extended with companion checks: stale
  Experimental status labels, stale independent-SemVer wording in CHANGELOG, and
  a guard ensuring companion rejection guards are not `#[cfg]`-gated (RFC-031).

### Fixed

- **P1 — dynamic rejection panic under Cargo feature unification (RFC-031).**
  When a downstream crate enabled `matten/dynamic` while leaving a companion
  `dynamic` mirror feature off, Cargo compiled one `matten` with `dynamic` active
  but the companion's `#[cfg(feature = "dynamic")]` guard was compiled out. A
  dynamic `Tensor` reaching `to_arrayd` or any `matten-mlprep` entry point
  would panic inside `Tensor::to_vec()` / `Tensor::as_slice()` instead of
  returning `Err`. Both companion guards now call `Tensor::is_dynamic()`
  unconditionally (no `#[cfg]`).
- **Clippy `manual_contains`** at `matten-ndarray/src/convert.rs:59` (surfaced
  on clippy ≥ 1.96): `shape.iter().any(|&d| d == 0)` → `shape.contains(&0)`.
- **CHANGELOG preamble** incorrectly stated independent per-crate SemVer
  (RFC-022 §7, superseded by RFC-030). Updated to state lock-step family
  versioning.

### Changed

- `matten-ndarray` status updated from **Experimental** to **production-ready
  candidate** in `src/lib.rs` and `Cargo.toml` description (RFC-029).
- `matten-mlprep` status updated from **Experimental (0.1.x)** to **beta** in
  `src/lib.rs` (RFC-029).
- Companion `dynamic` feature re-documented as a **compatibility forwarding
  feature** in both `lib.rs` crate-level docs. It is no longer required for
  dynamic-rejection correctness; the rejection guard is unconditional. Reconsider
  removal no earlier than v0.20.0.
- RFC-024 (`matten-mlprep` scope) moved from `rfcs/proposed/` to `rfcs/done/`
  with status "Implemented by RFC-028; maturity evaluated by RFC-029".
- RFC-025 (bridge-crate policy) moved from `rfcs/proposed/` to `rfcs/done/`
  with status "Implemented for `matten-ndarray`; `matten-nalgebra` and
  `matten-candle` deferred to future per-crate RFCs".
- RFC-031 added to `rfcs/done/`.

## [0.19.0] - 2026-06-21

**Companion maturity hardening (RFC-029), workspace versioning model (RFC-030),
and housekeeping. Docs, metadata, tests, and repo structure only; no code or
public API changes.**

### Versioning model — lock-step family versioning (RFC-030)

The workspace now uses **lock-step family versioning** (RFC-030, superseding
RFC-022 §7's independent per-crate SemVer): every crate shares one version via
`[workspace.package].version`, and **maturity is the Status label, not the
version number**. This is a one-time alignment of the family to `0.19.0`:

```text
matten          0.16.0 -> 0.19.0
matten-ndarray  0.1.1  -> 0.19.0
matten-mlprep   0.1.1  -> 0.19.0
```

No crate's public API or behavior changes in this alignment — only the version
number moves. The jump is **not** 18 minor releases of churn; it is the family
adopting a shared number. A user can now rely on matching versions
(`matten = "0.19"`, `matten-ndarray = "0.19"`, `matten-mlprep = "0.19"`) meaning a
compatible set. Per-crate `keywords`/`categories` stay per crate; only truly
shared metadata is inherited.

### Documentation

- README reworked into a single ecosystem landing page covering all three crates,
  with the family-versioning statement replacing the (now-incorrect)
  independent-SemVer claim. Per-crate READMEs are retained (each is what
  crates.io renders for that crate) and now link back to the workspace.
- RFC-022 §7 annotated as superseded by RFC-030; ROADMAP §10 updated.

### Maturity hardening (RFC-029)

- **`matten-ndarray` → production-ready candidate.** Added the compatibility
  policy (the last gate item) and strengthened roundtrip tests (rank-4, 3-D
  permuted-axes, NaN/Inf passthrough, fractional fidelity, standard-layout output).
- **`matten-mlprep` → beta.** Added documented limitations, a public-API
  snapshot, and a compatibility policy; added tests for NaN-column propagation
  and single-row degenerate inputs. This is an early beta (closed scope, limited
  field usage); the label is reversible while pre-1.0.
- Core `matten` has no code change (it aligns to the `0.19.0` family version per
  RFC-030). `matten-data`, `matten-nalgebra`, `matten-candle`, and streaming are
  **not** promoted (RFC-023, RFC-025 §10, RFC-026).
- **RFC-029** (maturity evaluation + decisions) → `done/`.
- **Workspace housekeeping (simplification).** Consolidated to a single root
  `CHANGELOG.md` (this file) and root-only `LICENSE`/`NOTICE`; removed the
  per-crate copies. Each crate is still licensed via its inherited SPDX
  `license = "Apache-2.0"` field, and per-crate version changes are recorded
  inside each milestone entry. Per-crate changelogs and license files will be
  reintroduced when crates publish to crates.io on independent cadences
  (RFC-022 §12).

### Security / threat model

This milestone changes only documentation, tests, and version numbers — no new
data flows, integrations, or auth. RFC-001 threat model unchanged; the
dependency-boundary gate still confirms core `matten` depends on nothing here.

## [0.18.0] - 2026-06-21

**Second companion crate: transparent preprocessing (RFC-024, RFC-028).**

- **`matten-mlprep` 0.1.0 (experimental)** added: `standardize_columns`
  (population-std z-score), `minmax_scale_columns` (to `[0, 1]`),
  `add_bias_column` (prepend a `1.0` intercept), and `train_test_split`
  (ordered, deterministic, no shuffle). Rank-2 only with the
  `rows = samples`, `columns = features` convention enforced. Constant
  (zero-variance / zero-range) columns are rejected explicitly via
  `ZeroVariance` rather than silently producing a zero column. Dynamic tensors
  are rejected, not panicked.
- Depends only on core `matten` (no default features); adds **no** third-party
  dependency (no `ndarray`, `candle`, or `rand`). Core `matten` (0.16.0) and
  `matten-ndarray` (0.1.0) are unchanged.
- **RFC-028** (`matten-mlprep` design) implemented → `done/`. RFC-024 (scope)
  remains `proposed/` pending its v0.19 beta decision. Per-crate implementation
  RFCs continue from RFC-029.

### Security / threat model

`matten-mlprep` is pure in-process numeric computation: no I/O, network, auth,
randomness, or new external data flow into core. The dependency-boundary gate
confirms core `matten` gained no dependency. RFC-001 threat model unchanged;
existing controls remain valid.

## [0.17.0] - 2026-06-21

**First companion crate; workspace introduced (RFC-025, RFC-027).**

- **Repository restructured into a Cargo workspace.** Core `matten` moved to
  `crates/matten/`; shared `rfcs/`, `docs/`, `ROADMAP.md`, and CI stay at the
  workspace root. The published `matten` crate's content is unchanged by the
  move; its version remains **0.16.0**.
- **`matten-ndarray` 0.1.0 (experimental)** added: `to_arrayd` / `from_arrayd`
  between `matten::Tensor` and `ndarray::ArrayD<f64>`, with logical-order
  conversion for non-standard-layout inputs, zero-axis rejection, and dynamic
  rejection (no panic). Depends on `matten` (no default features) and
  `ndarray` 0.16; adds **no** dependency to core `matten`.
- **RFC-027** (`matten-ndarray` design) implemented → `done/`. Per-crate
  implementation RFCs continue from RFC-028.
- Tooling made workspace-aware: the dependency-boundary and release-docs scripts
  and the CI matrix now scope core checks to `-p matten` and add a bridge job.

### Security / threat model

`matten-ndarray` is a pure in-process data-structure conversion: no I/O, no
network, no auth, no new external data flow into core. The dependency-boundary
gate proves core `matten` gained no new dependency. RFC-001 threat model
unchanged; existing controls remain valid.

## [0.16.0] - 2026-06-21

**Companion-crate boundary confirmation — RFC-022 resolved. No core code or public
API changes; this is a policy + documentation + CI release.**

This release draws the boundary between core `matten` and future `matten-*`
companion crates before any companion implementation begins. It lands the v0.16+
reconciliation (after architect review) with the four agreed corrections applied.

### Added

- **`ROADMAP.md`** (repo root) — now the canonical project roadmap for v0.16+,
  with an explicit document-authority order (accepted RFC > external design >
  roadmap > requirements > drafts). Replaces the older scheduling that placed
  `matten-data` at v0.17 and bundled all bridge crates at v0.19.
- **`scripts/check-core-dependency-boundary.sh`** — a mechanical CI gate proving
  core `matten` has no forbidden dependency direction
  (`ndarray`/`nalgebra`/`candle-core`/`polars`/`arrow`/`datafusion` or any
  `matten-*` companion). Wired into the `check` CI job and the release checklist.
- **`docs/design/external-design.md`** (v0.3.0) — external design reconciled with
  the companion-crate direction: in-core feature-gated bridge examples (§13.8/§13.12)
  are marked superseded, and a new §18 "Companion-Crate External Contract" codifies
  the dependency rule, companion order, independent SemVer, and per-crate error policy.

### RFC lifecycle

- **RFC-022 (Companion Crate Boundary Policy) → `done/`, Implemented (0.16.0).**
  Its acceptance criteria (boundary CI, canonical ROADMAP, superseded bridge
  examples, documented SemVer/error/maturity policies, clean core dependency graph)
  all ship here. Its open questions (workspace timing, changelog strategy) are
  recorded as deferred to v0.17.0; per-crate implementation RFCs start at RFC-027.
- **RFCs 023–026 reconciled (remain Proposed)** with corrected targets:
  `matten-ndarray` is the first companion (v0.17 experimental); `matten-data` is
  delayed to a v0.20+ beta decision; `matten-mlprep` is v0.18 experimental;
  `nalgebra`/`candle`/streaming are explicitly deferred behind later RFCs.

### Corrections applied during landing (architect-reviewed)

- **Boundary script must use `--all-features`.** The originally proposed
  `cargo tree -p matten` gave a *false pass* for an optional dependency behind a
  non-default feature (the most likely way a forbidden dep would enter core). The
  shipped script uses
  `cargo tree -p matten --all-features --edges normal,build --no-dedupe`; RFC-022
  §10 and ROADMAP §13 are updated to match. Verified: the check now fails on an
  optional `ndarray` dependency that the old form missed.
- **RFC-025 bridge correctness criteria added.** `from_arrayd` MUST convert by
  logical element order (not raw buffer order) so non-standard-layout `ArrayD`
  inputs are not silently transposed, and MUST reject zero-sized axes with a clear
  companion error. Added to §5.1 and the §9 acceptance criteria (and ROADMAP §5).
- **RFC-015–021 kept in `done/`/Implemented.** The reconciliation bundle's copies
  of these already-shipped RFCs carried regressed `Proposed` status *and* stale
  pre-0.15.2 content (e.g. the old `allow_bool_as_zero_one` / `parse_ascii_float_text`
  draft names corrected in 0.15.2). They were discarded; the repo's correct `done/`
  versions are retained unchanged.
- **Document metadata.** External design versioned `0.3.0` (continuing the `0.2.0`
  line; no `1.0` baseline existed) with the actual revision date; ROADMAP issued at
  `1.0.0`.

### Documentation

- `rfcs/README.md`: RFC-022 moved to Done (0.16.0); 023–026 targets updated.
- `docs/src/contributing/release-checklist.md`: boundary check added to source
  verification; allowed-root-exports list corrected to include `MattenLimits` and
  `NumericPolicy`; added the reduced-feature clippy passes.

### Security / threat model

No new data flows, external integrations, or auth logic. The dependency-boundary
script is a read-only `cargo tree` invocation that runs in CI only and is not part
of the published crate. The RFC-001 threat model is unchanged; existing controls
remain valid.

## [0.15.3] - 2026-06-21

**Polish patch — five post-review improvements (no behaviour changes).**

### Fixed — compiler warnings

- `src/limits.rs`: `MAX_DYNAMIC_ELEMENTS` is now gated on
  `#[cfg(all(feature = "dynamic", feature = "json"))]` to match its sole
  consumer (`src/dynamic/parse/json.rs`), which is already gated on both
  features. Previously this constant appeared dead — and triggered a
  `dead_code` warning — when `dynamic` was enabled without `json`.

- `src/tests/parse.rs`: the `use crate::{MattenError, Tensor}` import is now
  gated on `#[cfg(any(feature = "json", feature = "csv"))]` to match the
  tests that consume it. Previously it triggered an `unused_imports` warning
  under `--no-default-features --features dynamic`.

Both combinations now pass `cargo clippy -- -D warnings` cleanly.

### Fixed — CI coverage

- `.github/workflows/ci.yml`: the `check` job now runs three `clippy` passes
  instead of one: `--all-features`, `--no-default-features`, and
  `--no-default-features --features dynamic`. This ensures that warnings in
  non-default feature combinations are caught by CI with `-D warnings`, not
  only discovered during manual sweeps.

### Fixed — live doctests

- `src/dynamic/element.rs`: all five `Element` method doctests were annotated
  `ignore` even though they are valid under `--features dynamic` (the file
  itself is `#[cfg(feature = "dynamic")]`). Replaced all five `\`\`\`ignore`
  fences with plain `\`\`\`rust`. The doctest count under `--all-features`
  rises from 52 to 57; all pass.

### Fixed — stale scaffolding in error.rs

- Removed the `#[allow(dead_code)]` attributes and their M0-scaffold
  "wired up as features land" comments from `MattenError` and `DataFormat`.
  All variants have been constructed since v0.5.0; the allows were stale and
  misleading.

### Fixed — documentation drift

- `rfcs/README.md`: the "Shipped in" column for RFCs 001–014 and 020 was
  showing "—". Backfilled with the versions recorded in the RFC files'
  own `> RFC status:` lines (001 → 0.9.0, 002/003/005 → 0.1.0, 004 → 0.2.0,
  006 → 0.3.0, 007/008 → 0.4.0, 009 → 0.5.0, 013/014 → 0.6.0, 010 → 0.7.0,
  011/012 → 0.8.0, 020 → 0.13.3).

- `docs/src/reference/compatibility.md`: the opening sentence described
  "four public names" while the actual root exports include `MattenLimits`,
  `SliceBuilder`, and (under `dynamic`) `NumericPolicy` in addition to the
  four previously listed. Updated the listing to enumerate all six public
  exports accurately.

## [0.15.2] - 2026-06-20

**Spec/CI reconciliation patch (all v0.15.1 review findings addressed).**

### Fixed — code

- **PR-4 / minor note.** `Tensor::zeros` no longer redundantly calls
  `MattenLimits::default().check_shape` before `try_zeros` — `try_zeros`
  already calls it. Now matches the simpler `ones`/`full` pattern.

- **PR-4.** `arange` now reads its element budget through
  `MattenLimits::default().max_elements` directly instead of the
  `ARANGE_MAX_ELEMENTS` alias (same value, but now a single source of truth).

- **P2-5.** `flatten_rectangular` in `src/convert.rs` uses `checked_mul`
  for the `row_count × col_count` capacity calculation, consistent with the
  resource-safety theme.

### Fixed — CI and examples

- **P1-1.** `.github/workflows/ci.yml` smoke runs now include all four new
  examples: `13_resource_limits`, `27_axis_reductions`, `28_column_statistics`,
  `dynamic_06_numeric_policy --features dynamic`,
  `dynamic_07_on_ramp_summary --features dynamic`.

- **P2-4.** `dynamic_07_on_ramp_summary.rs` run header corrected from
  `--features dynamic,csv` to `--features dynamic` (the example uses
  `from_elements`, not `from_csv_dynamic`). Same fix in `examples/index.md`
  and `tutorial/start-here.md`.

- **P2-1.** `examples/13_resource_limits.rs` added — demonstrates
  `MattenLimits`, `try_zeros`/`try_ones`/`try_full`, custom limit enforcement,
  and the panicking fill constructors. Added to examples index and CI smoke.

### Fixed — RFC and spec reconciliation

- **P1-2.** `rfcs/done/018-shape-allocation-and-resource-safety-limits.md`
  updated with an explicit "Implementation notes" section listing implemented
  vs deferred scope. Unimplemented APIs (`try_new_with_limits`,
  `from_json_with_limits`, `from_csv_with_limits`) are now clearly marked
  DEFERRED. `max_parse_bytes` noted as a future extension point.

- **P1-3.** `src/limits.rs` `max_parse_bytes` field doc updated to state
  explicitly that the parsers do not yet enforce this limit at runtime.

- **PR-2.** `src/limits.rs` `max_elements` field doc documents the
  intentionally conservative `1<<20` default and explains the 2048×2048 case
  (4 M elements) exceeds it.

- **P1-4 / RFC-017.** All remaining stale draft method names removed from
  `rfcs/done/017-numeric-conversion-policy.md`:
  `allow_bool_as_zero_one` → `allow_bool`,
  `parse_ascii_float_text` → `allow_text_parse`,
  `reject_large_int_precision_loss` marked DEFERRED.

- **P1-5 / RFC-021.** All stale draft example names fixed in
  `rfcs/done/021-tutorial-path-and-example-quality-gate.md`:
  `28_column_mean.rs` → `28_column_statistics.rs`,
  `dynamic_06_numeric_mask.rs` → `dynamic_06_numeric_policy.rs`,
  `dynamic_07_on_ramp_to_matmul.rs` → `dynamic_07_on_ramp_summary.rs`,
  `29_row_scores.rs` and `14_readable_errors.rs` marked DEFERRED.

### Fixed — architecture documentation

- **PR-3.** `docs/src/contributing/architecture.md` completely updated:
  - Source layout extended to include `tensor/ops.rs`, `limits.rs`,
    `tests/math.rs`, and the `tests/dynamic/` submodule tree.
  - Public re-exports section updated to the actual v0.15.x root exports
    (`MattenLimits`, `SliceBuilder`, `Element`, `NumericPolicy`, hidden plumbing).
  - Milestone table extended from `0.6.0` through `0.15.x` and `0.16+`.

### Reviewer's residue scan result

```
RFC-017 stale names:              CLEAN
RFC-021 stale names:              CLEAN
architecture.md Phase 2 stale:   CLEAN
```

## [0.15.1] - 2026-06-20

**Review hardening patch (v0.15.0 architect review, all findings addressed).**

### Fixed — P0 (release blocker)

- **P0-1.** `Tensor::zeros`, `Tensor::ones`, and `Tensor::full` now delegate
  to `try_zeros` / `try_ones` / `try_full` (which route through
  `MattenLimits::check_shape`), so they enforce the default element budget.
  Previously they called `shape::validate_shape` directly and bypassed the
  `MattenLimits::max_elements` check entirely.

  Added three `#[should_panic(expected = "matten allocation error")]` tests:
  `zeros_panics_when_default_limit_exceeded`,
  `ones_panics_when_default_limit_exceeded`,
  `full_panics_when_default_limit_exceeded`.

### Fixed — P1 documentation

- **P1-1.** `README.md` status no longer says `0.13.x`; version snippets
  updated from `"0.13"` to `"0.15"`. "All 15 design RFCs" replaced with
  "RFC-000 through RFC-021 are in `rfcs/done/`".

- **P1-2.** `.github/workflows/ci.yml` smoke runs extended with all four new
  examples: `27_axis_reductions`, `28_column_statistics`,
  `dynamic_06_numeric_policy`, `dynamic_07_on_ramp_summary`.

- **P1-3.** `rfcs/README.md` regenerated cleanly: RFC-000 row corrected to
  `0.0.1`; RFC-019 row corrected to `0.15.0`.

- **P1-4.** `rfcs/done/019-axis-reductions-and-small-matrix-statistics.md`
  example name updated from `28_column_mean.rs` to `28_column_statistics.rs`.

- **P1-5.** `CHANGELOG.md` for v0.14.0 narrowed to accurately describe the
  MattenLimits scope: parser byte limits and `try_new` element budgets are
  documented as future work; fill-constructor and broadcast output limits are
  the implemented scope.

- **P1-6.** `rfcs/done/017-numeric-conversion-policy.md` updated to match the
  implemented `NumericPolicy` API (`allow_bool`, `allow_text_parse`, `none_as`,
  `none_as_nan`, `permissive`). The draft name `allow_bool_as_zero_one` removed.
  `reject_large_int_precision_loss` noted as deferred.

- **P1-7.** `schema_summary()` format string no longer contains embedded
  multi-space indentation between "numeric={}" and "(Float:".

### Fixed — P2 polish

- **P2-1.** `docs/src/contributing/architecture.md` milestone table extended
  through v0.15.0 and lists v0.16+ as companion-crate design phase.

- **P2-2.** Stale `0.1.x` wording removed from source comments in
  `src/shape.rs` and `src/tensor.rs`.

- **P2-3.** `try_numeric()` conversion error message continuation whitespace
  cleaned.

- **P2-4.** `Element::try_as_f64` docs updated: "losslessly cast" replaced
  with honest "cast with Rust `as f64` semantics; large values may lose
  precision."

### Structural (spec compliance)

- `src/tests/dynamic.rs` (713 ELOC) split into five submodules under
  `src/tests/dynamic/`: `element.rs`, `tensor.rs`, `lifecycle.rs`,
  `guards.rs`, `policy.rs`. All modules gated on `#[cfg(feature = "dynamic")]`.

- `src/tensor.rs` (354 ELOC) split: core struct, constructors, and accessors
  remain in `tensor.rs` (248 ELOC); shape operations, slicing, and boundary
  integration methods moved to `src/tensor/ops.rs` (91 ELOC). All files now
  below 300 ELOC.

## [0.15.0] - 2026-06-20

**Sedan math maturity and learning path (RFC-019 + RFC-021).**

### Added — RFC-021: Tutorial Path and Example Quality Gate

Four new examples:

| File | Demonstrates |
|---|---|
| `examples/27_axis_reductions.rs` | Axis reductions and NaN propagation |
| `examples/28_column_statistics.rs` | Per-column stats PoC pattern |
| `examples/dynamic_06_numeric_policy.rs` | `NumericPolicy` / `try_numeric_with` |
| `examples/dynamic_07_on_ramp_summary.rs` | Complete dynamic on-ramp lifecycle |

Two new docs pages:

- `docs/src/tutorial/start-here.md` — numbered learning path (Phase 1 + Phase 2)
- `docs/src/examples/index.md` — full examples index grouped by category

New mdBook sections: **Tutorial** (start-here) and **Examples** (index).

CI smoke runs extended with all four new examples.

`dynamic_04_numeric_coercion.rs` header updated to cross-reference
`dynamic_06_numeric_policy.rs` for the full policy API.

### Changed — RFC-019: Axis Reductions (already implemented; audit confirms compliance)

`sum_axis`, `mean_axis`, `min_axis`, `max_axis` were already implemented in
Phase 1 and verified to comply with the RFC-019 spec:

- reducing an axis removes it from the output shape;
- NaN propagates correctly via the `nan_axis_reduce` helper (has_nan vector
  + explicit NaN injection, never relying on `f64::min`/`max` ignoring NaN);
- dynamic tensors are rejected with a clear `matten unsupported error`.

RFC-019 deferred items (`var_axis`, `std_axis`, `keepdims`) remain deferred
as the RFC documents.

### Closed RFCs

- RFC-019: Axis Reductions and Small Matrix Statistics → `rfcs/done/`
- RFC-021: Tutorial Path and Example Quality Gate → `rfcs/done/`

All 22 completed RFCs (000–021) are now in `rfcs/done/`.
5 proposed RFCs (022–026) remain in `rfcs/proposed/`.

## [0.14.0] - 2026-06-20

**Dynamic on-ramp hardening (RFC-016 + RFC-017 + RFC-018).**

### Added — RFC-018: Shape, Allocation, and Resource Safety Limits

`MattenLimits` is now the single source of truth for all allocation and shape
bounds. Existing scattered constants are absorbed as its defaults.

New public type:

```rust
pub struct MattenLimits {
    pub max_dimensions: usize, // default: 8 (was MAX_NDIM)
    pub max_elements: usize,   // default: 1_048_576 (was ARANGE_MAX_ELEMENTS)
    pub max_parse_bytes: usize, // default: 128 MiB
}
```

New boundary-safe constructors (net-new public API):

```rust
Tensor::try_zeros(shape)            -> Result<Tensor, MattenError>
Tensor::try_ones(shape)             -> Result<Tensor, MattenError>
Tensor::try_full(shape, value)      -> Result<Tensor, MattenError>
Tensor::try_zeros_with_limits(shape, &MattenLimits) -> Result<Tensor, MattenError>
Tensor::try_ones_with_limits(shape, &MattenLimits)  -> Result<Tensor, MattenError>
Tensor::try_full_with_limits(shape, value, &MattenLimits) -> Result<Tensor, MattenError>
```

The panicking `zeros`, `ones`, `full` now delegate to `try_*_with_limits`,
so they route through the same limit policy.

Broadcast output size is now checked against `MattenLimits::default().max_elements`
with overflow detection before allocation.

9 new tests in `src/tests/tensor.rs::limits_tests`.

### Added — RFC-017: Numeric Conversion Policy

New public type `NumericPolicy` (under `#[cfg(feature = "dynamic")]`):

```rust
impl NumericPolicy {
    pub fn strict() -> Self;       // default: Float/Int only
    pub fn permissive() -> Self;   // Bool as 0/1, Text parsed, None as 0.0
    pub fn allow_bool(self) -> Self;
    pub fn allow_text_parse(self) -> Self;
    pub fn none_as(self, value: f64) -> Self;
    pub fn none_as_nan(self) -> Self;
}
```

New method on `Tensor`:

```rust
#[cfg(feature = "dynamic")]
pub fn try_numeric_with(&self, policy: NumericPolicy) -> Result<Tensor, MattenError>
```

Default `try_numeric()` behavior is unchanged (strict: Float/Int only).
13 new tests in `src/tests/dynamic.rs::numeric_policy_tests`.

### Added — RFC-016: Dynamic Ingestion and Explicit Numeric On-Ramp

Three new dynamic inspection methods:

```rust
#[cfg(feature = "dynamic")]
pub fn numeric_mask(&self) -> Tensor;           // 1.0/0.0 like none_mask
pub fn is_numeric_convertible(&self) -> bool;   // true if all Float/Int
pub fn schema_summary(&self) -> String;          // element-type count string
```

8 new tests in `src/tests/dynamic.rs::inspection_tests`.

### Internal changes

All scattered allocation constants centralised in `src/limits.rs`. `MattenLimits` is
the single source of truth for fill-constructor and broadcast limits. Parser byte limits
(`MAX_PARSE_BYTES`) and `try_new` element-budget checking are documented future work;
they are not yet enforced at runtime in v0.14.0. JSON/CSV element counts use their own
parser constants (`MAX_JSON_ELEMENTS`, `MAX_DYNAMIC_ELEMENTS`) imported from `limits.rs`.

### Closed RFCs

- RFC-016: Dynamic Ingestion and Explicit Numeric On-Ramp → `rfcs/done/`
- RFC-017: Numeric Conversion Policy → `rfcs/done/`
- RFC-018: Shape, Allocation, and Resource Safety Limits → `rfcs/done/`

All 20 completed RFCs (000–020) are now in `rfcs/done/`.
7 proposed RFCs (021–026) remain in `rfcs/proposed/`.

## [0.13.3] - 2026-06-20

**Stabilization and diagnostics release (RFC-015 + RFC-020).**

### Added

- `docs/src/contributing/release-checklist.md` — the formal pre-release gate
  defined by RFC-015. Covers source verification, feature matrix, examples, MSRV,
  public API audit, documentation truth pass, CHANGELOG discipline, and the v1.0.0
  explicit-confirmation requirement. Linked from the mdBook SUMMARY.

- `scripts/check-release-docs.sh` — automated release-documentation check that
  catches stale runtime version strings, version-stamped crate docs, root-export
  drift, and examples importing hidden plumbing. Passes clean on the current
  codebase.

- 2 diagnostic message format tests (`src/tests/dynamic.rs::diagnostic_message_tests`):
  - `as_slice_message_format` — asserts that the numeric-accessor guard message
    starts with `"matten unsupported error in as_slice:"`.
  - `sum_skip_none_message_format` — asserts that the non-numeric element panic
    starts with `"matten unsupported error in sum_skip_none:"`.

### Fixed

- **RFC-020.** `sum_skip_none` panic message normalized to the standard format:
  `"matten unsupported error in sum_skip_none: element <e> cannot be coerced to
  f64; use fill_none or filter non-numeric elements first"`. Previously it said
  `"sum_skip_none: non-numeric element <e>; ..."` without the `matten` prefix.

- **RFC-015.** `docs/src/reference/public-api-snapshot.md` rewritten as a full
  authoritative v0.13.x API reference, covering all public methods by category,
  the dynamic behaviour table, `MattenError` variants, `DataFormat`, `Element`,
  and conversion traits. Previous versions were incomplete or version-stamped.

### Closed RFCs

- RFC-015: Public API Stabilization and Compatibility Policy → `rfcs/done/`
- RFC-020: Human-Readable Diagnostics and Error Message Quality → `rfcs/done/`

All 17 completed RFCs (000–015, 019–020) are now in `rfcs/done/`.
10 proposed RFCs (016–018, 021–026) remain in `rfcs/proposed/`.

### Audit findings (RFC-015 PR-015-A)

All findings clean at v0.13.3:
- Root exports match the allowlist exactly: `Tensor`, `MattenError`, `DataFormat`,
  `SliceBuilder`, `Element` (dynamic-gated). Hidden plumbing `IntoSliceRange`,
  `SliceConvert`, `SliceSpecRepr` correctly marked `#[doc(hidden)]`.
- All modules are private (`mod`, not `pub mod`). No accidental public leaks.
- No examples import hidden plumbing.
- `MattenError` is `#[non_exhaustive]`.
- No stale version strings in runtime messages or crate-level docs.

## [0.13.2] - 2026-06-20

**Final cleanup release.** Addresses all remaining P1 and P2 findings from the
v0.13.1 review.

### Fixed — P1

- **P1-1.** `README.md` status updated from version-stamped text to
  `"active pre-1.0 development (0.13.x)"`, preventing future stale-version drift.

- **P1-2.** `docs/src/SUMMARY.md` entry changed from
  `"Public API snapshot (v0.10.0)"` to `"Public API snapshot"` — version removed
  to avoid needing updates on every patch release.

- **P1-3.** `examples/25_normalize_vector.rs` header updated: stale
  `"matten 0.5 has no built-in sqrt/dot; those arrive with RFC-010 reductions"`
  removed and replaced with a version-neutral description.

- **P1-4.** True sealed-trait pattern implemented for `IntoSliceRange` /
  `SliceConvert`. A private `mod sealed` with `pub trait Sealed` now acts as the
  sealing supertrait. `SliceConvert` extends `sealed::Sealed`, so downstream
  crates cannot implement it without access to the private module. The
  `#[doc(hidden)]` root-exports remain for compiler visibility.
  `docs/src/reference/compatibility.md` and `public-api-snapshot.md` updated to
  accurately describe the implementation.

- **P1-5.** `src/ser.rs` multi-space error message fixed: the embedded spaces from
  backslash-continuation indentation are now correct.

- **P1-6.** `src/error.rs` broken intra-doc link `[Result](crate::Result)` changed
  to `[Result](std::result::Result)`.

### Fixed — P2

- **P2-1.** Duplicate `## Missing-value utilities` heading in
  `docs/src/reference/dynamic.md` removed — the shorter first occurrence was
  eliminated, keeping the fuller second section.

- **P2-3.** Runtime error messages containing `"zero-sized dimensions are not
  supported in matten 0.1"` in `src/shape.rs` and `src/convert.rs` updated to
  `"zero-sized dimensions are not supported in the current matten shape model"`,
  avoiding version-stamped wording in runtime output.

## [0.13.1] - 2026-06-20

**Cleanup release.** Addresses all remaining findings from the v0.13.0 review.

### Fixed — P0

- **P0-1 (insurance).** `IntoSliceRange`, `SliceConvert`, and `SliceSpecRepr` are now
  root-exported with `#[doc(hidden)]` in `src/lib.rs`, making the sealed-trait chain
  visible to the compiler's public-API checks. `RUSTFLAGS="-D warnings" cargo check`
  confirmed clean (no `private_bounds` lint fires on Rust 1.85, but the export provides
  defence-in-depth for future toolchain versions).

### Fixed — P1 documentation

- **P1-1.** `README.md` status updated to `0.13.0` with an accurate, honest description
  of the dynamic feature scope: guard-model ingestion/cleanup, not "complete Phase 2".

- **P1-2.** `src/lib.rs` crate-level docs no longer contain stale `0.11.0` text or
  "Reductions, matmul, and examples arrive in later milestones". Replaced with a
  version-neutral scope description.

- **P1-3.** All user-facing Cargo version snippets updated to `"0.13"`:
  `README.md`, `docs/src/quick-start.md`, `docs/src/reference/boundary.md`,
  `docs/src/reference/dynamic.md`, `docs/src/contributing/architecture.md`,
  `src/lib.rs`, `rfcs/done/011-dynamic-element-model-and-coercion.md`.

### Fixed — P2 polish

- **P2-1.** Remaining embedded multi-space error messages fixed in
  `src/ops/broadcast.rs`, `src/ops/scalar_ops.rs`, `src/ser.rs`, `src/tensor.rs`.

- **P2-2.** `rfcs/README.md` RFC-000 "Shipped in" cell corrected to `0.0.1`
  (was accidentally set to lifecycle-policy prose during the previous regeneration).

- **P2-3.** `docs/src/reference/public-api-snapshot.md` body text updated from
  `v0.10.0` to `v0.13.0`.

### Added

- **PR-4.** Two additional dynamic regression tests in `src/tests/dynamic.rs`:
  - `into_vec_method_panics_on_dynamic` — `Tensor::into_vec()` must panic on dynamic.
  - `try_into_rows_returns_unsupported_on_dynamic` — `TryFrom<Tensor>` must return
    `MattenError::Unsupported`, not silently produce empty rows.

## [0.13.0] - 2026-06-20

**Post-review hardening.** Addresses all P0, P1, and P2 findings from the
v0.12.0 careful review.

### Fixed — P0 blockers

- **P0-1.** `dynamic + json` and `dynamic + csv` isolated builds now compile
  correctly. `src/dynamic/parse.rs` now gates each submodule on its own
  feature (`#[cfg(feature = "json")]` / `#[cfg(feature = "csv")]`). Added
  `dynamic,json` and `dynamic,csv` CI profiles to prevent regression.

- **P0-2.** Dynamic tensors no longer silently expose empty numeric data.
  Added `panic_if_dynamic(operation)` helper to `Tensor` and applied it to:
  `as_slice`, `to_vec`, `into_vec`, `get`, `get_flat`,
  `From<Tensor> for Vec<f64>`, `From<&Tensor> for Vec<f64>`,
  `TryFrom<Tensor> for Vec<Vec<f64>>`.
  All now panic with `matten unsupported error in <op>: use to_elements() or
  try_numeric() first`.

- **P0-3.** `Tensor::matmul` now delegates to `Tensor::dot`, which already
  has the dynamic guard. Previously `matmul` called `matmul_dispatch` directly,
  silently computing `0.0` for dynamic vectors.

- **P0-4.** `docs/src/reference/dynamic.md` no longer shows a `reshape`
  CoW example that implied public dynamic reshape is implemented. Replaced with
  an honest "Current limitations (guard model)" section.

### Fixed — P1 issues

- **P1-1.** `IntoSliceRange` and `SliceConvert` removed from the
  `pub use` root-export block in `public-api-snapshot.md` — they are module-
  level `pub` items but not root-exported. Only `SliceBuilder` is root-exported.

- **P1-2.** Stale `version = "0.1"` / `version = "0.8"` Cargo snippets in
  `docs/src/quick-start.md`, `docs/src/reference/dynamic.md`, and `README.md`
  updated to `"0.12"` (the last published release at time of writing).

- **P1-3.** `src/lib.rs` crate-level docs no longer mention `"0.11.0"`.
  Replaced with a version-neutral description of the current scope.

- **P1-4.** `rfcs/README.md` regenerated from the actual `rfcs/done/`
  directory. All 15 RFCs (000–014) now appear in the Done table; no stale
  Proposed rows remain.

- **P1-5.** `README.md` status section softened to accurately describe the
  guard model rather than claiming "complete Phase 2".

### Fixed — P2 polish

- **P2-1.** Error messages with backslash string-continuation whitespace
  cleaned. `reshape.rs` and `slice.rs` rewritten in full to eliminate the
  problem at the source. Individual messages in `math.rs`, `tensor.rs`, and
  `ops/unary_ops.rs` corrected.

- **P2-2.** `RangeInclusive<usize>` in `SliceConvert` now uses
  `saturating_add(1)` instead of `end() + 1`, preventing overflow panic on
  `usize::MAX..=usize::MAX` in debug builds.

- **P2-3.** Added `small_int_coercion_exact` and
  `large_int_coercion_may_lose_precision` tests in
  `src/tests/dynamic.rs::precision_tests` to document `Int(i64) → f64`
  behavior explicitly.

### Added

- 10 new tests in `src/tests/dynamic.rs`:
  - `accessor_guard_tests`: `as_slice`, `to_vec`, `into_vec`, `get`,
    `get_flat`, `From<&Tensor>` all panic on dynamic tensors.
  - `matmul_guard_tests`: `matmul` and `dot` panic on dynamic tensors.
  - `precision_tests`: small-int exact coercion, large-int precision loss.

## [0.12.0] - 2026-06-20

**Hardening release.** Addresses all P0, P1, P2, and P3 findings from the
v0.11.0 architect review.

### Fixed — Patch A: Release polish

- **A1.** `src/lib.rs` crate-level status docs updated from `0.5.0 / M5` to
  current `0.12.0` status.
- **A2.** `mean_axis` now validates `axis < self.ndim()` *before* indexing
  `self.shape()[axis]`, emitting a canonical `matten shape error in mean_axis:
  axis N is out of range for rank-M tensor` panic instead of a raw Rust index
  panic.
- **A3.** `examples/pairwise_distance.rs` cleaned — scratch comment
  `"wait — need &ref; fix below"` and the unused intermediate variable removed.
- **A4.** `README.md` links to `docs/` and `rfcs/` (excluded from the
  published crate package) replaced with `docs.rs` links.
- **A5.** Public API snapshot version label updated to match the package
  version.

### Fixed — Patch B: API and grammar consistency

- **B1.** `SliceBuilder` formally blessed as a public root export; updated in
  the API snapshot.
- **B2.** `slice_str("0::")` — empty trailing step segment is now **rejected**
  with `MattenError::Slice` and a clear message. Previously it silently parsed
  as `"0:"`.
- **B3.** The `slice_str_malformed_never_panics` test tightened to
  `slice_str_malformed_is_err` asserting `Err` for every malformed input.
- **B4.** `IntoSliceRange`/`SliceConvert` documented as intentionally
  public-but-hidden plumbing.

### Fixed — Patch C: Dynamic lifecycle hardening (P0-1, P0-2)

Every Phase 1-only `Tensor` method now has an explicit `is_dynamic()` guard:

- **C2.** `len()` returns `DynamicTensor::len` (the logical element count) for
  dynamic tensors — no longer returns `0`.
- **C3.** `Debug` prints `dynamic=[...]` with `Element` values for dynamic
  tensors — no longer shows empty numeric data.
- **C4.** `reshape`, `try_reshape`, `flatten`, `transpose`, `swap_axes` panic
  with `matten unsupported error in <op>: dynamic tensors do not support
  <op>; call try_numeric() first`.
- **C5.** `slice()` builder and `slice_str()` return `MattenError::Unsupported`
  for dynamic tensors with a clear message directing users to `get_element`.
- **C6.** All element-wise operators (`+`, `-`, `*`, `/`, unary `-`), scalar
  operators, and every reduction (`sum`, `mean`, `min`, `max`, `sum_axis`,
  `mean_axis`, `min_axis`, `max_axis`) and `dot`/`matmul` panic with
  `matten unsupported error` on dynamic tensors.
- **C7.** `Serialize` returns a serde error for dynamic tensors instead of
  silently serializing shape + empty data.
- **C8.** Dynamic examples `dynamic_00`, `dynamic_02`, `dynamic_05` rewritten
  to `assert!` correct behavior rather than only `println!` output.

Added **8 public lifecycle tests** in `src/tests/dynamic.rs::lifecycle_tests`
(P0-2 fix):
`dynamic_len_equals_shape_product`, `dynamic_len_2d`, `dynamic_debug_not_empty`,
`dynamic_reshape_is_unsupported`, `dynamic_flatten_is_unsupported`,
`dynamic_slice_builder_is_unsupported`, `dynamic_arithmetic_is_unsupported`,
`dynamic_sum_is_unsupported`.

### Fixed — Patch D: Boundary hardening

- **D1.** `try_matmul` (`pub(crate)`) removed — it was dead code that still
  panicked internally despite returning `Result`.
- **D2.** `ARANGE_MAX_ELEMENTS` lowered from `1<<28` (~268 M elements, ~2 GiB)
  to `1<<20` (~1 M elements, ~8 MiB) — appropriate for the family-car identity.
- **D3.** JSON shape parsing now uses `usize::try_from(n)` instead of
  `n as usize`, returning `MattenError::Parse` on overflow rather than silently
  truncating on 32-bit targets. Applied to both `src/parse/json.rs` and
  `src/dynamic/parse/json.rs`.

### Changed

- `MattenError` import removed from `math.rs` (only needed by the now-deleted
  `try_matmul`).
- `docs/src/reference/public-api-snapshot.md` updated to v0.12.0 and now
  includes a dynamic-behaviour table.
- `docs/src/reference/compatibility.md` documents the `ARANGE_MAX_ELEMENTS`
  budget change.

## [0.11.0] - 2026-06-20

**Post-audit release.** Systematic four-layer audit (RFC → implementation,
requirements → tests, tests → codebase, docs → codebase) with all findings
addressed.

### Audit findings and fixes

**GAP 1 (RFC-007 §10):** `Tensor::get_flat(index)` was specified but missing.  
→ Implemented `pub fn get_flat(&self, index: usize) -> Option<f64>` in `tensor.rs`.

**GAP 2 (RFC-011 §10):** RFC specified `is_none(&self) -> Tensor`; implementation used `none_mask()`.  
→ Added `is_none_mask()` as a documented alias; `none_mask()` remains the canonical name.

**GAP 3 (RFC-011 §14):** "default build does not expose Element" test was missing.  
→ Added compile-time isolation test in `src/tests/dynamic.rs`; the lean-core CI job is the definitive proof.

**GAP 5 (RFC-010 §14):** NumPy golden tests were a SHOULD requirement with no implementation.  
→ Added `tests/golden/numpy_broadcasting.json` and `tests/golden/numpy_matmul.json`; integration tests in `tests/smoke.rs` load and verify them.

**GAP 7 (RFC-009 §13):** Allocation/size-limit test for boundary parsers was missing.  
→ Added `from_json_oversized_array_is_err` and `from_json_slice_str_length_limit` tests in `src/tests/parse.rs`.

**Docs gaps fixed:**  
- `docs/src/reference/math.md`: added `min_axis`/`max_axis` section.  
- `docs/src/reference/dynamic.md`: added `none_mask`, `count_none`, `forward_fill_none`, `is_none_mask`, `sum_skip_none` section.  
- `docs/src/reference/public-api-snapshot.md`: added `get_flat` and `is_none_mask`.  
- `docs/src/reference/compatibility.md`: updated `get_flat` from "not implemented" to "implemented".

**Acceptable deferred items (documented):**
- GAP 4 (RFC-012): `materialize()`/`is_materialized()` public API — optional diagnostics, already in compatibility.md.
- GAP 6 (RFC-005/008): Fuzz target for `slice_str` — roadmap says "not required in every CI run".
- GAP 8 (RFC-013): Full NumPy golden suite beyond the two sets added above — SHOULD, not MUST.

## [0.10.0] - 2026-06-20

**Release Candidate — stabilisation milestone.**

### Added

- **Migration guide** (`docs/src/reference/migration.md`) — how and when to
  move data from `matten` to `ndarray`, `nalgebra`, or `candle`; code snippets
  for all three.
- **Compatibility and stability policy** (`docs/src/reference/compatibility.md`)
  — v0.x change policy, feature-flag stability, MSRV, deferred items table.
- **Public API snapshot** (`docs/src/reference/public-api-snapshot.md`) — full
  enumeration of every public item at v0.10.0, serving as the v1.0.0 baseline.

### Fixed

- Malformed whitespace in `axis_reduce` panic message (`"out of range
  for rank-{}"` had extra spaces; corrected to single space).

### Release Decision Checklist (roadmap §12)

1. **Family-car identity preserved?** Yes — no `Tensor<T>`, no BLAS, no GPU,
   no proc-macros. One primary type, two error zones.
2. **Beginner quick-start without lifetime annotations?** Yes — `00_quickstart`
   compiles with only `use matten::Tensor`.
3. **Panics actionable?** Yes — all start with `"matten <category> error in
   <operation>: …"` and include shape/axis context.
4. **I/O boundaries return `Result`?** Yes — `from_json`, `from_csv`,
   `load_json`, `load_csv`, `from_json_dynamic`, `from_csv_dynamic`,
   `try_new`, `try_reshape`, `try_arange`, `slice_str`, `build()`.
5. **Limitations stated honestly?** Yes — README, philosophy.md, migration.md,
   and every example that touches performance warns explicitly.
6. **Public APIs narrow enough?** Yes — 4 root exports, sealed slice traits,
   `SliceSpecRepr` is `#[doc(hidden)]`.
7. **Default compilation lightweight?** Yes — `cargo build --no-default-features`
   pulls in zero optional deps; default profile adds only `serde`/`serde_json`/`csv`.
8. **Examples realistic and runnable?** Yes — 35 examples, all `cargo run`-able,
   all smoke-tested in CI.
9. **Examples demonstrate accepted APIs?** Yes — no example creates new implicit
   scope; RFC-014 review complete.
10. **Feature flags understandable?** Yes — `default = ["serde","json","csv"]`;
    `dynamic` off by default; each feature is documented.
11. **Version appropriate?** `0.10.0` is the release candidate on the v0 line.
    v1.0.0 requires explicit maintainer confirmation.

## [0.9.0] - 2026-06-20

Milestones **M9, M10, M11** — Dynamic hardening, messy-data workflows, and
pattern examples. Closes all remaining open RFCs.

### Added

- **`min_axis(axis)` and `max_axis(axis)`** (`src/math.rs`) — axis reductions
  returning `NaN` when any element along the axis is `NaN` (explicit
  short-circuit, consistent with `min`/`max`).

- **Dynamic utility methods** (`src/dynamic/tensor_ext.rs`, RFC-011 §10):
  - `none_mask()` → Phase 1 tensor of `1.0`/`0.0` indicating `None` positions.
  - `count_none() → usize` — count of `None` elements.
  - `forward_fill_none(fallback)` — carry last non-None value forward in
    row-major order; leading `None` values take `fallback`.
  - `sum_skip_none()` — sum skipping `None`, panicking on non-numeric elements.

- **Dynamic examples** (RFC-014 §6.5, now complete):
  - `dynamic_01_mixed_elements.rs` — building and inspecting mixed tensors.
  - `dynamic_02_missing_values.rs` — detecting and counting `None` values.
  - `dynamic_03_fill_none.rs` — constant fill and forward-fill demonstration.
  - `dynamic_04_numeric_coercion.rs` — coercion policy demonstration.
  - `dynamic_05_dirty_csv_cleanup.rs` — end-to-end messy CSV workflow.

- **Recommended `0.1.x` pattern examples** (RFC-014 §6.4, now complete):
  - `standardize_columns.rs` — z-score normalisation per column.
  - `minmax_scaling.rs` — 0–1 scaling per column.
  - `rowwise_scoring.rs` — weighted row scoring.
  - `column_summary.rs` — per-column mean/min/max/std.
  - `moving_average.rs` — sliding-window mean.
  - `rolling_windows_basic.rs` — rolling sum and max.
  - `pairwise_distance.rs` — Euclidean distance matrix via matmul.
  - `gram_matrix.rs` — Gram matrix (X·Xᵀ).

- **Tests** — 5 new `min_axis`/`max_axis` tests in `src/tests/math.rs`;
  6 new dynamic utility tests in `src/tests/dynamic.rs` (`utility_tests`).

### Changed

- CI `.github/workflows/ci.yml`: added `dynamic_00_quickstart` and all
  pattern examples to the smoke job; added `dynamic+json+csv` feature profile.

### Closed

- **RFC-001 (Threat Model and Boundary Safety Policy)** moved to `rfcs/done/`.
  All controls in RFC-001 are now in place in the codebase:
  - `#![forbid(unsafe_code)]` throughout.
  - Checked arithmetic on all shape/product paths.
  - Parser size limits (`MAX_SLICE_STR_BYTES`, `MAX_JSON_ELEMENTS`, `MAX_DEPTH`).
  - Result-zone at all external boundaries (`try_new`, `from_json`, `from_csv`,
    `load_json`, `load_csv`, `slice_str`, `build()`).
  - Panic-zone clearly labelled and documented.
  - `MattenError::Allocation` for overflow before allocation.

  All 15 RFCs are now in `rfcs/done/`. No open RFCs remain.

## [0.8.0] - 2026-06-20

Milestone **M8 — Dynamic Feature, Phase 2** (RFC-011 + RFC-012).

### Added

- **`dynamic` Cargo feature** — enables Phase 2 heterogeneous tensor support.

- **`Element` enum** (`src/dynamic/element.rs`, RFC-011):
  - Variants: `Float(f64)`, `Int(i64)`, `Text(Arc<str>)`, `Bool(bool)`, `None`.
  - Text representation selected as `Arc<str>` after memory measurement:
    all candidates give `size_of::<Element>() == 24` bytes on 64-bit targets;
    `Arc<str>` chosen for cheap clone in CoW slices.
  - Coercion policy: `Float`/`Int` → `f64` allowed; `Bool`/`Text`/`None` → f64
    explicitly blocked (no silent coercion).
  - `From` conversions for `f64`, `i64`, `i32`, `bool`, `String`, `&str`, `Arc<str>`.
  - Exported as `matten::Element` under `#[cfg(feature = "dynamic")]`.

- **CoW storage** (`src/dynamic/storage.rs`, RFC-012):
  - `DynamicTensor` with `Arc<Vec<Element>>` shared backing storage.
  - `ViewKind::Contiguous` for construction/reshape; `ViewKind::Indexed` for
    non-contiguous slices (shared storage, no element copy).
  - `materialize()` produces fresh contiguous copy; `is_unique()` for CoW check.
  - No reference cycles; drops cleanly.

- **Dynamic `impl Tensor`** (`src/dynamic/tensor_ext.rs`):
  - `from_elements` / `try_from_elements` — construction from `Vec<Element>`.
  - `get_element(&[usize]) → Option<Element>` — safe element access.
  - `to_elements() → Vec<Element>` — extract in row-major order.
  - `is_dynamic() → bool`.
  - `fill_none(value) → Tensor` — replace all `None` values.
  - `try_numeric() → Result<Tensor, MattenError>` — convert to Phase 1 f64
    tensor; fails with `MattenError::Unsupported` on any non-numeric element.
  - `from_json_dynamic(input)` (requires `json` feature) — mixed JSON parser.
  - `from_csv_dynamic(input)` (requires `csv` feature) — mixed CSV parser.

- **Dynamic parsers** (`src/dynamic/parse/`):
  - JSON: null→`None`, bool→`Bool`, string→`Text`, integer→`Int`, float→`Float`.
  - CSV: empty field→`None`, `true`/`false`→`Bool`, integer→`Int`, float→`Float`,
    other→`Text`.

- **Tests** (`src/tests/dynamic.rs`) — 26 new tests across 5 submodules:
  element model, tensor construction, CoW storage, JSON parser, CSV parser.

- **Example** `examples/dynamic_00_quickstart.rs` — runs with
  `--features dynamic,json,csv`.

- **Doc page** `docs/src/reference/dynamic.md`.

### Notes

- Default `f64` Phase 1 API is completely unaffected; the `dynamic` field in
  `Tensor` is `#[cfg(feature = "dynamic")]` and zero-cost when the feature is
  off.
- RFC-011 and RFC-012 moved to `rfcs/done/`.
- Only RFC-001 (threat model) remains in `rfcs/proposed/`.

## [0.7.0] - 2026-06-20

Milestone **M7 — Reductions and Matrix Multiplication** (RFC-010).

### Added

- Whole-tensor reductions: `sum`, `mean`, `min`, `max` — all returning `f64`.
  `NaN` propagates through `sum`/`mean`; `min`/`max` return `NaN` if any
  element is `NaN` (explicit short-circuit, not `f64::min`/`max` which would
  silently ignore it).
- Axis reductions: `sum_axis(axis)`, `mean_axis(axis)` — both returning a new
  `Tensor` with the reduced axis removed.
- `dot(&rhs)` / `matmul(&rhs)` — four supported shapes:
  `[n]×[n]→[]`, `[m,n]×[n]→[m]`, `[n]×[n,p]→[p]`, `[m,n]×[n,p]→[m,p]`.
  `*` remains element-wise.
- New `src/math.rs` module with all of the above.
- `src/tests/math.rs` — 28 tests covering reductions, NaN policy, axis ops,
  all four matmul cases, dimension-mismatch panics, and `*`-vs-`matmul`
  regression.
- Examples 20–24 and 26 completed (were deferred stubs):
  `20_dot_product`, `21_matrix_vector_product`, `22_matrix_multiplication`,
  `23_sum_mean`, `24_min_max`, `26_cosine_similarity`.
- `docs/src/reference/math.md` — reference page for all new operations.

## [0.6.0] - 2026-06-20

Milestone **M6 — Example Suite and CI Hardening** (RFC-013 + RFC-014).

### Added

- **Example suite** — 21 runnable examples covering every implemented API
  surface (RFC-014 §6.3 required set):
  - `00_quickstart` through `12_boundary_error_handling` — creation, shape,
    reshape, operators, scalar ops, broadcasting, transpose, slicing (builder
    and `slice_str`), JSON round-trip, CSV loading, boundary error handling.
  - `20`–`24` — deferred placeholders pending RFC-010 (reductions / matmul).
  - `25_normalize_vector`, `26_cosine_similarity` — L2 normalisation and
    cosine similarity implemented using existing element-wise ops.
- **Fixture files** added: `examples/data/numeric_3x3.csv`,
  `examples/data/malformed_numeric.csv`.
- **CI gates** (`.github/workflows/ci.yml`) extended with:
  - `cargo check --examples` and `cargo test --examples` in every PR job.
  - Full feature-profile test matrix (lean, serde, json, csv, dynamic,
    all-features).
  - MSRV 1.85.0 full test run (was build-only).
  - Separate `smoke` job running the six RFC-014 required smoke examples.

### Notes

- RFC-013 and RFC-014 moved to `rfcs/done/`.
- Examples 20–24 are placeholder stubs clearly marked as deferred pending
  RFC-010 (reductions / matmul).

## [0.5.0] - 2026-06-20

Milestone **M5 — Boundary Integration** (RFC-009).

### Added

- `Tensor::from_json(input: &str)` — accepts the canonical
  `{"shape":[…],"data":[…]}` object form and the convenience nested-array
  form (rank 1 and 2); rejects non-numeric values, ragged arrays, and
  malformed JSON. Gated by the `json` feature (default).
- `Tensor::load_json(path)` — reads a file then delegates to `from_json`.
  File errors map to `MattenError::Io`; parse errors to `MattenError::Parse`.
- `Tensor::from_csv(input: &str)` — rectangular numeric CSV; infers shape
  `[rows, cols]`; includes row/column context in error messages. Gated by
  the `csv` feature (default).
- `Tensor::load_csv(path)` — reads a file then delegates to `from_csv`.
- `Serialize`/`Deserialize` for `Tensor` via the canonical object form
  (gated by the `serde` feature, default).
- New `src/ser.rs` (serde impls) and `src/parse/` module (`json.rs`,
  `csv.rs`).
- Fixture files `examples/data/tensor_2x2.json` and
  `examples/data/numeric_2x3.csv`.
- `src/tests/parse.rs` — 23 boundary tests covering both parsers, both
  file loaders, serde round-trips, error variants, and malformed-input
  safety.

## [0.4.0] - 2026-06-20

Milestone **M4 — Shape Operations and Slicing** (RFC-007 + RFC-008).

### Added

- **RFC-007 — Reshape, axis operations, and indexing:**
  - `reshape(&self, shape)` / `try_reshape` — element-count-preserving owned
    copy; panics / returns `Err` on mismatch.
  - `flatten(&self)` — collapses any shape to `[len]`; scalar becomes `[1]`.
  - `transpose(&self)` / `t(&self)` — reverses axis order (swap rows/cols for
    rank-2; reverses all axes for higher rank).
  - `swap_axes(&self, a, b)` — swaps two axes; both forms share an internal
    `permute_axes` helper that writes a fresh row-major result.
  - `get(&self, coord)` → `Option<f64>` — safe non-panicking element access.
  - New `src/reshape.rs` module for the above helpers.

- **RFC-008 — Slicing API:**
  - `tensor.slice()` returns a `SliceBuilder`; methods are `.all()`, `.index(n)`,
    and `.range(R)` (accepts `Range`, `RangeFrom`, `RangeTo`, `RangeFull`,
    `RangeInclusive`); `.build()` returns `Result<Tensor, MattenError>`.
  - `tensor.slice_str(spec)` — bounded NumPy-like convenience (max 512 bytes),
    always returns `Result`, never panics on malformed input. Grammar: `:`,
    `n`, `start:end`, `start:`, `:end`, `start:end:step`.
  - `IntoSliceRange` / `SliceConvert` — public sealed-trait pair (only std range
    types satisfy the bound; no external implementation possible).
  - New `src/slice.rs` module.

- **Test reorganisation (module-style fix):**
  - All `mod.rs` files replaced with the 2018+ `foo.rs` + `foo/` layout:
    `src/tests.rs`, `src/ops.rs`, `src/ops/broadcast.rs`.
  - `src/tests/reshape.rs` and `src/tests/slice.rs` added.

## [0.3.0] - 2026-06-20

Milestone **M3 — Broadcasting and Element-wise Operators** (RFC-006).

### Added

- `ops/` module directory with four sub-files:
  - `broadcast.rs` — `broadcast_shape` helper (right-aligned NumPy rules,
    returns `MattenError::Broadcast` on incompatible pairs) and
    `BroadcastCtx` (precomputed zero-stride index mapping, no expanded
    intermediates).
  - `tensor_ops.rs` — `Add`, `Sub`, `Mul`, `Div` for `&Tensor op &Tensor`.
  - `scalar_ops.rs` — all eight scalar forms: `&Tensor op f64` and
    `f64 op &Tensor` (the latter is legal; `&Tensor` is local in the trait
    parameter position and does not violate the orphan rule).
  - `unary_ops.rs` — `Neg` for `&Tensor`.
- Broadcasting handles scalar `[]`, missing leading axes, and `dim == 1`
  on either side without materialising expanded copies.
- `*` remains element-wise; matrix multiplication is RFC-010 / M6.
- Division by zero follows IEEE 754 (`inf`/`-inf`/`NaN`); no error produced.

## [0.2.0] - 2026-06-20

Milestone **M2 — Creation and Conversion APIs** (RFC-004): the full Phase 1
constructor and conversion surface.

### Added

- Fill constructors: `zeros`, `ones`, `full`, `from_vec`.
- `arange(start, end, step)` / `try_arange` — half-open range, checked
  allocation limit, rejection of zero/non-finite step and non-finite bounds.
- `into_vec(self)` — consuming flat extraction without a copy.
- `try_from_rows(rows)` — recoverable rectangular row construction.
- `From<Vec<f64>>` → 1-D tensor; `From<Vec<Vec<f64>>>` → 2-D (panic on ragged).
- `From<Tensor> for Vec<f64>` (consuming) and `From<&Tensor>` (borrowing copy).
- `TryFrom<Tensor> for Vec<Vec<f64>>` — fails for non-rank-2 tensors.
- New `convert.rs` module holding all trait impls and the `flatten_rectangular`
  helper.

### Notes

- Arithmetic, broadcasting, and reshape are M3 (v0.3.0); JSON/CSV I/O is M5.

## [0.1.0] - 2026-06-20

Milestone **M1 — Core Tensor Contract** (RFC-003): the validated shape model and
the observational `Tensor` surface. Still no arithmetic, reshape, or I/O.

### Added

- Full shape validation in a new `shape` module: rank cap (`MAX_NDIM = 8`),
  rejection of zero-sized dimensions, and checked element-count arithmetic.
  Constructors now route through it (`MattenError::Shape` / `Allocation`).
- Scalar semantics: `Tensor::scalar(value)` (shape `[]`, length `1`).
- Observational API: `is_scalar`, `is_vector`, `is_matrix`, and `to_vec`,
  alongside the existing `shape` / `ndim` / `len` / `as_slice`.
- `Tensor` now derives `Clone` and `PartialEq`.
- Internal row-major index helpers (`strides_for_shape`, `coord_to_flat`,
  `flat_to_coord`) with a flatten/unflatten round-trip property test.

### Notes

- `is_empty()` remains intentionally absent (deferred zero-sized-tensor RFC).
- Creation conveniences (`from_vec`, `zeros`, `ones`, `full`, `arange`,
  `From`/`TryFrom`) are M2; arithmetic and broadcasting are M3.

## [0.0.1] - 2026-06-20

The **M0 crate skeleton**: a compiling, lint-clean, CI-ready foundation aligned
with the v2 reconciled specification. No math is implemented yet.

### Added

- Crate manifest with the locked feature matrix
  (`default = ["serde", "json", "csv"]`), edition 2024, and MSRV `1.85`.
- `#![forbid(unsafe_code)]` crate-wide.
- Stable public error surface: the canonical `MattenError` enum (derives only
  `Debug`; matched by variant, never `==`) and the public `DataFormat` enum, with
  manual `Display` and `std::error::Error` implementations.
- Minimal public `Tensor`: `new`, `try_new`, `shape`, `ndim`, `len`, `as_slice`,
  and a shape-first `Debug`. Construction uses checked shape-product arithmetic.
- `examples/hello_tensor.rs` smoke example; unit and integration smoke tests.
- CI (fmt, clippy `-D warnings`, tests, doctests, feature-profile builds, MSRV).
- mdBook documentation scaffold under `docs/`.

### Notes

- `is_empty()` is intentionally absent (deferred to a future zero-sized-tensor RFC).
- The full Core Tensor Contract (scalar/vector/matrix predicates, `to_vec`,
  reshape, transpose, arithmetic, broadcasting, slicing, JSON/CSV) lands in M1
  and later milestones.
