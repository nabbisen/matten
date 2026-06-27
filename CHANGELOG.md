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

## [0.24.0] - 2026-06-27

**Result-form reductions — complete the fallible reduction surface (RFC-055 + RFC-056).**
Additive public API in core `matten`; no new dependency, no breaking change, f64-only core.
Every reduction that can fail now has a non-panicking `try_` form alongside its panic form,
joining the existing `try_var` / `try_std` family.

### Added

- **Scalar value reductions (RFC-055):** `try_sum`, `try_mean`, `try_min`, `try_max`, `try_norm`
  → `Result<f64, MattenError>`. Each returns `MattenError::Unsupported` on a dynamic tensor and
  is otherwise total; `NaN` is treated as a value and propagates as before.
- **Axis reductions (RFC-056):** `try_sum_axis`, `try_mean_axis`, `try_min_axis`, `try_max_axis`
  → `Result<Tensor, MattenError>`. Each returns `MattenError::Shape` for an out-of-range axis
  and `MattenError::Unsupported` on a dynamic tensor (dynamic checked first, matching
  `try_var_axis` / `try_std_axis`). The reduced axis is removed from the output shape, matching
  the panic form.

### Changed

- The panic forms (`sum`/`mean`/`min`/`max`/`norm` and the `*_axis` forms) now delegate to their
  `try_` engines via `unwrap_or_else(|e| panic!())`, the same pattern `var`/`std` already use, so
  the two forms can never diverge. Behaviour is unchanged: they still panic on a dynamic tensor
  and on an out-of-range axis. Panic **message text** for these forms is now sourced from the
  `MattenError` `Display` (consistent with `var`/`std`); message text is not part of the API
  contract. No new `MattenError` variant.
- **`norm` ruling reversed:** the prior v0.21 "norm panic-only" decision is reversed (architect
  deep review 2026-06-27). `norm` gains `try_norm`; its rustdoc no longer implies the value
  reductions will never have `try_` forms.
- Internal: the dynamic-rejection guard is now a single shared `reject_dynamic` helper reused by
  the core and statistics reductions; an axis-bounds `check_axis` helper centralises the
  out-of-range check.
- Docs: `public-api-snapshot.md` lists the nine new methods; `compatibility.md` gains a v0.24
  family entry; reduction rustdoc cross-links panic and `try_` forms.

### Version

- Family bump `0.23.5` → `0.24.0` (additive public API → minor). User-facing install pins and
  family labels retargeted `0.23` → `0.24` across READMEs, `lib.rs` rustdoc, and doc pages
  (flagged by the self-updating version-drift guard).

### Threat model

Additive numeric API over existing in-memory `f64` tensors. No new data flow, external
integration, or auth surface; the only failure paths reuse the existing dynamic-tensor
(`Unsupported`) and axis-bounds (`Shape`) controls, now also reachable as recoverable `Result`s
instead of only panics. Existing controls verified to remain valid; no threat-model change.

## [0.23.5] - 2026-06-27

**RFC-050–054 deep-review response (P1 + P2 fixes) and migration-batch lifecycle close.**
Documentation, release-tooling, and RFC bookkeeping only; no library code, public API, runtime
behavior, or dependency change in any crate. Applies the architect's v0.23.4 deep-review
findings.

### Fixed (P1)

- **CHANGELOG release headings restored.** A heading-eating regression in successive release
  edits had nested every block from v0.23.3 back to v0.21.4 under the single `## [0.23.4]`
  heading. Restored the 13 missing `## [x.y.z] - date` headings (v0.23.3, v0.23.2, v0.23.1,
  v0.23.0, v0.22.7–v0.22.0, v0.21.4) with authoritative versions/dates from the ROADMAP
  history; block content was intact, only the headings were lost.
- **Candle snippet `f64` fix** in `docs/src/reference/migration.md`: the example constructed a
  `Tensor` from `vec![1.0f32, …]`, but `matten::Tensor` is `f64`. Now builds with `f64` data
  and casts to `f32` only at the Candle boundary.

### Changed (P2)

- `docs/src/reference/migration.md`: softened "Migration is always one line" to "The simplest
  data-export path is"; made the `ndarray` section **bridge-first** (`matten_ndarray::to_arrayd`)
  with the manual `ArrayD::from_shape_vec` path kept as a "without the bridge crate" fallback;
  replaced the brittle "four exports" compatibility wording with a statement that points to the
  public API snapshot and CHANGELOG for the exact current surface.
- `scripts/check-release-docs.sh`: added a `CURRENT_MINOR` extraction sanity check (fails fast
  if the minor cannot be derived from `Cargo.toml`), and a CHANGELOG-heading guard that requires
  the top heading to match the workspace version and flags any release block containing more
  than one `### Threat model` section (the signature of a lost heading). Both verified
  positive/negative.

### RFC lifecycle

- **RFC-050, RFC-051, RFC-052, RFC-053 → Implemented** and moved from `rfcs/proposed/` to
  `rfcs/done/` (shipped in v0.23.0, v0.23.2, v0.23.0–v0.23.1, v0.23.4 respectively); status
  headers updated and the `rfcs/README.md` index Done/Proposed tables reconciled.
- **RFC-054** remains in `rfcs/proposed/` as accepted-future-direction with deferral confirmed
  by the deep review; no `matten-migrate` tool exists.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. Documentation,
release-tooling guards, and RFC status/location bookkeeping only; no new data flow,
integration, or auth surface.

## [0.23.4] - 2026-06-26

**Production migration guide — RFC-053 migration-readiness diagnostics (completes the
migration batch).** Documentation only; no library code, public API, runtime behavior, or
dependency change. This is the last RFC of the RFC-050–053 migration program; RFC-054
(`matten-migrate` CLI) remains deferred.

### Added

- **`docs/src/migration/readiness-checklist.md`** — turns "should I leave `matten`?" into ten
  concrete pressure signals (data-size, runtime, axis-reduction, linear-algebra, dataframe,
  ML/device, dynamic-ingestion, dependency policy, ecosystem preference, team language), each
  mapped to a target playbook, with explicit "stay with `matten`" outcomes. Advisory
  self-assessment — no source-scanning tool.
- **`docs/src/migration/readiness-report.md`** — a manual, fillable report template with the
  nine required sections (Summary, Current matten usage, Production pressure signals,
  Recommended target(s), Direct conversion candidates, Manual redesign areas, Bridge
  crates/tools, Risks, Next steps) and the required advisory disclaimer (the report does not
  prove production readiness, does not guarantee a target is better, and does not perform
  automatic conversion).
- **`docs/src/migration/examples/linear-regression-gd-readiness.md`** — the template filled in
  for `35_linear_regression_gradient_descent`, written against the example's actual structure
  (two `Tensor::matmul` per step, a reused transpose, an iterative loop): recommends moving the
  per-step matrix products to `ndarray` via the `matten-ndarray` bridge at real data sizes,
  with a `nalgebra` closed-form solve as an optional redesign, and "stay with `matten`" at toy
  size.

### Changed

- `docs/src/SUMMARY.md` and `migration/index.md` link the readiness pages.
- `scripts/check-release-docs.sh`: the migration overclaim guard now allows the negated
  advisory disclaimer ("does not perform automatic conversion") while still flagging positive
  "automatic conversion" claims (verified both directions).

### Migration program status

RFC-050 (foundation), RFC-051 (bridge contracts), RFC-052 (all target playbooks), and RFC-053
(readiness diagnostics) are **complete**. RFC-054 (`matten-migrate` CLI) is deferred.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. Documentation and a
guard refinement only; no new data flow, integration, or auth surface.



## [0.23.3] - 2026-06-26

**Version-string hygiene + self-updating drift guard.** Documentation and release-tooling
only; no library code, public API, runtime behavior, or dependency change in any crate. Fixes
stale version strings left by the v0.23.0 family bump and makes the guard that should have
caught them self-updating.

### Fixed

- **Stale `0.22` version strings corrected to `0.23`** across all four crate READMEs, the root
  README, the core crate's `lib.rs` rustdoc, and ~10 doc pages (quick-start, examples/data,
  reference/boundary, reference/dynamic, contributing/architecture, public-api-snapshot, and
  the `0.22.x`/`current 0.22 family` labels). The install pins were not merely cosmetic: a
  caret requirement like `matten = "0.22"` resolves to `>=0.22.0, <0.23.0`, so anyone copying
  a pin was silently held on the old family and missed the entire 0.23 migration guide.
  Genuinely historical references (`promoted to Beta in v0.22.0`, the per-family
  compatibility history) are preserved. Added a `v0.23 family` entry to
  `reference/compatibility.md`.

### Changed

- **`scripts/check-release-docs.sh`: the version-string guard now derives the current minor
  dynamically from `Cargo.toml`** instead of a hardcoded `CURRENT_MINOR="22"`. That hardcoded
  value — which required a manual bump each release — was missed at v0.23.0, which is exactly
  why the stale `0.22` pins shipped and went unflagged. The guard now cannot go stale on a
  bump; it checks install pins, `X.Y.x family` labels, and `current vX.Y family` prose (still
  requiring "family" adjacency, so generic patch-notation like `(0.13.x)` is not flagged).
  Verified: passes on the corrected docs, and a simulated `0.24.0` bump immediately flags the
  now-stale `0.23` strings.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. Documentation and a
release-tooling guard only; no new data flow, integration, or auth surface.



## [0.23.2] - 2026-06-25

**Production migration guide — RFC-051 bridge conversion contracts.** Documentation only; no
library code, public API, runtime behavior, or dependency change in any crate. Formalizes the
bridge conversion-contract template and the bridge-crate policy, and documents the reference
`matten-ndarray` contract verified against its implementation.

### Added

- **`docs/src/migration/bridge-contracts.md`** — the conversion-contract template (13
  dimensions: source/target, direction, copy/view, shape/rank, memory order, dynamic-tensor,
  NaN, missing-value, integer/text/bool, error behavior, performance, examples) plus the
  filled-in `matten-ndarray` reference contract. The `matten-ndarray` row was checked against
  `convert.rs`/`error.rs`: copies both ways, numeric-only, rejects dynamic tensors
  (`DynamicTensor`, unconditional, not a panic), preserves logical row-major order through
  non-standard layouts, rejects zero-sized axes, `Result` never panics. Records that RFC-051's
  generic error categories are illustrative, not a required enum schema.
- **`docs/src/migration/bridge-crate-policy.md`** — bridge crates own their target dependency,
  never re-export `Tensor` (confirmed: `matten-ndarray` exports only `to_arrayd`/`from_arrayd`/
  `MattenNdarrayError`), use `to_<target>`/`from_<target>` naming, return `Result`, and publish
  a contract; includes a future-bridge checklist and the rule that new bridge crates need
  separate approval. Notes the CI-enforced published-dependency isolation guard.

### Changed

- `crates/matten-ndarray/README.md`: added the conversion-contract table.
- `docs/src/examples/companions.md`: cross-links the bridge contract and policy pages.
- `docs/src/SUMMARY.md`: lists the two new migration pages.

### Still staged

RFC-053 migration-readiness diagnostics (report template, checklist, worked example) — the
last RFC in this migration batch. RFC-054 (`matten-migrate` CLI) remains deferred.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. Documentation only.



## [0.23.1] - 2026-06-25

**Production migration guide — RFC-052 completed (remaining target playbooks).** New
documentation only; no library code, public API, runtime behavior, or dependency change in
any crate. Adds the cross-paradigm/cross-language playbooks, completing RFC-052's target set.

### Added

- **`docs/src/migration/playbooks/polars-and-pandas.md`** — the dataframe path. States
  plainly that `matten-data` is an ingestion on-ramp and will **not** grow
  group-by/join/pivot/query, and that the usual pattern is to enter the dataframe library at
  the data source rather than round-trip a `Tensor`.
- **`docs/src/migration/playbooks/candle.md`** — the ML path, careful **not** to imply
  `matten` is an ML framework (no autograd/layers/optimizers/device); notes the `f64`→`f32`
  precision boundary.
- **`docs/src/migration/playbooks/python-numpy.md`** — the Python scientific path, framed as
  a manual/conceptual serialization hand-off with no in-process Rust↔Python bridge.

Each follows the standard playbook section set. Their positioning notes state honestly that
**no benchmark exists** for these targets (a cross-paradigm/cross-language comparison would
be RFC-049 Phase 3, which is not authorized), so target choice is by capability/ecosystem
fit, not measured speed.

### Changed

- `docs/src/migration/playbooks/index.md`, `target-selection.md`, `index.md`: the three
  targets moved from "later revision" to available, with links; `SUMMARY.md` lists all five
  playbooks.

### Still staged for v0.23.x

RFC-051 bridge-contract pages + the `matten-ndarray` contract table, and RFC-053
migration-readiness diagnostics. RFC-054 (`matten-migrate` CLI) remains deferred.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. Documentation only;
no new data flow, integration, or auth surface.



## [0.23.0] - 2026-06-25

**Production migration guide — first release (RFC-050 + RFC-052 Rust playbooks).** New
documentation only; no library code, public API, runtime behavior, or dependency change in
any crate. Core `matten` gains **no** new dependency. This is the first stage of the
"family-car → super-car" migration program (RFC-050–054); the architect prioritized the
Rust-target playbooks, which this release delivers.

### Added

- **`docs/src/migration/` guide (RFC-050 foundation):** `index.md` (the migration promise —
  "outgrowing matten is a successful PoC outcome"; matten stays dependency-light; not an
  automatic code rewriter), `when-to-migrate.md` (stay-vs-migrate pressure signals),
  `target-selection.md` (workload → ecosystem matrix + decision path), and
  `common-pitfalls.md` (row-major vs column-major, convert-once, `f64`/`f32`, dynamic →
  numeric first).
- **Target playbooks (RFC-052, Rust targets):** `playbooks/index.md` (decision tree),
  `playbooks/ndarray.md`, and `playbooks/nalgebra.md` — each with the full section set
  (choose / do not choose / concept mapping / example migrations / conversion path /
  pitfalls / positioning notes / checklist). The `ndarray` playbook leads with the
  contract-backed `matten-ndarray` bridge (`to_arrayd`/`from_arrayd`); `nalgebra` documents
  the manual `from_row_slice` path and notes no `matten-nalgebra` bridge exists yet.
  Positioning notes cite the **accepted** RFC-049 peer comparison task-scoped (no ranking,
  no "faster than" claims).
- **Migration release-doc guard:** `scripts/check-release-docs.sh` now fails on overclaim
  phrases in `docs/src/migration/` (speed-superiority, "drop-in replacement",
  auto-rewrite-your-code), phrase-anchored with a future/deferred exception for the one
  phrase that legitimately appears in RFC-054 context. Positive/negative tested.

### Changed

- `docs/src/SUMMARY.md`: new top-level **Migration** section.
- `docs/src/reference/migration.md`: now cross-links to the comprehensive guide and is
  framed as its quick copy-paste companion (no duplication).

### Not in this release (staged for v0.23.x)

The remaining RFC-052 playbooks (Polars/Pandas, Candle, NumPy), RFC-051 bridge-contract
pages + the `matten-ndarray` contract table, and RFC-053 migration-readiness diagnostics.
RFC-054 (`matten-migrate` CLI) remains deferred.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. New files are
documentation; the only guard change is a scoped documentation check. No new data flow,
integration, or auth surface.



## [0.22.7] - 2026-06-25

**RFC-049 Phase 2 accepted — documentation reconciliation.** Docs/RFC/report wording only;
no library code, public API, runtime behavior, or dependency change in any crate. The
architect accepted the official maintainer-run Rust peer comparison (commit `007031c`,
v0.22.6, baseline machine class) as the RFC-049 Phase 2 official report. Phase 2 is now
complete; Phase 3 (NumPy/Pandas) and Phase 4 (hard gates) remain unauthorized, and no
optimization work is required.

### Changed

- **`benchmarks/reports/peer-comparison-v0.1.md`:** added the architect acceptance marker and
  Report ID (`matten-rfc049-rust-peer-comparison-v0.1`); status changed from "pending
  acceptance" to accepted; added the "natural representation = task-level workflow cost, not
  identical internal strategy" clarification; **corrected the nalgebra version note** — `0.33.3`
  is pinned by the project's Rust 1.85 compatibility floor (Cargo's MSRV-aware resolver), and
  `nalgebra 0.35.0` (needs Rust 1.89) would require an explicit MSRV-policy decision; it is not
  a constraint of the maintainer's 1.93 toolchain.
- **Benchmark docs (`docs/src/benchmarks/index.md`, `methodology.md`, `benchmarks/README.md`):**
  Phase 2 wording flipped from "official numbers pending" to "complete and accepted (2026-06-25)".
- **RFC-049 (header, Phase 2 annotation, index line):** Phase 2 marked implemented and
  accepted; Phases 3–4 still deferred; remains in `proposed/` until those are resolved.
- **RFC-052:** added a peer-evidence citation note — playbooks may now cite the accepted
  task-scoped results under the prior constraints (no ranking, no "faster than X", no universal
  migration mandate).

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. Documentation/report
status text only; no new data flow, integration, or auth surface.



## [0.22.6] - 2026-06-24

**Production-migration RFC set (RFC-050–054) accepted and ingested** — planning/docs only.
No library code, public API, runtime behavior, or dependency change in any crate; this
release adds proposal documents and applies an architect ruling to them.

The architect accepted the "family-car → super-car" production-migration direction (a
documented, honest exit ramp from `matten` to heavier ecosystems — `ndarray`, `nalgebra`,
Polars, Candle, NumPy, Pandas — with no heavy dependencies in core `matten`) and ruled on a
peer code review of the bundle. The five RFCs and their handoff bundle are now tracked in
`rfcs/`, with the ruling's edits applied.

### Added

- `rfcs/proposed/050`–`054`: Production Migration Guide & Bridge Strategy (050), Bridge
  Conversion Contracts & Companion-Crate Policy (051), Production Target Playbooks (052),
  Migration Readiness Diagnostics & Report Format (053), and the deferred `matten-migrate`
  Assisted Migration Tool (054). All marked **Accepted** (054 as future direction).
- `rfcs/handoffs/`: the production-migration implementation handoff
  (`050-053-production-migration-implementation-handoff.md`), the RFC-054 deferred note, and
  the acceptance/QA and release-guard checklists. Indexed in `rfcs/README.md`.

### Changed (architect ruling, 2026-06-24)

- **RFC number collision resolved.** The migration set keeps 050–054; the earlier
  RFC-050 earmark for a future *Testing Strategy Refresh: Property Tests and Fuzz Boundary*
  is renumbered to **RFC-055** (note updated in `rfcs/done/013-…`).
- **RFC-051:** §9 clarifies its error categories are illustrative, not a required enum schema
  (`matten-ndarray`'s existing `DynamicTensor`/`ZeroSizedAxis`/`NdarrayShape`/`Matten` is
  compliant as-is); §15 marks the `matten-ndarray` audit **documentation-only** (no new error
  variant); §17 resolves the open question — `to_<target>`/`from_<target>` is the default
  bridge naming convention, following `to_arrayd`/`from_arrayd`.
- **RFC-052:** softened the deprecated `ndarray` `.into_shape(...)` example to current
  reshape APIs; added an acceptance rule (and matching handoff criterion) that playbook
  performance/positioning sections stay **pending** with no numeric claims until official
  RFC-049 Phase 2 peer numbers are accepted.
- **RFC-054:** notes that, if ever approved, the tool starts as workspace-excluded,
  `publish = false` tooling (like `benchmarks/`), not a published crate.
- **Release-guard checklist:** made phrase-anchored only — no bare-word bans
  (`always`/`never`/`automatic`/`replace`), which false-positive on legitimate text.

### Threat model

No change to any crate's code, API, dependency set, or runtime behavior. New files are
planning documents; the only edits to existing files are RFC-index/earmark/ROADMAP text. No
new data flow, integration, or auth surface.



## [0.22.5] - 2026-06-24

**v0.22.4 deep-review release-truth reconciliation.** Documentation, RFC status, and one
guard only — no library code, public API, runtime behavior, or benchmark *logic* change in
any crate. The architect's v0.22.4 codebase deep review accepted the Phase 2 harness/template
implementation (no P0/P1 source blockers) and requested status-text fixes: several benchmark
status docs still described Phase 2 as deferred after the harness/template shipped. The
companion benchmarking/positioning review separately accepted the baseline (archival-ready)
and the peer template, and confirmed nalgebra-on-all-six and official-numbers-pending — no
change required from it.

Throughout, the harness/template-implemented state is now consistently distinguished from the
*official peer report complete* state (which still awaits a maintainer run).

### Changed

- **`docs/src/benchmarks/index.md` (P1):** status section rewritten — Phase 1 accepted;
  Phase 2 harness/template implemented with official maintainer-run numbers pending; only
  Phases 3–4 still deferred (was "Current status: Phase 1 / Phase 2 deferred").
- **RFC-049 header (P1):** Status / Target Release / Acceptance reconciled with reality —
  Phase 2 harness/template implemented in v0.22.4, official peer numbers pending, Phases 3–4
  deferred; stays in `proposed/` per the 4-folder lifecycle. (The RFC body's Phase 2 section
  was already updated in v0.22.4; this removes the header/body inconsistency.)
- **`benchmarks/README.md` (P2):** title no longer says "Phase 1"; status distinguishes the
  implemented harness/template from the pending official report; the peer command comment now
  says "never in the default build or ordinary CI; compile-checked only by the
  manual/scheduled peers workflow".
- **`benchmarks/reports/peer-comparison-v0.1.md` (P2):** explicit top-level status marker
  ("Template only; official maintainer-run peer numbers pending; do not cite the sandbox
  appendix"); interpretation guidance aligned with the architect's positioning/migration tone
  (trades performance for approachability; no "loses/worse/superior" language).
- **`docs/src/benchmarks/methodology.md` (P2):** states the harness/template are implemented
  but the official peer report is not complete until maintainer-run medians fill the template.

### Added

- **`scripts/check-release-docs.sh` benchmark-status-drift guard (P2):** fails if the current
  benchmark docs (`docs/src/benchmarks/` only — not RFC history or CHANGELOG) describe Phase 2
  as unimplemented/deferred. Phase 3/4 deferral wording remains allowed. Positive/negative
  tested; rides the existing release-docs gate already in CI and the release checklist.

### Threat model

No change to any published crate's code, API, dependency set, or runtime behavior, and no
change to benchmark logic or the peers isolation properties. Documentation/status text and one
narrowly-scoped documentation guard only.



## [0.22.4] - 2026-06-24

**RFC-049 Phase 2 — Rust peer comparison (opt-in)**, plus the accepted Phase 1 internal
baseline report and a workspace-config fix. The published crates — core `matten`,
`matten-ndarray`, `matten-mlprep`, `matten-data` — are unchanged: no library code, public
API, or runtime behavior change. All new work lives in the workspace-excluded benchmark
harness, docs, CI, and repo config.

The architect accepted the maintainer-run internal baseline (Ubuntu 26.04, virtualized;
Baseline ID `matten-rfc049-internal-baseline-v0.1`) and authorized RFC-049 Phase 2 under
the previously settled constraints. Phase 3 (NumPy/Pandas) and hard performance gates
remain unauthorized.

### Added

- **RFC-049 Phase 2 peer-comparison harness** (benchmark crate only). A `peers` feature
  (`ndarray` + `nalgebra` as optional deps) that is **off by default**; peer task modules
  (`workloads/peers/{ndarray_tasks,nalgebra_tasks}.rs`) implementing the fixed comparable
  task set — cosine similarity, small `matmul`, Markov step, PageRank step, linear-
  regression GD step, heat step — each documenting why it is comparable; and a
  `required-features = ["peers"]` `peers` bench giving a three-way `matten`/`ndarray`/
  `nalgebra` comparison per task from identical logical data. It is a *Rust peer comparison
  for positioning*, never a ranking or "faster than X" claim.
- **`benchmarks/reports/peer-comparison-v0.1.md`**: peer-comparison report template with the
  comparable-task table, limitations, and the non-ranking disclaimer (to be filled by a
  maintainer run on the same machine class as the baseline).
- **`.github/workflows/benchmarks-peers.yml`**: a separate, manual/scheduled workflow that
  only compile-checks `--features peers --bench peers --no-run` and re-asserts published
  dependency isolation. Deliberately kept **out of ordinary CI** (no peer deps in the normal
  flow, no speed gates).
- Completed, accepted **Phase 1 internal baseline report** (`internal-baseline-v0.1.md`)
  with the real Ubuntu 26.04 medians, peak RSS (44,728 kB), Baseline ID, and architect
  acceptance marker.

### Changed

- `benchmarks/Cargo.toml`: optional `ndarray`/`nalgebra` deps, the `peers` feature, and the
  gated `peers` bench. The default build and ordinary CI `--no-run` remain peer-free
  (verified: zero peer crates compiled by default), and the published-isolation guard still
  passes — peer deps never reach any published crate.
- Methodology / RFC-049 / benchmark README: Phase 2 marked **implemented** (was
  designed-not-authorized), retaining the comparable-task-only policy and peer-free defaults.
- Workspace `Cargo.toml`: `exclude` now uses `tests/fixtures/*`. Because Cargo's `exclude`
  does not expand globs (unlike `members`), the RFC-031 fixture is made self-excluding with
  an empty `[workspace]` table so it still runs standalone; any future fixture should do the
  same.
- ROADMAP: recorded the `sum_mean_axis` axis-reduction cost (~1.31 ms; ~400× `sum_mean`,
  ~17× a 64×64 `matmul`) as a **P2 performance-watch** / regression-visibility anchor — not
  a fix-now item, and not a Phase 2 blocker (architect ruling).

### Threat model

No change to any published crate's code, API, dependency set, or runtime behavior. New
dependencies (`ndarray`, `nalgebra`) are optional, benchmark-only, and off by default;
isolation from published crates is positively proven by
`scripts/check-published-dependency-isolation.sh`. The peer comparison makes no performance
or ranking claim.



## [0.22.3] - 2026-06-24

**RFC-032 scope carve-out + published-crate dependency isolation guard** (benchmarking /
positioning review follow-up). Documentation, guard, and CI only — no library code, public
API, or runtime behavior change in any crate.

The architect's review of the two benchmarking/positioning review requests ruled: (A)
workspace-excluded `publish = false` internal tooling is outside RFC-032's published,
user-facing scope (record the carve-out; change nothing in `benchmarks/` or the fixture);
and (B) settle the RFC-049 Phase 2 design now but do **not** implement it — Phase 2 waits
for a maintainer-run credible baseline report and separate authorization.

### Added

- **`scripts/check-published-dependency-isolation.sh`** (RFC-049 §B1): positively proves
  every published crate is free of peer/benchmark dependencies, rather than relying only on
  the harness being workspace-excluded. Per-crate matrix: core / `matten-data` /
  `matten-mlprep` forbid `criterion`/`ndarray`/`nalgebra`; `matten-ndarray` forbids
  `criterion`/`nalgebra` but is allowed `ndarray` (its bridge reason). Passes today; wired
  into CI (after the RFC-022 core boundary check) and the release checklist. Inspected with
  `--all-features --edges normal,build`, mirroring the core guard.
- **`benchmarks/reports/BASELINE-READY-CHECKLIST.md`** (RFC-049 §B4): the "report ready"
  checklist that must be satisfied by a maintainer-run baseline before Phase 2 is authorized.

### Changed

- **RFC-032** gains a §5.1 scope clarification: workspace-excluded, `publish = false`
  internal tooling (the RFC-031 fixture, the RFC-049 harness) is outside the published-family
  convention's packaging scope, while still following its ownership-clarity spirit (no
  core-type re-export; import core types from `matten`). Records that the RFC-032 guard is
  intentionally not extended to scan that tooling, and that isolation is proven by the new
  per-crate guard instead.
- **RFC-049** Phase 2 section annotated with the settled design rulings (B1 isolation guard,
  B2 structural `peers`-feature/comparable-task enforcement, B3 opt-in/off-by-default build &
  CI shape, B4 baseline-report entry precondition) — marked **designed, not yet authorized**.
- **Benchmark methodology doc** and the **baseline report template** updated to record the
  Phase 2 design (opt-in `peers`, task-scoped comparison, published-crate isolation) and to
  point at the unlock checklist.

### Not done (deferred by ruling)

- RFC-049 Phase 2 implementation (the `peers` feature, peer `ndarray`/`nalgebra` workloads,
  peer benches/CI) is **not** started. It remains unauthorized until a maintainer-run
  credible internal baseline report is delivered and accepted.

### Threat model

No new runtime surface. The new guard strengthens published-crate dependency integrity (a
defensive check that passes against the current clean tree and pre-positions the isolation
proof before any peer dependency exists). No published crate gained or lost a dependency.



## [0.22.2] - 2026-06-24

**Lifecycle wording cleanup (v0.22.0 handoff-review P2 follow-up).** Documentation /
RFC-lifecycle only — no library code, public API, runtime behavior, examples, guards, or
CI change.

The architect's v0.22.0 handoff review accepted the release, the `matten-data` Beta
promotion, and the malformed-CSV test realization, with one P2 follow-up: reconcile the
"malformed-CSV **parser-error** test" wording to the structured-error framing actually
implemented.

### Changed

- **RFC-023 §9** (Beta gate) gains a clarification note: the malformed-CSV criterion is
  satisfied by a **structured-error / no-panic** malformed-input test (`Csv` or
  `RaggedRow`, never a panic or a silently-wrong `Table`), not a low-level parser-error
  test, because under the public `&str` API and lenient `flexible(true)` csv reader some
  malformed-quote cases resolve to structural `RaggedRow` validation. Records that a
  byte-level invalid-UTF-8 test is intentionally not added (no public path; would test the
  dependency, not `matten-data`).
- **RFC-036** Implemented note updated to point at that clarification.

Historical records in earlier `CHANGELOG.md`/`ROADMAP.md` entries are left unchanged
(they describe the finding accurately as of v0.22.0).



## [0.22.1] - 2026-06-24

**RFC-049 Phase 1 — internal benchmark baseline.** The architect accepted RFC-049 with a
staged mandate and authorized **Phase 1 only** (PR-049-1 + PR-049-2). This release adds a
small, isolated benchmark harness and its methodology docs. It is tooling + documentation
only — **no change to any published crate's code, public API, or runtime behavior**, and
no new dependency enters any published crate.

### Added

- **Benchmark harness** under `benchmarks/` (PR-049-2): a `criterion` harness kept
  **outside** the Cargo workspace (`workspace.exclude`), `publish = false`, invoked via an
  explicit `--manifest-path`. Workloads live in a plain library (`src/workloads/{core,scenarios}.rs`,
  `src/common.rs`) with no `criterion` dependency; only the bench targets
  (`benches/{core,scenarios}.rs`) use it. Covers the core micro set (construction,
  reshape/flatten, elementwise add/mul, broadcasting, `sum`/`mean`, `sum_axis`/`mean_axis`,
  `matmul`, slice; optional dynamic `try_numeric` behind a `dynamic` feature) and the five
  scenario workloads from examples 26/33/34/35/36 (cosine similarity, Markov step, PageRank
  step, linear-regression GD step, heat-equation step).
- **Benchmark methodology docs** (PR-049-1): `benchmarks/README.md`,
  `docs/src/benchmarks/index.md`, and `docs/src/benchmarks/methodology.md` (purpose,
  non-goals, metrics, environment recording, Linux peak-RSS memory policy, Phase-1-only
  scope, required disclaimer), wired into the mdBook summary.
- **Internal baseline report template** (`benchmarks/reports/internal-baseline-v0.1.md`) per
  the §6 structure, plus `benchmarks/results/README.md` recording the commit policy (curated
  reports and small sample schemas only — not bulky raw histories).

### Changed

- **Core dependency-boundary guard** now also forbids `criterion` in core `matten`'s tree
  (RFC-049 §7), making the benchmark-dependency isolation enforced, not just structural.
- **CI**: a dedicated `benchmarks` job compile-checks the harness
  (`cargo bench --manifest-path benchmarks/Cargo.toml --no-run`). Full benchmarks are never
  run in normal CI and there are **no speed/memory pass-fail gates** (§5).
- `benchmarks/Cargo.lock` is git-ignored (workspace-excluded crate, regenerated on demand).

### RFC lifecycle

- **RFC-049** → **Accepted** (Phase 1 authorized and implemented in 0.22.1; Phases 2–4
  deferred). Per the 4-folder lifecycle, it stays in `proposed/` until fully implemented.

### Threat model

No new runtime surface and no new dependency in any published crate. The benchmark harness is
a workspace-excluded, unpublished dev tool; its `criterion` dependency is isolated from the
published dependency graph and now guarded against regression. Supply-chain note: `criterion`
and its transitive deps are confined to the excluded `benchmarks/` crate (§12, §18).



## [0.22.0] - 2026-06-24

**`matten-data` promoted to Beta.** This release completes the documented Beta gate
(RFC-023 §9) for `matten-data` and flips its status label from Experimental to Beta.
It is an examples / documentation / tests / tooling release — there is **no change to
any library code, public API, or runtime behavior** in any crate. Maturity is a
per-crate status label under lock-step family versioning (RFC-030), so this is a new
minor family purely to communicate the visible maturity milestone.

### Added

- **`matten-data` example suite (RFC-036 §3).** Six numbered tutorial examples that
  teach the workflow one step at a time: `data_00_quickstart`, `data_01_schema_summary`,
  `data_02_select_columns`, `data_03_missing_values`, `data_04_to_tensor`, and
  `data_05_errors`. The existing `csv_to_tensor` is kept as the comprehensive overview
  (architect ruling: example Option 1).
- **Explicit malformed-CSV test** (`malformed_csv_is_a_structured_error_never_a_panic`),
  completing the §9 error-case coverage. `matten-data` is now 34 tests.
- **`docs/src/examples/data.md`** — a dedicated `matten-data` guide (purpose, install,
  quickstart, output `Tensor` shape, missing-value policy, numeric-conversion policy,
  limitations, status/maturity), wired into the mdBook summary.

### Changed

- **`matten-data` status: Experimental → Beta** across `crates/matten-data/README.md`,
  `crates/matten-data/src/lib.rs`, the root `README.md` crate table,
  `docs/src/examples/companions.md`, and `docs/src/reference/compatibility.md`.
- **Family bump `0.21` → `0.22`** for current-family labels and install pins across the
  user-facing docs (the internal `matten` path+version pin moved with it).
- **Release-docs guard** (`scripts/check-release-docs.sh`): `CURRENT_MINOR` `21` → `22`,
  plus a new check that current `matten-data` docs declare Beta and never Experimental
  (historical `rfcs/`, `CHANGELOG.md`, `ROADMAP.md` references remain allowed).
- **CI** (`.github/workflows/test.yaml`): the `matten-data` job now runs
  `cargo check -p matten-data --examples` and `cargo test -p matten-data` (RFC-036 §7),
  and the smoke job runs all six `data_*` examples.

### RFC lifecycle

- **RFC-036** (`matten-data` Examples, Documentation, and Release Gate) → Implemented (0.22.0).
- **RFC-023** (`matten-data` Scope and Non-goals) → Resolved (0.22.0): Outcome B selected
  (kept Experimental through the v0.21 family), then promoted to Beta once the gate was met.

### Threat model

No new runtime surface. The release adds examples, documentation, one test, and tooling/CI
only; no library code path changed. Note (surfaced from implementation): with the lenient
`csv` configuration (`flexible(true)`, `&str` input), the `csv` crate does **not** emit
parser errors for malformed input such as unterminated quotes — those resolve to a precise
structural `RaggedRow`, and header-structure problems surface as `Csv`. The new test
therefore asserts the real contract (malformed input is always a structured error, never a
panic or a silently-wrong table) rather than a parser-error variant that this configuration
cannot produce.



## [0.21.4] - 2026-06-24

**Release-truth and CI-gate patch (v0.21.3 deep-review P1 fixes). Documentation,
tooling, and examples only — no library code, API, or behavior change.**

The v0.21.3 deep review accepted the v0.21 implementation (no P0 blockers; Q1–Q5
developer choices confirmed, Q6 approved) and required a release-truth cleanup before
presenting v0.21 as a polished phase: several public-facing docs still said `0.20`
after the family bumped to `0.21`, the release-docs guard was not family-aware, and it
was not wired into CI.

### Fixed (P1 — release-truth)

- **0.20 → 0.21 documentation drift.** Corrected stale current-family labels and
  install snippets across `README.md`, all four crate READMEs, `crates/matten/src/lib.rs`,
  `docs/src/quick-start.md`, and `docs/src/reference/{boundary,dynamic,compatibility}.md`,
  `docs/src/introduction.md`, and `docs/src/contributing/architecture.md`
  (`0.20.x family` → `0.21.x family`; `= "0.20"` pins → `= "0.21"`).
- **Public-API snapshot** now reads "the current v0.21 family" — **family-only**, with
  no pinned patch version, to prevent the exact `(0.20.14)`-style drift that went stale.
- Reworded the introduction and compatibility "phase status" to state that the v0.21
  boundary batch is **delivered** (it previously read "planning" / "begins").
- Legitimate history was preserved (e.g. `matten-data` "available as of v0.20.1", and
  "the v0.20 family completed the materialization phase").

### Changed (P1/P2 — gates)

- **`check-release-docs.sh` is now current-family-aware.** A single `CURRENT_MINOR`
  value drives three checks that reject non-current install pins, `X.Y.x family`
  labels, and "current vX.Y family" prose — so a future minor bump catches the
  newly-previous family automatically. Full historical patch references (e.g.
  `v0.20.1`) and generic notation examples (e.g. `(0.13.x)`) are intentionally not
  matched.
- The retired "Phase 1 / Phase 2" wording scan now also covers
  `crates/matten/examples/`, and the four examples that still used it
  (`11_csv_numeric_loading`, `dynamic_01_mixed_elements`, `dynamic_05_dirty_csv_cleanup`)
  were updated to numeric-Tensor terminology.
- **`check-release-docs.sh` is wired into CI** (the main check job, after the
  dependency-boundary gate) per the Q6 ruling, so doc-truth, API-snapshot drift,
  retired wording, and examples naming are enforced on every push, not only at release.
- The release checklist now lists the matten-data scope guard and the release-docs
  guard alongside the core dependency-boundary gate.

### Notes

- **Deep-review confirmations (no change needed):** `norm` stays panic-only (Q1);
  `try_var_axis`/`try_std_axis` stay (Q2); the defensive empty-tensor guard stays (Q3);
  the dedicated `check-matten-data-scope.sh` stays (Q4); `composition.rs`/`linalg.rs`/
  `stats.rs` stay separate from `math.rs` (Q5).
- **Tracked future-optional (pre-v1.0, not required for v0.21):** a small additive
  "Result-form reduction consistency" RFC (`try_sum`/`try_mean`/`try_min`/`try_max`/
  `try_norm`) and a `try_*_axis` consistency RFC, to make the reduction families
  uniform if user demand appears.
- **Threat model:** unchanged (documentation, a CI/dev tooling script, and example
  comments; no runtime surface).
- With this patch, the v0.21 boundary-work batch is considered polished for public
  phase closure.

## [0.21.3] - 2026-06-24

**`matten-data` anti-scope guard (RFC-042). Tooling/docs only — no library code,
API, or behavior change. Completes the v0.21 boundary-work batch.**

`matten-data` may borrow named columns and table preparation, but it must not
become a dataframe library. This release makes that boundary mechanically
enforceable.

### Added

- **`scripts/check-matten-data-scope.sh`** — a three-check anti-scope guard
  (RFC-042 §8), wired into the `matten-data` CI job and the release checklist:
  1. **Example file-name guard** — rejects dataframe-story terms in
     `crates/matten-data/examples/` file names (e.g. `join_customers_orders.rs`,
     `pivot_monthly.rs`), matched as `_`-delimited tokens.
  2. **Public-API identifier guard** — rejects dataframe-shaped public definitions
     in `crates/matten-data/src` (`pub struct`/`enum`/`type DataFrame`/`Series`;
     `pub fn groupby`/`group_by`/`join`/`merge`/`pivot`/`query`/`loc`/`iloc`),
     matched as definitions, not arbitrary text.
  3. **README scope statement** — requires the `matten-data` README to state it is
     "not a dataframe library".

  The guard is deliberately **precise**: it does not body-scan for broad terms
  (`index`, `join`, `loc`, `query`), so `Path::join`, a loop variable named
  `index`, functions named `joined`/`join_tables`, and words like `location` all
  pass. Verified against every must-fail / must-not-fail case in RFC-042 §8.

### Changed

- `docs/src/contributing/release-checklist.md` now lists the matten-data scope
  guard and the release-docs guard alongside the core dependency-boundary gate.

### Notes

- No new public API, no library code change. The existing `matten-data` surface
  (`Table`, `SchemaSummary`, `select_columns`, `fill_missing`, `try_numeric`,
  `to_tensor`, …) already complies, and its README already carried the non-goal
  section — this release makes both enforceable.
- **Threat model:** unchanged (a CI/dev tooling script; no runtime surface).
- RFC-042 moved to `rfcs/done/` (Implemented, v0.21.3). With RFC-039/040/041/042 all
  shipped, the accepted v0.21 boundary-work batch is complete.

## [0.21.2] - 2026-06-24

**Statistics core (RFC-040): `var`/`std` + `var_axis`/`std_axis` added to core.
Additive — no breaking change.**

Third feature of the v0.21 boundary-work batch. Population variance only; `matten`
remains a family-car PoC library, not a statistics package.

### Added

- **`Tensor::var` / `try_var`, `Tensor::std` / `try_std`** (RFC-040) — **population**
  variance and standard deviation over all elements: `var = sum((x_i - mean)^2) / n`
  (`ddof = 0`, not sample variance), `std = sqrt(var)`. Two-pass algorithm; `NaN`
  propagates; a single-element tensor has variance `0.0`.
- **`Tensor::var_axis` / `try_var_axis`, `Tensor::std_axis` / `try_std_axis`**
  (RFC-040) — the same along one axis, removing it from the output shape (no
  `keepdims`): `[2, 3]` axis 0 → `[3]`, axis 1 → `[2]`. `NaN` propagates per slice.
- All in a new `stats.rs` module; `math.rs` is left untouched (kept under the
  300-ELOC split-consideration line).
- **`16_variance_std.rs`** example (core-tutorial band); new reference page
  `docs/src/reference/stats.md` with the population-variance and deferred-stats
  boundary; a statistics section in the public-API snapshot; a cross-reference from
  the math reference.

### Behavior

- **`var` / `std`** — panic on dynamic; `try_var` / `try_std` return `Unsupported`
  on dynamic. An empty-tensor guard returns `InvalidArgument`, but `matten` forbids
  zero-sized dimensions, so an empty tensor is not constructible and that branch is
  unreachable in practice (a test covers the construction rejection).
- **`var_axis` / `std_axis`** — panic if `axis >= rank` or dynamic; `try_*` forms
  return `Shape` on an out-of-range axis and `Unsupported` on dynamic.

### Notes

- **Out of scope for core** (RFC-040 §6/§8, unchanged): sample variance
  (`ddof = 1`), quantile, percentile, histogram, covariance, correlation, z-score,
  and `nanvar`/`nanstd`. These are deferred to a possible future `matten-stats`
  companion, which is not scaffolded (RFC-040 §9: only after ≥3 well-scoped APIs).
- **Threat model:** unchanged. Pure in-memory numeric reductions over owned `f64`
  data — no I/O, no parsing, no new dependency, no `unsafe`, no new allocation
  beyond the reduced-shape output.
- RFC-040 moved to `rfcs/done/` (Implemented, v0.21.2).

## [0.21.1] - 2026-06-24

**Linalg core-lite (RFC-041): `norm`, `trace`, and `outer` added to core.
Additive — no breaking change.**

Second feature of the v0.21 boundary-work batch. These are small linalg-adjacent
helpers; `matten` remains a family-car PoC library, not a linear algebra backend.

### Added

- **`Tensor::norm`** (RFC-041) — the L2 / Frobenius norm over **all** elements,
  `sqrt(sum(x_i^2))`. Works at any rank (Frobenius for matrices). `NaN` propagates.
  Panic-only on dynamic tensors, matching the other value reductions (`sum`,
  `mean`); no `try_norm` form.
- **`Tensor::trace` / `try_trace`** (RFC-041) — the diagonal sum of a **rank-2**
  tensor. Rectangular matrices are allowed: sums `self[i, i]` for
  `i in 0..min(rows, cols)`. `try_trace` returns `Shape` if not rank-2.
- **`Tensor::outer` / `try_outer`** (RFC-041) — the **rank-1 × rank-1** outer
  product, `out[i, j] = self[i] * other[j]`, shape `[m, n]`. The output is checked
  against `MattenLimits` before allocation. `try_outer` returns `Shape` if either
  input is not rank-1, or `Allocation` if oversized.
- All in a new `linalg.rs` module; `math.rs` is left untouched (kept under the
  300-ELOC split-consideration line).
- **`15_norm_trace_outer.rs`** example (core-tutorial band); new reference page
  `docs/src/reference/linalg.md` with the linalg-boundary statement; a linalg
  section in the public-API snapshot; a cross-reference from the math reference.

### Behavior

- **`norm`** — panics on dynamic input (convert with `try_numeric()` first).
- **`trace`** — non-rank-2 → `Shape` (`try_trace`) or panic (`trace`); dynamic →
  `Unsupported` (`try_trace`) or panic.
- **`outer`** — non-rank-1 → `Shape`; dynamic → `Unsupported`; oversized →
  `Allocation` (all via `try_outer`; `outer` panics with the same message).

### Notes

- **Out of scope for core** (RFC-041 §5, unchanged): `inverse`, `determinant`,
  `solve`, eigen-decomposition, SVD, QR, LU, Cholesky, sparse formats, and
  BLAS/LAPACK backends. Use `nalgebra` or `ndarray-linalg`; a future bridge crate
  would need its own RFC.
- **Threat model:** unchanged. Pure in-memory numeric operations on owned `f64`
  data — no I/O, no parsing, no new dependency, no `unsafe`. `outer`'s only new
  resource consideration (output size) is bounded by the existing `MattenLimits`
  check applied before allocation.
- RFC-041 moved to `rfcs/done/` (Implemented, v0.21.1).

## [0.21.0] - 2026-06-24

**Opens the v0.21 line. Shape composition (RFC-039): `concatenate` and `stack`
added to core. Additive — no breaking change.**

This is the first feature of the accepted v0.21 boundary-work batch (shape
composition, statistics, linear-algebra lite, and the `matten-data` scope guard
land across v0.21.0–v0.21.3). It opens a new minor line; subsequent additive
features in the batch land as patches.

### Added

- **`Tensor::concatenate` / `try_concatenate`** (RFC-039) — join tensors along an
  **existing** axis. All inputs must share the same rank and the same size on every
  non-concatenation axis; the output axis size is the sum of the inputs'. Valid
  axis range is `0..rank`.
- **`Tensor::stack` / `try_stack`** (RFC-039) — join identically shaped tensors
  along a **new** axis (output rank is input rank + 1; new axis size is the number
  of inputs). Valid axis range is `0..=rank`.
- All four are associated functions taking a borrowed slice `&[&Tensor]` (no
  cloning to pass inputs). The `try_*` forms return `Result`; the convenience forms
  panic with the same message.
- **`14_concatenate_stack.rs`** example (core-tutorial band) and a runnable
  walkthrough; new reference page `docs/src/reference/shape-composition.md`; a
  shape-composition section in the public-API snapshot.

### Behavior

- **Empty input list** → `InvalidArgument { argument: "tensors" }`.
- **Rank / dimension / shape mismatch, or out-of-range axis** → `Shape`.
- **Dynamic input** → `Unsupported` (convert with `try_numeric()` first); the
  `try_*` forms never panic on dynamic inputs.
- **Oversized result** → `Allocation` (the output shape is checked against
  `MattenLimits` before any data is copied), or `Shape` when the stacked rank would
  exceed the dimension limit.
- A single-input `concatenate` returns a clone; a single-input `stack` inserts a
  length-1 axis. Validation (axis, dynamic, allocation) still runs for `n = 1`.

### Notes

- `repeat`, `tile`, and `meshgrid` remain **deferred** (RFC-039 §8): they need a
  separate indexing/allocation policy and are not part of this release.
- **Threat model:** unchanged. These are pure in-memory numeric operations on
  owned `f64` data with no I/O, no parsing, no new dependency, and no `unsafe`.
  The only new resource consideration — output allocation size — is bounded by the
  existing `MattenLimits` check applied before allocation.
- RFC-039 moved to `rfcs/done/` (Implemented, v0.21.0).

## [0.20.19] - 2026-06-24

**Examples reorganization: new `50_`–`56_` practical-recipes band, two fossils
retired, naming-band guard added. No public API, behavior, or dependency change.**

Applied the architect's examples-reorganization ruling (flat layout, option C for
the unnumbered skill-demos, retire two fossils).

### Added

- **`50_`–`56_` practical numeric recipes band:** the seven previously unnumbered
  skill-demo examples are renamed into a coherent new band:

  | New name | Old name |
  |---|---|
  | `50_rowwise_scoring.rs` | `rowwise_scoring.rs` |
  | `51_standardize_columns.rs` | `standardize_columns.rs` |
  | `52_minmax_scaling.rs` | `minmax_scaling.rs` |
  | `53_gram_matrix.rs` | `gram_matrix.rs` |
  | `54_pairwise_distance.rs` | `pairwise_distance.rs` |
  | `55_moving_average.rs` | `moving_average.rs` |
  | `56_rolling_windows_basic.rs` | `rolling_windows_basic.rs` |

- **`docs/src/examples/practical-recipes.md`:** new docs page for the `50_`–`56_`
  band with descriptions and source links for all seven examples.
- **Naming-band guard** in `scripts/check-release-docs.sh`: fails if any
  `crates/matten/examples/*.rs` file does not follow the `NN_` or `dynamic_NN_`
  prefix convention; prevents future fossil accumulation.

### Removed

- **`hello_tensor.rs`** — skeleton-era fossil ("M0 skeleton" docstring); fully
  superseded by `00_quickstart.rs`.
- **`column_summary.rs`** — duplicated `28_column_statistics.rs` without adding
  a distinct teaching goal; `28_` is the better-documented version.

### Changed

- `Run:` lines in the seven renamed files updated to match their new example names.
- Stale `"Phase 2 dynamic feature quickstart"` doc-comment in
  `dynamic_00_quickstart.rs` corrected to `"Dynamic feature quickstart"`.
- CI smoke runs updated: old unnumbered names removed, `50_`–`56_` runs added.
- `docs/src/examples/index.md`: "Pattern examples" section replaced with the
  `50_`–`56_` band table (with source links); cross-reference to `pairwise_distance`
  updated to `54_pairwise_distance`.
- `docs/src/examples/beginner-math.md`: cross-reference updated to
  `54_pairwise_distance.rs`.
- `docs/src/SUMMARY.md`: "Practical numeric recipes" page added.

### Notes

- No subdirectories introduced (Cargo auto-discovery benefit preserved).
- The `14_`–`19_` gap remains reserved for future core tutorial additions
  (v0.21 API examples). The `41_`–`49_` gap remains for optional famous-problem
  extensions. The `50_`–`56_` band has room to grow.
- No public API, behavior, data flow, integration, or auth change. Threat model
  unchanged.

## [0.20.18] - 2026-06-23

**Build/repository hygiene and documentation fixes. No code, API, behavior, or
dependency change to any crate.**

The RFC-031 regression fixture (`tests/fixtures/dynamic_rejection_unification`) is a
standalone crate, intentionally excluded from the workspace so it can simulate a
downstream consumer's feature set (core `matten/dynamic` ON, companion mirrors OFF) in
isolation from workspace feature unification. Being excluded, it generates its own
`Cargo.lock`, which was appearing as a second tracked lock.

### Changed

- Added `/tests/fixtures/*/Cargo.lock` to `.gitignore`. The fixture's lock is regenerated
  on demand and is no longer tracked, so the repository keeps a single workspace
  `Cargo.lock`. The fixture remains excluded from the workspace (its isolation is
  required for the RFC-031 regression to hold).
- Clarified the root `Cargo.toml` exclusion comment to state the real reason for the
  exclusion (feature-unification isolation) and how the fixture's lock is handled.
- Release tarballs now also exclude the fixture's `Cargo.lock`.

### Documentation

- Root `README.md`: the documentation link now points to the published mdBook at
  <https://nabbisen.github.io/matten/> instead of the local `docs/` path (GitHub Pages
  is live).
- Examples docs: each example now links to its source file on GitHub in addition to the
  "how to run" command, so readers can see *what to write*, not just how to run it. The
  `30_`–`40_` band pages add a per-example `Source:` link; the companion page links each
  example identifier in its table to the corresponding source file.
- Retired four hyphenated `Phase-1` references in the example band pages that the
  v0.20.17 cleanup missed (it matched only the spaced form `Phase 1`). The
  `check-release-docs.sh` Phase guard now matches both the spaced and hyphenated forms
  (`Phase[ -]1` / `Phase[ -]2`).

### Notes

- No runtime behavior, data flows, integrations, or auth change — repository
  configuration only. The workspace still loads with the same four members; the RFC-031
  fixture still passes. Threat model unchanged.
- The fixture's `Cargo.lock` is newly git-ignored but was previously tracked; in a git
  checkout it must be untracked once with `git rm --cached
  tests/fixtures/dynamic_rejection_unification/Cargo.lock`.

## [0.20.17] - 2026-06-23

**Documentation / RFC-lifecycle housekeeping, applying the pre-v0.19.0 audit architect
rulings (Q1–Q4). No runtime code, API, behavior, or dependency change to any crate.**

The architect accepted the pre-v0.19.0 reflection audit (no correctness defect) and ruled
on the four raised questions. This release applies those rulings.

### Changed (Q1 — terminology, P1)

- Retired the "Phase 1 / Phase 2" vocabulary from all user-facing docs (48 occurrences
  across 14 files: root and crate READMEs, `crates/matten/src/lib.rs`, and `docs/src`),
  replacing it with stable domain terminology — "numeric Tensor / numeric core" for the
  static path and "dynamic ingestion / dynamic on-ramp / dynamic Tensor" for the
  `dynamic` path. The `(Phase 1 only)` labels were removed from the public-API snapshot's
  core method tables. Historical RFCs (`rfcs/`) and `CHANGELOG.md` intentionally retain
  the original wording.
- Hardened `scripts/check-release-docs.sh` to fail on "Phase 1 / Phase 2" in user-facing
  docs (scoped to README/crate-READMEs/`lib.rs`/`docs/src`; `rfcs/` and `CHANGELOG.md`
  allowed). Verified to pass on the cleaned tree and to catch reintroduction.

### Changed (Q2 / Q3 / Q4 — RFC lifecycle notes)

- **RFC-013** (Q2): added a lifecycle note recording that the broad property/fuzz testing
  strategy is partially aspirational — the shipped release gates are unit/integration/
  golden/example/feature/release-doc checks (374 tests); property and fuzz testing remain
  selective future-hardening candidates, not current gates. A future "Testing Strategy
  Refresh" (candidate RFC-050, after RFC-049) is tracked in the ROADMAP.
- **RFC-014 ↔ RFC-043** (Q3): added cross-references — RFC-043 is the current
  examples-program authority (v0.20+, the `30_`–`40_` band, future conventions); RFC-014
  remains the historical authority for the original `00_`–`28_` suite.
- **RFC-012** (Q4): added a clarification that the internal `Arc`-shared dynamic storage /
  CoW mechanics are implemented, while the public mutation API exposing CoW is
  intentionally deferred. The `compatibility.md` note was reworded accordingly (this also
  removed its "Phase 2" reference under Q1).

### Notes

- No runtime behavior, data flows, integrations, or auth change — documentation, RFC
  lifecycle notes, the ROADMAP, and one CI guard script only. Threat model unchanged.
- The separately-deferred Patch C from the v0.20.14 deep review (RFC-023/026/036/037
  lifecycle clarification) remains pending its own ruling and is not addressed here.

## [0.20.16] - 2026-06-23

**Audit fix: public-API snapshot completeness. No runtime code, API, behavior, or
dependency change.**

A four-dimension audit of the project since v0.19.0 (codebase↔RFCs, tests↔requirements/
external-design, codebase↔tests, docs↔codebase) found the codebase, tests, and
documentation consistent, with one documentation gap:

### Fixed

- **`docs/src/reference/public-api-snapshot.md`:** the `Element` (`dynamic`) section
  listed "Key methods" with only three of its six public methods. It now lists all
  public methods (`try_as_f64`, `is_numeric`, `is_none`, `as_text`, `as_bool`, and the
  `text(s)` constructor), consistent with the page's "every public item" contract. The
  `#[doc(hidden)]` slice plumbing (`SliceSpecRepr`, `IntoSliceRange`, `SliceConvert`)
  remains correctly excluded as non-public API.

### Notes

- No runtime behavior, data flows, integrations, or auth change — one documentation
  page only. Threat model unchanged. Full audit findings are recorded in the v0.20.16
  audit report.

## [0.20.15] - 2026-06-23

**Documentation release-truth cleanup and release-doc guard hardening, applying the
v0.20.14 codebase deep review. No runtime code, API, behavior, or dependency change to
any crate.**

The architect accepted the v0.20 series handoff (phase closed) and deep-reviewed the
v0.20.14 source. No P0 runtime/design blocker was found. The review's P1 findings were
documentation/release-truth drift (stale version strings and old-phase wording in
user-facing docs) plus a request to harden the release-doc guard so the drift cannot
recur. This release applies those findings before v0.21 work begins.

### Fixed (documentation truth — Patch A)

- Replaced stale `0.15` / `0.19` dependency-snippet versions with `0.20` across
  user-facing docs: `crates/matten/README.md`, `crates/matten/src/lib.rs`,
  `crates/matten-ndarray/README.md`, `crates/matten-mlprep/README.md`,
  `docs/src/quick-start.md`, `docs/src/reference/boundary.md`,
  `docs/src/reference/dynamic.md`, and `docs/src/contributing/architecture.md` (the
  last not in the review's evidence list; found by re-verifying against source).
- Updated companion maturity labels to the current family: `matten-ndarray`
  "Production-ready candidate (`0.20.x` family)", `matten-mlprep` "Beta (`0.20.x`
  family)"; made the `matten-ndarray` crate-doc family line version-neutral; corrected
  two further stale `0.19` family references surfaced by the new guard.
- **Root `README.md` crate table:** version cells now read `0.20.x family` (no
  per-patch churn), and a **`matten-data` row was added** (it had badges but no table
  entry).
- **`docs/src/reference/public-api-snapshot.md`:** header now states the current v0.20
  family instead of `v0.15.x`; the `MattenError` enum block now includes the shipped
  `InvalidArgument` variant; the `try_reshape` row now states it returns `Unsupported`
  on dynamic (it does not panic).
- **`crates/matten-data/README.md`:** removed pre-API "(when added)" / "When the public
  API lands" wording — the table/CSV→tensor API shipped in v0.20.1.
- **`docs/src/introduction.md`:** replaced the "M0 skeleton" line with current
  0.20-family / v0.21-boundary wording.
- **`docs/src/reference/operators.md`** (P2): `matmul` is described as implemented
  (`matmul` / `dot`), not "coming in a later milestone".
- **`docs/src/reference/compatibility.md`** (P2): added a short phase-status note
  (v0.20 materialization complete; v0.21 begins boundary implementation).

### Changed (release-doc guard — Patch B)

- Hardened `scripts/check-release-docs.sh` with documentation release-truth checks:
  stale `0.15`/`0.19` version references in user-facing docs, skeleton-era / pre-API
  wording, presence of `InvalidArgument` in the public-API snapshot, and bare
  per-patch versions in the root README crate table. The checks are scoped to
  user-facing docs; `CHANGELOG.md`, `ROADMAP.md`, and `rfcs/` are excluded as the
  curated historical-content allowlist. (Verified to catch planted drift and to pass
  on the cleaned tree.)

### Deferred

- **Patch C (P2) — RFC lifecycle clarification** for RFC-023 / RFC-026 / RFC-036 /
  RFC-037 against the shipped RFC-033/034/035 and accepted RFC-042. The architect
  marked this a non-blocker requiring per-RFC supersession judgment; it is deferred to
  v0.21 planning rather than resolved with premature calls here. Tracked in the ROADMAP
  history.

### Notes

- No runtime behavior, data flows, integrations, or auth change — documentation,
  ROADMAP, and one CI guard script only. `#![forbid(unsafe_code)]`, the core→companion
  boundary, and the dynamic-rejection guards are unchanged; the threat model is
  unchanged.

## [0.20.14] - 2026-06-23

**Planning/reconciliation: v0.21 boundary architect rulings ingested (RFC-039–042).
No code, API, behavior, or dependency change to any crate.**

The architect reviewed the v0.21 boundary decisions and accepted all 13 questions,
adding implementation constraints. Those rulings are now recorded so v0.21
implementation can proceed.

### Changed

- **RFC-039/040/041/042 marked accepted-for-implementation.** Each RFC's Status is
  updated and an "Architect Rulings — v0.21 Boundary Review" section records the
  accepted decisions and added constraints. The RFCs remain in `rfcs/proposed/` per
  the 4-folder lifecycle (RFC-000) until each ships, then they move to `rfcs/done/`.
  Key rulings:
  - **RFC-039** → v0.21.0: `concatenate` + `stack` in core (borrowed `&[&Tensor]`,
    `try_`/panic pairs, `MattenLimits`, dynamic-reject). Empty list →
    `InvalidArgument`; structural/axis errors → `Shape`; `stack` axis `0..=rank`,
    `concatenate` axis `0..<rank`; `n=1` allowed but still fully validated.
    repeat/tile/meshgrid deferred.
  - **RFC-041** → v0.21.1: `norm` (L2/Frobenius over all elements), `trace` (rank-2,
    rectangular via `min(rows,cols)`), `outer` (rank-1×rank-1). Decomposition/BLAS/
    sparse rejected from core.
  - **RFC-040** → v0.21.2: `var`/`std` + `var_axis`/`std_axis`, population variance
    (ddof=0), two-pass algorithm, NaN-propagating; quantile/histogram/cov/corr
    deferred; no `matten-stats` companion yet.
  - **RFC-042** → v0.21.3 (or earlier): keep standalone; three-check release-docs
    guard authorized.
- **ROADMAP** updated (document version 1.8.0): boundary RFC rows marked accepted with
  v0.21.x targets, and the accepted v0.21.0–.3 sequence added to the release-theme
  table.

### Notes

- No data flows, integrations, auth, or runtime behavior change — this release only
  records design rulings in planning documents. `#![forbid(unsafe_code)]`, the
  core→companion boundary, and the release-doc guards remain valid; the threat model
  is unchanged. Per-RFC threat-model review will accompany each v0.21.x implementation.

## [0.20.13] - 2026-06-23

**Examples: deferred numerical methods (RFC-046). This completes the RFC-043–048
example program. Additive examples and documentation under lock-step family
versioning; no API, behavior, or breaking change to any crate.**

These two examples were deferred until the RFC-038 comfort APIs landed; with
`linspace` shipped (v0.20.11), they are now implementable.

### Added

- **Numerical-methods examples** (RFC-046 §5–6), default Phase-1 numeric API plus the
  RFC-038 comfort APIs, small hard-coded inputs, deterministic self-checking output,
  each explicitly marked as a numerical approximation:
  - `examples/39_finite_difference_derivative.rs` — central-difference derivative of
    `x³` on a `linspace` grid (function values via elementwise `&x * &x`); for a cubic
    the error is exactly `h²`, shown directly.
  - `examples/40_trapezoidal_integration.rs` — composite trapezoidal rule for
    `∫₀¹ x² dx` (grid via `linspace`, values via elementwise squaring, total via a
    `Tensor::sum` reduction), compared against the exact value `1/3`.
  - CI smoke runs for both; write-ups added to `docs/src/examples/numerical-methods.md`
    and the index table.

### Changed

- **RFC-043–048 closed.** This finishes the additive `30+` example band (`30_`–`40_`).
  All six example-program RFCs are moved to `rfcs/done/` with shipped-version
  annotations, the index is updated, and ROADMAP Track C is marked complete (document
  version 1.7.0). The optional `41_adjacency_walks_extended` remains a not-reserved
  conditional candidate.

### Notes

- No new data flows, external integrations, or auth: examples use only in-memory
  hard-coded data. `#![forbid(unsafe_code)]`, the core→companion dependency boundary,
  and the release-doc guards remain valid; the threat model is unchanged.

## [0.20.12] - 2026-06-23

**Core numeric comfort APIs — shape band (RFC-038, final sub-band). This completes
RFC-038. Additive, non-breaking public API under lock-step family versioning.**

With this band, all four parts of RFC-038 are shipped: elementwise (v0.20.9),
selection (v0.20.10), creation (v0.20.11), and shape (this release). RFC-038 is moved
to `rfcs/done/` and Track B is marked complete in the ROADMAP.

### Added

- **Shape comfort ops on `Tensor`** (RFC-038 §4.5–4.6), added alongside the existing
  shape operations in `tensor/ops.rs` (`reshape`/`flatten`/`transpose`/`swap_axes`),
  which is the close-fit module per RFC-038 §5.3:
  - `squeeze()` — removes every length-1 axis (an all-ones shape such as `[1, 1]`
    becomes a scalar; a scalar stays a scalar). No failure mode, so no `try_` form.
  - `expand_dims(axis)` / `try_expand_dims(axis)` — inserts a length-1 axis at
    `axis`, valid over `0..=ndim` (inserting at `ndim` appends a trailing axis).
  - Both preserve data order. `expand_dims` with `axis > ndim` panics; the `try_`
    form returns `MattenError::InvalidArgument`. On a dynamic tensor the convenience
    forms panic and `try_expand_dims` returns `MattenError::Unsupported` — consistent
    with the other shape ops.

### Changed

- Reference docs updated to match: `shape-ops.md` (new section),
  `public-api-snapshot.md` (shape-operations rows + dynamic-behavior row).
- RFC-038 closed: moved `rfcs/proposed/038` → `rfcs/done/038` (Status: Implemented),
  index updated, ROADMAP Track B marked complete (document version 1.6.0).

### Notes

- No new data flows, external integrations, or auth: pure in-memory shape
  reinterpretation with data cloned into a fresh buffer. `#![forbid(unsafe_code)]`,
  the core→companion dependency boundary, and the release-doc guards remain valid; the
  threat model is unchanged.

## [0.20.11] - 2026-06-23

**Core numeric comfort APIs — creation band (RFC-038, third sub-band). Additive,
non-breaking public API under lock-step family versioning.**

Continues RFC-038 after the elementwise (v0.20.9) and selection (v0.20.10) bands. The
shape band (`squeeze`/`expand_dims`) remains.

### Added

- **Creation comfort constructors on `Tensor`** (RFC-038 §4.1–4.2), in a new
  top-level `creation.rs` module (per RFC-038 §5.3):
  - `linspace(start, end, count)` / `try_linspace(...)` — `count` evenly spaced
    values inclusive of both endpoints when `count >= 2`; `[start]` when
    `count == 1`. Endpoints are pinned exactly to avoid floating-point drift.
  - `eye(n)` / `try_eye(n)` — the `n × n` identity matrix.
  - Both are budget-checked through `MattenLimits`: a zero-sized result
    (`count == 0`, `n == 0`) is rejected with `MattenError::Shape`, and an oversized
    result with `MattenError::Allocation`. Convenience forms panic on those; `try_*`
    forms return them. Per the RFC's minimal-surface inclusion rule, only the two
    forms each are exposed (no `_with_limits` variants).

### Changed

- Reference docs updated to match: `construction.md` (new section) and
  `public-api-snapshot.md` (new rows).

### Notes

- With `linspace` available alongside the `sqrt` shipped in v0.20.9, the deferred
  `39_finite_difference_derivative` and `40_trapezoidal_integration` examples are now
  unblocked (a separate follow-up).
- No new data flows, external integrations, or auth: pure in-memory construction with
  existing budget enforcement. `#![forbid(unsafe_code)]`, the core→companion
  dependency boundary, and the release-doc guards remain valid; the threat model is
  unchanged.

## [0.20.10] - 2026-06-23

**Core numeric comfort APIs — selection band (RFC-038, second sub-band). Additive,
non-breaking public API under lock-step family versioning.**

Continues RFC-038 after the elementwise band (v0.20.9). Remaining bands (shape
`squeeze`/`expand_dims`, creation `linspace`/`eye`) follow as separate releases.

### Added

- **Index reductions on `Tensor`** (RFC-038 §4.4), in a new top-level `selection.rs`
  module (placed beside the reductions in `math.rs`, which sits at the 300-ELOC
  threshold, per RFC-038 §5.3):
  - `argmin()` / `argmax()` — flat row-major index of the smallest/largest element,
    first occurrence on ties;
  - `try_argmin()` / `try_argmax()` — non-panicking `Result` forms.
  - NaN policy (RFC-038 §5.1, selection branch): because an index is ill-defined when
    any element is `NaN`, the `try_*` forms return `MattenError::InvalidArgument` and
    the convenience forms panic — distinct from value reductions (`min`/`max`), which
    propagate `NaN`. On a dynamic tensor the `try_*` forms return
    `MattenError::Unsupported`; the convenience forms panic.

### Changed

- **Examples** `37_kmeans_small` and `38_nearest_neighbor_classification` now use
  `Tensor::argmin` for nearest-point selection, retiring their hand-rolled local
  argmin helpers. Output is unchanged.
- Reference docs updated to match: `math.md` (index-reductions section + NaN-policy
  row), `public-api-snapshot.md` (new rows), and `examples/ml-like.md`.

### Notes

- Reuses the `MattenError::InvalidArgument` variant introduced in v0.20.9.
- No new data flows, external integrations, or auth: pure in-memory numeric selection.
  `#![forbid(unsafe_code)]`, the core→companion dependency boundary, and the
  release-doc guards remain valid; the threat model is unchanged.

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
