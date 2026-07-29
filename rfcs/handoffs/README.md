# RFC Implementation Handoffs

This directory holds **implementation handoffs** for proposed RFCs. A handoff
translates an accepted RFC's design into developer-executable work: module layout,
PR boundaries, task breakdown, acceptance checklists, edge cases, and CI commands.

The RFC remains the design authority; a handoff never overrides it. Handoffs are
working documents for the implementation team and may be revised as work proceeds.

## Current handoffs (v0.20+ proposed set, RFC-033–042)

| Handoff | RFC |
|---|---|
| `033-implementation-handoff.md` | RFC-033 `matten-data` Beta-Decision and Scope Lock |
| `034-implementation-handoff.md` | RFC-034 `matten-data` Table Model and Public API Boundary |
| `035-implementation-handoff.md` | RFC-035 CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion |
| `036-implementation-handoff.md` | RFC-036 `matten-data` Examples, Documentation, and Release Gate |
| `037-implementation-handoff.md` | RFC-037 Deferred Streaming and Large CSV Policy |
| `038-implementation-handoff.md` | RFC-038 Core Numeric Comfort APIs |
| `039-implementation-handoff.md` | RFC-039 Shape Composition API Boundary |
| `040-implementation-handoff.md` | RFC-040 Small Statistics Boundary — Core vs Companion |
| `041-implementation-handoff.md` | RFC-041 Linear Algebra Boundary — Core Lite vs External Crates |
| `042-implementation-handoff.md` | RFC-042 Pandas-Inspired Scope Guard for `matten-data` |
| `063-phase-1-visual-docs-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 1 docs only |
| `063-phase-2-example-reports-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 2 example reports |
| `063-phase-3-local-report-tool-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 first local report tool slice |
| `063-phase-3-shape-flow-report-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 shape-flow local report slice |
| `063-phase-3-dynamic-readiness-report-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 dynamic-readiness local report slice |
| `063-phase-3-mlprep-standardization-report-handoff.md` | RFC-063 Visual Understanding and Reporting — Phase 3 mlprep-standardization local report slice |
| `026-037-large-data-streaming-policy-closure-handoff.md` | RFC-026 / RFC-037 Large Data and Streaming Policy — proposed policy-closure slice |
| `049-lifecycle-closure-handoff.md` | RFC-049 lifecycle closure / Phase 4 deferral handoff |
| `054-matten-migrate-readiness-audit.md` | RFC-054 `matten-migrate` readiness audit |
| `054-matten-migrate-first-tool-handoff.md` | RFC-054 `matten-migrate` first local advisory tool slice |
| `054-matten-migrate-target-suggestion-handoff.md` | RFC-054 `matten-migrate` target-specific suggestion slice |
| `054-matten-migrate-explain-api-handoff.md` | RFC-054 `matten-migrate` static API explanation slice |
| `054-matten-migrate-check-bridges-handoff.md` | RFC-054 `matten-migrate` bridge-readiness check slice |
| `054-lifecycle-closure-handoff.md` | RFC-054 lifecycle closure / future automation extraction handoff |
| `066-v1-readiness-audit-handoff.md` | RFC-066 v1.0 readiness audit report handoff |
| `065-educational-visualization-handoff.md` | RFC-065 Educational Visualization and Tensor Learning Path — first docs/positioning slice |
| `065-phase-2-educational-shape-data-path-handoff.md` | RFC-065 Educational Visualization and Tensor Learning Path — Phase 2 educational shape/data docs slice |
| `065-phase-3-educational-path-report-handoff.md` | RFC-065 Educational Visualization and Tensor Learning Path — Phase 3 local educational-path report slice |
| `068-local-html-educational-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — first local HTML educational artifact slice |
| `068-shared-educational-report-model-handoff.md` | RFC-068 Rich Local Visualization Artifacts — shared educational-path report data refactor slice |
| `068-shape-flow-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — shape-flow local HTML artifact slice |
| `068-post-032-visualization-continuation-audit.md` | RFC-068 post-0.32 visualization continuation audit and next-slice recommendation |
| `068-dynamic-readiness-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — dynamic-readiness local HTML artifact handoff |
| `068-post-033-visualization-continuation-audit.md` | RFC-068 post-0.33 visualization continuation audit and next-slice recommendation |
| `068-mlprep-standardization-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — mlprep-standardization local HTML artifact handoff |
| `068-post-034-visualization-gap-audit.md` | RFC-068 post-0.34 visualization gap audit and next-decision recommendation |
| `068-data-readiness-html-artifact-handoff.md` | RFC-068 Rich Local Visualization Artifacts — data-readiness demo-only local HTML artifact handoff |
| `068-post-035-fixed-demo-html-closure-audit.md` | RFC-068 post-0.35 fixed-demo local HTML closure audit |
| `069-input-mode-html-policy-audit.md` | RFC-069 input-mode HTML report policy audit |
| `069-input-mode-html-implementation-handoff.md` | RFC-069 input-mode HTML report implementation handoff |
| `069-post-036-input-mode-html-closure-audit.md` | RFC-069 post-0.36 input-mode HTML closure audit |
| `070-public-visualization-report-readiness-audit.md` | RFC-070 public visualization/report readiness audit |
| `070-private-report-model-extraction-handoff.md` | RFC-070 private report-model extraction handoff |
| `070-json-report-schema-policy-audit-handoff.md` | RFC-070 JSON report-schema policy audit handoff |
| `070-json-report-schema-policy-audit.md` | RFC-070 JSON report-schema policy audit |
| `070-fixed-demo-json-report-implementation-handoff.md` | RFC-071 fixed-demo private JSON report implementation handoff, drafted from the RFC-070 audit line |
| `070-post-037-public-visualization-closure-audit.md` | Accepted RFC-070 post-0.37 closure audit; recommends a separate modularization RFC |
| `072-matten-report-modularization-implementation-handoff.md` | Retained accepted RFC-072 implementation record; all phases reviewed and committed, lifecycle closed |
| `073-private-input-mode-json-implementation-handoff.md` | Accepted RFC-073 handoff; bounded private data-readiness input JSON implemented, reviewed (GO, no conditions), and released in `0.38.0` |
| `076-v1-release-preparation-handoff.md` | Accepted RFC-076 handoff; sequences the `1.0.0` release-preparation change (RFC-067 maturity table, compatibility.md rewrite, 19-site pre-1.0/0.x sweep, 29-string current-family retarget, version bump); reviewed (GO, no conditions); execution deferred pending pre-v1 feature work (RFC-077, RFC-078); no implementation authorized |
| `077-seeded-train-test-split-handoff.md` | Accepted RFC-077 handoff; `train_test_split_seeded` for `matten-mlprep` (SplitMix64, Fisher-Yates over row indices, locked-permutation test); implemented and reviewed (GO, no conditions, reproducibility contract proven by mutation), committed `4c554a4`; no version bump or release |
| `078-matten-stats-companion-handoff.md` | Accepted RFC-078 handoff; new `matten-stats` companion crate (fifth published crate, Experimental maturity): `covariance`/`correlation` (sample, `ddof = 1`) and `quantile` (linear interpolation); implemented and reviewed (GO, no conditions, ddof policy proven by mutation), committed `7f1cbba`; no version bump or release |
| `079-0390-pre-v1-feature-release-handoff.md` | Accepted RFC-079 handoff; `0.39.0` release-prep sequencing RFC-077 and RFC-078; reviewed (GO, no conditions); owner confirmed the version bump and deferred `matten-stats`'s first publish pending an external `ddof` read, narrowing the release to RFC-077 only; release-prep committed, then `0.39.0` tagged and published outside this project's assistant session — `matten-stats` was published anyway, corrected in the post-release alignment (`0390-post-release-alignment-handoff.md`) |
| `0390-post-release-alignment-handoff.md` | Post-release truth-alignment handoff (not tied to a single new RFC): determined via crates.io (never `cargo publish`) that `matten-stats` was published at `0.39.0` despite RFC-079 §3's deferral decision (Case B); corrected RFC-079/RFC-078 status, added a dated CHANGELOG note without erasing the original entry, and added a new ROADMAP history row leaving `3.25.0` intact; documentation only, no code/version/tag/publish action |
| `080-matten-mlprep-production-ready-handoff.md` | Accepted RFC-080 handoff; promotes `matten-mlprep` candidate → production-ready, closing RFC-058 §5.1's Option B exit criterion via RFC-077; review corrected an over-broad seven-file sites list to six verified sites, implementation found a seventh (`src/lib.rs`'s Status doc comment); added and proved a new "must not say candidate" guard; label-only, no code/API/version/release |
| `081-v1-family-experimental-crate-policy-handoff.md` | Rereviewed RFC-081 handoff (GO, one fix applied); refreshes RFC-076's stale inventory. First draft's five-site list was replaced with an actual sweep-and-classify pass (17 real sites, not 5); implemented and committed. RFC-076 stays in `proposed/`, now blocked on both `matten-stats` resolving an RFC-081 Exit A/B decision and RFC-081 §5's own reasoning receiving the owner's disposition (external read, narrowed rule, or accepted as argued) |
| `082-streaming-csv-batches-handoff.md` | Accepted RFC-082 handoff, after a first draft's mistaken guard-narrowing claim was fact-checked and dropped (`scripts/check-streaming-scope.sh` already permitted `CsvBatchReader` unmodified — verified via regex test, example-name test, and an end-to-end fixture run). Implements `CsvBatchReader` behind `matten-data`'s off-by-default `streaming` feature; 13 new tests including an equivalence test and a batch-boundary line-number-parity test; no code change to the other four crates, no version bump, no release |
| `083-matten-stats-expansion-handoff.md` | Accepted RFC-083 handoff; implemented and reviewed (approved, no corrections). Adds `covariance_population`, `skewness`, `kurtosis` to `matten-stats` (3 public functions → 6). Additive only: no new error variant, no new dependency, no feature gate, no version bump, no release, and no maturity change — `check-release-docs.sh` (lines 120/124) asserts the `Experimental` label and must keep passing unmodified. Opens with a mandatory pre-coding check: RFC-083 §4.1's SciPy/pandas estimator-default claims are explicitly marked unverified, and a mismatch is an escalation, not a local fix |
| `084-promote-matten-stats-production-ready-candidate-handoff.md` | Accepted RFC-084 handoff; implemented and reviewed (approved after one correction — the promotion guard's first check was over-broad and rejected legitimate maturity-history prose). Part of PART 1 landed before acceptance and was authorized retroactively. Two ordered parts: Part 1 adds the missing `matten-stats` CI job (including `--features dynamic`, without which the crate's one dynamic test stays MSRV-only) and its four example smoke runs; Part 2 moves the label and inverts `check-release-docs.sh`'s `Experimental` assertion rather than deleting it. Requires a deliberate-failure proof that the inverted guard can actually fail, and requires deriving the label-site list by sweep-and-classify rather than trusting the handoff's enumeration — three prior RFCs shipped incomplete site lists. `rfcs/done/`, `rfcs/handoffs/`, `CHANGELOG.md` and `docs/design/history/` are must-not-touch |

## Documentation-governance handoffs

These translate the specs supersession analysis into design-team work. They are not tied to a
single RFC; their authority is the supersession map. Run in order: 01 (gap closure) before 02
(archival); 03 (philosophy) may run after 01.

| Handoff | Purpose |
|---|---|
| `docs-governance-01-spec-coverage-gap-closure-handoff.md` | Resolve the three unowned spec fragments (perf targets, golden/fuzz/property tests, `Display`) before archival |
| `docs-governance-02-spec-archival-and-ownership-rule-handoff.md` | Archive the v0.19.0 specs as tracked banner-marked history; declare the ownership rule |
| `docs-governance-03-philosophy-distillation-handoff.md` | Expand `docs/src/philosophy.md` into a guarded, evergreen principles page distilled from the specs |

Some handoffs are planning records and some are retained implementation records.
A handoff alone does not authorize new work; each new implementation slice still
requires the corresponding RFC acceptance or follow-up review before coding begins.
