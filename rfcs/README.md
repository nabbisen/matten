# `matten` RFCs

Design decisions for `matten` are tracked here using the lifecycle policy in
[RFC-000](./done/000-rfc-lifecycle-policy.md). The folder is the source of truth
for RFC state; the Status field inside each file mirrors the folder.

The project uses the **5-folder variant** (RFC-092): `proposed/` → `accepted/` →
`done/`, because owner sign-off and implementation are separate events performed by
different parties. See [`accepted/README.md`](./accepted/README.md). `archive/` and
`draft/` exists in the policy but is unused here; `archive/` holds withdrawn or superseded RFCs — see
[`archive/README.md`](./archive/README.md).

The broader documentation ownership model is recorded in
[`docs/design/README.md`](../docs/design/README.md): RFCs are canonical normative decisions,
`docs/src/` is the evergreen user contract, `ROADMAP.md` owns schedule/history, and
`docs/design/history/` contains historical snapshots only.

## Done

| ID | Title | Shipped in |
|---:|---|---|
| 000 | [RFC Lifecycle Policy](./done/000-rfc-lifecycle-policy.md) | 0.0.1 |
| 001 | [Threat Model and Boundary Safety Policy](./done/001-threat-model-and-boundary-safety-policy.md) | 0.9.0 |
| 002 | [Public API Minimalism and `Tensor` Contract](./done/002-public-api-minimalism-and-tensor-contract.md) | 0.1.0 |
| 003 | [Shape Model, Scalar Semantics, and Validation](./done/003-shape-model-scalar-semantics-and-validation.md) | 0.1.0 |
| 004 | [Construction and Conversion APIs](./done/004-construction-and-conversion-apis.md) | 0.2.0 |
| 005 | [Error Model, Panic Messages, and Boundary APIs](./done/005-error-model-panic-messages-and-boundary-apis.md) | 0.1.0 |
| 006 | [Broadcasting and Element-Wise Operators](./done/006-broadcasting-and-element-wise-operators.md) | 0.3.0 |
| 007 | [Reshape, Axis Operations, and Indexing](./done/007-reshape-axis-operations-and-indexing.md) | 0.4.0 |
| 008 | [Slicing API: Builder and `slice_str`](./done/008-slicing-api-builder-and-slice-str.md) | 0.4.0 |
| 009 | [Serde, JSON, CSV, and Boundary Integration](./done/009-serde-json-csv-and-boundary-integration.md) | 0.5.0 |
| 010 | [Reductions, Basic Statistics, and Matrix Multiplication](./done/010-reductions-basic-statistics-and-matrix-multiplication.md) | 0.7.0 |
| 011 | [Dynamic `Element` Model and Coercion](./done/011-dynamic-element-model-and-coercion.md) | 0.8.0 |
| 012 | [Dynamic Storage, View Metadata, and Copy-on-Write](./done/012-dynamic-storage-view-metadata-and-cow.md) | 0.8.0 |
| 013 | [Testing, Compatibility, and Release Gates](./done/013-testing-compatibility-and-release-gates.md) | 0.6.0 |
| 014 | [Example Suite and Executable Documentation](./done/014-example-suite-and-executable-documentation.md) | 0.6.0 |
| 015 | [Public API Stabilization and Compatibility Policy](./done/015-public-api-stabilization-and-compatibility-policy.md) | 0.13.3 |
| 016 | [Dynamic Ingestion and Explicit Numeric On-Ramp](./done/016-dynamic-ingestion-and-explicit-numeric-on-ramp.md) | 0.14.0 |
| 017 | [Numeric Conversion Policy](./done/017-numeric-conversion-policy.md) | 0.14.0 |
| 018 | [Shape, Allocation, and Resource Safety Limits](./done/018-shape-allocation-and-resource-safety-limits.md) | 0.14.0 |
| 019 | [Axis Reductions and Small Matrix Statistics](./done/019-axis-reductions-and-small-matrix-statistics.md) | 0.14.0 (core); 0.15.0 (examples) |
| 020 | [Human-Readable Diagnostics and Error Message Quality](./done/020-human-readable-diagnostics-and-error-message-quality.md) | 0.13.3 |
| 021 | [Tutorial Path and Example Quality Gate](./done/021-tutorial-path-and-example-quality-gate.md) | 0.15.0 |
| 022 | [Companion Crate Boundary Policy](./done/022-companion-crate-boundary-policy.md) | 0.16.0 |
| 023 | [`matten-data` Scope and Non-goals](./done/023-matten-data-scope-and-non-goals.md) | 0.22.0 (resolved: Outcome B → Beta) |
| 024 | [`matten-mlprep` Scope and Non-goals](./done/024-matten-mlprep-scope-and-non-goals.md) | 0.18.0 (impl RFC-028; maturity RFC-029) |
| 025 | [Bridge Crate Policy for ndarray, nalgebra, and candle](./done/025-bridge-crate-policy-for-ndarray-nalgebra-and-candle.md) | 0.17.0 (matten-ndarray; nalgebra/candle deferred) |
| 026 | [Large CSV and Streaming Data Policy](./done/026-large-csv-and-streaming-data-policy.md) | superseded by RFC-037; retained as historical policy record |
| 027 | [`matten-ndarray` Design and Implementation](./done/027-matten-ndarray-design-and-implementation.md) | 0.17.0 |
| 028 | [`matten-mlprep` Design and Implementation](./done/028-matten-mlprep-design-and-implementation.md) | 0.18.0 |
| 029 | [Companion Maturity Evaluation (v0.19.0)](./done/029-companion-maturity-evaluation-v0-19.md) | 0.19.0 |
| 030 | [Workspace Versioning Model — Lock-step Family Versioning](./done/030-workspace-versioning-model-lockstep.md) | 0.19.0 |
| 031 | [Feature-Robust Dynamic Rejection and Unconditional `Tensor::is_dynamic()`](./done/031-feature-robust-dynamic-rejection.md) | 0.19.1 |
| 032 | [Companion Dependency and Import Convention](./done/032-companion-dependency-and-import-convention.md) | 0.19.2 |
| 033 | [`matten-data` Beta-Decision and Scope Lock](./done/033-matten-data-beta-decision-and-scope-lock.md) | 0.20.0 (experimental scaffold; beta deferred to v0.21+) |
| 034 | [`matten-data` Table Model and Public API Boundary](./done/034-matten-data-table-model-and-public-api-boundary.md) | 0.20.1 |
| 035 | [CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion](./done/035-csv-ingestion-schema-summary-missing-values-and-numeric-conversion.md) | 0.20.1 |
| 036 | [`matten-data` Examples, Documentation, and Release Gate](./done/036-matten-data-examples-documentation-and-release-gate.md) | 0.22.0 |
| 037 | [Deferred Streaming and Large CSV Policy](./done/037-deferred-streaming-and-large-csv-policy.md) | resolved deferral policy; streaming requires future implementation RFC |
| 038 | [Core Numeric Comfort APIs](./done/038-core-numeric-comfort-apis.md) | 0.20.9 (elementwise); 0.20.10 (selection); 0.20.11 (creation); 0.20.12 (shape) |
| 039 | [Shape Composition API Boundary](./done/039-shape-composition-api-boundary.md) | 0.21.0 (`concatenate` + `stack`; `repeat`/`tile`/`meshgrid` deferred) |
| 040 | [Small Statistics Boundary — Core vs Companion](./done/040-small-statistics-boundary-core-vs-companion.md) | 0.21.2 (`var`/`std` + `var_axis`/`std_axis`, population; quantile/histogram/cov/corr deferred) |
| 041 | [Linear Algebra Boundary — Core Lite vs External Crates](./done/041-linear-algebra-boundary-core-lite-vs-external-crates.md) | 0.21.1 (`norm` + `trace` + `outer`; decomposition/BLAS/sparse rejected) |
| 042 | [Pandas-Inspired Scope Guard for `matten-data`](./done/042-pandas-inspired-scope-guard-for-matten-data.md) | 0.21.3 (three-check anti-scope guard; CI-enforced) |
| 043 | [Example Program Structure, Quality Gate, and Documentation Policy](./done/043-example-program-structure-quality-gate-and-documentation-policy.md) | 0.20.3 |
| 044 | [Beginner Core Math Examples](./done/044-beginner-core-math-examples.md) | 0.20.3 (examples 30–32) |
| 045 | [Matrix Iteration and Graph/Probability Examples](./done/045-matrix-iteration-and-graph-probability-examples.md) | 0.20.4 (examples 33–34) |
| 046 | [Numerical Methods and Scientific Toy Examples](./done/046-numerical-methods-and-scientific-toy-examples.md) | 0.20.7 (35–36); 0.20.13 (39–40) |
| 047 | [Small ML-Like Examples Without ML-Framework Scope](./done/047-small-ml-like-examples-without-ml-framework-scope.md) | 0.20.8 (examples 37–38) |
| 048 | [Companion-Crate Examples](./done/048-companion-crate-examples.md) | 0.20.6 |
| 049 | [Benchmarking, Complexity Metrics, and Positioning Report](./done/049-benchmarking-complexity-metrics-and-positioning-report.md) | Phases 1-3 implemented; Phase 4 hard gates extracted to future RFC/release-policy ownership |
| 050 | [Production Migration Guide and Bridge Strategy](./done/050-production-migration-guide-and-bridge-strategy.md) | 0.23.0 |
| 051 | [Bridge Conversion Contracts and Companion-Crate Policy](./done/051-bridge-conversion-contracts-and-companion-crate-policy.md) | 0.23.2 |
| 052 | [Production Target Playbooks](./done/052-production-target-playbooks.md) | 0.23.0–0.23.1 |
| 053 | [Migration Readiness Diagnostics and Report Format](./done/053-migration-readiness-diagnostics-and-report-format.md) | 0.23.4 |
| 054 | [`matten-migrate` Assisted Migration Tool](./done/054-matten-migrate-assisted-migration-tool.md) | local advisory tool scope implemented; rewrite/apply and public crate extracted to future RFC/release-policy ownership |
| 055 | [Result-Form Scalar Reductions (`try_sum`/`try_mean`/`try_min`/`try_max`/`try_norm`)](./done/055-result-form-scalar-reductions.md) | 0.24.0 |
| 056 | [Result-Form Axis Reductions (`try_sum_axis`/`try_mean_axis`/`try_min_axis`/`try_max_axis`)](./done/056-result-form-axis-reductions.md) | 0.24.0 |
| 057 | [Promote `matten-ndarray` — Production-Ready Candidate → Production-Ready](./done/057-promote-matten-ndarray-production-ready.md) | 0.25.0 |
| 058 | [Promote `matten-mlprep` — Beta → Production-Ready Candidate](./done/058-promote-matten-mlprep-production-ready-candidate.md) | 0.26.0 |
| 059 | [`matten-data` Maturity Decision — Beta → Production-Ready Candidate](./done/059-promote-matten-data-production-ready-candidate.md) | 0.27.0 |
| 060 | [Surface Benchmark Evidence in the Rendered Documentation](./done/060-surface-benchmark-evidence-in-docs.md) | 0.27.1 |
| 061 | [Maturity-Label Clarity — Keep "Production-Ready", Add an Entrance Note](./done/061-maturity-label-clarity-entrance-note.md) | 0.27.1 |
| 062 | [`matten-ndarray` Supported `ndarray` Version — 0.16 → 0.17](./done/062-matten-ndarray-supported-ndarray-version.md) | 0.28.0 |
| 063 | [Visual Understanding and Reporting](./done/063-visual-understanding-and-reporting.md) | 0.29.0 (visual docs, examples, local `matten-report` tool; public report/viz crates deferred) |
| 064 | [Workspace Core Dependency Requirement Maintenance Policy](./done/064-workspace-core-dependency-requirement-maintenance-policy.md) | post-0.29.0 repository policy |
| 065 | [Educational Visualization and Tensor Learning Path](./done/065-educational-visualization-and-tensor-learning-path.md) | 0.30.0 (educational positioning, learner docs path, local `educational-path` report; public report/viz crates deferred) |
| 066 | [v1.0 Readiness Audit and Release Decision Gate](./done/066-v1-readiness-audit-and-release-decision-gate.md) | 0.31.0 (readiness audit and release-decision gate; no v1.0 release authorization) |
| 067 | [v1.0 Family Maturity Policy](./done/067-v1-family-maturity-policy.md) | repository policy; resolves RFC-066 MD-1 without v1.0 release authorization |
| 068 | [Rich Local Visualization Artifacts](./done/068-rich-local-visualization-artifacts.md) | 0.32.0 (educational-path and shape-flow HTML); 0.33.0 (dynamic-readiness HTML); 0.34.0 (mlprep-standardization HTML); 0.35.0 (data-readiness HTML; fixed-demo HTML line complete; input-mode HTML and public report/viz crates deferred) |
| 069 | [Input-Mode HTML Report Policy](./done/069-input-mode-html-report-policy.md) | 0.36.0 (`tools/matten-report` data-readiness input-mode HTML; public report/viz crates deferred) |
| 070 | [Public Visualization and Report Readiness Audit](./done/070-public-visualization-report-readiness-audit.md) | post-0.37 audit decision; closed without public implementation |
| 071 | [Private Fixed-Demo JSON Report Artifacts](./done/071-private-fixed-demo-json-report-artifacts.md) | 0.37.0 (`tools/matten-report` fixed-demo private JSON; public schema/crates deferred) |
| 072 | [Behavior-Preserving `matten-report` Modularization](./done/072-matten-report-modularization.md) | post-0.37 internal modularization and structural closure; no behavior or public-surface change |
| 073 | [Private Input-Mode JSON Report Policy](./done/073-private-input-mode-json-report-policy.md) | 0.38.0 |
| 074 | [v1.0 Readiness Re-Audit](./done/074-v1-readiness-reaudit.md) | post-0.38 audit; accepted, conditionally-ready verdict; no v1.0 release authorized |
| 075 | [v1.0 Release Decision](./done/075-v1-release-decision.md) | MD-2 resolved, serde format declared stable, RFC-067 family maturity table recorded; no v1.0 release authorized |
| 077 | [Seeded Train/Test Split for `matten-mlprep`](./done/077-seeded-train-test-split.md) | Implemented and reviewed (GO, no conditions), `4c554a4`; pre-v1 additive API on `0.38.x`; no release |
| 078 | [`matten-stats` Companion Crate](./done/078-matten-stats-companion.md) | Implemented and reviewed (GO, no conditions), `7f1cbba`; fifth published crate, Experimental maturity; no release |
| 079 | [`0.39.0` Pre-v1 Feature Release](./done/079-0390-pre-v1-feature-release.md) | Reviewed (GO, no conditions), committed, and released as `0.39.0` (tagged and published). Scope was narrowed to RFC-077 only, deferring `matten-stats`'s first publication — but the actual publish included it anyway; see §13 and the `0.39.0` post-release alignment |
| 080 | [Promote `matten-mlprep` to Production-Ready](./done/080-matten-mlprep-production-ready.md) | Reviewed and accepted (GO, conditional on three corrections, all applied); label-only promotion, closing RFC-058 §5.1's Option B exit criterion via RFC-077; no code, version, or release |
| 081 | [`Experimental` Crates in a v1.0 Family](./done/081-v1-family-experimental-crate-policy.md) | Reviewed (GO, rereview, conditional on one fix, applied); decides that no crate labelled `Experimental` may ship in a lock-step `1.0.0` family, and applies §6's mechanical RFC-076 inventory refresh (17 sites), committed `7a4b334`. §5 rests the rule on a checkable contradiction between `matten-stats/src/lib.rs:32-33` and `compatibility.md`'s `v0.x`-scoped breaking-change permission. `matten-stats`'s exit was subsequently decided as **Exit A (promotion)**, landing under its own RFC; no v1.0 release authorized |
| 082 | [Streaming CSV Batches for `matten-data`](./done/082-streaming-csv-batches.md) | Reviewed and accepted; `CsvBatchReader` added behind the off-by-default `streaming` feature, answering all six of RFC-037 §4's reopening criteria; no `matten-stream` crate (structural argument: it would need a companion-to-companion dependency on `matten-data` for `Table`, which RFC-078 §6 forbids); no version bump or release |
| 083 | [`matten-stats` Expansion](./done/083-matten-stats-expansion.md) | Implemented and reviewed (approved, no corrections). Adds `covariance_population`, `skewness`, `kurtosis` — `matten-stats` goes from 3 public functions to 6, with 12 new tests. Additive only: no new error variant, no new dependency, no feature gate, no version bump, no release, and **no maturity change** (`matten-stats` remains `Experimental`). Settles the estimator convention explicitly — ecosystem default per function, so `covariance` stays bias-corrected while `skewness`/`kurtosis` are not, and `kurtosis` reports **excess** (Fisher). No `correlation_population`: correlation is ddof-invariant. RFC-083 §4.1's SciPy/pandas defaults were flagged unverified and were confirmed by execution before implementation began. Deferred with reasons: histogram (RFC-040 §8 bin policy), matrix/axis-wise forms, z-score, percentile aliases, mode |
| 084 | [Promote `matten-stats` to Production-Ready Candidate](./done/084-promote-matten-stats-production-ready-candidate.md) | Implemented and reviewed (approved after one correction). Promotes `matten-stats` **Experimental → production-ready candidate**, discharging RFC-081 §3 **Exit A**. Not label-only: the audit found `matten-stats` was the only published crate with no CI job and no example smoke runs, and its `dynamic`-gated test ran only in the MSRV job — Part 1 closed that gap and was proven before the label moved; Part 2 moved the label at every live site and *inverted* rather than deleted the guard asserting `Experimental`. The review narrowed that guard: its first check banned the word anywhere in three whole files, rejecting legitimate maturity-history prose and an unrelated sentence on a general reference page. Concedes rather than hides the residual risks — no usage history, no external `ddof` read — as what the *candidate* rung exists to carry. No release, no version bump, no API change; full production-ready explicitly not claimed |
| 085 | [Promote `matten-data` to Production-Ready](./done/085-promote-matten-data-production-ready.md) | Implemented and reviewed (approved over two rounds). Closes the full-production review RFC-059 §6 deferred: streaming was discharged by RFC-082, the "wide CSV edge-case surface" gained evidence (4,000-case randomized differential run, zero mismatches), and the candidate cycle ran since 2026-06-27. Ten of eleven bar signals cleared outright; **stable API** got an argument rather than a checkmark — the default surface is unchanged across 38 releases, everything RFC-082 added is feature-gated and off by default, and every deferred streaming item is additive, with the residual async risk recorded rather than argued away. Review corrected the guard twice: the first pass *removed* an over-broad blanket check whose coverage two probes proved was not redundant, and the narrowed replacement then false-positived on ordinary prose. The restored guard also caught a genuine pre-existing bug — an example still calling the crate **Beta**, stale since `v0.22.0` across three promotions — which the label-keyed sweep structurally could not find. Label/docs/guard only; RFC-042 scope lock untouched |
| 086 | [`0.40.0` — Feature and Maturity Release](./done/086-0400-feature-and-maturity-release.md) | **Released** — tagged and published to crates.io 2026-07-30, all five crates live at `0.40.0`, matching the planned scope exactly (no post-release correction needed, unlike `0.39.0`). Reviewed and approved with no corrections. Publishes the accumulated user-facing work of RFC-082–085: `CsvBatchReader`, `covariance_population`/`skewness`/`kurtosis`, and three maturity promotions never before visible to users. §3's tag precondition is discharged: `0.38.0`/`0.39.0`, orphaned by the history rewrite, were re-tagged onto branch history and all 100 tags now resolve to ancestors of `origin/main`, GPG-signed invariant intact. §2 records the process failure that let the work accumulate: every RFC correctly said "no release" for its own slice and nothing asked about the accumulation, the mirror of the `0.31.0`→`0.38.0` eight-releases-with-no-content finding. The retarget was **37 strings across 17 files**, one more than §6 measured — the RFC pattern `0\.39\.[0x]` could not see a bare "0.39 release family", and the guard caught it: the third instance in three RFCs of an enforced invariant catching what a one-time sweep missed |
| 087 | [`repeat`, `tile`, and `meshgrid`](./done/087-repeat-tile-meshgrid.md) | Implemented and reviewed (approved, no corrections). Closes RFC-039 §8's three deferred APIs; core `matten` gains eight functions and `public-api-snapshot.md` moves — the first core public-surface change since `0.38.0`. **Unreleased**, family stays at `0.40.0`. First theme chosen against §1.1's planning baseline. Settled RFC-039's open decisions: separate `repeat`/`repeat_axis` per the `var`/`var_axis` precedent; `tile` accepts `reps` shorter than rank but rejects longer, refusing NumPy's silent rank promotion; `meshgrid` uses NumPy's `xy`. §6 states the boundary between the two competing principles — match the ecosystem where a divergence would be silent, diverge only where it surfaces as an error that teaches. Review found **three errors in the RFC and handoff, none in the implementation**: `MattenError::Axis` does not exist (axis errors are `Shape`), `crates/matten/README.md` has no public-API section, and core needs no `[[example]]` entry for an auto-discovered example — all caught by reading the handoff against the codebase |
| 088 | [Negative Indices in `slice_str`](./done/088-negative-slice-indices.md) | Implemented and reviewed (approved after one should-fix). Closes RFC-008's `0.1.0` deferral: `"-1"` is the last element, `"0:-1"` everything but it. **`slice_str` only** — the builder's `IntoSliceRange` is sealed over five `usize` range types, and adding `isize` impls would make every existing `range(1..3)` call ambiguous, a source-breaking change RFC-015 does not permit for a convenience feature. Out-of-range **errors rather than clamps**, diverging from Python but matching matten's own positive-index behaviour, justified by RFC-087 §6's rule since the divergence is visible. Review found a third failure shape the implementer's exclusivity argument had not enumerated — an inverted range whose message printed only resolved values, so `"-1:-3"` and `"-1:0"` produced identical text naming numbers the caller never wrote; fixed per-bound. §8 flags, without resolving, that `"0:0"` yields shape `[0]` which the constructor rejects. **Unreleased**; no public item changed |
| 089 | [`0.41.0` — Core Shape and Slicing Release](./done/089-0410-core-shape-and-slicing-release.md) | **Released** — tagged and published to crates.io 2026-07-31, all five crates live at `0.41.0`, matching the planned scope exactly. Reviewed and approved after one correction. Releases RFC-087 and RFC-088 — the first two themes chosen against §1.1's planning baseline. No blocking precondition, the `0.40.0` tag-defect repair having discharged it. Triggered by the release-readiness check answering **yes** at RFC-088's disposition after **not yet** at RFC-087's, with the condition written down — both answers on consecutive uses. 37 strings across 17 files, measured with `0\.40\b`; the suffixed pattern would have found 35, missing exactly the two bare-form sites that also needed content updates. The one correction was not in the implementation: a stale Status clause attributed RFC-077's seeded split (`0.39.0`'s content) to `0.40.0`, left by an earlier partial edit and swept up by a mechanical retarget — re-anchored, with `0.39.0`'s story delegated to its history rows |
| 090 | [Histogram — Bin-Selection Policy](./done/090-histogram-bin-policy.md) | Implemented and reviewed (approved after one correction). Resolves the policy RFC-040 §8 left open in v0.21.2 — the oldest question in the project — and adds the function it blocked; `matten-stats` now exposes seven. **Unreleased.** The policy: **no automatic bin rule**, `bins` is required, because each of Sturges / Freedman–Diaconis / Scott is a statistical assumption wearing a default's clothing. Matches NumPy on the closed last bin (an open one silently drops the maximum), diverges on constant input (errors rather than inventing a `±0.5` range) — both decided by RFC-087 §6's silent-vs-visible rule, now on its third RFC. **Amends RFC-078 §5's boundary** to "scalar where scalar, a small owned struct where inherently vector-valued, never a `Tensor`", which deliberately does not unblock the matrix-wide forms RFC-083 §6 deferred. Review found a silent-wrong-answer defect the specified tests could not see: finite inputs whose derived `hi - lo` overflowed returned `Ok` with `NaN`/`inf` edges while the sum invariant still passed |
| 091 | [`0.42.0` — Statistics Release](./done/091-0420-statistics-release.md) | 0.42.0 (released 2026-08-01). The smallest release the project has cut — one companion function. Records in §2 that the recorded §6.4 trigger did **not** fire: no second theme landed, and the owner overrode a recommendation to wait. Not Added-only — RFC-090 also changed two `Display` strings on functions that shipped in `0.41.0`, so a `Changed` section is mandatory. Review found one correction, on a page neither the RFC nor the handoff pointed at: `introduction.md` carried an inherited *"no other runtime behavior change"* clause, true for `0.41.0` and false here, contradicting the CHANGELOG of the same commit |
| 092 | [Adopt the 5-Folder RFC Lifecycle Variant](./done/092-five-folder-rfc-lifecycle.md) | Process change; unreleased and unversioned. Adds `accepted/` between review and implementation, because owner sign-off and implementer completion became separate events performed by different parties — RFC-000's own criterion for the variant, which resolved the other way when it was written. The cost was already in the corpus: RFC-090 and RFC-091 were both accepted while sitting in `proposed/`, each carrying a hand-written Status qualifier to reconcile the contradiction RFC-000 exists to prevent. Amends RFC-000, does not supersede it |
| 093 | [Browser Shape Playground](./done/093-browser-shape-playground.md) | Unreleased and unversioned; lands on the deployed book. Interactive shape/broadcast/reduction page, WebAssembly core, binding crate workspace-excluded and `publish = false`. **Its own §4 safety claim was false** — `wasm-bindgen` was on neither published-crate guard's blocklist, so both would have passed a direct leak; the implementation checked rather than trusted, and fixed them. Review added C1: the reproduced panic text was hand-transcribed, so a reword in core would have left the page quoting matten as saying something it no longer says, with the test still green — now asserted against the live panic payload |
| 094 | [Release Cadence Policy](./done/094-release-cadence-policy.md) | Process policy; unreleased and unversioned. RFC-015 owned *whether* a release is fit and RFC-030 owns lock-step, but **nothing owned *when***, so it was renegotiated per release — four minors in five days, two on one day, and eight no-change crate versions republished across three releases. Patch releases had also silently lapsed: 14 minors and zero patches since `0.28.5`. Patches now ship as soon as reviewed with no batching; minors batch on two-themes / 28-days / owner-asks; documentation-only work never releases, tested by whether `git diff -- crates/` is empty |
| 095 | [Two-Dimensional Matrix Rendering in the Playground](./done/095-matrix-grid-rendering.md) | Unreleased and unversioned; deploys with the book. Rank ≤ 2 renders as an aligned grid, so Reshape stops printing two identical value lists — the defect the owner found by using the page. **Amends RFC-093 §6** from *"text only"* to representation-versus-visualization, a **narrower** rule that newly forbids ASCII charts. Inherits the report tool's three display constants verbatim. Scope widened mid-flight to the page's presentation: forms above contributor notes, a stylesheet using mdBook theme variables, and links from `README.md`, `introduction.md` and `quick-start.md` |
| 096 | [Grid Rendering in the Shape and Axis Example](./done/096-example-grid-rendering.md) | **Unreleased** — reaches users at the next release; deploys nothing. RFC-095's fix applied to shipped code, where a `cargo run` reaches people a URL does not. Formatter kept **local**, which was only legal because the example **asserts its own rendered blocks** — a `#[test]` inside an example runs zero tests, and CI catches a panic but not a mis-aligned grid. Eight assertions where two were required. Review found one defect caused by the RFC's own wording: *"natural float rendering"* admitted `Display`, which printed `1 2 3` for `f64` data and contradicted the axis line in the same output |
| 097 | [Report Demos as Generated Book Pages](./done/097-report-demos-on-the-site.md) | **Unreleased**; deploys with the book. Phase 2 of RFC-093. Five demos publish as generated Markdown pages, **committed with a freshness guard** because mdBook creates a missing page empty and exits 0. Approved with **no corrections** — the one departure from instructions was where the instructions were wrong: §5.1's illustration said two demos held only column statistics, but `to_tensor()` builds `[rows, cols]` unconditionally and the masks preserve shape, making **15** defect sites rather than 9. Also found that mdBook collapses whitespace outside code blocks, so every grid needed a fence or the alignment would have vanished on render |
| 099 | [Result-Form `try_matmul` and `try_dot`](./done/099-result-form-matmul.md) | **Unreleased**; new public API, so it makes RFC-094's first minor trigger since `0.42.0` available. Robustness, not capability — the split of RFC-098 after the owner asked whether that recommendation was oriented to safety or functionality. Two of 43 core operations stopped being panic-only, **without altering a single observable string**, verified against pre-change captures rather than inferred from the `Display` impls. Approved with no corrections; the newly pinned dynamic-guard message was proven able to fail before being accepted as a net |
| 100 | [`Display` for `Tensor`](./done/100-display-for-tensor.md) | **Unreleased**; new public API, joining RFC-099 as RFC-094 minor-trigger content. Settles the formatting contract deferred since `0.1.0`. **Two of its own claims were wrong and are corrected inline**: it said two of three formatters would collapse to `Display` when only one does — the playground carries the same `{:.3}`-with-clamp constraint RFC-095 §4 gave it — and §5.5 contradicted §5.2, since `Element`'s `Display` made `Float(2.0)` and `Int(2)` both print `2` in the one view built for mixed types |
| 101 | [`0.43.0` — Core Surface Release](./done/101-0430-core-surface-release.md) | 0.43.0 (released 2026-08-04). The first release whose RFC-094 trigger fired on its own terms — two themes landed, where `0.42.0` proceeded on an owner override. Added-only, and the API snapshot took a **content** update where the previous release required the number only. Approved with no corrections; the one deviation was correct and exposed a defect in this RFC's own §5 table, which counted a **historical narration** as a live version pin |
| 102 | [Slicing on Dynamic Tensors](./done/102-slicing-on-dynamic-tensors.md) | **Unreleased**; lands as **Changed**, not Added, since it removes a public error. Wiring, not design: the view machinery (`slice_indices`) already existed, was tested, and had **no caller outside its own tests** — so the theme was materially smaller than §3.1 had claimed for three successive framings, all mine. Approved with one required correction that was **this RFC's gap, not the implementation's**: shared storage means a slice retains its source's entire allocation, with `materialize()` `pub(crate)` and no documented escape hatch (§8.1). The implementer also caught two handoff defects — a `CHANGELOG.md` requirement the RFC never listed, and a missing RFC-vs-handoff priority rule |
| 103 | [`0.44.0` — Dynamic Slicing Release](./done/103-0440-dynamic-slicing-release.md) | 0.44.0 (released 2026-08-08). The second **owner-directed** release, after `0.42.0` — RFC-094's cadence triggers had not fired (one theme, day 4 of 28), and this RFC says so plainly so the release table does not later read as though the policy produced it. Changed-only, the exact inverse of `0.43.0`'s Added-only shape four days earlier, which is where the risk sat. Separated **38 live pins from 21 historical narrations** and inspected all 38 rather than counting them — the check RFC-101's own §5 table failed. Approved with no corrections; the one correction was to **this RFC's own criterion**, which asserted a fixed count of `0.43` occurrences in `CHANGELOG.md` that correct work must violate, since the entry naming a bump has to cite the version it moves from |
| 104 | [Mutable Element Access](./done/104-mutable-element-access.md) | **Unreleased**; additive — `get_mut`/`get_flat_mut`/`get_element_mut`, mirroring the existing getters. Closes the last live feature row in §3.1's deferred table. **The RFC was wrong twice before it was right and says so inline**: first specified as `set`/`set_flat` returning `Result`, which the owner's *"is it clean and sophisticated design?"* exposed as the weaker primitive — `set` cannot express read-modify-write and diverges from the ecosystem without teaching (RFC-087 §6); then excluding dynamic mutation on a coercion decision that does not exist, since `DynamicTensor` carries no per-column type and the caller names the `Element`. Approved with no corrections. Review independently re-verified the implementer's finding that the mandated bounds-check-before-`materialize()` ordering is unreachable under today's invariants — by panic-injection rather than their statement swap — and confirmed it is correct defence-in-depth to keep |
| 105 | [Empty-Tensor Reduction Semantics](./done/105-empty-tensor-reduction-semantics.md) | **Unreleased**; a defect fix, so **Changed** rather than Added. Five reductions misbehaved on an empty tensor **reachable today** without any constructor accepting one: `argmin`/`argmax` raised a raw Rust slice panic — defeating the `try_` form outright — and `mean`/`min`/`max` returned `NaN`/`inf`/`-inf` sentinels, silently. `try_var`/`try_std` had solved it already, so the fix was five siblings copying one in-crate precedent. Approved with no corrections, and **both items the implementer raised were corrections to this RFC**: its claim that `stats.rs:112` justified inf-returning code (it is `try_var`'s own doc, above an already-correct guard), and its baseline that `[].iter().sum::<f64>()` yields `0.0` — **`std` yields `-0.0`**, the true additive identity for floats, so leaving `try_sum` untouched was right and normalising it would have diverged from the language |
| 106 | [Zero-Sized Dimensions — Audit Before Decision](./done/106-zero-sized-shape-model-audit.md) | **Audit complete**, no code changed. Classified every public operation in core and all four companions. **Found a severe defect neither RFC anticipated**: `try_dot`/`try_matmul` panic with a raw `chunks_mut` error on a zero-column product, escaping the very `Result` RFC-099 added so callers need not catch panics — found by *running* operations rather than reading them. Its structural finding reframed the question: `checked_shape_len` is a **single function** behind every rejection, so the mechanical layer is nearly a one-line change and the INCONSISTENT cluster is one validated path and one unvalidated one, split by accident of authorship. SEMANTIC count **six, five sharing one question**. Accepted with one correction — its loudest warning (that `matten-mlprep` would return NaN-filled output under a relaxed guard) was refuted by test: `out` has `rows * cols` slots, so the result is an empty tensor |
| 107 | [Architecture Overview and Data-Model Lifecycle Pages](./done/107-architecture-and-data-model-docs.md) | Docs only; no release trigger. Adds the reader-facing architecture overview and the data-model/lifecycle page the book lacked, with two mermaid blocks that render as code today and become diagrams if a renderer is ever added — no `mdbook-mermaid`, deliberately. Approved after two corrections, both of the class the pages exist to prevent: two section references a reader could not follow (`§8.1`/`§6.1` belong to RFC-102 and RFC-104, not to the pages linked), and a maturity cell marking core *not maturity-labeled* when `README.md` calls it **stable (v0.x)** |
| 108 | [Two Empty-Tensor Defects — Stage 1](./done/108-empty-tensor-defects-stage-1.md) | **Unreleased**; fixes a **live panic in published `0.44.0`** — `try_dot`/`try_matmul` raised a raw `chunks_mut` error on a zero-column product, escaping the `Result` RFC-099 added to prevent exactly that — and adds `is_empty()`, declined at two sites on a false premise. Approved after corrections, and **the defect was in this RFC's scope, not the implementation**: §4 forbade any companion-crate change while adding `is_empty()` makes one *mandatory*, since `clippy::len_zero` then fires on every `tensor.len() == 0` in the workspace and `matten-stats` had two. The branch failed the CI clippy gate; the request had diagnosed it as pre-existing from a `git log` on the file, but **the file was old and the lint's applicability was new** — that question is settled by toggling the change, not by dating the file |
| 109 | [`0.45.0` — Mutation and Empty-Tensor Release](./done/109-0450-mutation-and-empty-tensor-release.md) | 0.45.0 (released 2026-08-09). The first release in this sequence carrying **both** an `Added` and a `Changed` section — and therefore the first where **both** recent releases were the wrong template, `0.43.0` being Added-only and `0.44.0` Changed-only. The API-snapshot instruction likewise inverted: RFC-102 changed no public item so a new row would have been a defect, whereas RFC-104 and RFC-108 added four and omitting them would be. Approved with no corrections; the implementer measured at the base commit via `git grep <rev>` rather than checking out, which removes a moving-target problem the handoff had not solved. **The one correction was to the reviewer's own records**: the session crossed midnight and every date stamped after the rollover said `2026-08-08` instead of `2026-08-09`, which would have left the CHANGELOG and the ROADMAP disagreeing about when this release was prepared |
| 110 | [Empty-Axis Reduction Semantics — Stage 2](./done/110-empty-axis-reduction-semantics.md) | **Unreleased**; **Changed**. Extends RFC-105's answer to the five axis-wise siblings it deliberately excluded — `mean_axis`/`min_axis`/`max_axis`/`var_axis`/`std_axis` leaked `NaN`/`inf`/`-inf` on a zero-length **reduced** axis, while `sum_axis` correctly returned the additive identity and is untouched. **A confirmation rather than a design question**: the audit's one argument for keeping `NaN` — that an `Err` might interrupt a batch of otherwise-valid axis slices — cannot arise, since a reduced axis has a single length and every output slot is therefore empty together. Approved with no corrections; two of the implementer's three judgment calls were better than the reasons given for them, notably an added `reject_dynamic` reported as cosmetic that in fact prevents a dynamic tensor from receiving the length error instead of the dynamic one |

## Accepted

Signed off, implementation authorized, not yet shipped. See
[`accepted/README.md`](./accepted/README.md). Empty means no RFC is currently between
sign-off and implementation — not that the state is unused.

| ID | Title | Handoff |
|---:|---|---|
| 111 | [Zero-Sized Dimensions Accepted — Stage 3](./accepted/111-zero-sized-dimensions-accepted.md) | [`111-…-handoff.md`](./handoffs/111-zero-sized-dimensions-accepted-handoff.md) |

## Archive

Withdrawn or superseded. Numbers are never reused. See
[`archive/README.md`](./archive/README.md).

| ID | Title | Disposition |
|---:|---|---|
| 098 | [Batched Matrix Multiplication](./archive/098-batched-matmul.md) | **Superseded by RFC-099** before acceptance. Bundled a robustness fix to shipped API with a new capability; split so the fix does not depend on the feature. Batched matmul returns to ROADMAP §3.1 |

## Proposed

| ID | Title | Scope |
|---:|---|---|
| 076 | [v1.0 Release Preparation](./proposed/076-v1-release-preparation.md) | Reviewed and accepted (GO, no conditions); its inventory refreshed by RFC-081 (five-crate family, matten-mlprep production-ready, matten-stats row added, RFC-081 §3 precondition added). RFC-081's precondition is now **discharged** — `matten-stats` took **Exit A (promotion)** via RFC-084 — but this RFC remains deferred and no implementation is authorized: v1.0 is not currently wanted, a separate and unrelated reason |

## Remaining Themes And Issues

RFC-070 is closed as an audit decision without public implementation. RFC-072
is closed after its reviewed behavior-preserving modularization. RFC-073's
private input-mode JSON implementation is reviewed (GO, no conditions),
committed, and released in `0.38.0`. A post-`0.38.0` assessment found eight
consecutive releases (`0.31.0` -> `0.38.0`) with zero published-crate change
and recommended re-running the RFC-066 v1.0 readiness audit rather than
picking the next implementation theme by intuition; RFC-074 opens that
audit-only re-audit. The remaining themes below are not authorized unless
their current status says so. The current post-0.38 backlog is:

| Theme | Current authority | Current status |
|---|---|---|
| v1.0 readiness | RFC-066, RFC-067, RFC-074, RFC-075, RFC-076, RFC-081, RFC-084 | RFC-074 (audit) and RFC-075 (MD-2/serde/maturity-table decision) closed; RFC-076 (release preparation) reviewed and accepted (GO, no conditions); RFC-081 answers the question RFC-067 left open (an `Experimental` crate may not ship in a lock-step 1.0 family — Exit A promote or Exit B remove) and its mechanical RFC-076 inventory refresh (17 sites) is applied; RFC-081 §3's precondition is now **discharged** — `matten-stats` took Exit A via RFC-084 — but RFC-076 remains deferred and unauthorized regardless, since v1.0 is not currently wanted; RFC-081 §5's own reasoning still awaits the owner's decision (external read, narrowing the rule, or accepting it as argued); no v1.0 implementation is currently authorized |
| Pre-v1 feature work | RFC-077, RFC-078, RFC-083 | RFC-077 and RFC-078 implemented, reviewed (GO, no conditions), and closed. RFC-040's small-statistics theme was *partially* addressed by RFC-078 (`covariance`, `correlation`, `quantile`), then further by RFC-083 (`covariance_population`, `skewness`, `kurtosis` — six functions, all published at `0.39.0`); histogram, z-score, percentile aliases, matrix-wide/axis-wise variants, and mode remain deferred with reasons recorded in RFC-083 §6 — histogram specifically still blocks on RFC-040 §8's unresolved bin-selection policy. Family grows to five workspace crates; RFC-076 must be updated for five before it is executed |
| Pre-v1 feature release | RFC-079 | Reviewed (GO, no conditions), committed, and released as `0.39.0` — tagged and published outside this project's assistant session. Release-prep scoped RFC-077 only, deferring `matten-stats`'s first publication pending an external `ddof` read; the actual publish included `matten-stats` anyway (no `publish = false` key enforced the deferral). Corrected in the post-release alignment: RFC-079/RFC-078 status, a dated CHANGELOG note, and a new ROADMAP history row (`3.25.0` left unedited). The external `ddof` read is no longer a publication gate; it now informs a possible future policy change |
| Public `matten-report` / `matten-viz` readiness | RFC-070, RFC-063, RFC-065, RFC-068, RFC-069, RFC-071 | RFC-070 closed after audit; no public crate or API authorized |
| `matten-report` modularization | RFC-072, RFC-070 post-0.37 closure audit | Implemented and closed; internal ownership and size guards are established without behavior or public-surface change |
| More input-mode HTML paths | RFC-069, post-0.36 RFC-069 closure audit | Deferred until a concrete report path is reviewed |
| JSON / SVG / Vega-Lite report output | RFC-063, RFC-068, RFC-069, RFC-070 JSON policy audit, RFC-071, RFC-073 | RFC-073 private input-mode JSON implemented, reviewed, and released in `0.38.0`; other input kinds, raw CSV export, public schemas, SVG, and Vega-Lite remain unauthorized |
| Streaming / large CSV | RFC-026, RFC-037, RFC-082 | RFC-082 answers all six RFC-037 §4 reopening criteria and ships `CsvBatchReader` behind `matten-data`'s off-by-default `streaming` feature. Deliberately deferred by RFC-082 itself: async, resumability, backpressure, parallel reading, lenient/skip-malformed modes, schema inference, streaming numeric conversion, and a `matten-stream` crate (rejected on structure, not just cost) |
| `matten-nalgebra` bridge | RFC-025, RFC-041, RFC-054 bridge-readiness handoff | Deferred; per-crate RFC required |
| `matten-candle` bridge | RFC-025, RFC-049, RFC-054 bridge-readiness handoff | Deferred; per-crate RFC required |
| Benchmark hard gates | RFC-049 | Phases 1-3 implemented; Phase 4 hard gates extracted to future RFC/release-policy ownership |
| Broader stats APIs / `matten-stats` | RFC-040, RFC-078, RFC-083 | Core `var`/`std` shipped; `covariance`, `covariance_population`, `correlation`, `quantile`, `skewness`, `kurtosis` shipped in `matten-stats` (RFC-078, RFC-083); histogram, z-score, percentile aliases, matrix-wide/axis-wise forms, and mode require a future RFC |
| Broader linalg / linalg companion | RFC-041 | Core `norm`/`trace`/`outer` shipped; inverse/determinant/decomposition/BLAS/sparse scope requires future RFC |
| Companion full-production decisions | RFC-057, RFC-058, RFC-059, RFC-067, RFC-080, RFC-084, RFC-085 | `matten-ndarray`, `matten-mlprep`, and `matten-data` are production-ready (RFC-080 promoted `matten-mlprep`, closing RFC-058 §5.1's Option B exit criterion via RFC-077; RFC-085 promoted `matten-data`, closing RFC-059 §6's deferred full-production review); `matten-stats` is production-ready candidate (RFC-084, discharging RFC-081 §3 Exit A) and not near full production-ready for lack of usage history (RFC-084 §8); further promotion requires explicit review |

Implementation handoffs for the v0.20+ proposed set (RFC-033–042), the examples
program (RFC-043–048), and the benchmarking program (RFC-049) live in
[`./handoffs/`](./handoffs/). They translate each RFC into PR boundaries, checks,
and acceptance criteria; the RFC remains the design authority. The examples handoff
([`043-048-examples-implementation-handoff.md`](./handoffs/043-048-examples-implementation-handoff.md))
opens with a Phase 0 inventory of the existing example suite: new famous-problem
examples use an additive 30+ band, and existing distance/cosine/companion examples
are audited/improved rather than duplicated. The benchmarking handoff
([`049-benchmarking-developer-handoff.md`](./handoffs/049-benchmarking-developer-handoff.md))
keeps all benchmark tooling in an isolated `publish = false` package so core
`matten` gains no benchmark dependency. The Phase 3 Python reference-comparison
implementation handoff is tracked separately in
[`049-phase-3-python-reference-comparison-handoff.md`](./handoffs/049-phase-3-python-reference-comparison-handoff.md)
as the accepted code-shape-first NumPy/Pandas reference slice.

The production-migration set (RFC-050–054) ships its handoff bundle in
[`./handoffs/`](./handoffs/): the
[`050-053-production-migration-implementation-handoff.md`](./handoffs/050-053-production-migration-implementation-handoff.md)
covers the documentation/policy/template work (RFC-050–053) for v0.23.x, with an
[acceptance/QA checklist](./handoffs/050-053-acceptance-qa-checklist.md) and a
[release-guard checklist](./handoffs/050-053-release-guard-checklist.md); RFC-054's
[`054-deferred-implementation-note.md`](./handoffs/054-deferred-implementation-note.md)
records the original deferral boundary, while the
[`054-matten-migrate-readiness-audit.md`](./handoffs/054-matten-migrate-readiness-audit.md)
and
[`054-matten-migrate-first-tool-handoff.md`](./handoffs/054-matten-migrate-first-tool-handoff.md)
record the reviewed reopening and first local advisory tool slice. Later RFC-054
handoffs cover target suggestions, static API explanations, bridge-readiness
checks, and lifecycle closure. These RFCs add no core dependency: migration
support lives in docs, bridge crates, and workspace-excluded tooling.

The RFC-068 visualization phase is tracked by its handoffs. The first
slice,
[`068-local-html-educational-artifact-handoff.md`](./handoffs/068-local-html-educational-artifact-handoff.md),
starts with a local-only static HTML artifact for the existing
`tools/matten-report --demo educational-path` report. The follow-up slice,
[`068-shared-educational-report-model-handoff.md`](./handoffs/068-shared-educational-report-model-handoff.md),
extracts shared fixed educational-path report data before adding another HTML
report family. The shape-flow implementation slice,
[`068-shape-flow-html-artifact-handoff.md`](./handoffs/068-shape-flow-html-artifact-handoff.md),
extends the same local static HTML pattern to `tools/matten-report --demo
shape-flow` only. Public report/viz crates, SVG, Vega-Lite, expression tracing,
and core visualization APIs remain outside these slices.

The post-0.32 continuation audit
([`068-post-032-visualization-continuation-audit.md`](./handoffs/068-post-032-visualization-continuation-audit.md))
recorded that RFC-068 was not ready for direct public visualization APIs or
published report/viz crates. It recommended the reviewed follow-up handoff for
one more local static HTML artifact, starting with `tools/matten-report --demo
dynamic-readiness`, which shipped in `0.33.0`.

The dynamic-readiness local HTML handoff
([`068-dynamic-readiness-html-artifact-handoff.md`](./handoffs/068-dynamic-readiness-html-artifact-handoff.md))
translates that audit into a reviewable implementation boundary. Its
implementation slice extends local static HTML to `tools/matten-report
--demo dynamic-readiness` only and keeps `data-readiness`,
`mlprep-standardization`, input-mode HTML, public report/viz crates, and core
visualization APIs out of scope.

The post-0.33 continuation audit
([`068-post-033-visualization-continuation-audit.md`](./handoffs/068-post-033-visualization-continuation-audit.md))
records that the next recommended local-only artifact candidate is
`tools/matten-report --demo mlprep-standardization`, still without public
report/viz crates or core visualization APIs. The mlprep-standardization local
HTML handoff
([`068-mlprep-standardization-html-artifact-handoff.md`](./handoffs/068-mlprep-standardization-html-artifact-handoff.md))
translated that recommendation into the `0.34.0` review scope. Its implementation
extends local static HTML to `tools/matten-report --demo mlprep-standardization`
only and keeps `data-readiness`, input-mode HTML, public report/viz crates, SVG,
Vega-Lite, and expression tracing out of scope.

The post-0.34 gap audit
([`068-post-034-visualization-gap-audit.md`](./handoffs/068-post-034-visualization-gap-audit.md))
records the next decision point after the `0.34.0` release. It does not
authorize implementation. It asks review to choose whether the fixed-demo local
HTML line should close now or whether a separate demo-only `data-readiness` HTML
handoff should be drafted. Input-mode HTML, public report/viz crates, SVG,
Vega-Lite, JSON output, and core visualization APIs remain deferred.

The data-readiness local HTML handoff
([`068-data-readiness-html-artifact-handoff.md`](./handoffs/068-data-readiness-html-artifact-handoff.md))
translates the accepted post-0.34 audit path into a reviewable implementation
boundary. Its reviewed implementation adds demo-only HTML for
`tools/matten-report --demo data-readiness` as the `0.35.0` release scope,
completes the fixed-demo local HTML line, keeps input-mode HTML rejected, and
does not authorize public report/viz crates, SVG, Vega-Lite, JSON output, or
core visualization APIs.

The post-0.35 closure audit
([`068-post-035-fixed-demo-html-closure-audit.md`](./handoffs/068-post-035-fixed-demo-html-closure-audit.md))
records the recommended closure point after the `0.35.0` release. All fixed
`tools/matten-report --demo ...` families now support local static HTML, so the
audit recommends closing the RFC-068 fixed-demo HTML line rather than continuing
visualization work automatically. Input-mode HTML, public report/viz crates,
SVG, Vega-Lite, JSON output, expression tracing, autograd, and core
visualization APIs remain separate future RFC or handoff decisions.

RFC-069 starts that separate input-mode HTML decision path. Its policy audit
([`069-input-mode-html-policy-audit.md`](./handoffs/069-input-mode-html-policy-audit.md))
opened review for whether `tools/matten-report --input <csv> --kind
data-readiness --select <cols> --format html --output <path>` should become a
narrow, summary-only, bounded, escaped local artifact. It did not reopen
RFC-068 fixed-demo work and did not authorize public report/viz crates or core
visualization APIs.

The RFC-069 implementation handoff
([`069-input-mode-html-implementation-handoff.md`](./handoffs/069-input-mode-html-implementation-handoff.md))
translated the accepted policy direction into the reviewed implementation
target, including explicit display bounds for tensor previews, wide column
lists, long paths, long headers, and conversion errors.

The first RFC-069 implementation added local static HTML output for
`tools/matten-report --input <csv> --kind data-readiness --select <cols>
--format html --output <path>`. The implementation keeps Markdown/plain text as
default, keeps HTML explicit-file-only, covers success and numeric-conversion
error reports, escapes hostile input, and bounds column lists, long
fields, conversion errors, and tensor previews. The scope is released in
`0.36.0`; release metadata changes are limited to that scope.

The post-0.36 closure audit
([`069-post-036-input-mode-html-closure-audit.md`](./handoffs/069-post-036-input-mode-html-closure-audit.md))
records the recommended closure point after the `0.36.0` release. The audit
recommends closing RFC-069 for the reviewed data-readiness input-mode local HTML
scope rather than continuing input-mode HTML automatically. More input-mode HTML
paths, public report/viz crates, JSON/SVG/Vega-Lite output, notebook/browser
integration, expression tracing, autograd, and core visualization APIs remain
separate future RFC or handoff decisions.

RFC-070 resolved the public visualization/report readiness question as an
audit-only decision. Its readiness audit
([`070-public-visualization-report-readiness-audit.md`](./handoffs/070-public-visualization-report-readiness-audit.md))
records the current verdict: local `tools/matten-report` artifacts are useful
and maintained, but not ready to become public `matten-report` / `matten-viz`
crates or public renderer APIs. The audit recommends keeping renderers private,
keeping core `matten` visualization-free, and considering a separate JSON
report-schema policy audit or private report-model extraction before any public
crate work.

The private report-model extraction handoff
([`070-private-report-model-extraction-handoff.md`](./handoffs/070-private-report-model-extraction-handoff.md))
is the first post-audit prerequisite. Its implementation was reviewed and
committed in `783d757` as a behavior-neutral local-tool refactor for
`tools/matten-report` only: the repeated static HTML document shell is now
shared by private helpers, while report-family data models remain private and
family-specific. Public report schemas, reusable renderer APIs, public
`matten-report` / `matten-viz` crates, JSON/SVG/Vega-Lite output, dependency
changes, and release work remain separate future decisions.

The JSON report-schema policy audit handoff
([`070-json-report-schema-policy-audit-handoff.md`](./handoffs/070-json-report-schema-policy-audit-handoff.md))
is the next RFC-070 planning candidate. It asks whether JSON report data should
be considered at all, and if so whether it should remain private local-tool
output or become future public-contract material. It is audit/design-only: no
`--format json`, public schema, public renderer API, public crate, dependency,
release work, or generated artifact is authorized.

The JSON report-schema policy audit
([`070-json-report-schema-policy-audit.md`](./handoffs/070-json-report-schema-policy-audit.md))
recommends exploring JSON only as private local-tool output first. Public JSON
schema and public report/viz crates remain rejected for now; the recommended
next slice, if accepted, is a fixed-demo-only JSON implementation handoff with
`schema_version: 0`, no input-mode JSON, no public API, and no published-crate
dependency change.

RFC-071
([`071-private-fixed-demo-json-report-artifacts.md`](./done/071-private-fixed-demo-json-report-artifacts.md))
records the private fixed-demo JSON prerequisite as the normative authority for
the `0.37.0` release. It also records the project decision that selected
private-tool visualization milestones may be released as lock-step public family
checkpoints when release notes clearly state that published crates have no API,
runtime, dependency, feature, or maturity-label change.

The fixed-demo JSON implementation handoff
([`070-fixed-demo-json-report-implementation-handoff.md`](./handoffs/070-fixed-demo-json-report-implementation-handoff.md))
was implemented, reviewed, and committed in `d0ef169`. The slice adds private
`tools/matten-report --demo ... --format json --output <path>` output for the
five fixed demos only, with deterministic `schema_version: 0` snapshots and
direct `serde` / `serde_json` dependencies confined to the workspace-excluded
local tool. It does not authorize input-mode JSON, public schemas, public
crates, public renderer APIs, generated artifacts, or any dependency change in
published crates. The slice shipped as the RFC-071 `0.37.0` release scope.

The post-0.37 closure audit
([`070-post-037-public-visualization-closure-audit.md`](./handoffs/070-post-037-public-visualization-closure-audit.md))
reassesses RFC-070 using the private report-model and fixed-demo JSON evidence.
It closes RFC-070 without a public report/viz crate, public report model,
renderer API, or JSON schema. Its next ordered theme is a separate,
behavior-preserving `matten-report` modularization RFC. Private input-mode JSON,
broader mathematics, and new ecosystem bridges remain later candidates that
require their own RFC decisions.

RFC-072
([`072-matten-report-modularization.md`](./done/072-matten-report-modularization.md))
is implemented and closed. Its original 5,023-line local report binary
now has a reviewed/committed process baseline, and Slice 1 separates the entry,
request, CLI, orchestration, transitional renderer, output, and owned-test
boundaries. Phase 2 report-model extraction and total app dispatch are also
reviewed and committed; shared formatting, all five Markdown owners, all HTML
document/security and family owners, and the private JSON model/policy/family
mappings are also reviewed and committed. Phase 4 structural closure and the
mechanical file-size ceiling are reviewed and committed. These changes add no
features, dependencies, public APIs, or release scope.

The detailed RFC-072 handoff
([`072-matten-report-modularization-implementation-handoff.md`](./handoffs/072-matten-report-modularization-implementation-handoff.md))
defines the distinct process-baseline checkpoint, exact byte fingerprints,
module dependency guard, family/format-sized movement units, test placement,
and final gates. The accepted handoff also makes `app` construct report-owned
family values before renderer dispatch and assigns every normalization/helper
to a dependency-safe owner. Slice 0, Slice 1, and Phase 2 are reviewed and
committed. The first Phase 3 checkpoint moved shared fixed-number formatting,
data-readiness Markdown/list ownership, exact Markdown tests, and report-owned
selection-error tests and is committed. The remaining four fixed-demo Markdown
families and exact tests are also committed. Phase 3C HTML document/security,
family rendering, exact snapshots, hostile-input, and bounds tests are also
committed. Phase 3D private JSON model/mappings/tests and Phase 4 structural
closure are reviewed and committed. RFC-072 is terminal; further report-tool
features require a separate RFC-first decision.

RFC-073
([`073-private-input-mode-json-report-policy.md`](./done/073-private-input-mode-json-report-policy.md))
closes the report-tool implementation line opened after RFC-072. Its accepted
policy permits a bounded private schema-v0 JSON file for successful and failed
strict numeric conversion on the existing data-readiness CSV input path. It
keeps raw CSV export, public schemas/APIs/crates, other input report kinds,
broader formats, mathematics, bridges, and release work unauthorized until the
applicable review gate. Policy acceptance opened a detailed implementation
handoff
([`073-private-input-mode-json-implementation-handoff.md`](./handoffs/073-private-input-mode-json-implementation-handoff.md)).
That accepted handoff defined one coherent implementation checkpoint with exact
private schema shape, bounds, outcome taxonomy, destination-preservation tests,
and regression gates. The checkpoint was reviewed
(`matten-rfc073-private-input-mode-json-implementation-review-v0.1.md`, GO, no
conditions) and committed. Its `0.38.0` release-prep was reviewed
(`matten-0380-rfc073-input-mode-json-release-prep-review-v0.1.md`, GO
conditional on one fixed ROADMAP history-row gap) and committed. `matten`,
`matten-ndarray`, `matten-mlprep`, and `matten-data` `0.38.0` are tagged and
published to crates.io. RFC-073 is terminal; further report-tool features
require a separate RFC-first decision.

A post-`0.38.0` release-confirmation assessment
(`matten-post-0380-next-theme-assessment-v0.1.md`) confirmed the release
completed cleanly with no outstanding work, then found that
`0.31.0` -> `0.38.0` — eight releases — shipped zero functional change to any
published crate: the entire span's `crates/*/src/` diff is nine doc-comment
version-string lines in `crates/matten/src/lib.rs`. It recommended re-running
the RFC-066 v1.0 readiness audit as the next theme rather than picking among
the RFC-070 remaining-themes backlog by intuition, since RFC-066 is stale by
exactly the length of that drift and an audit produces a ranked,
evidence-based answer instead of a guess.

RFC-074
([`074-v1-readiness-reaudit.md`](./done/074-v1-readiness-reaudit.md))
was that audit-only re-audit. It re-measured RFC-066's original findings,
asked whether the published `Tensor` contract/error model/boundary APIs are
v1-stable, classified deferred stats/linalg/streaming scope as v1-blocking or
post-v1, asked whether `matten-mlprep`/`matten-data` should be recommended
for promotion to production-ready, and asked whether RFC-030 lock-step
versioning still serves the family given the observed drift — treated as a
first-class question, not a footnote.

The audit report
([`docs/design/v1-readiness-audit.md`](../docs/design/v1-readiness-audit.md))
was reviewed and accepted
(`matten-rfc074-v1-readiness-reaudit-review-v0.1.md`, GO). Every RFC-066
finding was re-verified without regression (BF-1 remains remediated; MD-1
remains resolved by RFC-067). Broader stats/linalg were found to be settled
core-scope decisions (linalg explicitly rejected; stats deferred toward a
companion, not rejected — corrected per review) rather than open questions,
and streaming remains an additive future capability outside `matten-data`'s
current documented contract — none of the three block v1.0. The report's
verdict is **conditionally ready**: the technical/API surface clears every
gate RFC-066 set (more strongly than at the original audit, since it has had
zero churn across eight releases), but release preparation should not start
until the maintainer resolves a new finding, **MD-2** — whether RFC-030
lock-step versioning's RFC-071 §6 reconsideration trigger should now fire,
and relatedly, whether the project's real direction is toward v1.0 or a
deliberate "0.x indefinitely" stance. The report offered two explicit paths
without choosing between them; **the owner chose Path B (pursue v1.0
deliberately)**. NF-1 and NF-2 (matten-data/matten-ndarray missing README
Public API blocks; `cargo public-api` not wired anywhere) were closed as the
review's H0 common first step, applicable to either path.

RFC-075
([`075-v1-release-decision.md`](./done/075-v1-release-decision.md))
was the maintainer-decision document Path B required. It resolved MD-2 (keeps
RFC-030 lock-step versioning unchanged, adding a CHANGELOG justification
requirement for future local-tool-only releases), declared the JSON
canonical serde format explicitly stable, and recorded the RFC-067 family
maturity table recommending `matten-mlprep`/`matten-data` enter a future
v1.0 family at their current `production-ready candidate` label without
promotion. It was reviewed and accepted
(`matten-rfc075-v1-release-decision-review-v0.1.md`, GO), with one required
follow-through applied before closure: the §3.1 CHANGELOG-justification rule
is now also recorded in `docs/src/contributing/release-checklist.md` §7 and
`CHANGELOG.md`'s conventions blockquote — not stored only inside the RFC,
which is exactly where RFC-071 §6's predecessor rule lived when it went
unfired for eight releases. RFC-075 authorizes no v1.0 release, version
bump, tag, publish, API change, dependency change, or maturity promotion; a
separate future v1.0 release-prep RFC (Unit 2) is required before any
release action, and must run the full gate set (clippy, full feature
matrix, doctests, MSRV, `cargo package`) this documentation-only line did
not run, and must reproduce the RFC-067 family maturity table in full
rather than merely citing RFC-075.

RFC-076
([`076-v1-release-preparation.md`](./proposed/076-v1-release-preparation.md))
is that release-prep unit. It specifies the `1.0.0` release-preparation
change in full: the RFC-067 family maturity table reproduced (neither
companion promoted), a rewritten `compatibility.md` stating what SemVer
covers and excludes, a 19-site `pre-1.0`/`0.x` documentation sweep across 9
files (including reconciling a second, contradictory compatibility promise
in `migration.md` down to a single canonical home), a 29-string
current-family version retarget across 14 files, and the `0.38.0` ->
`1.0.0` version bump. It went through two review rounds (NO-GO on missing
sweep/retarget scope, then GO conditional on two mechanical count
corrections) before two further maintainer decisions were applied and
independently re-reviewed: the three `#[doc(hidden)]` slice-plumbing items
(`IntoSliceRange`/`SliceConvert`/`SliceSpecRepr`) are **covered** by the
`1.0.0` promise rather than excluded, and `cargo public-api` is recorded as
**not required** for this release — both decisions dated, attributed, and
reasoned rather than silently applied. The final review was **GO, no
conditions**.

**Execution status: deferred.** An implementation attempt was made and then
fully reverted at the owner's explicit instruction, because it proceeded
without the owner's direct confirmation of the version bump specifically —
review acceptance is not the same authority as the owner's go-ahead to
execute. **No RFC-076 implementation is currently authorized.** The owner has
since directed that pre-v1 feature work (RFC-077's seeded train/test split,
RFC-078's `matten-stats` companion proposal) proceed first on the `0.38.x`
line while RFC-076 remains accepted-but-unexecuted. `0.38.0` remains the
current released version. RFC-076 authorizes no tag or crates.io publish in
any case; Unit 3 (release execution) remains a separate, maintainer-authorized
step required after a future release-prep commit is reviewed, committed, and
explicitly re-authorized to proceed — the point at which the project's
choices stop being reversible.

RFC-077
([`077-seeded-train-test-split.md`](./done/077-seeded-train-test-split.md))
was pre-v1 feature work on the `0.38.x` line: one additive function,
`train_test_split_seeded`, implementing the signature RFC-024 §6 specified
and left as "planned." It closes the one caveat RFC-076 §5's family maturity
table cites for `matten-mlprep`'s `production-ready candidate` label, without
promoting the crate — that stays a separate decision. The RNG is a
hand-rolled, dependency-free SplitMix64 (RFC-024 §6's pre-decided choice);
Fisher-Yates shuffles a row-index vector, never the data itself; and the
exact PRNG constants, shuffle direction, and seed-to-state mapping are a
reproducibility contract, enforced by a locked-permutation test. Two design
review rounds found and fixed three defects in the author-drafted documents
(a handoff sketch that would not compile, an error-surface undercount, and a
stale tracking claim, the last already fixed via the RFC-076 deferral
commit). The implementation was committed (`4c554a4`) and reviewed
afterward — the review independently re-verified spec conformance line by
line and proved the reproducibility contract by mutation (flipping the
Fisher-Yates direction and altering a SplitMix64 constant both correctly
failed the locked-permutation test). Final review: **GO, no conditions**.
RFC-077 is closed; it authorized no version bump or release, and none
occurred.

RFC-078
([`078-matten-stats-companion.md`](./done/078-matten-stats-companion.md))
added `matten-stats`, the fifth published crate, at **Experimental** maturity
(RFC-040 §9's pre-decided rung for a crate with no usage history). It
provides three scalar statistics RFC-040 §8 deliberately kept out of core:
`covariance`, `correlation` (both sample, `ddof = 1`, diverging deliberately
from core's population `var`/`std`), and `quantile` (linear interpolation).
The self-authored review found two mechanical defects in its own
handoff — an over-count of the guard scripts needing edits (three, not four;
`check-streaming-scope.sh` auto-covers the new crate via its `crates/*` glob)
and a missing step to re-verify the `ddof = 1` third-party-tool rationale
against current docs — both fixed before implementation. During
implementation, a further finding surfaced that neither document
anticipated: `matten::Tensor` cannot represent a zero-element tensor at all
(every shape dimension must be non-zero), so the handoff's requested "empty
tensor → `Empty`" test is unconstructible; the tests and crate README were
adjusted to cover what is actually reachable (`covariance`/`correlation`'s
real `n = 1` case) rather than silently dropping the scenario. Full gate set
green, including MSRV and `cargo package --workspace` packaging all five
crates. The implementation was committed (`7f1cbba`) without a prior
implementation review, despite the RFC-078 review's own recommendation to
review this slice before committing (a new published crate is permanent once
released). The post-commit review re-verified the three algorithms against
RFC-078 §4 line by line and proved the `ddof = 1` policy by mutation
(flipping covariance's `n-1` divisor to `n` correctly failed two tests,
including the one asserting `cov(x,x)` equals the sample variance). It also
confirmed all six guard scripts pass — three edited, one (`check-streaming-
scope.sh`) correctly left alone since it auto-covers the new crate — and that
the four existing crates are untouched. Final review: **GO, no conditions**.
Two standing items remain, neither a defect: the `ddof = 1` divergence has
still never been reviewed by anyone who did not propose it, and
review-before-commit did not happen for this slice even though it was
specifically recommended — both are noted for the next comparable slice, not
fixed here. RFC-078 is closed; it authorized no version bump or release, and
none occurred. RFC-076's release-prep specification now assumes four crates
and must be updated before it is executed — before this closure, that was
advisory; it is now a hard precondition, since `matten-stats` exists in the
workspace regardless of when it actually publishes.

RFC-079
([`079-0390-pre-v1-feature-release.md`](./done/079-0390-pre-v1-feature-release.md))
sequenced the release of RFC-077 and RFC-078 as `0.39.0`, a normal `0.x` minor bump and the
family's first consumer-visible release since `0.31.0` (RFC-074's MD-2 finding). Its
self-authored review found one defect: the retarget instruction (`git grep 0.38`) did not exempt
two files that legitimately keep `0.38` references after a correct retarget —
`docs/design/v1-readiness-audit.md`'s dated finding ("eight releases `0.31.0` -> `0.38.0`") and
`scripts/check-release-docs.sh`'s comment recording the `0.38.0` incident that motivated its own
ROADMAP parity guard. Following the instruction literally would have silently corrupted both,
since no gate checks either file's content; both exemptions were added before implementation.

RFC-079 §3 named one consequential, so-far-unreviewed decision: `matten-stats` carries no
`publish` key, so this release would otherwise ship it to crates.io for the first time, and its
`ddof = 1` covariance/correlation choice (RFC-078 §4.1) had only ever been proposed, argued, and
reviewed by the same author. Given that choice, the owner confirmed the version bump but declined
both options the RFC anticipated (external read now, or accept the risk now) in favor of a third:
defer `matten-stats`'s first publication entirely until an external read is obtained outside this
project's assistant session. `0.39.0` therefore releases `train_test_split_seeded` alone;
`matten-stats` still moves to `0.39.0` in the workspace under lock-step versioning (RFC-030), but
is excluded from this release's publish step, its CHANGELOG entry, and the release-checklist's
fifth-crate teaching — all deferred to whichever release actually first publishes it. Implementing
the narrowed scope caught one gap neither document anticipated: `docs/src/introduction.md`'s
"current 0.38 release family" sentence named RFC-073's release specifically and needed rewriting
for content, not just substitution, once `0.39.0` became a different release with different
contents. Release-prep was committed, then `0.39.0` was tagged and published outside this
project's assistant session.

**The exclusion did not hold.** `matten-stats` carried no `publish = false` key — a discipline
decision, not an enforced one — and the actual publish action shipped it alongside the other four
crates. Verified directly against crates.io (never via `cargo publish`, the wrong instrument for
checking a state that was never mechanically withheld): `matten-stats` shows exactly one version,
`0.39.0`, published the same day. The post-release alignment handoff's Case B corrections were
applied: RFC-079's and RFC-078's status now say "published," a dated correction was added to the
`[0.39.0]` CHANGELOG entry without erasing its original text, and ROADMAP records the divergence in
a new history row rather than editing the row that accurately described the decision as it stood
when made. The external `ddof = 1` read (RFC-078 §4.1) no longer gates a first publication that has
already happened — it now informs whether a future change to that policy is warranted, and remains
open.
