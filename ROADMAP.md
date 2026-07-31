# `matten` ROADMAP

**Project:** `matten`  
**Document Kind:** Canonical Project Roadmap  
**Document Version:** `3.51.0`
**Date:** 2026-07-31
**Status:** `0.41.0` is **released, tagged, and published to crates.io** (RFC-089) — all five crates
live at `0.41.0`, verified against the registry after publication, and the release matched the planned
scope exactly with no post-release correction needed. The tag is signed and sits on the *Prepare*
commit per the convention `0.37.0`–`0.40.0` established; 101 of 101 tags resolve to ancestors of
`origin/main`. `0.41.0` publishes the first two themes chosen against §1.1's planning baseline:
RFC-087's `repeat`/`repeat_axis`/`tile`/`meshgrid` and RFC-088's negative `slice_str` indices, both
core public-surface additions with no maturity-label, dependency, feature, or MSRV change —
`matten`, `matten-ndarray`, `matten-mlprep`,
`matten-data`, and `matten-stats` are all live at `0.41.0`. The preceding `0.40.0` published the accumulated content of RFC-082 through RFC-085 — `matten-data`'s `CsvBatchReader`, `matten-stats`'s `covariance_population`/`skewness`/`kurtosis`, and three maturity promotions — recapped in full later in this paragraph (RFC-086) and in the `v0.40.0` release-table row; `0.39.0`'s own content and its `matten-stats` publication divergence are recorded at rows `3.25.0`/`3.26.0`, not restated here. `matten-mlprep` is promoted to **production-ready** (RFC-080), closing RFC-058 §5.1's Option B exit criterion via RFC-077's seeded split — label/docs only, no code, API, version, or release change. `matten-data` was subsequently promoted to **production-ready** (RFC-085), closing RFC-059 §6's deferred full-production review — label/docs/guard only, no code, API, version, or release change. `matten-stats` was expanded to six functions (RFC-083) and then promoted to **production-ready candidate** (RFC-084), discharging RFC-081 §3's Exit A — label/CI/docs only, no code, API, version, or release change. `matten-data` gains **row-count-bounded batched CSV reading** (`CsvBatchReader`, RFC-082), behind an off-by-default `streaming` feature that implies `csv` — no new dependency, no version bump. This reopens streaming/large-CSV, deferred since RFC-026/RFC-037, by answering all six of RFC-037 §4's reopening criteria; it deliberately ships no `matten-stream` crate (a structural argument, not just a cost one: such a crate would need `Table`, which lives in `matten-data`, making it a companion-to-companion dependency RFC-078 §6 already forbids). RFC-070 remains closed without a public report/viz crate or API, and RFC-072 remains closed after behavior-preserving `matten-report` modularization. RFC-074 and RFC-075 are closed in `done/`. RFC-076 (v1.0 release preparation) is reviewed and accepted (GO, no conditions), but its execution is **deferred**: an implementation attempt was made and fully reverted at the owner's explicit instruction (it proceeded without the owner's direct confirmation of the version bump), and no RFC-076 implementation is currently authorized. **RFC-081 answers the question RFC-067 left open — no crate labelled `Experimental` may ship in a lock-step `1.0.0` family; `matten-stats` must take Exit A (promote) or Exit B (remove from lock-step) first** — and its mechanical RFC-076 inventory refresh (17 sites, corrected from an initial five-site draft that review found badly incomplete) is applied: five-crate family, `matten-mlprep` now shown production-ready, a `matten-stats` row added and marked blocked, and RFC-081's precondition sentence added to RFC-076 itself. **Both of RFC-076's blockers now have owner decisions (2026-07-29).** `matten-stats` took **Exit A (promotion)**, executed by RFC-083 (surface expansion) and RFC-084 (the promotion itself, discharging RFC-081 §3's precondition); and RFC-081 §5 is rewritten to rest the rule on a checkable contradiction between two of the project's own documents (`crates/matten-stats/src/lib.rs:32-33`'s *"surface may still change"* against `compatibility.md`'s `v0.x`-scoped breaking-change permission) rather than on single-party reasoning about what `Experimental` ought to mean — RFC-081 is closed and moved to `done/`. RFC-076 nonetheless remains **deferred and unauthorized**: RFC-081's precondition is discharged, but **v1.0 is not currently wanted** — a separate, unrelated reason. Pre-v1 feature work (RFC-077, RFC-078) is complete and closed; RFC-079 sequenced and shipped their release. No v1.0 tag, publish, or implementation is authorized. Broader mathematics, ecosystem bridges, public schemas, public renderer APIs, public report/viz crates, SVG, and Vega-Lite remain unauthorized. **RFC-086 then prepared, tagged, and published `0.40.0`** — the lock-step family bump publishing RFC-082 through RFC-085's accumulated content (`CsvBatchReader` and three new `matten-stats` functions; the `matten-mlprep`/`matten-stats`/`matten-data` maturity promotions, none of which had been visible to a user before this release) — after finding that nothing in the RFC process ever asks whether an accumulation of "no release" slices has itself become release-worthy. **The `0.38.0`/`0.39.0` orphaned-tag defect that had blocked tagging `0.40.0` was repaired during that release**: both were re-tagged onto their correct commits, `main` was pushed to `origin` (previously stalled at `0.37.0`), and all 100 remote tags now resolve to ancestors of `origin/main` with the GPG-signed invariant intact. RFC-087 then added `repeat`/`repeat_axis`/`tile`/`meshgrid` (eight functions, the first core public-surface change since `0.38.0`) and RFC-088 added negative `slice_str` indices, both left unreleased behind `0.40.0` under the release-readiness check recorded at `3.39.0` — the first answered "not yet" with a written trigger (one further feature slice), the second answered "yes" once that trigger fired. **RFC-089 now prepares `0.41.0`** to release both, with no blocking precondition this time. No code, API, dependency, feature, or MSRV change beyond the version bump itself; no tag, no publish.
**Planning Baseline:** core `matten` completed RFC-015 through RFC-021 (shipped through v0.15.3); RFC-022 through RFC-030 established the companion-crate and lock-step family model; RFC-033 through RFC-042 materialized the v0.20+ data, comfort-API, statistics, linalg-lite, and guard work; RFC-043 through RFC-048 completed the additive examples program; RFC-049 implemented benchmarking Phases 1-3 and extracted hard gates to future policy ownership; RFC-050 through RFC-054 completed production-migration documentation and local advisory tooling; RFC-055 through RFC-062 completed result-form APIs, companion maturity promotions, benchmark-doc surfacing, and the ndarray 0.17 update; RFC-063 through RFC-069 completed the reviewed local visualization/reporting line through `0.36.0`; RFC-071 shipped private fixed-demo JSON in `0.37.0`; RFC-070 closed public visualization/report readiness without public implementation; RFC-072 completed behavior-preserving modularization of the local report tool; RFC-073's private input-mode JSON implementation shipped in `0.38.0`; RFC-074's v1.0 readiness re-audit found the technical/API surface conditionally ready and was accepted; RFC-075 resolved MD-2, declared the serde format stable, and recorded the RFC-067 family maturity table, and was accepted and closed; RFC-076 specifies the `1.0.0` release-preparation change (family maturity table reproduced, compatibility.md rewrite, pre-1.0/0.x documentation sweep, current-family version retarget, version bump) and was reviewed and accepted (GO, no conditions) after two review rounds and two recorded maintainer decisions (covering the `#[doc(hidden)]` slice-plumbing items under the 1.0 promise; not running `cargo public-api` for this release); its execution was attempted, reverted at the owner's explicit instruction, and is now deferred pending pre-v1 feature work under RFC-077 and RFC-078, both now closed; RFC-077 (`train_test_split_seeded` for `matten-mlprep`) is implemented, reviewed (GO, no conditions, reproducibility contract proven by mutation), and closed; RFC-078 (`matten-stats` companion, fifth published crate) is implemented, reviewed (GO, no conditions, `ddof = 1` policy proven by mutation), and closed — RFC-076 must be updated to account for five crates before it can be executed; RFC-079 sequenced the release of both features, reviewed and accepted (GO, no conditions, after one correction protecting two historical records from an over-broad version-string retarget), with the owner's scope narrowing that `0.39.0`'s release-prep releases RFC-077 only, deferring `matten-stats`'s first publication pending an external `ddof` read; `0.39.0` was then tagged and published outside this project's assistant session, and the actual publish action included `matten-stats` despite that deferral, which the post-release alignment corrects without rewriting the original decision's record; RFC-080 then promoted `matten-mlprep` to production-ready, closing RFC-058 §5.1's Option B exit criterion via RFC-077 — label/docs only, no code or release; RFC-081 then decided that no `Experimental`-labelled crate may ship in a lock-step `1.0.0` family (`matten-stats` must take Exit A or Exit B first) and refreshed RFC-076's stale inventory accordingly — mechanical refresh applied and committed, the policy's own §5 reasoning subsequently rewritten onto a checkable contradiction and the RFC closed to `done/`; RFC-082 then reopened streaming/large-CSV (deferred since RFC-026/RFC-037), adding `CsvBatchReader` to `matten-data` behind an off-by-default `streaming` feature, with no new dependency, no `matten-stream` crate, and no version bump. Under lock-step family versioning (RFC-030), every crate shares the family version; maturity is expressed by per-crate Status labels, not by separate version numbers.

---

## 0. Authority and purpose

This `ROADMAP.md` is the canonical roadmap for v0.16 and later.

When documents disagree, resolve in this order:

1. accepted RFC for the specific topic;
2. external design public contract;
3. this roadmap and milestone gates;
4. requirements documents;
5. drafts, prototypes, and discussion memos.

The v0.16+ prospect supersedes older schedule lines that placed `matten-data` at v0.17 and bundled all bridge crates at v0.19.

**RFC numbering note:** RFC-032 is reserved/consumed by another issue. New v0.20+ roadmap RFCs therefore begin at **RFC-033**. The examples program follows the v0.20+ boundary/design RFCs and uses **RFC-043 through RFC-048**.

---

## 0.1 Documentation-governance track

The v0.19.0 requirements, external-design, and roadmap snapshot documents are
historical inputs, not current authority. The tracked docs-governance handoffs
close their remaining value in this order:

```text
1. docs-governance-01-spec-coverage-gap-closure-handoff.md
   Resolve the three unowned spec fragments before archival:
   non-binding performance targets, golden/fuzz/property testing status, and Display formatting.
   Status: implemented and reviewed in docs/design/coverage-gap-resolution.md.

2. docs-governance-02-spec-archival-and-ownership-rule-handoff.md
   Archive the v0.19.0 specs as tracked history and write down the ownership rule.
   Status: implemented and reviewed in docs/design/README.md and docs/design/history/.

3. docs-governance-03-philosophy-distillation-handoff.md
   Distill the tracked archived specs into an evergreen Philosophy page after archival exists.
   Status: implemented and reviewed in docs/src/philosophy.md.
```

The intended ownership model is:

```text
rfcs/                = normative decisions
docs/src/            = user-facing evergreen contract and positioning
ROADMAP.md           = forward schedule and milestone history
docs/design/history/ = dated historical design snapshots only
```

These handoffs are tracked in `rfcs/handoffs/README.md`. They are docs/design
work only: no public API, dependency, version, or release-scope change.
The tracked ownership rule lives in `docs/design/README.md`; the v0.19.0 snapshots live under
`docs/design/history/` and must not be cited as current authority.

---

## 1. Long-term positioning

`matten` core remains a **Sedan-first** Rust tensor library:

- one primary public type: `Tensor`;
- concrete `f64` numeric computation by default;
- clear shape, broadcasting, slicing, reduction, and matrix APIs;
- dynamic ingestion/cleanup as an explicit on-ramp;
- boundary-safe `Result` APIs for parsing and I/O;
- readable panic messages for local mathematical misuse.

The core crate is **not** a dataframe engine, ML framework, streaming engine, GPU backend, or wrapper around external numeric crates.

Companion crates may extend workflows, but they must remain optional and must not pollute the dependency graph of core `matten`.

### 1.1 Project objective — planning baseline

**Stated by the owner, 2026-07-30, in the §3.3 replanning discussion.** This is the planning baseline
Phase 0 requires; the project had operated without a written objective until now.

```text
WHAT IT IS      A "family car" around Tensor and matrix work.
INTENDED USE    Education, learning, proof-of-concept, prototyping.
ADOPTION        Explicitly NOT a success measure.
TARGET WINDOW   None yet.
```

This confirms rather than redirects §1's existing **Sedan-first** framing: a family car is reliable,
approachable and everyday — not a race car (performance) and not a truck (production scale). The
established philosophy already matches it — one primary type, explicit numeric conversion, readable
errors, a small teachable surface, and the standing refusals to become a dataframe engine, ML
framework, or GPU backend.

**Prioritisation criteria that follow.** Favour work that a learner or prototyper meets early and
often; treat examples, error messages and documentation as product rather than overhead; prefer an
obvious default with a documented rationale over a configurable policy. Deprioritise performance
work, production-scale concerns, and ecosystem breadth.

**Consequence for adoption-derived reasoning — flagged, not silently fixed.** Because adoption is not
a success measure, download counts carry no signal and should not be cited as evidence for or against
any decision. One existing rule now sits at odds with this baseline: RFC-084 §3 rested
`matten-stats`'s candidate-only ceiling on its lack of *usage history*, a criterion this project does
not measure and does not intend to satisfy. Under this objective that ceiling is permanent by
construction rather than by evidence. Recorded here as a known tension; resolving it needs its own
RFC and the owner's decision, not an edit in passing.

Use this rule for every new proposal:

> If the feature makes `Tensor` itself simpler, safer, clearer, or easier to construct/inspect/clean/explicitly convert, it may belong in core `matten`.  
> If it adds table semantics, ML semantics, external framework dependencies, streaming lifecycle, domain workflow, or bridge behavior, it belongs in a companion crate.

Good dependency direction:

```text
matten-ndarray -> matten
matten-mlprep  -> matten
matten-data    -> matten
```

Forbidden dependency direction:

```text
matten -> matten-ndarray
matten -> matten-mlprep
matten -> matten-data
matten -> ndarray
matten -> nalgebra
matten -> candle-core
matten -> polars
matten -> arrow
matten -> datafusion
```

---

## 3. v0.16+ release themes

| Version | Theme | Primary milestone | Implementation posture |
|---|---|---|---|
| **v0.16.0** | Companion boundary confirmation | RFC-022 policy, workspace structure, dependency-boundary CI | Core policy + mechanics only |
| **v0.17.0** | `matten-ndarray` experimental | First low-risk companion crate | Small conversion implementation |
| **v0.18.0** | `matten-mlprep` experimental | Transparent numeric preprocessing | Small helper implementation |
| **v0.19.0** | Companion maturity hardening | `matten-ndarray` production-ready candidate; `matten-mlprep` beta decision | Hardening / QA / docs |
| **v0.19.1** | Companion hardening patch | RFC-031 feature-robust dynamic rejection; RFC lifecycle/doc cleanup | Patch / quality release |
| **v0.19.2** | Companion dependency/import policy | RFC-032: explicit dependency style confirmed; companion `pub use matten;` deferred; release-doc guard added | Documentation/tooling patch |
| **v0.19.3** | v0.20+ planning materialization | RFC-033–042 added as proposed design set; ROADMAP reconciled to lock-step + RFC-032; architect rulings applied | Documentation/planning patch |
| **v0.20.0** | v0.20+ design/materialization start | RFC-033 `matten-data` experimental scaffold (shell only); workspace member + dependency pins | Design + selective implementation approval |
| **v0.20.1** | `matten-data` table API | RFC-034 + RFC-035 implemented: `Table`/CSV ingestion/schema/numeric → `Tensor` (Experimental); `examples/csv_to_tensor.rs` shipped | Low-risk implementation |
| **v0.20.2** | Examples program planning | RFC-043–048 added as proposed examples RFC set + compact handoff; reconciled to the additive 30+ band, dedup against the existing suite, and shipped `matten-data` | Documentation/planning patch |
| **v0.20.3** | Examples: structure + beginner band | RFC-043 example structure/policy + RFC-044 beginner examples (`30_`–`32_`: magic square, Fibonacci-by-matrix, graph path counting); docs + smoke runs | Additive examples/docs |
| **v0.20.4** | Examples: matrix-iteration band | RFC-045 examples (`33_` Markov chain, `34_` tiny PageRank); docs + smoke runs | Additive examples/docs |
| **v0.20.5** | Benchmarking program planning | RFC-049 added as proposed (benchmark harness, complexity metrics, positioning report); ROADMAP Track D added | Documentation/planning patch |
| **v0.20.x** | Minimal implementation phase | Small core comfort APIs; new 30+ famous-problem examples; audit/improve existing companion examples | Low-risk implementation only |
| **v0.21.0** | Shape composition (RFC-039) | `concatenate` + `stack` in core (borrowed slices, try_/panic, MattenLimits, dynamic-reject); repeat/tile/meshgrid deferred | Additive core API (v0.21 boundary review) |
| **v0.21.1** | Linalg core-lite (RFC-041) | `norm` (L2/Frobenius), `trace` (rectangular via min(rows,cols)), `outer`; decomposition/BLAS/sparse rejected | Additive core API |
| **v0.21.2** | Statistics core (RFC-040) | `var`/`std` + `var_axis`/`std_axis`, population variance (ddof=0), two-pass; quantile/histogram/cov/corr deferred | Additive core API |
| **v0.21.3** | matten-data scope guard (RFC-042) | Three-check release-docs guard (filename / public-API identifier / non-goal context); may land earlier | Docs/tooling |
| **v0.21.4** | Release-truth & CI-gate patch | v0.21.3 deep-review P1 fixes: 0.20→0.21 doc drift, family-aware release-docs guard wired into CI | Docs/tooling |
| **v0.22.0** | **`matten-data` promoted to Beta** | RFC-036 six-example suite (`data_00`–`data_05`) + explicit malformed-CSV test cleared the RFC-023 §9 gate; status Experimental→Beta; `data.md` guide; guards/CI updated. No library/API change | Maturity milestone |
| **v0.22.1** | RFC-049 Phase 1 — internal benchmark baseline | Accepted RFC-049 (staged). Added isolated `benchmarks/` criterion harness (workspace-excluded, publish=false) + methodology docs + baseline report template; core micro set & 5 scenario workloads; boundary guard now forbids criterion in core; CI compile-checks harness only (no speed gates). Phases 2–4 deferred. No published-crate code change | Tooling/docs |
| **v0.22.2** | Lifecycle wording cleanup | v0.22.0 handoff-review P2 follow-up: RFC-023 §9 gains a clarification that the malformed-CSV criterion is met by a structured-error/no-panic test (Csv or RaggedRow, never panic/silent), not a parser-error test; RFC-036 note updated. Historical CHANGELOG/ROADMAP entries left intact. No code/API/guard/CI change | Docs/lifecycle |
| **v0.22.3** | RFC-032 scope carve-out + published-dep isolation guard | Benchmarking/positioning review follow-up. RFC-032 §5.1 records that workspace-excluded publish=false tooling (RFC-031 fixture, RFC-049 harness) is outside the published-family convention. Added scripts/check-published-dependency-isolation.sh (per-crate peer-dep matrix; matten-ndarray→ndarray allowed) wired into CI + checklist. RFC-049 Phase 2 design settled & annotated (B1–B4) but NOT implemented; added BASELINE-READY-CHECKLIST. No library/API change | Docs/tooling |
| **v0.32.0** | Rich local visualization artifacts | RFC-068 implemented: static local HTML artifacts for `tools/matten-report --demo educational-path` and `tools/matten-report --demo shape-flow`, preserving Markdown default and deferring public crates/SVG/Vega-Lite/expression tracing | Local-tool visualization artifact |
| **v0.33.0** | Visualization continuation release | RFC-068 continuation implemented: static local HTML artifact for `tools/matten-report --demo dynamic-readiness`, preserving Markdown default and deferring data-readiness/input-mode HTML, public crates, SVG/Vega-Lite, and expression tracing | Local-tool visualization artifact |
| **v0.34.0** | Visualization continuation release | RFC-068 continuation implemented: static local HTML artifact for `tools/matten-report --demo mlprep-standardization`, preserving Markdown default and deferring data-readiness/input-mode HTML, public crates, SVG/Vega-Lite, and expression tracing | Local-tool visualization artifact |
| **v0.35.0** | Visualization continuation release | RFC-068 continuation implemented: static local HTML artifact for `tools/matten-report --demo data-readiness`, completing fixed-demo HTML coverage while preserving Markdown default and deferring input-mode HTML, public crates, SVG/Vega-Lite, and expression tracing | Local-tool visualization artifact |
| **v0.36.0** | Input-mode HTML release | RFC-069 implemented: static local HTML artifact for `tools/matten-report --input ... --kind data-readiness`, preserving Markdown default and keeping output summary-only, bounded, escaped, and explicit-file-only | Local-tool visualization artifact |
| **v0.37.0** | Fixed-demo private JSON reports | RFC-071 released: private `tools/matten-report --demo ... --format json --output <path>` artifacts for the five fixed demos, preserving Markdown default and deferring input-mode JSON, public schemas, public crates, SVG/Vega-Lite, and expression tracing | Local-tool visualization artifact |
| **v0.38.0** | Input-mode private JSON reports | RFC-073 released: private `tools/matten-report --input <csv> --kind data-readiness --format json --output <path>` artifacts for successful and bounded strict-conversion-error outcomes, preserving Markdown default, byte-identical fixed-demo JSON, and deferring input-mode JSON for other kinds, raw CSV export, public schemas, and public crates | Local-tool visualization artifact |
| **Post-v0.38.0** | Report-tool policy and maintainability | RFC-070 public-readiness audit, RFC-072 modularization, and RFC-073 input-mode JSON all closed/released | Next decision: further report-tool scope requires a separate RFC |
| **Post-v0.38.0** | v1.0 readiness re-audit | RFC-074 re-audit of RFC-066 accepted (`docs/design/v1-readiness-audit.md`, `done/`): public API/error/boundary surface conditionally ready (zero churn across eight releases); deferred stats/linalg found to be settled scope decisions, not blockers; streaming remains additive, not a blocker; companion maturity promotion not evidenced; NF-1/NF-2 closed | Owner chose Path B; RFC-075 proposed to resolve MD-2, no implementation, version bump, or release authorized |
| **Post-v0.38.0** | v1.0 release decision | RFC-075 accepted and closed: resolves MD-2 (keeps RFC-030 lock-step versioning, adds a CHANGELOG justification requirement for local-tool-only releases, now recorded in the release checklist and CHANGELOG conventions, not only the RFC), declares the JSON canonical serde format stable, records the RFC-067 family maturity table (matten-mlprep/matten-data included at candidate label, not promoted) | Decision-only, closed; a separate future v1.0 release-prep RFC (Unit 2) required before any release action |
| **Post-v0.38.0** | v1.0 release preparation | RFC-076 reviewed and accepted (GO, no conditions) after two review rounds: reproduces the RFC-067 family maturity table, rewrites compatibility.md, sweeps 19 pre-1.0/0.x sites across 9 files, retargets 29 current-family version strings across 14 files, bumps `0.38.0` -> `1.0.0`. Covers the three `#[doc(hidden)]` slice-plumbing items under the 1.0 promise; records cargo public-api as not required for this release, with rationale | Accepted but **execution deferred**: an implementation attempt was reverted at the owner's explicit instruction; no implementation currently authorized |
| **Post-v0.38.0** | Pre-v1 feature work | RFC-077 (`train_test_split_seeded` for `matten-mlprep`, closing the RFC-076 §5 maturity caveat without promoting the crate) and RFC-078 (`matten-stats` companion: `covariance`/`correlation`/`quantile`, fifth published crate at Experimental maturity) both implemented, reviewed (GO, no conditions), and closed, on the `0.38.x` line while RFC-076 stays accepted-but-unexecuted | Both closed; no version bump for either theme; RFC-076 must be updated for a five-crate family before it can be executed |
| **v0.39.0** | Pre-v1 feature release | RFC-079 reviewed and accepted (GO, no conditions): bumped the lock-step family version `0.38.0` -> `0.39.0`, retargeted current-family version strings, and released `train_test_split_seeded` (RFC-077). Tagged and published outside this project's assistant session; `matten-stats` (RFC-078) was published alongside it at Experimental maturity, diverging from RFC-079 §3's decision to defer that crate's first publication pending an external `ddof = 1` read — see the post-release alignment correction | **Released.** `matten`, `matten-ndarray`, `matten-mlprep`, `matten-data`, and `matten-stats` all published at `0.39.0`; the external `ddof` read now informs a possible future change rather than gating an already-completed first publication |
| **v0.40.0** | Feature and maturity release | **Released, tagged, and published** — all five crates live at `0.40.0`, matching the planned scope exactly. RFC-086 reviewed and accepted (GO): bumped the lock-step family version `0.39.0` -> `0.40.0`, retargeted 37 current-family version strings across 17 files (one more than §6 measured; see `3.38.0`), and published the accumulated content of RFC-082 through RFC-085 — `CsvBatchReader` and three new `matten-stats` functions, plus the `matten-mlprep`/`matten-stats`/`matten-data` maturity promotions, none of which had been user-visible before this release. Motivated by finding that no step in the RFC process ever asks whether an accumulation of individually-no-release slices has itself become release-worthy (RFC-086 §2); RFC-086 §10 proposes closing that gap as a future amendment, not adopted by this release | **Released, tagged, and published.** The `0.38.0`/`0.39.0` orphaned-tag defect was repaired during this release (re-tagged onto their correct commits; all 100 remote tags now resolve to ancestors of `origin/main`), discharging the precondition this row previously recorded as blocking |
| **v0.41.0** | Core shape and slicing release | **Released, tagged, and published** — all five crates live at `0.41.0`, matching the planned scope exactly; no post-release correction needed. RFC-089 reviewed and accepted (GO, no conditions): bumped the lock-step family version `0.40.0` -> `0.41.0`, retargets 37 current-family version strings across 17 files (measured with `0\.40\b`, the corrected pattern from `3.38.0`), and releases RFC-087 (`repeat`/`repeat_axis`/`tile`/`meshgrid`, eight functions) and RFC-088 (negative `slice_str` indices), the first two themes chosen against §1.1's planning baseline. No maturity-label, dependency, feature, edition, or MSRV change; no `Maturity` CHANGELOG section, since none changed | **Prepared, not tagged or published.** No blocking precondition this time — unlike `0.40.0`, whose orphaned-tag defect is already repaired |
| **v0.21+** | Selective production readiness | `matten-ndarray` promoted to production-ready (RFC-057); `matten-mlprep` and `matten-data` promoted to production-ready candidate (RFC-058/RFC-059); remaining full-production companion decisions require separate review. RFC-049 Phases 1-3 are implemented; Phase 4 hard gates are extracted to future policy/RFC ownership | Per-crate decisions |
| **Post-v0.39.0** | `matten-mlprep` full production-ready | RFC-080 reviewed and accepted (GO, conditional on three corrections: a corrected six-site list — the self-authored draft was over-broad by four files and short by three sites, review found both; a new `check-release-docs.sh` guard mirroring `matten-ndarray`'s, proven to fail on a reintroduced violation; RFC-076's resulting maturity-table staleness recorded, not fixed). Closes RFC-058 §5.1's Option B exit criterion (the ordered-only-split caveat) via RFC-077. Label/docs only across the crate's own README/`src/lib.rs` banner, the root README table, `compatibility.md`, and `rfcs/README.md` — no code, test, example, dependency, MSRV, or version change | **Promoted.** `matten-data` stays candidate for its own reasons; `matten-stats` stays Experimental |
| **Post-v0.39.0** | `Experimental` crates in a v1.0 family (RFC-081) | Rereviewed (GO, conditional on one fix, applied): decides no lock-step `1.0.0` family may include an `Experimental` crate (`matten-stats` must take Exit A — promote via its own RFC — or Exit B — remove from lock-step via an RFC amending RFC-030). Refreshes RFC-076's stale inventory: a first five-site draft was replaced, after review found it badly incomplete, with an actual sweep-and-classify pass yielding 17 real sites (family size, `matten-mlprep`'s label, a new `matten-stats` row, and — the most consequential single fix — a per-crate reasoning block that argued for admitting `matten-mlprep` *as a candidate*, an argument RFC-080's actual promotion had already mooted, rewritten rather than word-swapped). Implemented and committed; no code, version, or release change | **Mechanical refresh applied.** RFC-076 stays in `proposed/`, now blocked on `matten-stats` resolving its exit AND on RFC-081 §5's own reasoning receiving the owner's disposition (external read / narrowed rule / accepted as argued) — neither resolved yet |
| **Post-v0.39.0** | Streaming CSV batches (RFC-082) | Reviewed and accepted, after a first draft's central technical claim (that `scripts/check-streaming-scope.sh` needed narrowing for `CsvBatchReader`) was fact-checked and dropped — the guard already permitted the exact proposed surface unmodified, confirmed both by isolated regex/example-name tests and an end-to-end run against the real implementation. Adds `CsvBatchReader::{open, next_batch}` to `matten-data` behind an off-by-default `streaming` feature (implies `csv`, no new dependency), answering all six of RFC-037 §4's reopening criteria and rejecting a `matten-stream` crate on structural grounds (it would need `Table`, forcing a companion-to-companion dependency RFC-078 §6 forbids). 13 new tests incl. an equivalence test and a batch-boundary line-number-parity test | **Implemented and committed.** Feature-off build unchanged; other four crates untouched; no version bump or release |
| **Later** | Public report/viz implementation follow-up, JSON/SVG/Vega-Lite output, async CSV streaming, streaming numeric conversion, a `matten-stream` crate, `nalgebra`, `candle`, broader stats/linalg companions | Separate RFCs or reviewed handoffs required | Design-only until reopened |

> **Performance-watch (P2, not a release blocker).** The RFC-049 Phase 1 internal baseline
> showed `sum_mean_axis` (~1.31 ms on 64×64) is the most expensive core path by a wide
> margin — ~400× the whole-tensor `sum_mean` and ~17× a 64×64 `matmul`. Recorded as a
> regression-visibility anchor: investigate the axis-reduction implementation cost only if
> benchmarks or real user workflows show axis reductions becoming a practical bottleneck.
> `matten` is DX-first, not a performance crate, so this is not a fix-now item, and Phase 2
> was not blocked on it (architect ruling, 2026-06-24).

---

## 3.1 Candidate themes — recorded, NOT authorized

**Status: a recorded inventory, not a roadmap.** These are the open themes surviving in the RFC
corpus as of `3.34.0` (2026-07-30), gathered so the owner can select from a written list rather than
from recall. Nothing here is approved, scheduled, or authorized; selecting a theme is a joint
planning decision (org policy §6.1), and the high-capability model must not adopt any of these
unilaterally. Entries are removed as they are taken up or explicitly dropped.

### Advancement (maturity / version)

| Theme | Authority | State |
|---|---|---|
| `matten-stats` → full production-ready | RFC-084 §8 | **Not viable.** RFC-084 committed in writing that its reasoning supports the candidate rung only, because the crate has no usage history. Revisit only when that changes |
| `1.0.0` release | RFC-076, RFC-081 | Unblocked on RFC-081 §3 (Exit A discharged by RFC-084); deferred solely because v1.0 is not currently wanted. Timing is the owner's alone (§6.7) |
| RFC-081 §5 external read | RFC-081 | Still not obtained. D2 narrowed the rule so it is no longer load-bearing, but the read itself was never done |

### Functions — core `matten`

| Theme | Authority | State |
|---|---|---|
| `is_empty()` | compatibility.md | Deferred; zero-sized dims are rejected by the shape model, so it would always be false |
| Batched matmul (rank > 2) | RFC-010, compatibility.md | Deferred; core caps at `[m,n]×[n,p]` |
| Dynamic slicing via the builder | compatibility.md | Deferred; `slice().build()` is numeric-only |
| Public mutation API exposing CoW | RFC-012 | Deliberately deferred; internal Arc-shared CoW is implemented, the public surface is not |

### Functions — `matten-stats`

| Theme | Authority | State |
|---|---|---|
| Matrix-wide / axis-wise covariance and correlation | RFC-083 §6, RFC-090 §5 | Still blocked, and now more explicitly. RFC-090 §5 amended the crate boundary to *"scalar where scalar, a small owned struct where inherently vector-valued, never a `Tensor`"* — the amendment was written so that it does **not** unblock these forms, which return a `Tensor`. Needs its own boundary RFC, and that RFC must argue against the "never" clause rather than around the old `Tensor -> f64` phrasing it replaced |
| z-score, percentile aliases, mode | RFC-083 §6 | Rejected with reasons (z-score belongs to `matten-mlprep`; aliases are sugar; mode is ill-defined for `f64` without a binning or tolerance policy). Reopening requires new information |

### Functions — linear algebra

| Theme | Authority | State |
|---|---|---|
| inverse / determinant / decomposition / BLAS / sparse | RFC-041 | Rejected **for core**. Whether a linalg companion should exist at all is the unanswered prior question — a boundary RFC before any function work |

### Functions — streaming / large CSV

| Theme | Authority | State |
|---|---|---|
| async, resumability, backpressure, parallel reading, lenient/skip-malformed modes, schema inference, streaming numeric conversion, CSV writing, other formats, a `matten-stream` crate | RFC-082 §5 | All deferred by RFC-082 itself. `matten-stream` was rejected on structure, not cost: it would need `Table`, forcing a companion-to-companion dependency RFC-078 §6 forbids |

### Ecosystem bridges

| Theme | Authority | State |
|---|---|---|
| `matten-nalgebra` | RFC-025, RFC-041 | Deferred; per-crate RFC required |
| `matten-candle` | RFC-025, RFC-049 | Deferred; per-crate RFC required |

### Reporting / visualisation

| Theme | Authority | State |
|---|---|---|
| Public `matten-report` / `matten-viz` crate or API | RFC-070 | Closed after audit; no public crate or API authorized |
| More input-mode HTML paths; other JSON input kinds; raw CSV export; public schemas; SVG; Vega-Lite | RFC-069, RFC-071, RFC-073 | Unauthorized |

### Infrastructure and hygiene

| Theme | Authority | State |
|---|---|---|
| Benchmark hard gates (Phase 4) | RFC-049 | Extracted to future RFC/release-policy ownership and never reclaimed |
| Property / fuzz testing | RFC-013 | Recorded as aspirational, explicitly not a current gate |

## 4. v0.16.0 milestone: companion boundary confirmation

### Goal

Make the companion-crate model concrete without expanding core `matten`.

### Required work

- Implement RFC-022 as policy and project mechanics.
- Decide workspace layout.
- Define the workspace versioning model (independent per-crate SemVer initially;
  superseded by lock-step family versioning in v0.19.0, RFC-030).
- Define companion error-type policy.
- Define maturity labels.
- Add mechanical dependency-boundary CI.
- Mark old in-core bridge examples/features as superseded.
- Update RFC-023 through RFC-026 target headers to match this roadmap.

### Acceptance gate

`v0.16.0` is complete only if all of the following hold:

```text
[ ] core matten has no direct dependency on ndarray/nalgebra/candle/polars/arrow/datafusion
[ ] core matten has no dependency on matten-* companion crates
[ ] companion crate policy is documented in RFC-022
[ ] ROADMAP.md is the canonical future schedule
[ ] external design bridge sections are marked superseded
[ ] users can still ignore all companion crates
```

### Explicit non-goals

- No dataframe API in core.
- No ML preprocessing API in core.
- No external bridge API in core.
- No streaming CSV API in core.

---

## 5. v0.17.0 milestone: `matten-ndarray` experimental

### Goal

Prove the companion-crate pattern with the lowest-risk useful crate.

### Why first

`matten-ndarray` is the best first companion because it is small, useful in mathematical/laboratory workflows, and unlikely to change the product identity.

### Experimental scope

```rust
use matten_ndarray::{from_arrayd, to_arrayd};

let arr = to_arrayd(&tensor)?;
let tensor = from_arrayd(arr)?;
```

Allowed:

- `Tensor -> ndarray::ArrayD<f64>`;
- `ndarray::ArrayD<f64> -> Tensor`;
- scalar/vector/matrix/N-D conversion tests;
- clear conversion errors;
- dynamic tensors return `Err` unless converted through `try_numeric()` first;
- copy behavior documented honestly.

Forbidden:

- adding `ndarray` to core `matten`;
- wrapping the `ndarray` API broadly;
- promising zero-copy before it is designed and tested;
- adding `nalgebra`/`candle` in the same milestone.

### Acceptance gate

```text
[ ] conversion roundtrips are tested (scalar/vector/matrix/N-D)
[ ] from_arrayd preserves logical order for non-standard-layout ArrayD inputs
[ ] from_arrayd rejects zero-sized axes with a clear companion error
[ ] dynamic input returns Result::Err, not panic
[ ] ndarray version policy is documented
[ ] core matten dependency-boundary check still passes
[ ] examples live in matten-ndarray, not core matten
```

---

## 6. v0.18.0 milestone: `matten-mlprep` experimental

### Goal

Provide small, transparent numeric preprocessing helpers without becoming an ML framework.

### Experimental scope

Allowed initial APIs:

```rust
standardize_columns(&x)
minmax_scale_columns(&x)
add_bias_column(&x)
train_test_split(&x, 0.8)
```

Default `train_test_split` semantics:

```text
2D tensors only
rows = samples
columns = features
ordered deterministic split
no hidden randomness
first floor(n_rows * train_ratio) rows -> train
remaining rows -> test
```

If shuffled split is added later, it must be explicit:

```rust
train_test_split_seeded(&x, 0.8, seed)
```

### Forbidden

- model training;
- autograd;
- neural networks;
- optimizers;
- hidden randomness;
- implicit Candle dependency;
- automatic ML pipelines.

### Acceptance gate

```text
[ ] row/sample and column/feature convention is enforced
[ ] split ratio validation is tested
[ ] zero-variance policy is documented
[ ] examples are deterministic
[ ] core matten dependency-boundary check still passes
```

---

## 7. v0.19.0 milestone: maturity hardening

### Goal

Promote only companion crates that stayed small and useful.

### `matten-ndarray` production-ready candidate gate

```text
[ ] scalar/vector/matrix/N-D conversions work
[ ] roundtrip tests are reliable
[ ] dynamic tensors are rejected clearly
[ ] copy behavior is documented
[ ] no zero-copy promise unless implemented
[ ] examples run in CI
[ ] core matten has no ndarray dependency
```

**Status:** All criteria met; `matten-ndarray` promoted **candidate → production-ready** in
v0.25.0 (RFC-057). Both bridge examples (`to_arrayd`, `from_arrayd`) are already executed in CI by
the pre-existing `smoke` job — RFC-057's initial audit missed it; no CI change was needed.

### `matten-mlprep` beta decision gate

```text
[ ] API is small and teachable
[ ] functions are deterministic
[ ] shape rules are documented
[ ] zero-variance behavior is explicit
[ ] train/test split behavior is explicit
[ ] no ML-framework scope entered
```

**Status:** All criteria met; `matten-mlprep` promoted **Beta → production-ready candidate** in
v0.26.0 (RFC-058). Full production-ready is deferred — `train_test_split` is ordered-only (no
shuffle); the candidate → production-ready exit criteria are recorded in RFC-058 §5.1.

---

## 8. v0.19.1 milestone: companion hardening patch

### Goal

Finish the v0.19 maturity work before expanding scope.

### Required work

- Implement RFC-031: feature-robust dynamic rejection and unconditional `Tensor::is_dynamic()`.
- Keep companion `dynamic` mirror features for compatibility; document them as compatibility forwarding features.
- Move / mark RFC-024 as done.
- Move / mark RFC-025 as done for `matten-ndarray`, with `nalgebra` and `candle` explicitly deferred.
- Align companion rustdoc, README, Cargo descriptions, and status labels.
- Strengthen release-doc checks for stale version snippets, stale maturity labels, and active independent-SemVer wording.
- Fix known small lints such as `manual_contains` in `matten-ndarray`.

### Acceptance gate

```text
[ ] dynamic Tensor passed to matten-ndarray returns MattenNdarrayError::DynamicTensor, not panic
[ ] dynamic Tensor passed to matten-mlprep returns MattenMlprepError::DynamicTensor, not panic
[ ] the guarantee does not depend on enabling companion dynamic mirror features
[ ] companion dynamic mirror features remain present for v0.19.1 compatibility
[ ] RFC-024 / RFC-025 lifecycle status is no longer contradictory
[ ] release-doc script detects stale status/version/versioning drift
[ ] workspace tests and core dependency-boundary check pass
```

### Explicit non-goals

- No `matten-data` implementation in v0.19.1.
- No removal of companion `dynamic` features.
- No breaking change.
- No v0.20 scope bundled into the patch.


---

## 9. v0.20+ milestone: materialize the next safe expansion

### Goal

v0.20+ has four parallel tracks:

```text
Track A: matten-data decision/materialization
  Decide whether a small table-to-Tensor companion is worth building.

Track B: core numeric comfort APIs
  Add small NumPy-inspired Tensor conveniences only if they preserve the Sedan-first philosophy.

Track C: examples program
  Demonstrate accepted APIs through famous small math / numerical-computing problems
  without creating hidden dataframe, SciPy, or ML-framework scope.

Track D: benchmarking & positioning
  Build a reproducible, honest evidence base (time, memory, ELOC, dependency
  footprint, regression visibility). Measurement and positioning only — not a
  performance contest, and not a reason to chase larger ecosystems.
```

The release must not become a broad clone of NumPy, SciPy, or Pandas.

The v0.20+ motto is:

> Borrow familiar API ideas. Shrink them to `matten`-sized scope. Stop before dataframe, SciPy, or ML-framework expectations.

---

### 9.1 RFC numbering for v0.20+

RFC-032 is already consumed by another issue. v0.20+ roadmap RFCs therefore start at RFC-033.

| RFC | Theme | Target |
|---:|---|---|
| RFC-033 | `matten-data` Beta-Decision and Scope Lock | v0.20.0 |
| RFC-034 | `matten-data` Table Model and Public API Boundary | v0.20.0 |
| RFC-035 | CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion | v0.20.0 |
| RFC-036 | `matten-data` Examples, Documentation, and Release Gate | v0.20.0 |
| RFC-037 | Deferred Streaming and Large CSV Policy | v0.20.0 / later |
| RFC-038 | Core Numeric Comfort APIs | **Done** (v0.20.9–v0.20.12) |
| RFC-039 | Shape Composition API Boundary | **Implemented** (v0.21.0) — `concatenate` + `stack` in core; `repeat`/`tile`/`meshgrid` deferred |
| RFC-040 | Small Statistics Boundary: Core vs Companion | **Implemented** (v0.21.2) — `var`/`std` + `var_axis`/`std_axis` (population); quantile/histogram/cov/corr deferred |
| RFC-041 | Linear Algebra Boundary: Core Lite vs External Crates | **Implemented** (v0.21.1) — `norm` + `trace` + `outer` in core; decomposition/BLAS/sparse rejected |
| RFC-042 | Pandas-Inspired Scope Guard for `matten-data` | **Implemented** (v0.21.3) — three-check anti-scope guard (file names / public API / README); CI-enforced |
| RFC-043 | Example Program Structure, Quality Gate, and Documentation Policy | v0.20.x |
| RFC-044 | Beginner Core Math Examples | v0.20.x |
| RFC-045 | Matrix Iteration and Graph/Probability Examples | v0.20.x |
| RFC-046 | Numerical Methods and Scientific Toy Examples | v0.21+ or after needed APIs |
| RFC-047 | Small ML-Like Examples Without ML-Framework Scope | v0.21+ |
| RFC-048 | Companion-Crate Examples | v0.20.x / v0.21+ |
| RFC-049 | Benchmarking, Complexity Metrics, and Positioning Report | v0.20.x planning / v0.21+ maturity hardening |

RFC-042 may be folded into RFC-033 if the scope guard is already strong enough. RFC-043 through RFC-048 are examples/documentation RFCs: they demonstrate accepted APIs and workflows, but do not authorize new product scope by themselves. RFC-049 is a non-API measurement/positioning RFC: it adds a benchmark harness and reports in an isolated, `publish = false` package and must not add runtime dependencies to core `matten` or any companion.

---

### 9.2 Track A: `matten-data` decision/materialization

#### Goal

Decide whether `matten-data` deserves beta without becoming a dataframe engine.

`matten-data` may be scaffolded earlier, but it must not be promoted before the v0.20+ decision gate.

> **Resolved:** `matten-data` reached **Beta** in v0.22.0 (RFC-036) and was promoted **Beta →
> production-ready candidate** in v0.27.0 (RFC-059), with the RFC-042 scope lock preserved (still an
> on-ramp, not a dataframe engine). Full production-ready is deferred to a separate future review.

#### Required proof

The crate must prove this small workflow:

```text
CSV / table-like data
  -> inspect schema
  -> clean missing values
  -> select numeric columns
  -> explicit numeric conversion
  -> matten::Tensor
```

Possible API shape:

```rust
use matten_data::Table;

let table = Table::from_csv_path("sales.csv")?;
println!("{}", table.schema_summary());

let x = table
    .select_columns(["sales", "cost", "quantity"])?
    .fill_missing(0.0)?
    .try_numeric()?
    .to_tensor()?;
```

#### Allowed beta scope

- CSV string/path ingestion;
- schema summary;
- column names;
- column selection;
- missing-value cleanup;
- explicit numeric conversion;
- Tensor output.

#### Still forbidden

- joins;
- group-by;
- pivot;
- SQL-like query API;
- lazy execution;
- large-data streaming;
- window functions;
- dataframe-style indexing;
- ML preprocessing.

#### Decision outcomes

At v0.20+, choose one:

```text
A) promote to beta
B) keep experimental
C) freeze/defer
```

Keeping it experimental is acceptable if the API is useful but not mature. Freezing is acceptable if the crate starts drifting into dataframe territory.

#### Acceptance gate

```text
[ ] RFC-033 through RFC-036 accepted before implementation expands
[ ] RFC-037 explicitly defers streaming / large CSV implementation
[ ] core matten has no dependency on matten-data
[ ] matten-data has no dataframe/query/lazy API
[ ] missing-value and numeric-conversion policy is explicit
[ ] duplicate-header and ragged-row policy is documented
[ ] error type is crate-local
[ ] examples are small and do not imply Pandas replacement
```

---

### 9.3 Track B: core numeric comfort APIs

**Status: Complete (RFC-038, shipped across v0.20.9–v0.20.12).** The four bands below
all shipped: elementwise math (v0.20.9), selection `argmin`/`argmax` (v0.20.10),
creation `linspace`/`eye` (v0.20.11), and shape `squeeze`/`expand_dims` (v0.20.12).

#### Goal

Make core `matten` more pleasant for PoC mathematical work by adding small familiar APIs inspired by NumPy, without changing project identity.

Candidate RFC:

```text
RFC-038: Core Numeric Comfort APIs
```

#### Good core candidates

```rust
Tensor::linspace(start, end, count)
Tensor::eye(n)
tensor.clip(min, max)
tensor.abs()
tensor.sqrt()
tensor.exp()
tensor.ln()
tensor.argmin()
tensor.argmax()
tensor.squeeze()
tensor.expand_dims(axis)
```

These fit core if they remain:

- Tensor-centered;
- dependency-light;
- easy to document;
- shape-obvious;
- useful for beginner/intermediate numeric workflows.

#### Needs separate boundary review

```rust
stack(...)
concatenate(...)
repeat(...)
tile(...)
meshgrid(...)
var(...)
std(...)
quantile(...)
histogram(...)
```

These are useful but have enough shape/statistics policy risk to need focused RFC review.

#### Core comfort acceptance gate

```text
[ ] no heavy dependency added
[ ] API is small and teachable
[ ] behavior is obvious for scalar/vector/matrix/N-D where applicable
[ ] NaN/Inf behavior is documented where relevant
[ ] panic-zone vs Result-zone is clear
[ ] examples compile in CI
[ ] no generic Tensor<T> or dtype system introduced
```

---

### 9.4 Track C: examples program

**Status: Complete (RFC-043–048, shipped across v0.20.3–v0.20.13).** All example
bands shipped: structure/policy (v0.20.3), beginner 30–32 (v0.20.3), matrix iteration
33–34 (v0.20.4), companion audit (v0.20.6), numerical methods 35–36 (v0.20.7) and
39–40 (v0.20.13), and ML-like 37–38 (v0.20.8). RFC-043–048 are in `rfcs/done/`. The
optional `41_adjacency_walks_extended` remains a not-reserved conditional candidate.

#### Goal

Increase `matten` examples using famous and recognizable small math / numerical-computing problems while preserving the project philosophy.

The examples program should make users understand:

```text
what Tensor can represent
how small vector/matrix algorithms look in matten
where companion crates fit
what matten intentionally does not do
```

The examples must not become hidden product commitments.

#### RFCs

| RFC | Theme | Implementation posture |
|---:|---|---|
| RFC-043 | Example Program Structure, Quality Gate, and Documentation Policy | Start first; docs/CI/policy foundation |
| RFC-044 | Beginner Core Math Examples | Low-risk examples; can start early |
| RFC-045 | Matrix Iteration and Graph/Probability Examples | Intermediate examples; can start after RFC-044 |
| RFC-046 | Numerical Methods and Scientific Toy Examples | Add after APIs are available; some examples may wait for RFC-038 |
| RFC-047 | Small ML-Like Examples Without ML-Framework Scope | Add cautiously; no ML framework implication |
| RFC-048 | Companion-Crate Examples | All companions shipped (incl. `matten-data` v0.20.1); audit/improve existing examples, do not duplicate |

#### Example groups

New famous-problem examples use a fresh additive **30+ band**; the existing
`00_`–`28_` core suite, the `dynamic_*` set, and the named examples are preserved
and never renumbered (architect ruling, RFC-043–048 review Q1).

Beginner core examples (new files):

```text
30_magic_square_checker.rs
31_fibonacci_matrix_power.rs
32_graph_path_counting.rs
```

Cross-reference / improve in place instead of duplicating (already shipped):

```text
existing 26_cosine_similarity.rs   (cosine similarity)
existing pairwise_distance.rs      (vector distance)
existing 25_normalize_vector.rs
```

Matrix iteration / graph / probability examples:

```text
33_markov_chain_weather.rs
34_tiny_pagerank.rs
```

Optional candidate, not reserved — `41_adjacency_walks_extended.rs`: add only if the
Phase 0 inventory shows it teaches a distinct concept beyond `32_graph_path_counting.rs`
(otherwise drop it).

Numerical methods examples:

```text
35_linear_regression_gradient_descent.rs
36_heat_equation_1d.rs
39_finite_difference_derivative.rs   # shipped v0.20.13 (RFC-038 linspace)
40_trapezoidal_integration.rs        # shipped v0.20.13 (RFC-038 linspace)
```

Small ML-like examples:

```text
37_kmeans_small.rs
38_nearest_neighbor_classification.rs
```

Companion examples (all shipped; audit/improve existing files, do not duplicate):

```text
crates/matten-ndarray/examples/from_arrayd.rs, to_arrayd.rs
crates/matten-mlprep/examples/standardize_columns.rs, train_test_split.rs
crates/matten-data/examples/csv_to_tensor.rs   # shipped in v0.20.1; audit/improve as needed
```

#### Implementation order

```text
0. Inventory existing examples first (audit before adding anything)
1. RFC-043: docs/src/examples/index.md, example structure, CI/example policy
2. RFC-044: beginner examples (30+ band; cross-reference existing distance/cosine)
3. RFC-045: matrix-iteration examples
4. RFC-048: audit/improve existing companion examples
5. RFC-046: numerical-method examples
6. RFC-047: small ML-like examples
```

#### Acceptance gate

```text
[ ] existing examples inventoried before adding any new file (audit-first)
[ ] new examples use the additive 30+ band; existing 00-28 suite not renumbered
[ ] no example duplicates a concept the existing suite already teaches
[ ] examples compile in CI
[ ] examples run deterministically
[ ] examples use small hard-coded data
[ ] examples explain problem, math idea, Tensor representation, and expected output
[ ] examples use only accepted APIs
[ ] companion examples live in companion crates
[ ] no example implies dataframe, SciPy, ML-framework, GPU, or large-data scope
[ ] future-only examples are deferred until their required APIs exist
[ ] the test.yaml smoke list is extended deliberately as runnable examples land
```

#### Non-goals

The examples program must not add examples for:

```text
large CSV
streaming CSV
dataframe group-by
join / merge / pivot
SVD / PCA as core examples
neural network training
autograd
GPU/device usage
sparse matrices
database ingestion
web/network data loading
```

---

### 9.5 Track D: benchmarking & positioning (RFC-049)

#### Goal

Build a reproducible, honest evidence base for where `matten` sits, measured rather
than asserted: execution time, memory behavior where practical, example-code ELOC,
dependency footprint, and regression visibility. The deliverable is a positioning
report, not a leaderboard.

This is a non-API, measurement-only program. It does not add public runtime APIs and
must not pull benchmark tooling into core `matten` or any companion.

#### Posture and sequencing

RFC-049 Phases 1-3 are implemented and accepted: internal baseline, Rust peer
comparison, and code-shape-first NumPy/Pandas reference comparison. Phase 4 hard
gates remain outside RFC-049's closed scope and require a separate future RFC or
explicit release-policy decision. Scenario benchmarks should continue to track
shipped examples and companion workflows without turning benchmark results into
marketing claims or release-blocking speed thresholds by default.

#### Phases

```text
Phase 1: internal baseline (matten only) — implemented and accepted
Phase 2: Rust peer comparison (ndarray, nalgebra) — implemented and accepted
Phase 3: ecosystem reference (NumPy, Pandas table-to-Tensor only) — implemented and accepted
Phase 4: regression tracking / hard gates — extracted to future policy/RFC ownership
```

SciPy and Candle are deferred references; they are out of scope until a separate,
task-specific decision accepts them.

#### Hard constraints (binding)

```text
[ ] benchmark code lives in an isolated `publish = false` package; never a core/companion dependency
[ ] the core dependency-boundary script still passes (no criterion/ndarray/nalgebra in core)
[ ] no Python required for ordinary Rust build/test/CI
[ ] no network access and no external dataset downloads during benchmarks
[ ] no hard CI speed-fail gate initially (harness/schema failures may fail; "slower" may not)
[ ] reports use tradeoff language; never "matten beats / replaces X"
```

#### Acceptance gate (initial)

```text
[ ] methodology docs + non-goals documented (docs/src/benchmarks/*)
[ ] internal baseline harness compiles and runs on one maintainer machine
[ ] selected core + companion benchmarks compile under correct features
[ ] ELOC methodology documented; report template exists with environment metadata
[ ] no runtime dependency added to core matten; boundary check passes
[ ] reports avoid replacement/marketing claims
```

---

### 9.6 What v0.20+ must not do

v0.20+ must not become:

```text
a NumPy clone
a SciPy clone
a Pandas clone
a dataframe engine
an ML framework
a large-data streaming engine
a linalg backend wrapper
```

Borrow ergonomic ideas, not ecosystem scope.


---

## 10. Later themes

### Public reporting / visualization readiness

RFC-063 through RFC-069 prove local, static, tool-owned reporting artifacts,
and RFC-071 adds private fixed-demo JSON released in `0.37.0`. This evidence
still does not prove a stable public report model, renderer API, dependency
policy, or published `matten-report` / `matten-viz` crate. The RFC-070
post-0.37 closure audit therefore closes the public-readiness audit without
public implementation.

RFC-072 completed the behavior-preserving `tools/matten-report`
modularization. Its accepted design and handoff define
module ownership, dependency direction, test placement, migration sequence,
and exact behavior-preservation gates. Slice 0 is reviewed and committed.
Slice 1 makes the entry point thin, separates request/CLI/app/output ownership,
and installs the durable module-boundary gate. Phase 2 report-model extraction
is reviewed and committed. The first Phase 3 checkpoint moved shared numeric
formatting and data-readiness Markdown/test ownership and is reviewed and
committed. The remaining fixed-demo Markdown families are also reviewed and
committed. Phase 3C shared HTML document/security helpers and every HTML
family/test owner are also reviewed and committed. Phase 3D private JSON model,
finite-value policy, family mappings, and exact tests are reviewed and
committed. Phase 4 adds the mechanical file-size ceiling and closes structural
ownership; it is reviewed and committed, and the RFC lifecycle is terminal.

`render` remains a meaningful private ownership boundary: `app` constructs
report-family data and selects a format, while `render::{markdown,html,json}`
owns representation and `render::common` owns only format-neutral presentation.
It is not a forwarding layer and does not construct reports.

### JSON / SVG / Vega-Lite report output

RFC-071 fixed-demo private JSON shipped in the 0.37.0 local-tool release.
RFC-073 bounded private data-readiness input-mode JSON is implemented and
prepared for review. The accepted handoff defines exact schema-v0
representation types, structured user-data bounds, success/error report
outcomes, pre-write destination preservation, unchanged non-atomic write-time
semantics, fixed-demo byte preservation, and a single `render::common`
display-limit owner. Release work remains unauthorized. Public JSON schemas,
SVG, and Vega-Lite remain deferred.

### `matten-nalgebra`

Deferred until after `matten-ndarray` proves the bridge pattern. Requires a separate RFC. RFC-025 is considered implemented for `matten-ndarray`; future `nalgebra` work must not rely on implied acceptance.

### `matten-candle`

Deferred longer because it brings device, dtype, ML, and dependency-tree complexity. Requires a separate RFC.

### Streaming / large CSV

**Reopened and shipped (RFC-082, post-`0.39.0`).** `CsvBatchReader` reads a CSV file in row-count-bounded
`Table` batches, behind `matten-data`'s off-by-default `streaming` feature — batch lifecycle, schema drift,
malformed-row policy, memory budget, sync-vs-async, and crate placement are all decided (RFC-037 §4,
answered in full by RFC-082 §4). Placement is settled too: `matten-data`, not a separate `matten-stream` —
a `matten-stream` crate would need `Table`, which lives in `matten-data`, making it a companion-to-companion
dependency RFC-078 §6 already forbids. Remaining deferred: async, resumability, backpressure, parallel
reading, and streaming numeric conversion — each would need its own RFC if a real need appears.

### `matten-stats`

Possible later companion or small-core extension area beyond RFC-040. Core
already shipped population `var`/`std` and axis variants in v0.21.2. Remaining
topics such as sample variance, covariance, correlation, quantile, percentile,
histogram, and z-score require a separate RFC because they carry policy traps
(`ddof`, NaN behavior, interpolation, binning, and shape semantics).

### Examples program continuation

The examples program may continue after RFC-043 through RFC-048, but only as demonstration work over accepted APIs. New examples that require new public API should cite or wait for the relevant RFC. Examples must not be used to smuggle in dataframe, SciPy, ML-framework, large-data, GPU, or serious-linalg scope.

### `matten-linalg-lite`

RFC-041 shipped the accepted core-lite helpers: `norm`, `trace`, and `outer`.
Any broader linear algebra such as inverse, determinant, eigenvalues, SVD, QR,
Cholesky, BLAS/LAPACK integration, sparse tensors, or a dedicated linalg
companion requires a separate RFC and should normally be delegated through
external crates or bridges.

---

## 11. Workspace versioning policy

The workspace uses **lock-step family versioning** (RFC-030, which supersedes the
earlier independent-per-crate-SemVer plan). Every crate shares one version, set in
`[workspace.package].version`:

```text
matten          0.19.0
matten-ndarray 0.19.0
matten-mlprep  0.19.0
```

- **Version = compatibility.** Matching numbers mean a matched, compatible set —
  no per-crate compatibility matrix for users.
- **Maturity = the Status label** (experimental / beta / production-ready
  candidate / production-ready), declared per crate in its README/docs. A crate
  at `0.19.0` may still be `beta`; the version does not imply maturity.

This fits the project's reality: the crates are released together as milestone
artifacts. If a crate ever needs an independent release cadence, the model is
revisited (back to independent SemVer, with the per-crate `CHANGELOG`/`LICENSE`
split of RFC-022 §12).

### 11.1 Workspace file conventions (resolved v0.19.0)

While the crates ship together as **milestone tarballs** (not yet published to
crates.io), the workspace keeps the structure simple:

- a **single root `CHANGELOG.md`**, ordered by milestone, recording each crate's
  version change inside the relevant entry;
- **root-only `LICENSE`/`NOTICE`**; each crate is licensed by its inherited SPDX
  `license = "Apache-2.0"` field (no per-crate license file is required by cargo
  or crates.io when that field is set).

Per-crate `CHANGELOG`s and per-crate `LICENSE`/`NOTICE` files are reintroduced at
the point crates begin **independent crates.io publication** — the moment a
crate's own version history and self-contained `.crate` artifact start to earn
their maintenance cost (RFC-022 §12).

---

## 12. Maturity labels

### Experimental

Useful for feedback. API may change. Not recommended for production dependency without pinning.

Signals:

- README warning;
- version 0.x;
- docs say experimental;
- changelog may include breaking changes;
- examples are small.

### Beta

Useful for small real workflows. API is intended to be mostly stable, but still pre-1.0.

Signals:

- README beta badge/text;
- examples in CI;
- documented limitations;
- public API snapshot or equivalent;
- breaking changes require migration notes.

### Production-ready candidate

The team believes the crate can be used seriously if the documented limits are acceptable.

Signals:

- strong tests;
- examples in CI;
- clear error types;
- documented compatibility policy;
- no known P0/P1 issues;
- release checklist complete.

### Production-ready

Stable enough to recommend as a normal dependency for its documented scope.

Signals:

- mature docs;
- stable API;
- compatibility and MSRV policy;
- clear release notes;
- no hidden dependency surprises.

This label does not automatically imply version 1.0. A v1 release still requires explicit maintainer confirmation.

---

## 13. Companion dependency and import style

Canonical documentation should preserve this ownership model:

```text
matten owns Tensor.
companions add focused workflows around Tensor.
```

Official examples SHOULD prefer explicit user dependencies:

```toml
[dependencies]
matten = "0.19"
matten-ndarray = "0.19"
```

and canonical imports:

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
```

In the current policy a companion MUST NOT re-export `matten`. The limited
single-dependency convenience path (`pub use matten;`) is **deferred by RFC-032**
(§3.3) and may be revisited only after demonstrated user demand and a follow-up
RFC/decision. The release-doc check (`scripts/check-release-docs.sh`) enforces this:
it fails if any companion contains `pub use matten`.

```rust
// FORBIDDEN in the current policy (RFC-032 §3.2/§3.3)
pub use matten;            // whole-crate convenience re-export: deferred
pub use matten::Tensor;    // broad core-type re-export: forbidden
pub use matten::MattenError;
pub use matten::Element;
pub use matten::NumericPolicy;
```

This policy keeps ownership, feature selection, maturity labels, and dependency/security review clear.


---

## 14. Companion error policy

Each companion crate defines its own error type.

Core `matten::MattenError` is for core tensor and boundary failures only. Companion crates may wrap `MattenError`, but core must not grow variants for companion-specific failure modes.

Bridge and conversion functions return `Result`:

```rust
to_arrayd(&tensor) -> Result<ArrayD<f64>, MattenNdarrayError>
```

Dynamic inputs to companion bridge/prep/data APIs should return `Err`, not panic, unless the API is explicitly documented as an internal panic-zone convenience.

---

## 15. Mechanical dependency-boundary gate

The v0.16 release must add a CI check proving that core `matten` has no forbidden dependency direction.

The check should fail if core `matten` depends on:

```text
ndarray
nalgebra
candle-core
polars
arrow
datafusion
matten-ndarray
matten-mlprep
matten-data
```

A script such as `scripts/check-core-dependency-boundary.sh` should run in CI. It
MUST inspect the core package with all features enabled so optional dependencies
behind non-default features cannot slip past:

```bash
cargo tree -p "$CORE_PACKAGE" --all-features --edges normal,build --no-dedupe
```

A plain `cargo tree -p matten` is insufficient: an `ndarray = { optional = true }`
dependency gated by a non-default feature would not appear, producing a false pass.


---

## 16. Document history

| Version | Date | Change |
|---|---|---|
| 1.0.0 | 2026-06-21 | First canonical v0.16+ roadmap after companion-crate reconciliation. |
| 1.1.0 | 2026-06-22 | Updated v0.20+ materialization plan. RFC-032 is reserved/consumed elsewhere, so v0.20+ planning starts at RFC-033. Added v0.19.1 hardening milestone, `matten-data` RFC sequence RFC-033–037, core comfort RFC-038+, companion dependency/import style, and later stats/linalg boundary themes. |
| 1.2.0 | 2026-06-22 | Reconciled to shipped reality and architect rulings (v0.19.3): §13 corrected so the companion `pub use matten;` convenience re-export is deferred per RFC-032 (release-doc guard forbids it); planning baseline corrected to lock-step family versions (no per-crate `0.1.x`); added v0.19.2 and v0.19.3 release-theme rows. |
| 1.3.0 | 2026-06-23 | Added examples program planning for RFC-043–048 and compact examples implementation handoff. Added v0.19.4 release-theme row; expanded v0.20+ to Track C for examples; added RFC-043–048 table entries, example groups, implementation order, acceptance gates, and non-goals. |
| 1.4.0 | 2026-06-23 | Reconciled the examples program to architect rulings (v0.20.2): new famous-problem examples use an additive 30+ band (existing 00-28 suite preserved); cosine/distance and companion examples are cross-referenced/audited, not duplicated; matten-data csv_to_tensor marked shipped in v0.20.1; docs path examples/index.md; CI smoke-list update requirement added. Fixed the v0.19.4 version regression: replaced with accurate v0.20.0/v0.20.1/v0.20.2 release-theme rows. |
| 1.5.0 | 2026-06-23 | Added the benchmarking & positioning program (RFC-049) as Track D: goal, posture/sequencing, phases, hard constraints, and acceptance gate; added RFC-049 to the v0.20+ RFC table; recorded the shipped v0.20.3/v0.20.4 example bands and the v0.20.5 benchmarking-planning row in the release-theme table. RFC-049 is non-API and measurement-only. |
| 1.6.0 | 2026-06-23 | Marked Track B (core numeric comfort APIs, RFC-038) complete: all four bands shipped across v0.20.9 (elementwise), v0.20.10 (selection), v0.20.11 (creation), and v0.20.12 (shape). Updated the RFC-038 row to Done and added a completion status note to §9.3. RFC-038 moved to `rfcs/done/`. |
| 1.7.0 | 2026-06-23 | Marked Track C (examples program, RFC-043–048) complete: shipped the deferred numerical examples 39–40 (finite-difference derivative, trapezoidal integration) in v0.20.13, which finishes the additive 30+ band. Closed RFC-043–048 to `rfcs/done/` with shipped-version annotations; added a §9.4 completion note and corrected the 39/40 lines from deferred to shipped. The optional `41_adjacency_walks_extended` remains a not-reserved conditional candidate. |
| 1.8.0 | 2026-06-23 | Ingested the v0.21 boundary architect rulings (RFC-039–042, all 13 questions accepted with added constraints). Marked RFC-039/040/041/042 accepted-for-implementation (Status updated; rulings recorded in each RFC) and set targets v0.21.0 (039 concatenate/stack), v0.21.1 (041 norm/trace/outer), v0.21.2 (040 var/std/var_axis/std_axis), v0.21.3 (042 scope guard). Added the v0.21.0–.3 release-theme rows. RFCs remain in `proposed/` per the 4-folder lifecycle until each ships. |
| 1.9.0 | 2026-06-23 | Architect accepted the v0.20 series handoff (phase closed) and completed a v0.20.14 codebase deep review (no P0; P1 documentation/release-truth findings). Applied the review as v0.20.15: Patch A (doc-truth cleanup — stale 0.15/0.19 strings → 0.20, root README crate table + matten-data row, public-API snapshot header/InvalidArgument/try_reshape row, matten-data and intro skeleton wording, operators matmul) and Patch B (hardened `check-release-docs.sh` with doc-truth checks). Patch C (RFC-023/026/036/037 lifecycle clarification, P2) deferred to v0.21 planning. Optional `41_` confirmed as a conditional candidate. |
| 1.10.0 | 2026-06-23 | Audited the project since v0.19.0 across four dimensions (codebase↔RFCs, tests↔requirements/external-design, codebase↔tests, docs↔codebase). Result: consistent. Confirmed accepted-but-unshipped RFCs (039–042) are not prematurely implemented, RFC-038/033–035/043–048 are implemented with accurate done-status, the full suite passes with zero ignored tests, and docs match the public surface. One documentation gap fixed in v0.20.16 (public-API snapshot Element method list). |
| 1.11.0 | 2026-06-23 | Ingested and applied the pre-v0.19.0 audit architect rulings (Q1–Q4) as v0.20.17. Q1: retired "Phase 1/Phase 2" wording from user-facing docs (48 occurrences across 14 files) in favor of numeric-Tensor/dynamic-ingestion terminology, plus a release-docs guard against reintroduction (history retained in rfcs/ and CHANGELOG). Q2: added an RFC-013 lifecycle note (property/fuzz testing is aspirational, not a current gate); tracked an optional future "Testing Strategy Refresh" (candidate RFC-050, after RFC-049). Q3: added RFC-014↔RFC-043 cross-references (RFC-043 is the current examples-program authority; RFC-014 historical). Q4: added an RFC-012 clarification (internal Arc-shared CoW implemented; public mutation API intentionally deferred). The separately-deferred Patch C (RFC-023/026/036/037 lifecycle) remains pending its own ruling. |
| 1.12.0 | 2026-06-23 | Build/repo hygiene (v0.20.18): git-ignored the RFC-031 fixture's Cargo.lock (`/tests/fixtures/*/Cargo.lock`) so the repository tracks a single workspace lock; the fixture stays excluded (feature-unification isolation is required for the regression). Clarified the root Cargo.toml exclusion comment. Evaluated and rejected a proposed members/exclude `tests/*` manifest change (it fails `cargo metadata` and would not remove the second lock). No code/API/behavior change. Also (same v0.20.18): pointed the README documentation link at the published mdBook (nabbisen.github.io/matten); added per-example source-code links across the examples pages; and retired four hyphenated `Phase-1` references missed by v0.20.17 (guard now matches `Phase[ -]1`). |
| 1.13.0 | 2026-06-24 | Examples reorganization (v0.20.19) per architect ruling: renamed the seven unnumbered skill-demos into a new `50_`–`56_` practical-recipes band; retired the `hello_tensor.rs` and `column_summary.rs` fossils; created docs/src/examples/practical-recipes.md and updated index.md/beginner-math.md/SUMMARY.md; fixed a stale "Phase 2" docstring in dynamic_00; updated CI smoke runs; added a naming-band guard (core examples must match `NN_` or `dynamic_NN_`). No public API or behavior change. |
| 1.14.0 | 2026-06-24 | Opened the v0.21 line with RFC-039 shape composition (v0.21.0): `concatenate` (existing axis) and `stack` (new axis) added to core as borrowed-slice associated functions with try_/panic pairs, MattenLimits allocation checks, and dynamic rejection; `repeat`/`tile`/`meshgrid` remain deferred. Added 20 unit tests + the `14_concatenate_stack` example; new reference page shape-composition.md and a public-API-snapshot section; RFC-039 moved to done/. |
| 1.15.0 | 2026-06-24 | v0.21.1: RFC-041 linalg core-lite. Added `norm` (L2/Frobenius over all elements, NaN-propagating, panic-only like `sum`/`mean`), `trace` (rank-2, rectangular via `min(rows,cols)`, with `try_trace`), and `outer` (rank-1 × rank-1 → `[m,n]`, MattenLimits-checked, with `try_outer`) in a new `linalg.rs` module (math.rs kept under 300 ELOC). Decomposition/inverse/determinant/eigen/SVD/QR/LU/Cholesky/sparse/BLAS remain rejected from core. Added 16 unit tests + the `15_norm_trace_outer` example; new reference page linalg.md (with the required "not a linear algebra backend" boundary wording) and a public-API-snapshot section; RFC-041 moved to done/. |
| 1.16.0 | 2026-06-24 | v0.21.2: RFC-040 statistics core. Added `var`/`std` and `var_axis`/`std_axis` (population variance, ddof=0; `var = sum((x-mean)^2)/n`; two-pass; NaN-propagating) in a new `stats.rs` module (math.rs kept under 300 ELOC), with `try_*` forms (`Unsupported` on dynamic, `Shape` on invalid axis, defensive `InvalidArgument` on the not-constructible empty case). Sample variance / quantile / percentile / histogram / covariance / correlation / z-score remain deferred; no `matten-stats` companion scaffolded. Added 14 unit tests + the `16_variance_std` example; new reference page stats.md and a public-API-snapshot section; RFC-040 moved to done/. |
| 1.17.0 | 2026-06-24 | v0.21.3: RFC-042 matten-data anti-scope guard — completes the v0.21 boundary-work batch. Added scripts/check-matten-data-scope.sh with three PRECISE checks (RFC-042 §8 / Q13): (1) example file-name guard (rejects dataframe-story names like join_customers_orders.rs), (2) public-API identifier guard over crates/matten-data/src (rejects pub DataFrame/Series types and pub groupby/join/merge/pivot/query/loc/iloc fns, matched as definitions), (3) positive README scope-statement check ("not a dataframe library"). Deliberately NO broad body-scan of index/join/loc/query (so Path::join, var index, joined/join_tables, location all pass). Wired into the matten-data CI job and the release checklist; tested against all RFC §8 must-fail / must-not-fail cases. No Rust code change. |
| 1.18.0 | 2026-06-24 | v0.21.4: applied the v0.21.3 deep-review P1 release-truth fixes. Architect confirmed Q1–Q5 (norm panic-only; var_axis/std_axis try_ forms; defensive empty guard; dedicated scope-guard script; new modules) and ruled Q6 = yes (wire release-docs guard into CI). Fixes: corrected 0.20→0.21 documentation drift across READMEs, lib.rs, quick-start, boundary/dynamic/architecture/introduction/compatibility, and the public-API snapshot (now family-only "current v0.21 family" to prevent future patch drift); retired "Phase 1" wording from four examples; made check-release-docs.sh current-family-aware (CURRENT_MINOR variable; rejects non-current install pins / X.Y.x family labels / "current vX.Y family" prose; allows historical full-patch refs and generic examples) and extended its retired-wording scan to examples; wired check-release-docs.sh into the CI check job; added scope-guard + release-docs guard to the release checklist. Deep review flagged two future-optional (pre-v1.0) consistency RFCs — Result-form reductions (try_sum/try_mean/try_min/try_max/try_norm) and try_*_axis — tracked, not required for v0.21. No library code change. |
| 1.19.0 | 2026-06-24 | v0.22.0: **matten-data promoted to Beta.** Implemented the full RFC-036 six-example suite (data_00_quickstart, data_01_schema_summary, data_02_select_columns, data_03_missing_values, data_04_to_tensor, data_05_errors), keeping csv_to_tensor.rs as the comprehensive overview (architect Option 1); added an explicit malformed-CSV test (`malformed_csv_is_a_structured_error_never_a_panic`, 34 tests total). Completes the RFC-023 §9 Beta gate. Flipped status Experimental→Beta across matten-data README, lib.rs, root README table, companions.md, compatibility.md; bumped family 0.21→0.22; added docs/src/examples/data.md (wired into SUMMARY + companions). Guards: CURRENT_MINOR 21→22 and a new matten-data-must-say-Beta check. CI: matten-data job gains `cargo check --examples` + `cargo test -p matten-data`; smoke job runs all six data_* examples. RFC-036 → Implemented, RFC-023 → Resolved (Outcome B → Beta); both moved to done/. **Finding surfaced:** the lenient+flexible csv config never emits parser errors for &str input (unterminated quote → structural RaggedRow; bad header → Csv), so the malformed-CSV test asserts the real no-panic/structured-error contract rather than a parser-error variant the config cannot produce. No library/API/behavior change. |
| 1.20.0 | 2026-06-24 | v0.22.1: RFC-049 accepted with a staged mandate; implemented **Phase 1 only** (PR-049-1 methodology docs + PR-049-2 internal Rust baseline harness). Added a workspace-excluded, publish=false `benchmarks/` criterion harness (workloads in a criterion-free lib; benches isolated), covering a core micro set + five scenario workloads from examples 26/33/34/35/36; methodology docs under docs/src/benchmarks/; an internal-baseline report template + results-commit policy. Extended the core dependency-boundary guard to forbid criterion in core matten's tree (§7); added a CI benchmarks job that compile-checks the harness only (no speed/memory gates, §5); git-ignored benchmarks/Cargo.lock. RFC-049 → Accepted (stays in proposed/ until fully implemented; Phases 2–4 deferred until Phase 1 yields a credible baseline report). Memory policy = Linux peak RSS via /usr/bin/time -v (informative, not a gate). No published-crate code/API/behavior change. |
| 1.21.0 | 2026-06-24 | v0.22.2: applied the v0.22.0 handoff-review P2 follow-up. Added a clarification note to RFC-023 §9 (and a pointer in RFC-036) recording that the malformed-CSV Beta-gate criterion is satisfied by a structured-error/no-panic malformed-input test — Csv or RaggedRow, never a panic or silently-wrong Table — rather than a low-level csv parser-error test, since the lenient flexible(true) &str reader resolves unterminated quotes to structural RaggedRow validation; a byte-level invalid-UTF-8 test is intentionally not added (no public path; tests the dependency, not matten-data). Historical CHANGELOG/ROADMAP entries left unchanged. No Rust code, API, guard, or CI change. |
| 1.22.0 | 2026-06-24 | v0.22.3: applied the benchmarking/positioning review. Part A (RFC-032 scope): Option A — added RFC-032 §5.1 confirming workspace-excluded publish=false internal tooling (the RFC-031 fixture, the RFC-049 benchmark harness) is outside the published, user-facing family convention's packaging scope, while still following ownership-clarity (no core-type re-export; import from matten); no change to benchmarks/ or the fixture; RFC-032 guard deliberately NOT extended to scan excluded tooling. Guards: added scripts/check-published-dependency-isolation.sh (RFC-049 §B1) proving each published crate is peer-dep-free — core/matten-data/matten-mlprep forbid criterion/ndarray/nalgebra; matten-ndarray forbids criterion/nalgebra but is allowed ndarray (bridge); passes today; wired into CI after the RFC-022 core guard and into the release checklist; negative-tested. Part B (RFC-049 Phase 2): design settled and annotated onto RFC-049 (B1 isolation guard, B2 structural peers-feature + fixed comparable-task list, B3 opt-in/off-by-default build & CI, B4 baseline-report entry precondition) and marked DESIGNED-NOT-AUTHORIZED; added benchmarks/reports/BASELINE-READY-CHECKLIST.md; updated methodology + report template. Phase 2 implementation deliberately NOT started (awaits a maintainer-run credible baseline report + separate authorization). No Rust code/API/runtime change. |
| 1.23.0 | 2026-06-24 | v0.22.4: **RFC-049 Phase 2 — Rust peer comparison (opt-in)**, plus the accepted Phase 1 internal baseline and a workspace-config fix. Architect accepted the maintainer-run baseline (Ubuntu 26.04, virtualized; Baseline ID matten-rfc049-internal-baseline-v0.1) and authorized Phase 2 under prior constraints. Implemented the peer harness in the workspace-excluded benchmark crate: a `peers` feature (ndarray+nalgebra optional deps) OFF by default; workloads/peers/{ndarray,nalgebra}_tasks.rs covering the fixed comparable set (cosine, small matmul, Markov, PageRank, linreg GD, heat), each documenting comparability; a required-features=[peers] peers bench giving a three-way matten/ndarray/nalgebra comparison from identical data; peer-comparison-v0.1.md template (limitations + non-ranking disclaimer, "Rust peer comparison" wording). Verified default --no-run compiles ZERO peer crates and the published-isolation guard still passes (peer deps never reach published crates). Separate benchmarks-peers.yml workflow (manual/weekly) compile-checks --features peers only — kept out of ordinary CI, no speed gates. Marked Phase 2 implemented across RFC-049/methodology/README; completed the accepted baseline report (real medians, peak RSS 44,728 kB, Baseline ID + acceptance marker, sum_mean_axis clarified). Fixed the workspace exclude glob (tests/fixtures/* does not expand in Cargo exclude) by making the RFC-031 fixture self-excluding with an empty [workspace] table. Recorded sum_mean_axis (~1.31 ms; ~400x sum_mean, ~17x 64x64 matmul) as a P2 performance-watch / regression-visibility anchor (not a fix-now item, not a Phase 2 blocker). Phase 3 (NumPy/Pandas) and hard speed gates remain unauthorized. No published-crate code/API/runtime change. |
| 1.24.0 | 2026-06-24 | v0.22.5: v0.22.4 deep-review release-truth reconciliation (docs/RFC/guard only; no library code, API, runtime, or benchmark-logic change). The architect's v0.22.4 codebase deep review accepted the Phase 2 harness/template (no P0/P1 source blockers) and requested status-text fixes; the companion benchmarking/positioning review accepted the baseline (archival-ready) + peer template and confirmed nalgebra-on-all-six + official-numbers-pending. Fixes: (P1) rewrote docs/src/benchmarks/index.md status (Phase 1 accepted; Phase 2 harness/template implemented, official numbers pending; only Phase 3/4 deferred); (P1) reconciled the RFC-049 header Status/Target/Acceptance with the already-updated Phase 2 body (removes the internal inconsistency); (P2) benchmarks/README.md title drops "Phase 1", peer command comment clarified to "ordinary CI ... manual/scheduled peers workflow"; (P2) peer-comparison-v0.1.md gained a top-level "Template only; official numbers pending; do not cite sandbox" marker and migration-tone interpretation guidance; (P2) methodology distinguishes harness/template-implemented from official-report-complete. Added a scoped benchmark-status-drift guard to check-release-docs.sh (flags docs/src/benchmarks describing Phase 2 as unimplemented; excludes RFC history/CHANGELOG; Phase 3/4 deferral still allowed; positive/negative tested; rides the existing CI release-docs gate). Official Phase 2 peer numbers still pending a maintainer run; Phase 3 + hard gates remain unauthorized. |
| 1.25.0 | 2026-06-24 | v0.22.6: accepted and ingested the production-migration RFC set (RFC-050–054) — planning/docs only, no library code/API/runtime/dependency change. Theme: a documented, honest "family-car → super-car" exit ramp from matten to heavier ecosystems (ndarray, nalgebra, Polars, Candle, NumPy, Pandas) with NO heavy dependency added to core matten; migration support lives in docs, bridge crates, and (later, if ever) workspace-excluded tooling. Added rfcs/proposed/050–054 (Production Migration Guide & Bridge Strategy; Bridge Conversion Contracts & Companion-Crate Policy; Production Target Playbooks; Migration Readiness Diagnostics & Report Format; deferred matten-migrate CLI) + the handoff bundle in rfcs/handoffs/ (implementation handoff, RFC-054 deferred note, acceptance/QA + release-guard checklists); updated rfcs/README.md index. Applied the architect's review-of-review ruling: resolved the RFC-050 number collision by KEEPING the migration set at 050–054 and renumbering the earlier Testing-Strategy-Refresh earmark to RFC-055 (rfcs/done/013 note updated); clarified RFC-051 §9 error categories are illustrative (matten-ndarray's DynamicTensor/ZeroSizedAxis/NdarrayShape/Matten is compliant as-is) and §15 audit is documentation-only (no new error variant); resolved RFC-051 §17 → to_<target>/from_<target> default naming; softened RFC-052 deprecated ndarray .into_shape wording and added a pending-peer-numbers acceptance rule (no numeric claims until official RFC-049 Phase 2 numbers accepted); added an RFC-054 workspace-excluded/publish=false placement note; made the release-guard checklist phrase-anchored only (no bare-word bans). Implementation of RFC-050–053 targets v0.23.0/v0.23.x; RFC-054 remains deferred; RFC-055 (testing refresh) remains a future candidate. |
| 1.26.0 | 2026-06-25 | v0.22.7: RFC-049 Phase 2 accepted — documentation reconciliation (docs/RFC/report wording only; no library code/API/runtime/dependency change). Architect accepted the official maintainer-run Rust peer comparison (commit 007031c, v0.22.6, baseline machine class) as the RFC-049 Phase 2 official report; Phase 2 is now COMPLETE, Phase 3 (NumPy/Pandas) + Phase 4 (hard gates) remain unauthorized, no optimization required. Official medians (matten/ndarray/nalgebra): cosine 674ns/231ns/160ns; matmul 64x64 118.9us/12.74us/16.23us; markov 1.03us/1.34us/1.41us (matten competitive/inverted); pagerank 6.21us/607ns/607ns; linreg 1.75us/769ns/832ns; heat 5.23us/556ns/565ns. Honest finding: peers generally lower-overhead on dense kernels (expected for DX-first matten); widest ~7-10x on matmul/pagerank/heat, modest ~2-4x on cosine/linreg, inverted on markov; matrix-vector path widest vs vector-matrix competitive (recorded as positioning/regression-visibility, NOT a defect). Reconciled peer-comparison-v0.1.md (acceptance marker + Report ID matten-rfc049-rust-peer-comparison-v0.1; natural-representation clarification; CORRECTED nalgebra note: 0.33.3 pinned by MSRV-1.85 floor via MSRV-aware resolver, 0.35 needs Rust 1.89 = future MSRV-policy decision, not a 1.93-toolchain constraint), benchmark docs/README/methodology (pending -> accepted), RFC-049 header/annotation/index, and added an RFC-052 task-scoped citation note (playbooks may now cite results; no ranking/faster-than/migration-mandate). RFC-049 stays in proposed/ until Phases 3-4 resolve. |
| 1.27.0 | 2026-06-25 | v0.23.0: production migration guide — first release (RFC-050 foundation + RFC-052 Rust playbooks). Docs only; NO library code/API/runtime/dependency change, core matten gains no dependency. First stage of the family-car -> super-car migration program (RFC-050-054); architect prioritized the Rust-target playbooks, delivered here. Added docs/src/migration/: index (migration promise: outgrowing matten is a successful PoC outcome; dependency-light; not an auto code-rewriter), when-to-migrate (stay-vs-migrate pressure signals), target-selection (workload->ecosystem matrix + decision path), common-pitfalls (row-major vs column-major, convert-once, f64/f32, dynamic->numeric); playbooks/index (decision tree) + full ndarray and nalgebra playbooks (choose/don't-choose/concept-mapping/example-migrations/conversion-path/pitfalls/positioning/checklist). ndarray playbook leads with the matten-ndarray bridge (to_arrayd/from_arrayd); nalgebra documents manual from_row_slice (no matten-nalgebra bridge yet). Positioning notes cite the ACCEPTED RFC-049 peer comparison task-scoped (no ranking/faster-than). Added a scoped migration overclaim guard to check-release-docs.sh (phrase-anchored, future/deferred exception, positive+negative tested); new SUMMARY Migration section; reference/migration.md cross-links the full guide (no duplication). Staged for v0.23.x: remaining RFC-052 playbooks (Polars/Pandas, Candle, NumPy), RFC-051 bridge-contract pages + matten-ndarray contract table, RFC-053 readiness diagnostics. RFC-054 (matten-migrate CLI) deferred. |
| 1.28.0 | 2026-06-25 | v0.23.1: production migration guide — RFC-052 completed (remaining target playbooks). Docs only; no library code/API/runtime/dependency change. Added the cross-paradigm/cross-language playbooks: polars-and-pandas.md (dataframe path; states matten-data is an on-ramp and will NOT grow group-by/join/pivot/query; enter the dataframe lib at the data source), candle.md (ML path; careful NOT to imply matten is an ML framework; f64->f32 boundary), python-numpy.md (Python scientific path; manual/conceptual serialization hand-off, no in-process Rust<->Python bridge). Each follows the standard 8-section playbook structure. Positioning notes state honestly that NO benchmark exists for these targets (cross-paradigm/cross-language = RFC-049 Phase 3, not authorized) — choose by capability/ecosystem fit, not measured speed. Moved the three targets from later-revision to available across playbooks/index, target-selection, index; SUMMARY lists all five playbooks. RFC-052 target set now complete. Still staged for v0.23.x: RFC-051 bridge-contract pages + matten-ndarray contract table, RFC-053 readiness diagnostics. RFC-054 (matten-migrate CLI) deferred. |
| 1.29.0 | 2026-06-25 | v0.23.2: production migration guide — RFC-051 bridge conversion contracts. Docs only; no library code/API/runtime/dependency change. Added bridge-contracts.md (13-dimension conversion-contract template + filled matten-ndarray reference contract, verified against convert.rs/error.rs: copies both ways, numeric-only, rejects dynamic via DynamicTensor (unconditional, not panic), preserves logical row-major through non-standard layouts, rejects zero-sized axes, Result never panics; RFC-051 error categories noted as illustrative not required) and bridge-crate-policy.md (own target dep; never re-export Tensor — confirmed matten-ndarray exports only to_arrayd/from_arrayd/MattenNdarrayError; to_<target>/from_<target> naming; future-bridge checklist; new bridges need separate approval; CI isolation guard). Added the contract table to crates/matten-ndarray/README.md; cross-linked docs/src/examples/companions.md; SUMMARY lists the two pages. RFC-051 acceptance criteria met. Still staged: RFC-053 readiness diagnostics (last in batch). RFC-054 (matten-migrate CLI) deferred. NOTE/FINDING: the v0.23.0 family bump left stale '0.22' version references across all four crate READMEs + ~15 doc locations, and the pin examples (matten = "0.22") now mis-pin users to the old family (caret excludes 0.23.x); the release-docs guard did not catch this. Recommended a focused version-string-hygiene + guard release next. |
| 1.30.0 | 2026-06-26 | v0.23.3: version-string hygiene + self-updating drift guard. Docs/release-tooling only; no library code/API/runtime/dependency change. FIXED: stale '0.22' strings -> '0.23' across all four crate READMEs, root README, core lib.rs rustdoc, and ~10 doc pages (quick-start, examples/data, reference/boundary, reference/dynamic, contributing/architecture, public-api-snapshot, plus 0.22.x/'current 0.22 family' labels). Install pins were a real bug, not cosmetic: caret 'matten = "0.22"' resolves >=0.22.0,<0.23.0, so copying it held users on the old family and hid the 0.23 migration guide. Historical refs (promoted to Beta in v0.22.0; per-family compatibility history) preserved; added a v0.23 family entry to compatibility.md. CHANGED: check-release-docs.sh version guard now derives the current minor DYNAMICALLY from Cargo.toml instead of a hardcoded CURRENT_MINOR=22 — that hardcoded value (manual-bump-per-release) was missed at v0.23.0, which is precisely why the stale 0.22 pins shipped unflagged. Guard keeps 'family' adjacency so generic patch-notation like (0.13.x) is not flagged; verified green on corrected docs and that a simulated 0.24.0 bump immediately flags stale 0.23 strings. Root cause of the v0.23.0-0.23.2 stale-version finding now closed. Next: RFC-053 readiness diagnostics (last in the migration batch); RFC-054 (matten-migrate CLI) deferred. |
| 1.31.0 | 2026-06-26 | v0.23.4: production migration guide — RFC-053 migration-readiness diagnostics (COMPLETES the RFC-050-053 migration batch). Docs only; no library code/API/runtime/dependency change. Added readiness-checklist.md (10 pressure signals — data-size/runtime/axis-reduction/linear-algebra/dataframe/ML-device/dynamic-ingestion/dependency-policy/ecosystem/team-language — each mapped to a target playbook, with explicit stay-with-matten outcomes; advisory, no source-scanner), readiness-report.md (manual fillable template with the 9 required sections: Summary/Current usage/Pressure signals/Recommended targets/Direct conversion candidates/Manual redesign areas/Bridge crates-tools/Risks/Next steps + required advisory disclaimer), and examples/linear-regression-gd-readiness.md (template filled for 35_linear_regression_gradient_descent, written against its actual structure: two matmul/step + reused transpose + iterative loop; recommends moving per-step matrix products to ndarray via the matten-ndarray bridge at real sizes, nalgebra closed-form as optional redesign, stay-with-matten at toy size). SUMMARY + migration/index link the pages. Guard: migration overclaim check now allows the negated advisory disclaimer (does not perform automatic conversion) while still flagging positive automatic-conversion claims (verified both directions). MIGRATION PROGRAM COMPLETE: RFC-050 (foundation) + RFC-051 (bridge contracts) + RFC-052 (all playbooks) + RFC-053 (readiness) done; RFC-054 (matten-migrate CLI) deferred. |
| 1.32.0 | 2026-06-27 | v0.23.5: RFC-050-054 deep-review response (P1+P2) and migration-batch lifecycle close. Docs/release-tooling/RFC-bookkeeping only; no library code/API/runtime/dependency change. P1 FIXED: restored 13 CHANGELOG release headings (v0.23.3 back to v0.21.4) that a heading-eating regression had nested under a single ## [0.23.4] — block content was intact, only headings lost; versions/dates taken from this ROADMAP history. P1 FIXED: Candle snippet in reference/migration.md now builds the Tensor from f64 data (was vec![1.0f32,..], but Tensor is f64) and casts to f32 only at the Candle boundary. P2: reference/migration.md softened 'always one line', made ndarray section bridge-first (matten_ndarray::to_arrayd) with manual ArrayD::from_shape_vec as fallback, replaced brittle 'four exports' wording with a pointer to the public-API snapshot/CHANGELOG; check-release-docs.sh gained a CURRENT_MINOR extraction sanity check and a CHANGELOG-heading guard (top heading must equal workspace version; no release block may hold >1 ### Threat model — the lost-heading signature; both tested). LIFECYCLE: RFC-050/051/052/053 -> Implemented and moved proposed/ -> done/ (v0.23.0, v0.23.2, v0.23.0-v0.23.1, v0.23.4); rfcs/README Done/Proposed tables reconciled; RFC-054 stays proposed as accepted-future-direction with deferral confirmed. Architect deep review (2026-06-27) accepted RFC-050-053 as implemented and approved the done/ move after P1. |
| 1.33.0 | 2026-06-27 | v0.24.0: Result-form reductions — complete the fallible reduction surface (RFC-055 scalar + RFC-056 axis). Additive public API in core matten; no new dependency, no breaking change, f64-only. ADDED: try_sum/try_mean/try_min/try_max/try_norm (Result<f64>; Unsupported on dynamic; NaN propagates as a value) and try_sum_axis/try_mean_axis/try_min_axis/try_max_axis (Result<Tensor>; Shape on out-of-range axis, Unsupported on dynamic, dynamic checked first to match try_var_axis; reduced axis removed from output shape). CHANGED: panic forms now delegate to their try_ engines via unwrap_or_else(panic!) — same pattern var/std already use, so forms cannot diverge; behaviour unchanged (still panic on dynamic / bad axis), but panic-message TEXT now comes from MattenError Display (architect-accepted; text is not API contract). norm RULING REVERSED (deep review 2026-06-27): prior v0.21 norm-panic-only decision reversed, try_norm added, rustdoc corrected. No new MattenError variant; reused Shape (axis) / Unsupported (dynamic). Internal: shared reject_dynamic helper (now reused by stats reductions too) + check_axis helper. Tests: try_ Ok-path equals panic form, axis==rank/>rank -> Shape, dynamic -> Unsupported (incl. precedence), panic forms still panic, no exact-panic-string asserts; doctests on each try_ form; lean/dynamic/all-features + MSRV 1.85 green. Family bump 0.23.5->0.24.0; user-doc pins/labels retargeted 0.23->0.24 (caught by self-updating drift guard); compatibility.md v0.24 entry; public-api-snapshot lists the 9 new methods. Architect accepted both RFCs full set (no P0; P1 = rustdoc/error-contract clarity, met). RFC-055/056 -> rfcs/done/. |
| 1.34.0 | 2026-06-27 | v0.24.1: v0.24.0 deep-review response (P1+P2+optional P3). Docs/release-tooling/test only; no library code, public API, runtime behaviour, or dependency change. Architect deep review (2026-06-27) accepted v0.24.0 as the correct RFC-055/RFC-056 implementation (no P0; all main claims verified by static source review) subject to one P1 release-truth fix. P1 FIXED: docs/src/introduction.md still said 'current 0.23 family' — updated to current 0.24 family + RFC-055/056 reduction-surface completion (layered on v0.23 migration guide, v0.22 matten-data Beta). The stale line used the un-prefixed 'current 0.N family' spelling, which the release-docs guard missed because it only matched 'current v0.N family'. P2 FIXED: check-release-docs.sh current-family-prose check now matches 'current v?0.N family' (optional v prefix), so a stale family ref can no longer hide behind a spelling difference; verified green on corrected docs and that both spellings of a non-current minor are flagged. P3 (optional) ADDED: try_axis_reductions_on_vector_give_scalar — rank-1 try_*_axis(0) scalar-output test for all four axis try forms (collapse to scalar identically to panic forms). Patch bump 0.24.0->0.24.1 (minor unchanged, so no user-doc version retarget needed). Full gate green (fmt, 4 guards incl. broadened family check, lean/dynamic/all-features tests + doctests, MSRV 1.85, RFC-031 fixture). Also carried (no separate changelog entry, per maintainer): crates/matten/README.md documentation link now points to the published nabbisen.github.io mdBook. Architect: after the P1 fix lands, v0.24.0 is review-clean. |
| 1.35.0 | 2026-06-27 | v0.24.2: test-organization refactor — co-locate unit tests with their modules. Internal only; no library code, public API, runtime behaviour, or dependency change; no test added/removed/re-gated (counts identical to v0.24.1: lean 243, dynamic 323, all-features 355 main + 100 doctests). Prompted by maintainer: the centralized src/tests/ tree was unfamiliar/non-standard and the testing guideline was clarified to require co-location. AUDIT FINDING: no inline #[test]/mod tests{} blocks anywhere (tests were already externalized) — and the layout was already INCONSISTENT (src/ops/broadcast.rs already co-located via src/ops/broadcast/tests.rs while 22 files sat in src/tests/). MIGRATED: 18 test files src/tests/<mod>.rs -> src/<mod>/tests.rs, each wired by #[cfg(test)] mod tests; in its parent (matching the broadcast precedent); removed src/tests.rs, src/tests/, and the central mod tests; in lib.rs. The two non-eponymous test files co-located with the module they exercise: shape_ops -> src/tensor/ops/tests.rs, elementwise -> src/ops/elementwise/tests.rs. SPLIT math tests (~478 lines) into themed groups src/math/tests/{whole,axis,matmul,dynamic}.rs (matmul group added because math.rs also covers dot/matmul, which the original 3-group suggestion overlooked); all groups well under 300. dynamic test sub-tree moved intact to src/dynamic/tests/ with feature-gating preserved (still skipped in lean). Migration verified behavior-neutral via absolute crate:: paths (no super::, no cross-file test helpers). Full gate green: fmt, 4 guards, lean/dynamic/all-features + doctests, MSRV 1.85, RFC-031 fixture. Patch bump 0.24.1->0.24.2 (minor unchanged, no user-doc version retarget). Also corrected a stale RFC-013 earmark (future Testing Strategy Refresh candidate RFC-055->RFC-057, since 055/056 took the v0.24 reductions). |
| 1.36.0 | 2026-06-27 | v0.24.3: fix an unused-import warning in the split math dynamic tests. Test-only; no library code, public API, runtime behaviour, or dependency change. FIXED: crates/matten/src/math/tests/dynamic.rs (created by the v0.24.2 split) imported MattenError+Tensor unconditionally, but all its tests are #[cfg(feature="dynamic")], so the import was unused in any non-dynamic build (incl. default cargo test/clippy) — flagged by the release checklist's cargo clippy -D warnings step. Now feature-gated (#[cfg(feature="dynamic")] use crate::{MattenError, Tensor};) to match the gated tests: neither unused without the feature nor missing with it. Verified: a maintainer-uploaded fix that DELETED the import was rejected because it breaks --features dynamic (E0433 cannot find Tensor/MattenError); the gated-import form is the correct fix. Clippy clean across default/lean/dynamic/all-features (-D warnings). PROCESS NOTE: v0.24.2 shipped with this warning because the local pre-tarball gate skipped the checklist's clippy -D warnings steps (lines 16-19) — those steps already existed and CI enforces them; they are now run on every release. No gate/CI change was needed (the protection already existed). Separately FLAGGED (not fixed here, out of scope): matten-data example data_02_select_columns lacks required-features=["csv"], so it fails to build under a workspace-wide --no-default-features --all-targets clippy (CI/checklist use -p matten for the lean clippy, so neither hits it). Patch bump 0.24.2->0.24.3. |
| 1.37.0 | 2026-06-27 | v0.25.0: companion-maturity line opens — promote matten-ndarray to production-ready (RFC-057). Label/docs/CI only; no API, runtime, error-variant, or dependency change to any crate; core matten unchanged. Architect accepted RFC-057 (no P0) with one required condition: examples must EXECUTE in CI, not just compile (P1). Applied: matten-ndarray status candidate->production-ready in crate README + lib.rs + Cargo.toml description + workspace README crate table + external-design maturity progression (added v0.25.0 entry; v0.19.0 candidate entry kept as history) + ROADMAP gate marked passed. bridge examples (to_arrayd/from_arrayd) were ALREADY executed in CI by the pre-existing smoke job — RFC-057's initial audit examined only the bridge/check jobs and missed smoke, so its 'compiled-not-executed' gap was inaccurate; no CI change was needed (P1 already met). Verified locally both examples print ok. API-snapshot file SKIPPED per architect ruling (two-function surface; README conversion-contract table is the snapshot-equivalent — verified it states both fns, error enum, copy, dynamic rejection, ndarray 0.16 minor, zero-axis rejection (made explicit), no-zero-copy). P2 stale-label guard added to check-release-docs.sh (context-aware: fails if matten-ndarray's own status files still say candidate; historical CHANGELOG/RFC/migration refs untouched). NOT v1.0 (status label only; lock-step family version retained; v1.0 needs explicit maintainer confirmation). matten-mlprep / matten-data stay Beta (separate decisions). Minor bump 0.24.3->0.25.0; user-doc pins/labels retargeted 0.24->0.25 (drift guard); introduction.md + compatibility.md describe the v0.25 family. Full gate green incl. clippy -D warnings + CI example execution. RFC-057 -> rfcs/done/. |
| 1.38.0 | 2026-06-27 | v0.26.0: companion-maturity continues — promote matten-mlprep Beta->production-ready candidate (RFC-058). Label/docs only; no API, runtime, error-variant, or dependency change; core matten unchanged; matten-mlprep stays matten-only. Architect accepted RFC-058 (no P0): rung is production-ready CANDIDATE, full production-ready DEFERRED because train_test_split is ordered-only (no shuffle) — a real caveat fitting 'usable seriously if documented limits acceptable'. Audited against candidate signals (verified, incl. CI): 17-test suite (determinism/zero-variance/NaN/shape/split edges), all FOUR examples executed in smoke job (namespaced [[example]] targets), non_exhaustive MattenMlprepError w/ Display+source, documented compat/MSRV policy, matten-only dep, Beta gate re-verified. Applied: status Beta->candidate in crate README + lib.rs (Status rewritten: stable small surface, ordered-split caveat noted) + workspace README table + external-design progression (added v0.26.0; v0.19.0 beta entry kept as history) + compatibility.md v0.26 entry + ROADMAP gate marked passed. Cargo.toml description verified maturity-neutral (no change). API-snapshot file SKIPPED per architect ruling (README Public API block + rustdoc is snapshot-equivalent; verified 8 elements incl. ordered-split, zero-variance, matrix-shape, dynamic rejection, no-ML scope, matten-only dep). P1: no stale Beta wording (added context-aware mlprep stale-label guard to check-release-docs.sh; mirrors ndarray guard) + all four examples still execute in CI; README/rustdoc accurately describe ordered-only split. P2: future full-production-ready exit criteria recorded (RFC-058 §5.1, Options A/B/C). NOT v1.0 (status label; lock-step family version retained). matten-data stays Beta (separate decision; may bundle in a future minor but did not block this RFC). Minor bump 0.25.0->0.26.0; user-doc pins/labels retargeted 0.25->0.26 (drift guard); introduction.md + compatibility.md describe v0.26 family. Full gate green incl. clippy -D warnings. RFC-058 -> rfcs/done/. |
| 1.39.0 | 2026-06-27 | v0.27.0: companion-maturity line COMPLETES — promote matten-data Beta->production-ready candidate (RFC-059). Label/docs/packaging only; no API, runtime, error-variant, or dependency change; NO scope expansion (RFC-042 lock preserved — still a CSV->tensor on-ramp, not a dataframe engine); core matten unchanged. Architect accepted RFC-059 CONDITIONALLY (no P0): two promotion-blocking hygiene fixes required first, both applied + verified: (1) stale Cargo.toml description 'Experimental...' (stale since v0.22.0 Beta) -> maturity-neutral 'CSV/table-to-Tensor preparation companion for matten (small PoC datasets).'; (2) data_00-data_05 examples lacked required-features=[csv] (cargo build --examples --no-default-features failed E0599 from_csv_str) -> added [[example]] entries, verified the --no-default-features build now succeeds by skipping gated examples + all 7 still execute in smoke. Audited vs candidate signals (verified incl. CI): 34 tests (most-tested companion), all 7 examples executed in smoke, 11-variant non_exhaustive MattenDataError w/ Display+source, compat/MSRV policy, RFC-042 anti-scope guard, own clippy gate. Rung: production-ready CANDIDATE; hold-at-Beta REJECTED (findings are packaging hygiene, not immature runtime); full production-ready DEFERRED (newest companion, wide CSV edge-case surface, streaming deferred) - separate future review. Applied: status Beta->candidate in crate README+lib.rs + workspace README table + companions.md/data.md/index.md (index.md was stale 'Experimental') + external-design progression (v0.27.0 entry; v0.19/0.22 history kept) + compatibility.md v0.27 + ROADMAP Track-A resolution note. README line-8 history extended (promoted to Beta v0.22.0, then candidate v0.27.0). API-snapshot file SKIPPED per ruling (README Public API block + rustdoc snapshot-equivalent; larger surface so kept exact: 7 types/methods + csv-feature/missing-value/numeric/scope-lock/error behaviors). P2: updated the v0.22.0 'must say Beta' check to enforce candidate, context-aware (historical Beta narrative + compatibility per-family history allowed; lead label/lib.rs/Cargo.toml checked). NOT v1.0. Minor bump 0.26.0->0.27.0; pins/labels retargeted 0.26->0.27 (drift guard); introduction.md + compatibility.md describe v0.27 family. Full gate green incl. clippy -D warnings + RFC-042 scope guard + no-default-features example build. RFC-059 -> rfcs/done/. NOTE for future tidy (out of scope): matten-ndarray Cargo.toml description embeds 'Production-ready' — the same neutrality principle applied here would suggest making it neutral too. |
| 1.40.0 | 2026-06-27 | v0.27.1: documentation & packaging legibility (RFC-060 + RFC-061). Docs/metadata only; no code, API, runtime, or dependency change; maintainer-authorized (docs-only, not an architect-ruling cycle). RFC-060: added docs/src/benchmarks/results.md (wired into SUMMARY under Benchmarks) — a CURATED summary of the accepted Phase 1 internal baseline + Phase 2 Rust peer comparison with representative medians and every RFC-049 caveat (workload/environment-specific, machine class + commit, accepted Baseline/Report IDs, 'not a ranking / not a faster-than claim'); full reports in benchmarks/reports/ stay the single source of truth; harness isolation preserved (no criterion/nalgebra in book/workspace/published graph). Added a release-docs freshness guard tying the page's cited Baseline/Report IDs to the report files. RFC-061 (maintainer chose Option D): kept the term 'production-ready'; added a small clarifying note at the TWO doc entrances only (root README by the crate table; mdbook introduction) that maturity labels describe stability within matten's documented PoC/small-data scope, NOT performance/scale — no rung renamed, no per-occurrence qualifier. Also applied the agreed description-neutrality tidy: matten-ndarray Cargo.toml description 'Production-ready conversion bridge...' -> 'Conversion bridge...'; all four crate descriptions now maturity-neutral (matten-ndarray maturity unchanged — still production-ready in README/lib.rs/table). PATCH bump 0.27.0->0.27.1 (minor unchanged; no family-label retarget). RFC-060/061 -> rfcs/done/. Reduced docs gate green (fmt, 4 guards incl. new freshness guard, cargo check). |
| 1.41.0 | 2026-06-27 | v0.28.0: matten-ndarray supported ndarray version 0.16->0.16+0.17 (RFC-062). Public-dependency compatibility event (the bridge exposes ndarray::ArrayD<f64> in to_arrayd/from_arrayd, so the supported ndarray minor is part of its PUBLIC type identity) — NOT a routine cargo update. Architect ACCEPTED Option B (range >=0.16.1, <0.18), subject to a compatibility-matrix CI; no P0. Requirement widened from "0.16" to ">=0.16.1, <0.18"; Cargo resolves ndarray to the consumer's minor (a project with no other ndarray dep gets latest-in-range 0.17.2). DECISION-DETERMINING verification (per the ruling's hard line): the UNCHANGED bridge source compiled + passed 17 conversion tests + 3 doctests + both examples against BOTH 0.16.1 AND 0.17.2 via cargo update -p ndarray --precise — NO version-conditional code needed, so Option B holds (fallback to Option A not triggered). P1 satisfied: added CI matrix job bridge-ndarray-compat (ndarray=[0.16.1,0.17.2]; test + --doc + both examples per pin, fresh checkout so per-job --precise lock edits are not committed); docs state public type identity + yanked-0.17.0 caveat (not a tested target) + docs.rs-renders-one-minor caveat (README Compatibility + Supported-ndarray bullet + lib.rs rustdoc); core matten remains ndarray-free (published-dependency-isolation green). MSRV untouched (1.85; ndarray 0.17.2 declares rust-version 1.64). No bridge API/signature/behavior/copy-semantics/dynamic-rejection/error/zero-copy change. Committed Cargo.lock resolves ndarray to 0.17.2 (latest in range). Family minor 0.27.1->0.28.0 (lock-step RFC-030; whole family bumps though only matten-ndarray materially affected); pins/family labels retargeted 0.27->0.28; introduction.md + compatibility.md describe v0.28 family. RFC-049 peer benchmark (snapshot at ndarray 0.16.1) deliberately NOT re-run — future separate task (out of scope per ruling §9). P2 (deferred, low): release-checklist item for future public-dependency-minor changes. Full gate green. RFC-062 -> rfcs/done/. |
| 1.43.0 | 2026-06-28 | v0.28.1 (FINAL, unpublished — consolidates the prior 1.42.0 entry): matten-ndarray ndarray support NARROWED to Option A. (A) RFC-062 reversal: maintainer chose Option A (ndarray = "0.17", single-version) over the architect-accepted Option B range (>=0.16.1, <0.18) that shipped in v0.28.0 — to keep Cargo.toml simple/readable; ndarray 0.17 is a small backwards-compatible upgrade so the range's only benefit (sparing 0.16 users) was judged not worth the baggage. NOT a CI-forced fallback (bridge compiled fine on both minors); a legibility judgment call. Architect ruling pre-listed Option A as acceptable (§3.1/§13) -> applied directly, no re-review. Cargo resolves ndarray to 0.17.2 (latest non-yanked 0.17 patch). REMOVED the bridge-ndarray-compat CI matrix (one supported minor -> standard bridge job against resolved 0.17.2 suffices). Docs simplified: matten-ndarray README compatibility + Supported-ndarray lines + lib.rs rustdoc + compatibility.md v0.28 entry + introduction.md -> 'supports the 0.17 minor' (resolved minor still part of public type identity; 0.17.0 yanked -> use non-yanked patch); range-specific docs.rs/multi-minor caveats dropped. RFC-062 (in done/) amended: header status -> Option A as of v0.28.1 + Addendum recording the reversal. v0.28.0 CHANGELOG entry (Option B range) PRESERVED as the delivered tarball's record; [0.28.1] now documents the narrowing. No bridge API/behavior/error/copy-semantics/zero-copy change; core matten ndarray-free; MSRV 1.85 holds with 0.17.2. (B) RFC-062 P2 RESOLVED: 'Public-dependency-minor changes' gate added to release-checklist.md (precedent ref updated: single-version vs range both covered; no longer cites the removed matrix). (C) Entrance README.md: small dynamic on-ramp example (Element heterogeneous tensor -> try_numeric_with(NumericPolicy::default().none_as(0.0)) -> clean f64 [1.0,2.5,0.0,4.0]); verified compiles+runs under --features dynamic; off-by-default + dynamic-guide link. Held at 0.28.1 (v0.28.0 and this revision both unpublished); minor unchanged, no family-label retarget. Full gate green (code release: dependency requirement change). No open RFC-062 items remain. |
| 1.44.0 | 2026-06-28 | v0.28.2: benchmark docs/reports only (no code/API/runtime/dependency change; maintainer-directed). (A) BENCHMARK RESULT REFRESH: added v0.2 reports (internal-baseline-v0.2.md, peer-comparison-v0.2.md) from a maintainer run at workspace 0.28.1 (commit ef06369, rustc 1.93.1, same 8vCPU AMD VM class as v0.1) under the UNCHANGED RFC-049 methodology. New IDs ...-v0.2. Done as a VERSIONED refresh (not an in-place overwrite) because the v0.1 reports are architect-ACCEPTED artifacts with v0.1-suffixed IDs and the v0.x naming was designed for refreshes; v0.1 retained as the accepted reference with a 'superseded for current numbers' banner. v0.2 numbers match v0.1 within VM variance (no internal regression v0.22.x->v0.28.1; sum_mean_axis still ~400x sum_mean; peer pattern holds — markov competitive/inverted, matmul/pagerank/heat ~8-11x). Peak RSS NOT captured this run (VM lacked GNU /usr/bin/time; informative-only, never a gate) — noted honestly. v0.2 explicitly labeled maintainer-run, NOT separately architect-reviewed (methodology+program remain accepted). (B) ENV-CAPTURE SNIPPET: added the missing runnable capture block (generalized from the maintainer's bench-01 script, stale '(0.22.3)' comment dropped) to benchmarks/README.md under a new 'How to regenerate (with environment capture)' section that consolidates capture+compile-check+timings+memory+peers; methodology.md Environment-recording now points to it; README's duplicate Running/Memory sections folded in. (C) TWO-AUDIENCE RESTRUCTURE: book benchmarks index routes reader -> results.md (curated readable summary, refreshed to v0.2 numbers, reframed as the reader view) vs maintainer -> methodology.md + harness README regenerate section. RFC-060 freshness guard extended to also map the v0.2 report IDs. PATCH bump 0.28.1->0.28.2 (minor unchanged; no family-label retarget). Reduced docs gate green. Note: 0.28.0/0.28.1/0.28.2 all unpublished. |
| 1.45.0 | 2026-06-28 | v0.28.3: benchmark-harness config only (no published crate touched; maintainer-directed). Bumped the out-of-workspace peer benchmark's optional ndarray pin 0.16->0.17 (benchmarks/Cargo.toml) so the peer comparison tracks the bridge (which moved to ndarray 0.17 in v0.28.x) rather than lagging a minor. Reasoning (maintainer asked 'any reason NOT to bump given the implementation was bumped?'): none of substance — the 0.16 pin was lag, not a deliberate choice; VERIFIED the one thing that could have blocked it: the peers bench compiles clean against ndarray 0.17.2 (lock 0.16.1->0.17.2, no errors). Harness is publish=false / excluded from the workspace, so no published crate, workspace dep, or public API changes. SEQUENCING (honest transient): the v0.2 peer numbers were measured at 0.16.1 and predate the pin; refreshed 0.17 numbers must come from the maintainer's machine class on the next peers run (NOT generated in-container — environment consistency), so peer-comparison-v0.2.md + results.md now state pin=0.17, numbers=0.16.1-pending. Split out from v0.28.2 at maintainer direction (initially folded into the unpublished v0.28.2; maintainer required it be its own v0.28.3 — v0.28.2 restored to benchmark-docs/reports-only). PATCH bump 0.28.2->0.28.3. Reduced docs gate green. 0.28.0-0.28.3 all unpublished. |
| 1.46.0 | 2026-06-28 | v0.28.4: benchmark results refresh + dependency-sync drift guard (no published crate touched; maintainer-directed). (A) DRIFT GUARD: added scripts/check-benchmark-dependency-sync.sh — parses the workspace ndarray requirement (root Cargo.toml [workspace.dependencies]) and the harness peer pin (benchmarks/Cargo.toml) and FAILS if they diverge. The benchmark harness is workspace-excluded so it can't inherit { workspace = true }; the pin is manual, and this guard makes 'forgot to sync' impossible to miss (the exact v0.28.3 situation). Verified: passes when both 0.17, fails with a clear fix-it message when harness set to 0.16. Wired into CI check job (after published-dependency-isolation) + release-checklist source verification + referenced from the RFC-062 P2 public-dependency-minor checklist item. (B) BENCHMARK REFRESH: replaced the v0.2 reports (internal-baseline-v0.2.md, peer-comparison-v0.2.md) + reader results.md with a fresh v0.28.3 run (commit 5953c9f, same 8vCPU AMD VM, rustc 1.93.1). KEY: the peer comparison now runs at ndarray 0.17.2 (env-capture log showed ndarray 0.16.1->0.17.2) — matching the shipped bridge, resolving the 'measured at 0.16.1, pending 0.17 refresh' caveat from v0.28.3. v0.2 IDs kept (v0.2 was never an accepted/frozen artifact, unlike v0.1; user said 'replace the existing'). Internal numbers within VM variance of v0.1; relative peer positioning unchanged (markov competitive/ahead of BOTH peers ~924ns; matmul/pagerank/heat ~7.5-9x; cosine/linreg 2-4x). Absolute peer timings ~40% lower than the 0.16.1 run but ALL THREE libs moved together = VM-load effect, not a code change; noted honestly (positioning is the durable signal). Peak RSS again not captured (no GNU time on VM). v0.1 remains the architect-accepted reference; v0.2 maintainer-run, not separately reviewed. PATCH bump 0.28.3->0.28.4. Reduced docs gate + new guard green. 0.28.3 PUBLISHED; 0.28.4 is the next release. |
| 1.47.0 | 2026-06-28 | v0.28.5: dynamic-JSON ingestion example + equal-on-ramps framing (docs/examples only; maintainer-directed). Motivated by a maintainer question — JSON felt unsupported. Audit found JSON support itself is solid (default `json` feature; from_json/from_json_dynamic/load_json; thorough boundary.md ## JSON section; core examples 10_json_roundtrip + 11_csv_numeric_loading symmetric and indexed). The ONE asymmetry was in the dynamic on-ramp examples: from_csv_dynamic had two dedicated examples (dynamic_02 missing values, dynamic_05 dirty cleanup) while from_json_dynamic appeared only inside dynamic_00. (data.md being CSV-only is correct — it's the matten-data companion page, CSV-only by RFC-042.) FIX: added crates/matten/examples/dynamic_08_json_ingestion.rs mirroring dynamic_02's structure exactly — cfg(not dynamic) fallback main; cfg(json) from_json_dynamic("[[1, 2.5, null], [4.0, 5, 6]]") with cfg(not json) from_elements fallback; demonstrates count_none/none_mask, strict try_numeric Err, then try_numeric_with(NumericPolicy::default().none_as(0.0)) -> clean [1.0,2.5,0.0,4.0,5.0,6.0]. All APIs verified against source before writing (from_json_dynamic accepts nested 2D mixed int/float/null; null->None, int->Int, float->Float per dynamic/parse/json.rs; as_slice/is_dynamic/none_mask/try_numeric_with confirmed). Verified runs under dynamic,json and compiles clean under dynamic-only + no-features (both fallbacks, zero warnings). No [[example]] entry needed (compiles feature-less like siblings 00-07; only 10/11/12 carry required-features). FRAMING: index.md dynamic section now states from_json_dynamic and from_csv_dynamic are equal on-ramps differing only in format + lists dynamic_08 + adds a json run line; dynamic_07 Step 1 comment made format-neutral (was 'a CSV row'). Wired dynamic_08 into CI smoke (--features dynamic,json). PATCH bump 0.28.4->0.28.5. Gate: fmt, 5 guards, clippy all-targets all-features, build --examples all-features, ran dynamic_08 + siblings, release-docs — all green. 0.28.4 PUBLISHED; 0.28.5 next. |
| 1.48.0 | 2026-07-03 | v0.29.0-pre.1: RFC-063 Phase 1 visual-understanding docs prerelease. Added RFC-063 + compact Phase 1 handoff; implemented Markdown/ASCII-only diagrams across operators, shape ops, math, shape composition, statistics, dynamic, matten-data, and tutorial start-here. Scope remained docs/RFC/handoff only: no public API, runtime behavior, dependency, tool, generated artifact, image asset, or maturity-label change. RFC-063 stays in `rfcs/proposed/` as an umbrella: Phase 1 implemented; Phase 2 examples require a compact handoff; Phase 3 tooling and Phase 4 companion crates require later approval. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.1`), and the release-doc guard now accepts SemVer prerelease versions/pins. |
| 1.49.0 | 2026-07-03 | v0.29.0-pre.2: RFC-063 Phase 2 canonical visual-summary examples prerelease. Added exactly the accepted first implementation set: `57_visual_shape_axis_summary`, `dynamic_09_visual_readiness_summary`, and `data_06_visual_readiness_summary`; helpers remain local to examples, output is deterministic/plain terminal text, and no public API/dependency/tool/image/generated-artifact/plotting/notebook/GUI scope was added. Wired canonical examples into CI smoke + release checklist; `data_06` is CSV feature-gated; dynamic example compiles without `dynamic` and runs with it. Example docs and tutorial path link the new readability summaries without user-facing process-phase wording. RFC-063 remains proposed as umbrella; Phase 3+ requires later approval. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.2`). |
| 1.50.0 | 2026-07-03 | v0.29.0-pre.3: RFC-063 optional `matten-mlprep` visual-standardization summary prerelease. After Phase 2 acceptance, added exactly one standardization-only companion example, `mlprep_visual_standardize_summary`, showing before/after column mean, before/after column std, and unchanged shape using deterministic hard-coded data. Also applied accepted Phase 2 P2 wording polish: dynamic readiness output now says converted shape/values instead of clean shape/values. Wired the mlprep example into CI smoke + release checklist + companion docs. No public API, dependency, tool, generated artifact, image, plotting, notebook, GUI, runtime, MSRV, or maturity-label change. Phase 3 tooling remains deferred. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.3`). |
| 1.51.0 | 2026-07-04 | v0.29.0-pre.4: RFC-063 Phase 3 first local-tool prerelease. Added `tools/matten-report`, a workspace-excluded `publish = false` local Markdown/plain-text report tool for `matten-data` readiness only, with explicit `--demo data-readiness` and `--input <csv> --kind data-readiness --select <cols>` modes plus optional `--output`. Added deterministic fixtures and exact-output tests for success, missing values, non-numeric values, and CLI policy; wired manifest-path check/test/Clippy/smoke commands into CI and release checklist. Accepted dependency policy delta: path-only local deps for this unpublished excluded tool, with API drift caught by local gates and no prerelease version-sync chore. No public API, published crate, workspace membership, dependency leak, JSON/SVG/HTML/Vega-Lite, plotting, notebook, GUI, telemetry, network, runtime, MSRV, or maturity-label change. Future report families remain deferred. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.4`). |
| 1.52.0 | 2026-07-04 | v0.29.0-pre.5: RFC-063 Phase 3 shape-flow local-tool prerelease. Extended `tools/matten-report` with the accepted second report family: `--demo shape-flow`, a fixed deterministic Markdown/plain-text report for broadcasting, reshape, `mean_axis(0)`, `mean_axis(1)`, and matmul shape flow. Shape-flow remains demo-only: no `--input` mode, no arbitrary expression parser, no source scanning, no automatic Tensor operation tracing, no lazy graph, no public API, no published crate, no workspace membership, no new dependency, no SVG/HTML/Vega-Lite/JSON/images/ANSI/notebook/GUI scope. Added exact-output tests and parser-policy coverage, kept data-readiness exact-output tests, documented the fixed-demo boundary in the tool README, and wired shape-flow smoke commands into CI + release checklist. Current-family docs retargeted to exact prerelease pins (`0.29.0-pre.5`). |
| 1.53.0 | 2026-07-12 | Docs-governance Handoff 01 implementation prepared for review. Added `docs/design/coverage-gap-resolution.md` to resolve the three pre-archival coverage gaps: retired unmaintained numeric compile/rebuild/memory targets as live requirements, inventoried current NumPy golden coverage while keeping property/fuzz work as future hardening, and recorded that `Tensor` has `Debug` but no `Display` contract. Aligned contributing docs, RFC-013, and compatibility docs. No public API, dependency, version, release-scope, runtime, benchmark, or test-gate change. |
| 1.54.0 | 2026-07-12 | Docs-governance Handoff 02 implementation prepared for review. Archived the v0.19.0 requirements, external-design, and roadmap snapshots under `docs/design/history/` with historical-only banners; added `docs/design/README.md` with the four-plane ownership rule and the README-note-over-RFC-066 disposition; linked the rule from `rfcs/README.md`; kept `docs/design/**` outside the mdBook. No public API, dependency, version, release-scope, runtime, or user-doc contract change. |
| 1.55.0 | 2026-07-13 | Docs-governance Handoff 03 implementation prepared for review. Expanded `docs/src/philosophy.md` from a stub into an evergreen principles page distilled from tracked `docs/design/history/` snapshots: developer-experience-first tensor work, family-car positioning, one concrete `Tensor`, no visible lifetime burden, concrete-before-generic dynamic ingestion, panic-local/Result-boundary split, explicit non-goals, and a short migration pointer. No public API, dependency, version, release-scope, runtime, benchmark guarantee, or mdBook structure change. |
| 1.56.0 | 2026-07-13 | RFC-054 lifecycle closure status alignment. Closed `matten-migrate` as implemented for the reviewed local advisory tool scope, moved RFC-054 to `rfcs/done/`, and recorded that rewrite/apply, source mutation, Cargo.toml editing, public `matten-migrate` packaging, and stronger migration automation are extracted to future RFC/release-policy ownership. The active RFC index now has no proposed RFCs. Roadmap-only alignment; no public API, dependency, version, release-scope, runtime, or tool behavior change. |
| 1.57.0 | 2026-07-13 | Proposed RFC-066: v1.0 readiness audit and release decision gate. Opens an audit-only RFC to review public API snapshot evidence, panic/Result boundary stability, serde/canonical format stability, documented limitations/non-goals, companion maturity under lock-step family versioning, and release-gate evidence before any v1.0 decision. This does not authorize a v1.0 release, version bump, tag, publish, API change, dependency change, or companion promotion. |
| 1.58.0 | 2026-07-13 | Proposed RFC-067: v1.0 family maturity policy. Drafts the RFC-066 MD-1 policy answer: production-ready-candidate companions are not automatic v1.0 blockers if a future v1.0 release RFC explicitly lists each crate's maturity label, confirms API stability and documented caveats, and avoids silent companion promotion. This is policy planning only; no v1.0 release preparation, version bump, tag, publish, API change, dependency change, or companion promotion is authorized. |
| 1.59.0 | 2026-07-13 | Implemented RFC-067 as repository policy and recorded the RFC-066 MD-1 resolution in the v1 readiness audit, compatibility policy, and release checklist. Candidate-labeled companions are not automatic v1.0 blockers, but any future v1.0 release RFC must include the RFC-067 family maturity table and decide each candidate-labeled crate explicitly. RFC-067 moved to `rfcs/done/`. No v1.0 release preparation, version bump, tag, publish, API change, dependency change, or companion promotion is authorized. |
| 1.60.0 | 2026-07-13 | Prepared v0.31.0 as an RFC-066/RFC-067 cleanup release. Closed RFC-066 as implemented for the reviewed audit-only scope, kept RFC-067 implemented as repository policy, retargeted current-family documentation to 0.31.0, and added release notes for the v1.0 readiness audit / family maturity policy cleanup. No v1.0 release preparation, tag, publish, public API change, dependency change, runtime behavior change, MSRV change, feature-flag change, maturity-label change, or companion promotion is authorized. |
| 1.61.0 | 2026-07-13 | Proposed RFC-068: rich local visualization artifacts. Opens the next visualization phase after RFC-063/RFC-065 with a conservative first slice: static self-contained HTML output for the existing local `tools/matten-report --demo educational-path` report, with Markdown/plain text remaining the default. This is planning/handoff work only; no implementation, public API, public report/viz crate, SVG/Vega-Lite/JSON output, expression tracing, autograd, published-crate dependency change, version bump, tag, publish, or companion maturity change is authorized. |
| 1.62.0 | 2026-07-14 | RFC-068 Phase 1 implementation prepared for review. Added `tools/matten-report --demo educational-path --format html --output <path>` as a static self-contained local HTML artifact, kept Markdown/plain text as the default, required explicit `--output` for HTML, and rejected HTML for all other report families/input mode. Added std-only escaping/rendering, parser and HTML-safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, new dependency, published-crate graph change, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.63.0 | 2026-07-15 | RFC-068 shared educational report data handoff drafted. Audit found that the educational-path Markdown and HTML renderers duplicate the same fixed tensor computations and derived values; the next proposed slice is a behavior-neutral private data-model extraction inside `tools/matten-report` before expanding HTML to another report family. No CLI behavior, output format, public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.64.0 | 2026-07-15 | RFC-068 shared educational report data implementation prepared for review. Extracted the educational-path fixed tensor computations and derived values into one private data builder consumed by both Markdown and HTML renderers, preserving byte-identical output and adding an exact HTML snapshot test alongside the existing HTML safety test. No CLI behavior, output format, public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.65.0 | 2026-07-15 | RFC-068 shape-flow HTML handoff drafted. The next proposed feature slice extends local static HTML output to exactly one additional fixed report family, `tools/matten-report --demo shape-flow`, with explicit `--output`, exact HTML snapshot coverage, CI/release-checklist smoke commands, and the same static/no-JS/no-network/no-external-asset boundary as educational-path HTML. Markdown/plain text remains default. No HTML for other report families or input mode, public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.66.0 | 2026-07-15 | RFC-068 shape-flow HTML implementation prepared for review. Added `tools/matten-report --demo shape-flow --format html --output <path>` as the second static self-contained local HTML artifact, generalized HTML policy/error text to accept educational-path and shape-flow only, kept Markdown/plain text as default, and kept HTML rejected for data-readiness, dynamic-readiness, mlprep-standardization, and input mode. Added exact shape-flow HTML snapshot and safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.67.0 | 2026-07-15 | Prepared 0.32.0 as an RFC-068 rich local visualization-artifact release. Closed RFC-068 as implemented for local static HTML artifacts covering `educational-path` and `shape-flow`, retargeted current-family documentation to 0.32.0, and added release notes for the local-tool visualization scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, tag, publish, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, or autograd scope is authorized. |
| 1.68.0 | 2026-07-15 | Drafted the post-0.32 RFC-068 visualization continuation audit. The 0.32.0 release scope remains local static HTML artifacts for `tools/matten-report --demo educational-path` and `tools/matten-report --demo shape-flow`; the follow-up audit recommends handoff review for `tools/matten-report --demo dynamic-readiness` HTML before any implementation. No direct implementation, public report/viz crate, core visualization API, expression tracing, autograd, dependency change in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, tag, publish, or companion maturity change is authorized. |
| 1.69.0 | 2026-07-15 | Drafted the RFC-068 dynamic-readiness local HTML artifact handoff. The proposed next slice is local static HTML for `tools/matten-report --demo dynamic-readiness`, keeping Markdown/plain text as default and requiring explicit `--output`; `data-readiness`, `mlprep-standardization`, input-mode HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, version bump, tag, publish, and companion maturity changes remain out of scope. |
| 1.70.0 | 2026-07-15 | RFC-068 dynamic-readiness local HTML implementation prepared for review. Added `tools/matten-report --demo dynamic-readiness --format html --output <path>` as the third static self-contained local HTML artifact, shared the fixed dynamic-readiness report data between Markdown and HTML renderers while preserving Markdown output, generalized HTML policy/error text to accept educational-path, shape-flow, and dynamic-readiness only, and kept HTML rejected for data-readiness, mlprep-standardization, and input mode. Added exact dynamic-readiness HTML snapshot and safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.71.0 | 2026-07-15 | Prepared 0.33.0 as an RFC-068 visualization-continuation release. Stopped feature work for the current release after the reviewed dynamic-readiness local HTML artifact, retargeted current-family documentation to 0.33.0, and added release notes for the local-tool visualization continuation scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, tag, publish, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, or autograd scope is authorized. |
| 1.72.0 | 2026-07-15 | Drafted the post-0.33 RFC-068 visualization continuation audit and mlprep-standardization local HTML artifact handoff. The proposed next reviewed slice is local static HTML for `tools/matten-report --demo mlprep-standardization`, keeping Markdown/plain text as default and requiring explicit `--output`; `data-readiness`, input-mode HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, version bump, tag, publish, and companion maturity changes remain out of scope. |
| 1.73.0 | 2026-07-15 | RFC-068 mlprep-standardization local HTML implementation prepared for review. Added `tools/matten-report --demo mlprep-standardization --format html --output <path>` as the fourth static self-contained local HTML artifact, shared the fixed mlprep-standardization report data between Markdown and HTML renderers while preserving Markdown output, generalized HTML policy/error text to accept educational-path, shape-flow, dynamic-readiness, and mlprep-standardization only, and kept HTML rejected for data-readiness and input mode. Added exact mlprep-standardization HTML snapshot and safety tests, README documentation, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.74.0 | 2026-07-15 | Recorded 0.34.0 as an RFC-068 visualization-continuation release. Stopped feature work for the current release after the reviewed mlprep-standardization local HTML artifact, retargeted current-family documentation to 0.34.0, and added release notes for the local-tool visualization continuation scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, autograd, data-readiness HTML, or input-mode HTML scope is authorized. |
| 1.75.0 | 2026-07-16 | Drafted the post-0.34 RFC-068 visualization gap audit. The audit records that `educational-path`, `shape-flow`, `dynamic-readiness`, and `mlprep-standardization` now have local static HTML artifacts, while `data-readiness` and input-mode reports remain Markdown/plain-text only. It authorizes no implementation and asks review to decide whether to close the fixed-demo local HTML line or draft a dedicated demo-only `data-readiness` HTML handoff. Input-mode HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG, Vega-Lite, JSON report, notebook, GUI, version bump, tag, publish, and companion maturity changes remain out of scope. |
| 1.76.0 | 2026-07-16 | Drafted the RFC-068 data-readiness demo-only HTML handoff. The proposed next reviewed slice is local static HTML for `tools/matten-report --demo data-readiness`, keeping Markdown/plain text as default and requiring explicit `--output`; input-mode HTML for CSV files, failure-fixture HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG/Vega-Lite/JSON/notebook/browser scope, version bump, tag, publish, and companion maturity changes remain out of scope. |
| 1.77.0 | 2026-07-16 | RFC-068 data-readiness demo-only HTML implementation prepared for review. Added `tools/matten-report --demo data-readiness --format html --output <path>` as a static self-contained local HTML artifact for the fixed success-path demo, generalized HTML policy/error text so all fixed demos support HTML, and kept input-mode HTML rejected as the remaining negative policy case. Added exact data-readiness HTML snapshot and safety tests, README documentation, visual-understanding docs, and CI/release-checklist smoke commands. No public API, public report/viz crate, dependency, published-crate graph, SVG/Vega-Lite/JSON/notebook/browser scope, expression tracing, autograd, version bump, tag, publish, or companion maturity change is authorized. |
| 1.78.0 | 2026-07-16 | Prepared 0.35.0 as an RFC-068 visualization-continuation release. Stopped feature work after the reviewed data-readiness local HTML artifact, retargeted current-family documentation to 0.35.0, and added release notes for completing fixed-demo local HTML coverage. Markdown/plain text remains default, HTML requires explicit `--output`, input-mode HTML remains rejected, and no public report/viz crate, public API, published dependency, runtime behavior, MSRV, feature-flag, maturity-label, companion-promotion, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, or autograd scope is authorized. |
| 1.79.0 | 2026-07-16 | Drafted the post-0.35 RFC-068 fixed-demo HTML closure audit. The audit records that all five fixed `tools/matten-report --demo ...` families now support local static HTML after 0.35.0 and recommends closing the RFC-068 fixed-demo local HTML line rather than continuing visualization work automatically. Input-mode HTML, public report/viz crates, core visualization APIs, expression tracing, autograd, dependency changes in published crates, SVG, Vega-Lite, JSON report, notebook, GUI, version bump, tag, publish, and companion maturity changes remain out of scope unless a separate future RFC or handoff is reviewed. |
| 1.80.0 | 2026-07-16 | Proposed RFC-069 input-mode HTML report policy and drafted its policy audit. This opens a separate post-RFC-068 review path for deciding whether `tools/matten-report --input <csv> --kind data-readiness --select <cols> --format html --output <path>` may become a narrow local static HTML artifact. The proposed boundary is summary-only, bounded, escaped, explicit-output, local-tool-only, and data-readiness-only. No implementation, public report/viz crate, core visualization API, expression tracing, autograd, dependency change in published crates, SVG, Vega-Lite, JSON report, notebook, GUI, version bump, tag, publish, or companion maturity change is authorized. |
| 1.81.0 | 2026-07-16 | Drafted the RFC-069 input-mode HTML implementation handoff. The handoff proposes the first code slice for `tools/matten-report --input <csv> --kind data-readiness --select <cols> --format html --output <path>`, with Markdown/plain text still default, explicit `--output`, summary-only static HTML, success and numeric-conversion-error reports, hostile-input escaping tests, and concrete display caps for column lists, long paths/headers/errors, and row-major tensor previews. No implementation, public report/viz crate, core visualization API, expression tracing, autograd, dependency change in published crates, SVG, Vega-Lite, JSON report, notebook, GUI, version bump, tag, publish, or companion maturity change is authorized until handoff review accepts it. |
| 1.82.0 | 2026-07-17 | RFC-069 input-mode HTML implementation prepared for review. Added `tools/matten-report --input <csv> --kind data-readiness --select <cols> --format html --output <path>` as a local static HTML artifact for data-readiness input mode, keeping Markdown/plain text as default and requiring explicit `--output`. The implementation renders summary-only success and numeric-conversion-error reports, uses hostile-input escaping tests, bounds source/selected/left-out column displays, long paths/headers/errors, and row-major tensor previews, and adds README, CI, and release-checklist smoke coverage. No public report/viz crate, core visualization API, expression tracing, autograd, dependency change in published crates, SVG, Vega-Lite, JSON report, notebook, GUI, version bump, tag, publish, release prep, or companion maturity change is authorized. |
| 1.83.0 | 2026-07-17 | Released 0.36.0 as an RFC-069 input-mode HTML release. Stopped feature work after the reviewed input-mode data-readiness local HTML artifact, retargeted current-family documentation to 0.36.0, moved RFC-069 to implemented status, and added release notes for the local-tool visualization scope. Markdown/plain text remains default, HTML requires explicit `--output`, and no public report/viz crate, public API, published dependency, runtime behavior outside the reviewed local-tool command, MSRV, feature-flag, maturity-label, companion-promotion, SVG, Vega-Lite, JSON report, notebook, GUI, expression tracing, autograd, tag, publish, or general raw CSV HTML rendering scope is authorized. |
| 1.84.0 | 2026-07-17 | Drafted the post-0.36 RFC-069 input-mode HTML closure audit. The audit records that data-readiness input-mode local HTML shipped in 0.36.0 and recommends closing RFC-069 for that reviewed scope rather than adding more input-mode HTML automatically. More input-mode HTML paths, public report/viz crates, JSON/SVG/Vega-Lite output, notebook/browser integration, expression tracing, autograd, core visualization APIs, dependency changes in published crates, version bump, tag, publish, release prep, and companion maturity changes remain out of scope unless a separate future RFC or handoff is reviewed. |
| 1.85.0 | 2026-07-17 | Audited remaining RFC/theme tracking after RFC-069 closure. Confirmed there are no active proposed RFCs, refreshed stale roadmap baseline text that still pointed at v0.20+ materialization, updated RFC-049 roadmap posture to Phases 1-3 implemented with Phase 4 hard gates extracted, refreshed later-theme notes for public visualization/report readiness, JSON/SVG/Vega-Lite output, streaming/large CSV, broader stats/linalg scope, `matten-nalgebra`, `matten-candle`, and companion full-production decisions. No new RFC, implementation, version bump, release prep, public API, dependency, tag, publish, or generated artifact is authorized. |
| 1.86.0 | 2026-07-17 | Proposed RFC-070 as an audit-only public visualization/report readiness decision gate. The RFC asks whether local `tools/matten-report` artifacts are mature enough to justify a future public `matten-report` or `matten-viz` product surface, and requires evidence on report-model ownership, renderer boundaries, crate/dependency policy, output-format readiness, and core `Tensor` boundary. No implementation, public crate, public renderer API, JSON/SVG/Vega-Lite output, notebook/browser integration, expression tracing, autograd, dependency change, version bump, release prep, tag, publish, or generated artifact is authorized. |
| 1.87.0 | 2026-07-17 | Drafted the RFC-070 public visualization/report readiness audit. The audit finds that local `tools/matten-report` artifacts are useful but not ready for public `matten-report` / `matten-viz` crates, public report-model APIs, or reusable renderer APIs. It recommends keeping renderers private, keeping core `matten` visualization-free, and treating JSON report-schema policy or private report-model extraction as possible future prerequisites. No implementation, public crate, public API, dependency change, output-format expansion, version bump, release prep, tag, publish, or generated artifact is authorized. |
| 1.88.0 | 2026-07-17 | Drafted the RFC-070 private report-model extraction handoff as the first post-audit prerequisite candidate. The handoff allows only a future reviewed behavior-neutral refactor inside `tools/matten-report` to reduce private renderer/model duplication, while keeping report types private and preserving all CLI behavior, exact Markdown/HTML outputs, rejection paths, escaping, and bounds. Public report schemas, public renderer APIs, public `matten-report` / `matten-viz` crates, JSON/SVG/Vega-Lite output, dependency changes, version bump, release prep, tag, publish, and generated artifacts remain unauthorized. |
| 1.89.0 | 2026-07-17 | RFC-070 private report-model extraction implementation prepared for review. Extracted the repeated static HTML document shell in `tools/matten-report` into private helpers shared by all existing HTML report paths, while keeping report-family data models private and preserving byte-identical exact Markdown/HTML snapshots, CLI behavior, rejection paths, escaping, and bounds. No public report schema, public renderer API, public `matten-report` / `matten-viz` crate, workspace membership change, dependency change, JSON/SVG/Vega-Lite output, version bump, release prep, tag, publish, or generated checked-in artifact is authorized. |
| 1.90.0 | 2026-07-17 | Aligned roadmap truth after the RFC-070 private report-model extraction implementation review and commit `783d757`. The private `tools/matten-report` HTML-shell helper extraction is now recorded as implemented, reviewed, and committed; JSON report-schema policy, public renderer APIs, public `matten-report` / `matten-viz` crates, output-format expansion, dependency changes, release prep, version bump, tag, publish, and generated artifacts remain separate future review decisions. |
| 1.91.0 | 2026-07-17 | Drafted the RFC-070 JSON report-schema policy audit handoff as the next audit/design-only planning gate. The handoff asks whether JSON report data is worthwhile at all, whether it should remain private local-tool output or become future public-contract material, and what compatibility, bounds, security, ownership, and dependency policies would be required before implementation. No JSON implementation, `--format json`, public schema, public renderer API, public crate, dependency change, release prep, version bump, tag, publish, or generated artifact is authorized. |
| 1.92.0 | 2026-07-21 | Drafted the RFC-070 JSON report-schema policy audit. The audit recommends exploring JSON only as private local-tool output first, with a common private envelope, family-specific payloads, `schema_version: 0`, deterministic exact snapshots, and no public compatibility promise. It recommends fixed-demo JSON before input-mode JSON and keeps public JSON schema, public renderer APIs, public `matten-report` / `matten-viz` crates, core visualization APIs, dependency changes in published crates, release prep, version bump, tag, publish, generated artifacts, and broader linalg scope unauthorized. |
| 1.93.0 | 2026-07-21 | Drafted the RFC-070 fixed-demo private JSON report implementation handoff. The proposed implementation boundary would add `tools/matten-report --demo ... --format json --output <path>` for the five fixed demos only, using deterministic `schema_version: 0` / `schema_status: private-local` JSON with exact snapshots. Input-mode JSON, public schemas, public renderer APIs, public `matten-report` / `matten-viz` crates, dependency changes in published crates, release prep, version bump, tag, publish, generated artifacts, SVG/Vega-Lite/browser/notebook output, expression tracing, autograd, and broader linalg scope remain unauthorized. |
| 1.94.0 | 2026-07-21 | RFC-070 fixed-demo private JSON report implementation prepared for review. Added `tools/matten-report --demo ... --format json --output <path>` for the five fixed demos only, using deterministic `schema_version: 0` / `schema_status: private-local` JSON with exact snapshots for data-readiness, shape-flow, dynamic-readiness, mlprep-standardization, and educational-path. JSON requires explicit output, input-mode JSON is rejected, Markdown/HTML behavior remains covered by existing exact snapshots, and direct `serde` / `serde_json` dependencies are confined to the workspace-excluded `publish = false` local tool. No public schema, public renderer API, public crate, dependency change in published crates, release prep, version bump, tag, publish, generated artifacts, SVG/Vega-Lite/browser/notebook output, expression tracing, autograd, or broader linalg scope is authorized. |
| 1.95.0 | 2026-07-21 | Aligned roadmap truth after the RFC-070 fixed-demo private JSON report implementation review and commit `d0ef169`. The private `tools/matten-report --demo ... --format json --output <path>` slice is now recorded as implemented, reviewed, and committed for the five fixed demos only. Input-mode JSON, public schemas, public renderer APIs, public `matten-report` / `matten-viz` crates, dependency changes in published crates, release prep, version bump, tag, publish, generated artifacts, SVG/Vega-Lite/browser/notebook output, expression tracing, autograd, and broader linalg scope remain separate future review decisions. |
| 1.96.0 | 2026-07-21 | Prepared `0.37.0` as an RFC-071 fixed-demo private JSON report release. Retargeted the lock-step family version and current-family documentation from `0.36.0` to `0.37.0`, added release notes, and recorded the reviewed `tools/matten-report --demo ... --format json --output <path>` slice as the release scope. RFC-070 remains proposed/open for public visualization/report readiness; input-mode JSON, public schemas, public renderer APIs, public `matten-report` / `matten-viz` crates, dependency changes in published crates, extra feature work, tag, publish, generated artifacts, SVG/Vega-Lite/browser/notebook output, expression tracing, autograd, and broader linalg scope remain unauthorized. |
| 1.97.0 | 2026-07-21 | Addressed 0.37.0 release-prep review blockers: moved private fixed-demo JSON release authority into RFC-071 while keeping RFC-070 audit-only for public visualization/report readiness; retargeted stale rendered docs from the 0.36 current family to 0.37; hardened `scripts/check-release-docs.sh` so `current 0.N release family` wording cannot evade stale-family checks; and recorded the business decision that private-tool visualization milestones may be lock-step public family checkpoints when the changelog explicitly states no published-crate API/runtime/dependency change. |
| 1.98.0 | 2026-07-21 | Drafted the RFC-070 post-0.37 closure audit and aligned release truth after the owner-confirmed 0.37.0 release. The audit recommends closing RFC-070 without a public report/viz crate or API, then opening a new RFC for behavior-preserving `tools/matten-report` modularization before further feature work. After modularization, the next theme remains a separate choice among private input-mode JSON, broader mathematics, and a new ecosystem bridge. No implementation, public API, dependency, version, release-prep, tag, or publish action is authorized. |
| 1.99.0 | 2026-07-21 | Applied the accepted RFC-070 post-0.37 lifecycle closure. Moved RFC-070 from proposed to done with status explicitly identifying an audit decision rather than public implementation; aligned roadmap/RFC/handoff indexes and inbound links; and made a new behavior-preserving `matten-report` modularization RFC the next ordered design action. Private input-mode JSON, broader mathematics, and new ecosystem bridges remain later RFC choices. No implementation, public API, dependency, version, release-prep, tag, or publish action is authorized. |
| 2.0.0 | 2026-07-21 | Proposed RFC-072 for behavior-preserving `tools/matten-report` modularization. The design splits the 5,023-line binary into a thin entry point, internal request model, CLI policy, application orchestration, family report builders, format-owned renderers, output I/O, and ownership-aligned tests. It preserves the current 59-test baseline, exact Markdown/HTML/private-JSON artifacts, CLI errors, escaping, bounds, dependency set, workspace-excluded `publish = false` binary boundary, and no-public-API posture. No code movement, new feature, dependency, version, release-prep, tag, or publish action is authorized before RFC and handoff review. |
| 2.1.0 | 2026-07-21 | Revised proposed RFC-072 after design review. Added a mandatory pre-movement process-boundary harness covering help, invalid invocation, Markdown stdout, explicit-file artifact bytes, and filesystem failure across exit status/stdout/stderr/file routing; corrected the source inventory to 59 top-level functions; required builder and renderer phases to be subdivided by family/format; and applied the reviewed-above-500-ELOC rule equally to production and test Rust files. No source movement or feature, public API, dependency, version, release-prep, tag, or publish action is authorized. |
| 2.2.0 | 2026-07-21 | Recorded acceptance of the RFC-072 design and applied its two rereview notes: Structural Requirement 3.2.8 now applies the reviewed-above-500-ELOC rule symmetrically to production and test Rust files, and Slice 0 is explicitly a distinct process-baseline checkpoint that cannot be combined with source movement. The next action is a detailed implementation handoff; no harness, Rust source movement, feature, public API, dependency, version, release-prep, tag, or publish action is authorized yet. |
| 2.3.0 | 2026-07-21 | Drafted the detailed RFC-072 implementation handoff. It defines a separately reviewed/committed Slice 0 process harness with committed-monolith byte counts and SHA-256 anchors, a module-boundary guard, concrete request/CLI/app/report/render/output ownership, family-sized report-builder moves, format/family-sized renderer moves, symmetric production/test size gates, and seven owner-visible review points. No harness, Rust source movement, feature, public API, dependency, version, release-prep, tag, or publish action is authorized before handoff acceptance. |
| 2.4.0 | 2026-07-21 | Revised the RFC-072 handoff after review. Defined final flow as app-owned construction of family-specific report values followed by borrowed renderer consumption; unified private data-readiness success/error data so Markdown no longer queries/converts Table; kept dynamic-element and data-error normalization report-owned; moved Markdown list emission to Markdown ownership; required an explicit target directory for the process binary; and required Slice 1 delivery, mutation self-test, CI wiring, and release-checklist wiring for the module-boundary guard. No harness or Rust source movement is authorized before rereview acceptance. |
| 2.5.0 | 2026-07-21 | Recorded acceptance of the detailed RFC-072 handoff and authorized Slice 0 only. Incorporated the managed-environment note by requiring a validated repository-local `target/matten-report-process/tmp` linker temporary directory passed as `TMPDIR` to the explicit-target Cargo build. Slice 0 may add/wire the five-case process harness and prove deliberate digest mutation; Slice 1, Rust source movement, features, public APIs, dependencies, versions, release prep, tags, and publishing remain unauthorized. |
| 2.6.0 | 2026-07-21 | RFC-072 Slice 0 process baseline implemented and prepared for review. Added a dependency-free five-case shell harness that builds into explicit `target/matten-report-process`, uses validated repository-local linker and case temporary directories, executes the built binary directly, checks exact help/Markdown/private-JSON bytes and routing, checks the exact HTML policy error, and tolerates platform-specific filesystem details behind the stable process-error prefix. Wired the harness into CI and the release checklist and added a deliberate Markdown-digest mutation mode for negative proof. No Rust source moved; Slice 1, features, public APIs, dependencies, versions, release prep, tags, and publishing remain unauthorized until Slice 0 is reviewed and committed. |
| 2.7.0 | 2026-07-21 | RFC-072 Slice 1 implemented and prepared for review after the reviewed Slice 0 commit `83e242d`. Reduced `main.rs` to module declarations and process error handling; extracted private request/ReportKind, CLI policy, app dispatch, and output I/O owners; moved the unchanged combined report/render implementation and snapshots into transitional `render.rs` / `render/tests.rs`; and kept all 59 tests while splitting CLI tests below 300 lines and placing dispatch coverage with app. Added a dependency-free module-boundary guard covering forbidden module directions, external public items, library targets, and `publish = false`, with direct/grouped/qualified/public mutation self-tests and a `pub(crate)` control; wired both guard modes into CI and the release checklist. The transitional renderer files remain above 500 lines only to avoid combining Slice 1 with the later family/format extraction checkpoints; they must not grow and must be split by those authorized units. Report-family extraction, features, public APIs, dependencies, versions, release prep, tags, and publishing remain unauthorized until Slice 1 review and commit. |
| 2.8.0 | 2026-07-21 | Remediated the RFC-072 Slice 1 review NO-GO. Removed the leading newline from `render/tests.rs` and verified the exact CI `cargo fmt --check` commands for both local tools. Reworked grouped-import inspection to collect complete one-line or multiline use statements, then extended self-tests to prove forbidden-module-second, forbidden-module-first, single-item, and multiline groups all fail with direct diagnostics. The normal guard, 59 tests, and unchanged process fingerprints pass. Report-family extraction and later slices remain unauthorized until focused rereview and commit. |
| 2.9.0 | 2026-07-21 | Remediated the RFC-072 Slice 1 rereview blockers. Grouped-import inspection now collects through the statement semicolon and parses outer-group roots while tracking nested brace depth, catching same-line and multiline allowed-nested-group-then-forbidden-module forms without rejecting an allowed nested item named like a forbidden module. External visibility inspection now rejects bare `pub`, qualified function forms including `pub unsafe fn` / `pub extern "C" fn`, and non-crate restricted visibility while retaining the `pub(crate)` control. Expanded mutation self-tests cover every bypass and control. Later RFC-072 extraction remains unauthorized until second focused rereview and commit. |
| 3.0.0 | 2026-07-21 | RFC-072 Phase 2 shape-flow family extraction implemented and prepared for review after the accepted Slice 1 commit `fa21180`. Added private `report::shape_flow` ownership for broadcast/reshape/axis/matmul data and computation; app constructs exactly one shape-flow value before format dispatch; Markdown, HTML, and private JSON renderers borrow that value and perform no shape-flow Tensor computation. Moved the four shape-flow renderer tests into a 289-line family test module while retaining all 59 tests and exact snapshots. `render.rs` shrank from 2,272 to 2,202 lines and `render/tests.rs` from 1,814 to 1,538 lines. Dynamic-readiness and later family/format extraction remain unauthorized until review and commit. |
| 3.1.0 | 2026-07-22 | RFC-072 Phase 2 dynamic-readiness family extraction implemented and prepared for review after the accepted shape-flow commit `ba25227`. Added private `report::dynamic_readiness` ownership for dynamic Tensor construction, readiness conversion, schema summary data, and element normalization; app constructs exactly one dynamic-readiness value before format dispatch; Markdown, HTML, and private JSON renderers borrow that value and perform no dynamic-readiness Tensor computation. Moved the four dynamic-readiness renderer tests into focused JSON/Markdown and HTML family modules while retaining all 59 tests and exact snapshots. `render.rs` shrank from 2,202 to 2,085 lines and `render/tests.rs` from 1,538 to 1,237 lines; the new production owner is 133 lines and both new test modules stay below 200 lines. MLPrep standardization and later family/format extraction remain unauthorized until review and commit. |
| 3.2.0 | 2026-07-22 | RFC-072 Phase 2 MLPrep-standardization family extraction implemented and prepared for review after the accepted dynamic-readiness commit `67bd842`. Added private `report::mlprep_standardization` ownership for fixed Tensor construction, column standardization, and before/after statistics; app constructs exactly one MLPrep-standardization value before format dispatch; Markdown, HTML, and private JSON renderers borrow that value and perform no MLPrep-standardization computation. Moved the four MLPrep renderer tests into focused JSON/Markdown and HTML family modules while retaining all 59 tests and exact snapshots. `render.rs` shrank from 2,085 to 2,065 lines and `render/tests.rs` from 1,237 to 1,001 lines; the new production owner is 35 lines and both new test modules stay below 150 lines. Educational-path and later family/format extraction remain unauthorized until review and commit. |
| 3.3.0 | 2026-07-22 | RFC-072 Phase 2 educational-path family extraction implemented and prepared for review after the accepted MLPrep-standardization commit `2961c92`. Added private `report::educational_path` ownership for the composite broadcasting, reshape/transpose, axis-reduction, matmul, dynamic-readiness, and standardization data/computation; app constructs exactly one educational-path value before format dispatch; Markdown, HTML, and private JSON renderers borrow that value and perform no educational-path computation. Moved the five educational renderer/determinism tests into focused Markdown, JSON, and HTML family modules while retaining all 59 tests and exact snapshots. `render.rs` shrank from 2,065 to 1,921 lines and `render/tests.rs` from 1,001 to 529 lines; the new production owner is 154 lines and all new test modules stay below 300 lines. Data-readiness and later format extraction remain unauthorized until review and commit. |
| 3.4.0 | 2026-07-22 | RFC-072 final Phase 2 data-readiness family extraction implemented and prepared for review after the accepted educational-path commit `db298ab`. Added one private `report::data_readiness` model/builder for fixed-demo and input-mode source/selection/missing/conversion data, with report-owned error normalization and success/error conversion variants. App now uses one total `(input, kind)` dispatch, builds every family once before format selection, and contains no `unreachable!()` panic guards. Markdown, fixed HTML, input HTML, and private JSON borrow data; the obsolete no-model JSON dispatcher was removed. Moved the remaining data-readiness tests into focused Markdown/JSON, fixed HTML, and input HTML modules while retaining all 59 tests and exact snapshots. `app.rs` is 92 lines, `render.rs` shrank from 1,921 to 1,766 lines, and `render/tests.rs` from 529 to 7 lines; the new report owner is 109 lines and all new test modules stay below 300 lines. Phase 3 renderer extraction and later units remain unauthorized until review and commit. |
| 3.5.0 | 2026-07-22 | RFC-072 first Phase 3 renderer checkpoint implemented and prepared for review after the accepted Phase 2 commit `e4a88ae`. Moved `format_fixed_values` / `format_fixed_value` to private `render::common`; moved data-readiness Markdown and its private list writer to `render::markdown::data_readiness`; routed fixed and input Markdown through that owner; moved the three exact success/missing/nonnumeric Markdown tests beside the renderer; and moved selection-error coverage beside the report builder. All 59 tests and the 404-byte process fingerprint remain unchanged. After final formatting, `render.rs` shrank from 1,766 to 1,695 lines; the common helper is 13 lines, Markdown renderer 67 lines, Markdown tests 138 lines, and report tests 27 lines. Remaining Markdown families, HTML, JSON, and later structural work remain unauthorized until review and commit. |
| 3.6.0 | 2026-07-22 | RFC-072 remaining Phase 3B fixed-demo Markdown extraction implemented and prepared for review after the accepted shared/data-readiness commit `5e93b01`. Moved shape-flow, dynamic-readiness, MLPrep-standardization, and educational-path Markdown rendering into private family-owned `render::markdown` modules; routed app dispatch to those owners; and moved each unchanged exact Markdown test beside its renderer. All five Markdown families now have format/family owners, all 59 tests remain, and `render.rs` shrank from 1,695 to 1,333 lines. New production modules are 67-159 lines and their tests are 40-138 lines. HTML, JSON, and structural closure remain unauthorized until review and commit. No Cargo, dependency, public API, feature, version, release, or behavior change. |
| 3.7.0 | 2026-07-22 | RFC-072 Phase 3C HTML extraction implemented and prepared for review after the accepted Markdown commit `1dedfab`. Moved the exact static document shell, escaping, preformatted-block, and shape/value-table helpers to private `render::html::document`; moved fixed data-readiness, bounded input data-readiness, shape-flow, dynamic-readiness, MLPrep-standardization, and educational-path HTML rendering to family owners; routed app HTML dispatch to those owners; and moved exact/static/security/bounds tests beside each renderer. All 59 tests remain. `render.rs` shrank from 1,333 to 504 lines and now retains private JSON only; every new HTML production/test file is below 225 lines. Private JSON extraction and structural closure remain unauthorized until review and commit. No Cargo, dependency, public API, feature, version, release, or behavior change. |
| 3.8.0 | 2026-07-22 | RFC-072 Phase 3D private JSON extraction implemented and prepared for review after the accepted HTML commit `ca28353`. Moved the private schema-v0 envelope, all family-specific payload structs, tensor-preview policy, and non-finite-value guard to `render::json::model`; moved each fixed-demo mapping and exact snapshot to a family-owned JSON module; routed app JSON dispatch to those owners; removed the obsolete central renderer test index; and added focused rejection coverage for NaN and positive/negative infinity while confirming finite extremes pass. All 59 original tests remain and the total is now 60. `render.rs` is a four-line module index; the largest JSON production/test files are 266 and 261 lines. Input-mode JSON remains rejected. Structural closure remains unauthorized until review and commit. No Cargo, dependency, public API, feature, version, release, or established-output change. |
| 3.9.0 | 2026-07-22 | RFC-072 Phase 4 structural closure implemented and prepared for full implementation review after the accepted JSON commit `005e337`. Audited the completed source graph, every `pub(crate)` item, report-before-render dispatch, Cargo privacy/dependencies, test ownership, and obsolete structure; retained `render` as the intentional report-data-to-representation boundary; removed the empty legacy `render/tests` directory tree; and extended `module-boundaries.sh` with a fail-closed 500-line Rust source ceiling. Self-tests prove 501 lines fail with a direct diagnostic and exactly 500 pass. Every real Rust file is below 300 lines (largest 271), `main.rs` remains 13 lines, all 59 original tests plus the non-finite policy test remain, and process/output fingerprints are unchanged. RFC lifecycle closure remains unauthorized until review and commit. No Cargo, dependency, public API, feature, version, release, or behavior change. |
| 3.10.0 | 2026-07-23 | Closed RFC-072 after the Phase 4 full implementation review and commit `d3db053`. Moved the RFC from proposed to done, recorded the completed private ownership graph and mechanically enforced 500-line Rust-source ceiling, and aligned the roadmap and RFC/handoff indexes. No code, Cargo metadata, dependency, public API/schema, feature, version, release-prep, tag, publish, or generated artifact changed. Private input-mode JSON, broader mathematics, and ecosystem bridges remain separate RFC-first candidates. |
| 3.11.0 | 2026-07-23 | Proposed RFC-073 as the policy-first decision for bounded private `tools/matten-report` data-readiness input-mode JSON. The RFC defines the candidate command, report-outcome versus command-failure taxonomy, summary-only user-data allowlist, structured truncation metadata, conservative 12-column/120-character/240-error-character/12-value limits, private schema-v0 ownership, fixed-demo preservation, and the requirement for a reviewed detailed handoff before coding. No implementation, public schema/API/crate, dependency, version, release-prep, tag, publish, broader format, mathematics, bridge, or generated artifact is authorized. |
| 3.12.0 | 2026-07-23 | Drafted the accepted RFC-073 detailed implementation handoff as one coherent private data-readiness input-mode JSON checkpoint. The handoff fixes the exact schema-v0 envelope, bounded-string/list metadata and field order, 12/120/240/12 limits, success versus conversion-error representation, header-only/zero-row pre-write destination tests, unchanged non-atomic write-time contract, fixed-demo byte preservation, module/test ownership, CI/checklist smoke commands, and full gates. Implementation, dependency, public schema/API/crate, version, release-prep, tag, publish, mathematics, bridge, broader format, and generated artifact work remain unauthorized until handoff review. |
| 3.13.0 | 2026-07-23 | Recorded acceptance of the RFC-073 detailed implementation handoff and applied its no-rereview editorial correction: all four shared report display limits move to the boundary-legal `render::common` owner so HTML and JSON cannot drift, while rendering behavior and fixed-demo bytes remain unchanged. One coherent bounded private data-readiness input-mode JSON implementation checkpoint is authorized. Dependency, public schema/API/crate, version, release-prep, tag, publish, mathematics, bridge, broader format, and generated artifact work remain unauthorized. |
| 3.14.0 | 2026-07-23 | Implemented the accepted RFC-073 checkpoint after base `06faa29`: data-readiness CSV input now supports explicit-file private schema-v0 JSON with bounded source, selection, missing-count, conversion-error, and finite tensor-preview data; success and conversion-error artifacts are deterministic, while zero-row tensor construction and non-finite representation failures occur before output writes. Shared 12/120/240/12 display limits now have one `render::common` owner without changing HTML or fixed-demo JSON bytes. Added exact renderer snapshots, policy/CLI/process coverage, CI/checklist smokes, and current docs. The implementation is prepared for one review; no dependency, public schema/API/crate, version, release-prep, tag, publish, mathematics, bridge, broader format, or checked-in generated artifact is authorized. |
| 3.15.0 | 2026-07-28 | Prepared `0.38.0` as the RFC-073 private input-mode JSON release. Bumped the lock-step family version `0.37.0` -> `0.38.0`, retargeted current-family documentation and install snippets to `0.38.0`/`0.38.x`, added CHANGELOG release notes for the reviewed `tools/matten-report --input <csv> --kind data-readiness --format json --output <path>` slice, added the v0.38 compatibility-family paragraph, and moved RFC-073 to `done/` as a release-prep candidate. No public API, published dependency, MSRV, feature-flag, maturity-label, or runtime change. Publish and tag remain unauthorized until the release-prep review passes. |
| 3.16.0 | 2026-07-28 | Released `0.38.0` after the release-prep review (GO, conditional on appending this history row, which is applied here). Tagged `0.38.0` (bare SemVer, no `v` prefix) and published `matten`, `matten-ndarray`, `matten-mlprep`, and `matten-data` to crates.io in dependency order. Added a `check-release-docs.sh` guard asserting this header's Document Version/Date match the last document-history row, per the release review's H3 recommendation. RFC-073 and its implementation handoff moved from release-prep language to `Implemented (0.38.0)`. No public API, dependency, MSRV, feature-flag, or maturity-label change beyond the release itself. |
| 3.17.0 | 2026-07-28 | Post-`0.38.0` release-confirmation assessment found eight releases (`0.31.0` -> `0.38.0`) with zero published-crate change (`crates/*/src/` diff over that span is nine doc-comment version-string lines) and recommended a v1.0 readiness re-audit over picking the next theme by intuition. Opened RFC-074 (`v1.0 Readiness Re-Audit`) as audit-only, re-measuring RFC-066's original findings, public API stability, deferred stats/linalg/streaming scope, companion maturity promotion, and whether RFC-030 lock-step versioning still serves the family. Roadmap/RFC-index tracking only: no implementation, version bump, maturity promotion, or release action authorized. |
| 3.18.0 | 2026-07-28 | Completed the RFC-074 v1.0 readiness re-audit (`docs/design/v1-readiness-audit.md`, updated in place per the docs/design ownership rule). Every RFC-066 finding re-verified without regression (BF-1 remediated, MD-1 resolved by RFC-067, NF-1/NF-2 still open non-blocking). Found broader stats/linalg to be settled rejected-core-scope (RFC-040/041), not open blockers; streaming (RFC-037) remains additive, not a blocker; companion maturity promotion not evidenced by any source change since RFC-058/059. Recorded a new maintainer-decision finding, MD-2: RFC-071 §6's lock-step-versioning reconsideration trigger has not fired despite eight consecutive qualifying releases. Verdict: conditionally ready on technical grounds; not authorized for v1.0 release preparation until the maintainer resolves MD-2. No implementation, version bump, maturity promotion, or release action authorized. |
| 3.19.0 | 2026-07-28 | RFC-074's re-audit was reviewed and accepted (GO); applied two precision corrections (RFC-040's stats stance softened from rejection to directed deferral; NF-1 scope corrected to include matten-ndarray) and moved RFC-074 to `done/`. Closed NF-1/NF-2 (review H0): added `## Public API` blocks to `crates/matten-data/README.md` and `crates/matten-ndarray/README.md`, and documented `cargo public-api` as a manual release-checklist step. The owner chose Path B (pursue v1.0 deliberately); opened RFC-075 (`v1.0 Release Decision`, proposed) to resolve MD-2, declare the JSON canonical serde format stable, and record the RFC-067 family maturity table recommending `matten-mlprep`/`matten-data` enter a future v1.0 family at their current candidate label without promotion. No v1.0 release, version bump, tag, publish, API change, dependency change, or maturity promotion authorized; a separate future release-prep RFC is required before any release action. |
| 3.20.0 | 2026-07-28 | RFC-075 was reviewed and accepted (GO, `matten-rfc075-v1-release-decision-review-v0.1.md`). Applied the review's one required follow-through before closure: the §3.1 CHANGELOG-justification rule is now also recorded in `docs/src/contributing/release-checklist.md` §7 and `CHANGELOG.md`'s conventions blockquote, with a cross-reference added in RFC-075 §3.1 itself, so the rule is not stored only inside the RFC that predecessor RFC-071 §6's unfired trigger was stored in. Moved RFC-075 to `done/`. `rfcs/proposed/` is empty; RFC-074 and RFC-075 are both closed. A separate future v1.0 release-prep RFC (Unit 2, per the developer handoff at `matten-v1-path-developer-handoff-v0.1.md`) is required before any release action: full gate set including an actually-executed `cargo public-api` snapshot, the `pre-1.0` documentation sweep, version bump, and the RFC-067 maturity table reproduced in full. No v1.0 release, version bump, tag, publish, API change, dependency change, or maturity promotion authorized here. |
| 3.21.0 | 2026-07-28 | Opened and closed the RFC-076 v1.0 release-preparation review line: proposed the release-prep specification (RFC-067 family maturity table reproduced, compatibility.md rewrite, 19-site pre-1.0/0.x documentation sweep across 9 files including a migration.md/compatibility.md contradiction reconciliation, 29-string current-family version-string retarget across 14 files, version bump `0.38.0` -> `1.0.0`); NO-GO'd twice for missing scope and a missing site/stale count, both fixed; then two maintainer decisions applied and re-reviewed — the three `#[doc(hidden)]` slice-plumbing items are covered under the 1.0 promise (reversing the original exclude proposal, the conservative fallback both reviews recommended absent an independent read), and `cargo public-api` is recorded as not required for this release with dated rationale rather than silently dropped. Final review: GO, no conditions. RFC-076 and its implementation handoff are indexed in `rfcs/README.md` and `rfcs/handoffs/README.md`. Implementation of the release-prep change is now authorized; tag and crates.io publish (Unit 3) remain a separate, maintainer-authorized step after the release-prep commit is reviewed. |
| 3.22.0 | 2026-07-28 | Recorded that the RFC-076 implementation was attempted and then fully reverted at the owner's explicit instruction: the agent proceeded to execute the accepted release-prep change, including the `Cargo.toml`/`Cargo.lock` version bump to `1.0.0`, without first obtaining the owner's direct confirmation of that specific action, and the owner stated a standing rule that a version bump always requires explicit confirmation regardless of review acceptance. All uncommitted release-prep changes were discarded (`git checkout --`); the working tree returned to the last commit, and `0.38.0` remains the current version. RFC-076 stays accepted (GO, no conditions) but unexecuted; its status is corrected wherever `rfcs/README.md` previously read "implementation authorized." The owner directed that pre-v1 feature work (RFC-077, RFC-078) proceed first on the `0.38.x` line. No v1.0 implementation, version bump, tag, or publish is authorized by this entry. |
| 3.23.0 | 2026-07-28 | Closed RFC-077: `train_test_split_seeded` for `matten-mlprep` was implemented (commit `4c554a4`) and reviewed after the fact at the owner's request (`matten-rfc077-implementation-review-v0.1.md`, GO, no conditions). The review independently re-verified spec conformance line by line and proved the reproducibility contract by mutation testing -- flipping the Fisher-Yates direction and altering a SplitMix64 constant both correctly failed the locked-permutation test. Moved RFC-077 and its handoff to `done/`; `rfcs/README.md` and this file updated accordingly. The review recommended restoring review-before-commit discipline for RFC-078, which introduces a new published crate. No version bump, CHANGELOG entry, or release action occurred; `matten-mlprep` is not promoted. |
| 3.24.0 | 2026-07-28 | Closed RFC-078: `matten-stats` (fifth published crate, Experimental maturity -- `covariance`/`correlation` sample `ddof = 1`, `quantile` linear interpolation) was implemented (commit `7f1cbba`) without a prior implementation review, despite the RFC-078 review's own recommendation to restore review-before-commit for this slice. Reviewed after the fact at the owner's request (`matten-rfc078-implementation-review-v0.1.md`, GO, no conditions): the review independently re-verified all three algorithms against RFC-078 Sec4 line by line, proved the `ddof = 1` policy by mutation (flipping covariance's `n-1` divisor to `n` correctly failed two tests, including the one asserting `cov(x,x)` equals the sample variance), and confirmed all six guard scripts pass -- three edited, one (`check-streaming-scope.sh`) correctly left alone since it auto-covers the new crate via its `crates/*` glob. Moved RFC-078 and its handoff status to `done/`; `rfcs/README.md` and this file updated accordingly, including a corrected "Pre-v1 feature work" remaining-themes row noting RFC-040's stats theme is now only partially addressed (histogram, z-score, percentile aliases, skew, kurtosis, and matrix-wide/axis-wise variants remain deferred). Two standing items noted, neither a defect: the `ddof = 1` divergence has still never been reviewed by anyone who did not propose it, and review-before-commit did not happen for this slice even though specifically recommended. RFC-076's release-prep specification now assumes four crates and must be updated for five before it can be executed -- previously advisory, now a hard precondition. No version bump, CHANGELOG entry, or release action occurred; `matten-stats` is not published. |
| 3.25.0 | 2026-07-28 | Prepared `0.39.0` as the RFC-079 pre-v1 feature release. Reviewed and accepted (GO, no conditions after one required correction protecting `docs/design/v1-readiness-audit.md`'s dated findings and `scripts/check-release-docs.sh`'s incident-recording comment from an over-broad version-string retarget). The owner then made both decisions the handoff could not: confirmed the `0.38.0` -> `0.39.0` version bump specifically, and chose to obtain an external read of RFC-078's `ddof = 1` policy from outside this project's assistant session before `matten-stats` ever publishes, rather than accept the risk on the record or delay this release -- narrowing `0.39.0`'s scope to RFC-077 only. Bumped the lock-step family version across all five workspace crates (`matten-stats` included, unpublished), retargeted 33 version strings across 15 files, and caught one further gap the RFC's own list missed: `docs/src/introduction.md`'s bare "0.38 release family" sentence, which described RFC-073's release and needed rewriting for content, not just substitution, since RFC-079 is a different release. Added a `[0.39.0]` CHANGELOG entry naming only `train_test_split_seeded`, with zero mention of `matten-stats` per the owner's decision. Publish and tag remain a separate owner-authorized step; `matten-stats` is explicitly excluded from that step's publish list. |
| 3.26.0 | 2026-07-28 | Post-release alignment for `0.39.0` (documentation only; no code, version, manifest, tag, or publish change). `0.39.0` was tagged and published outside this project's assistant session. Step 0 of the alignment handoff required determining, without running `cargo publish`, whether `matten-stats` had been published alongside it despite `3.25.0`'s recorded deferral -- verified directly against crates.io (`cargo search matten-stats` plus the crates.io API): exactly one version exists, `0.39.0`, created the same day as the rest of the family. **Case B**: `matten-stats` published anyway, since its manifest never carried a `publish = false` key and nothing else mechanically enforced RFC-079 Sec3's decision. Per the handoff's explicit instruction, `3.25.0` is left unedited -- it accurately recorded the decision as it stood when made -- and this row records the outcome that diverged from it instead. Corrected: RFC-079's status (released, tagged, published; the deferral did not hold), RFC-078's status (published at `0.39.0`, not merely released-in-repo), and the `[0.39.0]` CHANGELOG entry (a dated correction note added, the original text left intact, stating `matten-stats` shipped at Experimental maturity with the `ddof = 1` divergence). The external `ddof` read is restated, not deleted: it no longer gates a first publication that has already happened, but still informs whether a future change to the policy is warranted. `rfcs/README.md` updated to match. |
| 3.27.0 | 2026-07-28 | Promoted `matten-mlprep` from production-ready candidate to production-ready (RFC-080): the promotion RFC-058 deferred in 2026-06-27 pending a resolved split story, taken now that RFC-077 satisfied §5.1's Option B exactly (a separate feature RFC adding a shuffled/seeded split, raising and resolving the anticipated API/RNG/dependency-policy questions). Self-review found one defect in the RFC-080 handoff's own sites list (seven files listed by raw occurrence count, four of which had zero `matten-mlprep` maturity mentions); the developer's independent fact-finding caught this and confirmed it, but the review found the corrected list was itself short by three sites (`scripts/check-release-docs.sh`'s two guard blocks, one already stale since RFC-058 independently of this RFC, and `rfcs/README.md`'s remaining-themes row) -- six verified sites across four files. Implementation then found a **seventh** real site neither list caught: `crates/matten-mlprep/src/lib.rs`'s crate-level `//! # Status` doc comment, one of the three canonical status files `check-release-docs.sh` itself already polices. Added a new "must not say production-ready candidate" guard mirroring `matten-ndarray`'s (RFC-057); the first draft of that guard was case-sensitive and silently failed to catch the real banner phrasing (`Production-ready candidate`, capital P) -- caught by deliberately reintroducing the violation and observing the guard pass when it should have failed, then fixed with a case-insensitive match and re-verified failing/passing correctly. `matten-data` (still candidate) and `matten-stats` (still Experimental) are untouched; RFC-076's RFC-067 family maturity table now also carries a stale `matten-mlprep` label, recorded but not fixed. No code, test, example, dependency, MSRV, version, or release change; no CHANGELOG entry (the label ships with whatever release comes next). |
| 3.28.0 | 2026-07-28 | Decided RFC-081: no crate labelled `Experimental` may ship in a lock-step `1.0.0` family; `matten-stats` (the only such crate, published `0.39.0`) must take Exit A (promotion, via its own RFC audited against the RFC-057 bar) or Exit B (removal from lock-step, via an RFC amending RFC-030) before a v1.0 release RFC may proceed. The two exits are stated as non-symmetric: Exit A is the ordinary path; Exit B, taken now, means either a divergent version line or withdrawing an already-published crate -- neither cheap, withdrawal not fully reversible. Rereview corrected the accompanying handoff's site list: a first draft named five `rfcs/proposed/076-v1-release-preparation.md` sites by manual read-through; review found it missed at least seven more, including one whole per-crate reasoning block (lines 193-198) that argued for admitting `matten-mlprep` *as a candidate* -- an argument RFC-080's actual promotion had already mooted. Replaced with an actual sweep-and-classify pass (four `grep` patterns, every hit classified edit/historical/generic) yielding **17 real sites**; a rereview found one further site orphaned by the other edits (line 204's "same reasoning as matten-mlprep" cross-reference, broken once the block it pointed at was rewritten) and confirmed the other 16. All 17 sites applied to RFC-076 (family size, `matten-mlprep`'s label, a new `matten-stats` row marked blocked, the per-crate reasoning block rewritten rather than word-swapped, and RFC-081's precondition sentence added), plus a version-base drift note (RFC-076 was written against `0.38.0`; two releases have since shipped) and a note that its version-string retarget figures are not re-measured here. `matten-stats`'s own Experimental label is unchanged everywhere -- verified by grep. RFC-081 §5's reasoning (why `Experimental` differs from `candidate` enough to justify the rule) was authored and reviewed by one party throughout; that is recorded as open, with three concrete paths offered (external read, narrow the rule to what the crate's own `Experimental` doc-comment already says, or accept as argued on the record) rather than resolved unilaterally. RFC-076 stays in `proposed/` -- corrected, not accepted, executed, or closed. No code, version, tag, publish, or maturity change; no cargo gates apply (nothing compiles differently). |
| 3.29.0 | 2026-07-28 | Implemented RFC-082: reopened streaming/large-CSV (deferred since RFC-026/RFC-037), adding `CsvBatchReader::{open, next_batch}` to `matten-data` behind an off-by-default `streaming` feature that implies `csv` (no new dependency). Answers all six of RFC-037 §4's reopening criteria: batch lifecycle (one open file handle, `Ok(None)` at and after EOF), schema drift (none -- a field-count mismatch is a malformed row, not schema evolution), malformed-row policy (fail-fast, same `MattenDataError::RaggedRow` variant and one-based line number as `Table::from_csv_path`), memory budget (row-count bound, not byte-denominated, stated honestly), sync-only (no async), and crate placement (`matten-data`, feature-gated; no `matten-stream` -- rejected on structure, not just cost, since a `matten-stream` crate would need `Table`, forcing a companion-to-companion dependency RFC-078 §6 already forbids). A first review round found the draft's central technical claim wrong: it asserted `scripts/check-streaming-scope.sh` needed narrowing because `BatchReader` is a substring of `CsvBatchReader`; tested three ways (isolated regex, example-name pattern, an end-to-end fixture run) the guard already permitted the exact proposed surface unmodified. The revision dropped that work entirely rather than doing it anyway, and confirmed the same result again against the real implementation post-commit. Construction reuses `Table::from_csv_path`'s exact header validation, ragged-row detection (`.flexible(true)`), and cell-parsing path (`parse_cell` made `pub(crate)`, `Table::from_parts` reused) -- no hand-rolled parsing. 13 new tests in `tests/streaming.rs`, including an equivalence test (concatenated batches == `Table::from_csv_path`, compared via `try_numeric().to_tensor()` since `Table` exposes no raw-cell accessor and adding one was out of scope), a batch-boundary line-number-parity test (a malformed row in a later batch compared directly against `from_csv_path`'s error on the same file, not a hard-coded number), and trailing-newline/blank-line equivalence tests. One new error variant, `MattenDataError::InvalidBatchSize`, for `batch_rows == 0`; I/O errors mid-stream now distinguished from parser errors via `csv::Error::into_kind()`, a distinction `from_csv_path` could only make at its separate upfront file-read stage. Feature-off build is unchanged (verified via three build configurations); the streaming and RFC-042 scope guards both pass with zero modification; the other four crates are untouched. No version bump, CHANGELOG entry, or release action. |
| 3.30.0 | 2026-07-29 | Documentation and RFC-portfolio work; no code, version, or release change (`0.39.0` throughout). **RFC-082 follow-up** (`80d31bf`): corrected the streaming docs and added four tests (13 → 17). Implementation review found no data-correctness defect — a 4,000-case randomized differential harness comparing `CsvBatchReader` against `Table::from_csv_path` at full `Debug` fidelity (text, bool, missing, quoted-comma, non-ASCII, blank lines, ragged rows, varied file endings) produced zero mismatches — but two malformed-input divergences were undocumented and are now recorded as accepted exceptions in RFC-082 §4.3: a *blank-but-not-empty* file (one containing any whitespace character other than a line terminator) gives `Csv` here versus `EmptyInput` from `from_csv_path`, and invalid UTF-8 gives `Csv` mid-stream versus `Io` upfront, with one or more valid batches possibly already returned. Feature-off absence of `CsvBatchReader` was proven externally (`error[E0433]` with the feature off, compiles with it on), closing a claim the original commit had argued from `#[cfg]` semantics rather than evidence. **RFC-081 closed** (`83594ee`, completed by `cf266ea`): §5 rewritten to rest the rule on a contradiction between two of the project's own documents — `crates/matten-stats/src/lib.rs:32-33` says the crate's *"surface may still change"* while `compatibility.md`'s breaking-change permission is scoped to the `v0.x` line by its own heading, so at `1.0.0` under lock-step versioning the crate would still make the claim while no longer being on the line that grants it — replacing reasoning about what `Experimental` ought to mean, and dropping a bullet that argued from authorship provenance rather than from evidence. Moved to `done/` with Status `Implemented`. **Owner decisions (2026-07-29):** v1.0 is *not* wanted yet; `matten-stats` takes **Exit A (promotion)**; RFC-081 §5 narrowed as above; the four AI-role-document edits approved (mid-model must verify Handoff claims against the code and escalate irreproducible ones; high-model must attach the command or `file:line` that established each factual claim, and must not run publish/tag/release or history-rewriting commands). **RFC-083 proposed and accepted:** adds `covariance_population`, `skewness`, `kurtosis` to `matten-stats`, 3 public functions → 6, additive only — no new error variant, no new dependency, no feature gate, no version bump, no release, and no maturity change (`matten-stats` stays `Experimental`; `check-release-docs.sh` lines 120/124 assert that label and must keep passing unmodified). No `correlation_population`, because `covariance.rs:76-84` records that the `n-1` factors cancel and correlation is ddof-invariant. The estimator convention is deliberately asymmetric — `covariance` stays bias-corrected while `skewness`/`kurtosis` are not — because RFC-078 §4.1's operative principle is *"match the ecosystem default for a function of this name"*, not *"always bias-correct"*. §4.1's SciPy/pandas default claims are explicitly marked **unverified** (those packages are not installed in the authoring environment) and confirming them is a pre-coding escalation gate. The formulas *were* verified by execution: `cov_pop * n == cov_sample * (n-1)` exactly, and `kurtosis([1,2,3,4,5]) == -1.3`, which pins Fisher's excess convention against Pearson's — a run that also caught and removed a bad acceptance criterion of the draft's own ("a normal-ish sample has excess kurtosis near 0" is false for small discrete samples). Deferred with reasons: histogram (RFC-040 §8 bin policy), matrix and axis-wise forms (would change the crate's `Tensor -> f64` shape), z-score (belongs to `matten-mlprep`; `standardize_columns` already does it), percentile aliases, mode. RFC-083 is sequenced **before** the `matten-stats` promotion RFC so the RFC-057 audit covers the final six-function surface, following RFC-080's precedent of promoting `matten-mlprep` only after RFC-077 landed the feature closing its exit criterion. |
| 3.31.0 | 2026-07-29 | Implemented RFC-083: `matten-stats` expanded from three public functions to six — `covariance_population` (population, `ddof = 0`), `skewness` and `kurtosis` (both uncorrected `g1`/`g2`, kurtosis reporting **excess**/Fisher so a normal distribution scores `0.0`). Additive only: no new error variant, no new dependency, no feature gate, no version bump, no release, and **no maturity change** — `matten-stats` remains `Experimental`, and `check-release-docs.sh`'s positive assertion of that label passed unmodified throughout. No `correlation_population`, because the `n-1` factors cancel and correlation is ddof-invariant; `covariance` remains the only genuine `ddof` policy decision. The estimator convention is deliberately asymmetric — `covariance` bias-corrected, `skewness`/`kurtosis` not — because RFC-078 §4.1's operative principle is *"match the ecosystem default for a function of this name"*, not *"always bias-correct"*. RFC-083 §4.1 had flagged its SciPy/pandas default claims as **unverified** (those packages were absent from the authoring environment) and made confirming them a pre-coding escalation gate; the implementer installed them and confirmed all three claims exactly before writing code, which is the gate working as intended. Implementation review approved with no corrections: every numeric result reproduced against the shipped code from an external crate (`skewness([1,2,3,10])` and `kurtosis([1,2,3,10])` matching SciPy's defaults, `skewness([1,2,3,4,5])` and `kurtosis([1,2,3,4,5])` bit-exact at `0.0` and `-1.3`, the `cov_pop*n == cov_sample*(n-1)` identity at 30.5, `ZeroVariance` rather than `NaN` on constant input, `correlation` unchanged), and the "unreachable zero-element branch" limitation was confirmed rather than accepted — `Tensor::zeros(&[0])` errors because zero-sized dimensions are unsupported in the current shape model. Two additions beyond the handoff's literal scope were flagged by the implementer and kept on review: the `Empty` variant's `Display` message (which had become user-visibly false, the handoff's "doc comment only" scoping having been too narrow) and one stale clause in `docs/src/reference/stats.md` that still described `matten-stats` as a hypothetical future companion. **Repository note:** a `filter-branch` pass rewrote all 209 commits to strip `Co-Authored-By` trailers. Tree content was unaffected, but every commit SHA changed, leaving 21 of the 36 SHAs cited in tracked documentation pointing off-branch. All 21 were repointed via the exact `refs/original` mapping, verified by re-running the dangling-reference sweep to zero and by confirming that all 12 touched files differ only in seven-hex tokens. Going forward, commits are cited by **subject line rather than SHA**, so the reference survives any future rewrite. |
| 3.32.0 | 2026-07-30 | Implemented RFC-084: `matten-stats` promoted from Experimental to production-ready candidate, discharging RFC-081 §3's Exit A. Not label-only, unlike RFC-080: the audit found one candidate-bar signal genuinely unmet — `matten-stats` was the only published crate with no CI job and no example smoke runs, and its `dynamic`-gated test ran only in the MSRV job. **PART 1** (partly landed early, in the commit that drafted this RFC, and authorized retroactively by the owner rather than reverted): added a dedicated `matten-stats` CI job (`test`, `--features dynamic`, `--doc`) and four example smoke runs (`stats_covariance`, `stats_correlation`, `stats_quantile`, `stats_expansion`) to `.github/workflows/test.yaml`, plus a `matten-stats` entry in `docs/src/contributing/release-checklist.md`'s `cargo public-api` list. Proven independently before any label moved: all four commands exit 0, with the expected 32/33 test-count split (33 only with `--features dynamic`, confirming the crate's one dynamic-gated test was previously exercised only in the MSRV job). **PART 2:** moved the label at every live site found by a sweep-and-classify pass (not by trusting the handoff's four-site list, which this project has been burned by trusting before) — `crates/matten-stats/{README.md,src/lib.rs}`, root `README.md`'s crate table, `docs/src/reference/{compatibility.md,stats.md}`, `rfcs/README.md`'s remaining-themes and RFC-076 entries, and ten sites across `rfcs/proposed/076-v1-release-preparation.md` (RFC-081 §3's precondition marked **discharged**, not deleted — RFC-076 itself stays deferred, unrelated reason: v1.0 is not currently wanted). `scripts/check-release-docs.sh`'s `matten-stats` block was **inverted, not deleted**, mirroring `matten-data`'s shape (negative + positive + shared-docs checks); a first pass produced three false positives, since the guard's own new negative check caught the word "Experimental" inside legitimately historical promotion narrative the implementer had just written — fixed by following the existing project convention (confirmed against `matten-mlprep`'s and `matten-data`'s own banners) of never restating a crate's own prior maturity label by name in live status text, pointing at the promoting RFC number instead. The inverted guard's ability to actually fail was proven by a deliberate-failure test (reverting one site to "Experimental", observing the guard fail and name that site, then restoring it). `rfcs/done/`, `rfcs/handoffs/`, `CHANGELOG.md`, and this file's own history rows below `3.32.0` are untouched. No code, API, test-logic, or behavior change in `matten-stats` itself; no version bump, release, tag, or publish; full production-ready is explicitly not claimed. |
| 3.33.0 | 2026-07-30 | Closed RFC-084: **`matten-stats` promoted `Experimental` → production-ready candidate**, discharging RFC-081 §3's **Exit A** and removing that precondition from RFC-076. Documentation, CI and guard changes only — no code, API, behaviour, dependency, feature, MSRV, version, or release change; `matten-stats`'s public surface is byte-identical to RFC-083's shipped state and `0.39.0` is unchanged. **Not the label-only promotion RFC-080 was.** The audit against RFC-057's candidate bar found one signal genuinely unmet: `matten-stats` was the only published crate with **no CI job and no example smoke runs**, and because the workspace test step passes `--all-targets` rather than `--all-features`, its single `dynamic`-gated test ran only in the MSRV job. PART 1 closed that gap — a dedicated job (`test`, `--features dynamic`, `--doc`), four example smoke runs, and a release-checklist entry — and was proven independently *before* any label moved; PART 2 then moved the label and **inverted rather than deleted** `check-release-docs.sh`'s assertion of `Experimental`, with a deliberate-failure proof that the new assertion can actually fail. RFC-084 §3 answers RFC-080 §7's *"not near promotion — three-week-old APIs and an unreviewed policy divergence"* objection head-on instead of editing it away: the `ddof` divergence is substantially resolved by RFC-083 (conventions verified against SciPy/pandas by execution, and `covariance_population` makes the other estimator reachable), while the **absence of usage history is conceded, not argued away** — what carries the promotion is what the *candidate* rung means per RFC-081 §5, a settled surface with a narrowed recommendation rather than field-tested maturity. The same reasoning explicitly does **not** support full production-ready, which is not claimed. Implementation review approved after one correction: the new guard's first check banned the word "experimental" anywhere in three entire files, rejecting both a legitimate promotion-history sentence and an unrelated sentence on a general statistics reference page — it had already forced three documentation sites to be rewritten less informatively. Narrowed to a present-tense label claim, with both directions proven. Two stale API-coverage rows in `rfcs/README.md` were also corrected: they still described `covariance`/`correlation`/`quantile` as unpublished or awaiting a future RFC, and listed `skewness`/`kurtosis` as deferred, all shipped by RFC-078 and RFC-083 — missed by the label-keyed sweep because a maturity change also falsifies claims about what a crate *does*, which share no keyword with the label. RFC-076's precondition is discharged but the RFC **remains deferred and unauthorized** for the separate, unrelated reason that **v1.0 is not currently wanted**. |
| 3.34.0 | 2026-07-30 | Recorded the open-theme inventory as a new **§3.1 Candidate themes — recorded, NOT authorized**, at the owner's request, so future theme selection reads from a written list rather than from recall. The section is explicitly an inventory and not a roadmap: nothing in it is approved, scheduled, or authorized, selecting a theme remains a joint planning decision (org policy §6.1), and the high-capability model may not adopt any entry unilaterally. Gathered by surveying the surviving deferrals across the RFC corpus — `rfcs/README.md`'s remaining-themes table, `compatibility.md`'s deferred-feature table, and the explicit non-goal or deferral sections of RFC-008, RFC-010, RFC-012, RFC-013, RFC-025, RFC-039, RFC-040, RFC-041, RFC-049, RFC-059, RFC-069 through RFC-071, RFC-073, RFC-076, RFC-081, RFC-082, RFC-083 and RFC-084 — grouped as advancement, core functions, `matten-stats` functions, linear algebra, streaming, bridges, reporting, and infrastructure. Each entry records its governing authority and current state, including why several are blocked rather than merely unscheduled: histogram on RFC-040 §8's unresolved bin-selection policy, `matten-stream` on the companion-to-companion dependency RFC-078 §6 forbids, a linalg companion on the prior unanswered question of whether it should exist at all, and `matten-stats`'s full-production promotion on the absence of usage history that RFC-084 §8 committed to in writing. Also records one small hygiene item found during the RFC-084 review but out of its scope: `check-release-docs.sh:89` still carries for `matten-data` the same over-broad blanket `experimental` grep that review finding C1 removed for `matten-stats`, harmless today only because `matten-data` was never at that label. No code, API, version, or release change. |
| 3.35.0 | 2026-07-30 | Implemented RFC-085: `matten-data` promoted from production-ready candidate to production-ready, closing RFC-059 §6's deferred full-production review. Unlike RFC-084, no CI gap needed closing first — `matten-data` already had a dedicated job and all nine examples in the smoke run — so this is a single part. Of RFC-059's four named concerns, two had moved: `matten-data` is no longer the newest companion (`matten-stats`, RFC-078, is two months newer), and large/streaming CSV — explicitly deferred at RFC-059 — was discharged by RFC-082's `CsvBatchReader`, whose implementation review ran a 4,000-case randomized differential harness over the exact edge-case surface RFC-059 was worried about with zero mismatches. The one substantive question RFC-085 §5 had to answer explicitly: whether `CsvBatchReader` (added by RFC-082 days earlier, behind the off-by-default `streaming` feature) blocks a stable-API claim. Answered by scope, not by argument: the default surface is unchanged since `0.22.0` (38 releases), `CsvBatchReader`'s two methods have deliberately fixed semantics (RFC-082 §4), and RFC-082 §5's nine deferred items are all additive — the one honest residual risk is a future async design wanting the signature to change, recorded rather than argued away, and covered by adding a "stable in what it does, scope may still grow" note to the crate's own docs, README, and `docs/src/examples/data.md`, not just to this RFC. `scripts/check-release-docs.sh`'s `matten-data` block was inverted (candidate -> production-ready) and its old blanket `experimental` grep across four whole paths was removed as redundant with the two already-anchored checks in the same block — the ROADMAP §3.1 hygiene item this closes. Building the candidate-label negative check caught its own substring trap live: a first version whole-file-banned the phrase `"production-ready candidate"`, which also matches `matten-data`'s own legitimate historical sentence ("...then to production-ready candidate in v0.27.0 (RFC-059), then to production-ready (RFC-085)") -- caught by running the guard against the crate's own just-written docs, fixed by anchoring the candidate check to the banner/Status-line start exactly like the pre-existing Beta/Experimental check, mirroring the fix RFC-084's review required for `matten-stats`. Label moved at every live site found by a sweep-and-classify pass, including six sites in `rfcs/proposed/076-v1-release-preparation.md` beyond its own four named sites and two in `rfcs/README.md` -- the same class of "resolved-exit placeholder" edit already applied there for `matten-stats`, not a new kind of decision; `docs/design/`'s two frozen snapshots (`external-design.md`, explicitly marked historical; `v1-readiness-audit.md`, a point-in-time 0.38.0 audit report) were left untouched, as were `rfcs/done/`, `rfcs/handoffs/`, `CHANGELOG.md`, and this file's history rows below `3.35.0`. `matten-data`'s own diff is doc-comments only in `src/lib.rs`; no code, test, or example changed. Four of five family crates are now production-ready; `matten-stats` remains candidate for its own separate reason (RFC-084 §8, no usage history) -- both consumed §3.1 rows removed. No version bump, release, tag, or publish. |
| 3.36.0 | 2026-07-30 | Closed RFC-085: **`matten-data` promoted production-ready candidate → production-ready**, closing the separate full-production review RFC-059 §6 deferred. Label, documentation and guard changes only — no code, API, behaviour, dependency, feature, MSRV, version, or release change; `0.39.0` is unchanged and RFC-042's scope lock is untouched with its guard passing unmodified. Two of RFC-059's four stated concerns had moved: large/streaming CSV, "explicitly deferred" at the time, was discharged by RFC-082, and the "wide CSV edge-case surface" gained direct evidence from RFC-082's 4,000-case randomized differential run (zero mismatches); the candidate cycle had run since 2026-06-27, and `matten-data` is no longer the newest companion. Ten of eleven bar signals cleared outright. The eleventh, **stable API**, was given an argument rather than a checkmark, since RFC-082 had added `CsvBatchReader` days earlier and RFC-082 §5 defers nine further streaming items: the default surface is unchanged across **38 releases** (`0.22.0` through `0.39.0`), everything RFC-082 added is feature-gated and off by default, and every deferred item is additive rather than a reshape — with the residual risk recorded and accepted, that a future async or resumability design wanting `CsvBatchReader`'s existing signature changed would now face a breaking change rather than a free one. Implementation took two review rounds, both on the guard rather than the promotion. Round 1: the `matten-data` block's blanket `grep -rIni "experimental"` was *removed* on a redundancy argument that two probes disproved — a present-tense stale claim in `docs/src/examples/data.md` not matching the shared-docs pattern, and anything at all under `crates/matten-data/examples/`, went uncaught — so the review required narrowing rather than removal, as originally specified. Round 2: the narrowed pattern then false-positived on ordinary prose (`is` followed within fifty non-period characters by an unrelated "experimental"), fixed by permitting only articles and adverbs between the verb and the label, verified 9/9 across both false positives and every true positive. The restored `examples/` coverage immediately found a genuine pre-existing defect unrelated to this RFC: `crates/matten-data/examples/csv_to_tensor.rs` still described the crate as **Beta**, stale since `v0.22.0` across three subsequent promotions, in an example CI executes. Fixing it crossed RFC-085 §6's explicit "no example change" line; the implementer flagged rather than assumed, and the review authorized it on the grounds that the conflict was created by the review's own coverage requirement, that a doc comment is not behaviour, and that both alternatives — shipping a guard that fails on a real bug, or narrowing coverage back — were worse. Also recorded: a **guard** caught what a **sweep** structurally could not, since the sweep keyed on the current label ("candidate") and the defect said "Beta" — the second demonstration in two RFCs that sweeps and guards fail differently. Four of the five published crates now sit at production-ready or above (`matten` stable, `matten-ndarray`, `matten-mlprep` and `matten-data` production-ready); `matten-stats` remains a production-ready candidate, which is the family's maturity ceiling until it accumulates usage history (RFC-084 §3). RFC-076 remains deferred and unauthorized: v1.0 is not currently wanted. |
| 3.37.0 | 2026-07-30 | Implemented RFC-086: prepared `0.40.0`, bumping the lock-step family version `0.39.0` -> `0.40.0` and publishing the accumulated content of RFC-082 through RFC-085 -- two features (`matten-data`'s `CsvBatchReader`, RFC-082; `matten-stats`'s `covariance_population`/`skewness`/`kurtosis`, RFC-083) and three maturity promotions (`matten-mlprep` production-ready, RFC-080; `matten-stats` Experimental -> production-ready candidate, RFC-084; `matten-data` production-ready candidate -> production-ready, RFC-085), none of which had been visible to a user before this release. Re-measured rather than trusted the RFC's own figures throughout: the 36-string/16-file version retarget matched exactly (measured at the same commit the RFC cited), but the rfcs/** historical-occurrence count came in at 127, not the RFC's stated 115 -- expected drift, since more RFC text referencing `0.39.0` had been written since the RFC's own measurement, and confirms the handoff's own instruction to re-measure rather than assume. CHANGELOG gained a `[0.40.0]` entry (Added/Changed/Maturity/Version sections) naming `kurtosis` as EXCESS, `CsvBatchReader` as NOT equivalent to `Table::from_csv_path` on malformed input, and all three maturity labels explicitly with `matten-stats` stated as CANDIDATE, not production-ready (RFC-067); the CHANGELOG preamble's four-crate family list was also corrected to five, adding `matten-stats` -- the one pre-existing CHANGELOG text this release was authorized to touch, since it is current description rather than a historical entry. `cargo metadata` confirmed all five crates at `0.40.0`; the only `.rs` change is `crates/matten/src/lib.rs`'s install-pin doc comment. **No tag created, nothing published** -- both are separate owner actions, and tagging is additionally blocked: RFC-086 §3 found the `0.38.0` and `0.39.0` git tags orphaned by the earlier `filter-branch` history rewrite (which rewrote every commit but not the tags), pointing at commits absent from `main` on both the local repo and `origin` -- a real defect in two already-published releases, not merely a preparation nicety, and the owner's repair to make before `0.40.0` is tagged. RFC-086 §10 proposes folding a release-readiness check into the existing RFC-disposition checkpoint so future accumulation is asked about rather than noticed by accident; recorded as a proposal, not adopted by this release. |
| 3.38.0 | 2026-07-30 | Closed RFC-086's **preparation** phase: the lock-step family is at `0.40.0` in the tree, publishing the accumulated user-facing work of RFC-082 through RFC-085 — `matten-data`'s `CsvBatchReader` (feature-gated, RFC-082), `matten-stats`'s `covariance_population`/`skewness`/`kurtosis` (RFC-083), and three maturity promotions that had never been visible to users (`matten-mlprep` production-ready, RFC-080; `matten-stats` production-ready candidate, RFC-084; `matten-data` production-ready, RFC-085). **Not yet tagged or published** — both remain owner actions, and tagging is blocked by RFC-086 §3. Implementation reviewed and approved with no corrections. RFC-086 §2 records the process failure that allowed the accumulation: each RFC correctly declared "no release" for its own slice and no step ever asked whether the *accumulation* warranted one — the mirror image of the post-`0.38.0` finding of eight consecutive releases (`0.31.0`→`0.38.0`) with zero published-crate change. Both failures share one cause: releases have no trigger. §10 proposes folding a three-question release-readiness check into the §6.4 RFC-disposition checkpoint the high-capability model already runs; that proposal is **not** adopted by RFC-086's acceptance and remains a separate owner decision. **Tag defect recorded, unrepaired:** the `filter-branch` trailer-strip rewrote all commits but not the tags, leaving `0.38.0` (→ `16356bd`) and `0.39.0` (→ `041c115`) — both *published* releases — pointing at commits no longer on `main`, with `origin` carrying the same two tags at the same orphaned commits. Tagging `0.40.0` before repairing them would bury the breakage behind a working newest tag. **Measurement correction, recorded rather than edited away:** the version-string retarget was **37 strings across 17 files**, not the 36/16 RFC-086 §6 measured. The pattern `0\.39\.[0x]` structurally could not match `docs/src/introduction.md`'s bare *"the current 0.39 release family"*; `check-release-docs.sh` caught it, because the guard matches a phrase shape rather than a version literal. That is the third instance in three RFCs of an enforced invariant catching what a one-time sweep missed — after RFC-084's sweep keyed on the old maturity label and RFC-085's keyed on "candidate" while the stale claim said "Beta". Future release-preps should measure with `0\.NN\b` and treat the guard, not the sweep, as the authority on completeness. Two beyond-bare-retarget documentation fixes were flagged by the implementer and confirmed on review: `docs/src/introduction.md`, whose content description still summarised RFC-079's single feature, and `docs/src/reference/public-api-snapshot.md`, which had carried a pre-existing contradiction since `0.38.0` (its opening said "current v0.38 release family" while a later sentence said "the 0.39.0 local-tool JSON release" — itself wrong, that release being RFC-073 at `0.38.0`); its rewritten claim that core `matten`'s public API is unchanged was verified independently, the page listing no companion items and core showing no public-item change since `0.38.0`. |
| 3.39.0 | 2026-07-30 | Post-release alignment for `0.40.0` (documentation only; no code, version, manifest, tag, or publish change). **`0.40.0` is released, tagged, and published to crates.io — all five crates live at `0.40.0`, matching the planned scope exactly.** Verified against the registry after publication rather than assumed: `matten`, `matten-ndarray`, `matten-mlprep`, `matten-data` and `matten-stats` all report `0.40.0` as their newest and max version. **Unlike `0.39.0`, no post-release correction was required** — that release needed one because `matten-stats` was published despite RFC-079 §3 deferring it, no `publish = false` key having enforced the deferral. This release shipped what the RFC said it would. The release makes public, for the first time, `matten-data`'s `CsvBatchReader` (RFC-082), `matten-stats`'s `covariance_population`/`skewness`/`kurtosis` (RFC-083), and three maturity promotions (RFC-080, RFC-084, RFC-085) — crates.io had been advertising `matten-stats` as `Experimental` for two days after three RFCs superseded that label. **RFC-086 §3's tag precondition is discharged.** The `filter-branch` trailer-strip had rewritten all commits but not the tags, leaving `0.38.0` and `0.39.0` — both published releases — pointing at commits no longer on `main`; investigation during the release found the breakage was worse than first recorded, since `origin/main` had never been pushed past `0.37.0`, so those two tags were off-branch on the remote independently of the rewrite. Repaired by re-tagging `0.38.0` onto `16356bd` and `0.39.0` onto `041c115`, pushing `main` (59 commits, fast-forward), and force-updating the tags. Verified afterwards on the remote: `origin/main` identical to local `HEAD`, **100 of 100 remote tags resolving to ancestors of `origin/main`** (previously 97 of 99), zero local/remote tag divergence, and the **100/100 GPG-signed invariant preserved** — the re-tagging reproduced signed annotated tags rather than downgrading two published releases to unsigned ones. `0.40.0` itself was tagged on the *Prepare* commit (`ba7a1f3`) per the convention `0.37.0`–`0.39.0` established, verified to produce crate artifacts identical to `HEAD`. Also recorded: the ROADMAP release-table row for `v0.40.0` carried RFC-086 §6's superseded 36-strings-across-16-files figure and is corrected here to the actual 37/17. |
| 3.40.0 | 2026-07-30 | **Recorded the project objective as the Phase 0 planning baseline (new §1.1)** — the project had run 86 RFCs and 100 releases without one written down, which is why theme selection had repeatedly fallen back on the high-capability model proposing and the owner ratifying, and why one theme was once correctly rejected as manufactured. Stated by the owner in the §3.3 replanning discussion: `matten` is a **"family car" around Tensor and matrix work**, intended for **education, learning, proof-of-concept and prototyping**; **adoption is explicitly not a success measure**; no target window yet. This confirms rather than redirects §1's existing *Sedan-first* framing — a family car is reliable, approachable and everyday, not a race car or a truck — and the established philosophy already matches it. Prioritisation criteria recorded: favour what a learner or prototyper meets early and often, treat examples/errors/docs as product rather than overhead, prefer an obvious documented default over a configurable policy, deprioritise performance, production scale and ecosystem breadth. The baseline was reached after the replanning discussion surfaced adoption data the project had never examined: 100 versions over 40 days with a **median 21 downloads per version**, which is the range crates.io mirrors, docs.rs builds and dependency bots produce unaided — no positive evidence of human users, against 86 RFCs and 164 review documents for 225 commits. Under the recorded objective that finding is **not a defect**: adoption is not being measured, and download counts should no longer be cited as evidence in any decision. **One known tension recorded rather than silently fixed:** RFC-084 §3 rested `matten-stats`'s candidate-only ceiling on absent *usage history*, a criterion this project does not measure and does not intend to satisfy, making that ceiling permanent by construction rather than by evidence; resolving it needs its own RFC and an owner decision. Also noted: the release-readiness check added at `3.39.0` was calibrated for a project that under-releases and will answer "yes" almost always at a release every ~9.6 hours, so it may need a threshold rather than a bare question. No code, version, or release change. |
| 3.41.0 | 2026-07-30 | Closed RFC-087: **`repeat`, `repeat_axis`, `tile` and `meshgrid` added to core `matten`** (eight functions with their `try_` forms), closing the three APIs RFC-039 §8 deferred in v0.21.0 with "needs decisions" lists rather than designs. **The first core public-surface change since `0.38.0`**, so `docs/src/reference/public-api-snapshot.md` moves with it. **Unreleased** — the family stays at `0.40.0`; see the release-readiness note below. This is the first theme selected against §1.1's planning baseline, chosen on its first criterion — what a learner meets early and often: `meshgrid` is where tutorials go on leaving 1-D, and `repeat` exists to contrast with broadcasting. RFC-039's open decisions were settled as: separate `repeat`/`repeat_axis` following the project's own `var`/`var_axis` precedent rather than NumPy's `axis: Option<usize>`, since an `Option` whose `None` case silently changes both the operation and the output rank is the implicit behaviour this project avoids; `tile` accepts `reps` shorter than rank (prepending `1`s, NumPy-compatible) but **rejects** longer, refusing NumPy's silent rank promotion; and `meshgrid` takes two rank-1 inputs, returns a tuple, and uses NumPy's `xy` indexing. The `xy` choice reversed the author's initial preference for `ij` on one consideration recorded in RFC-087 §5: with equal-length inputs the two conventions differ only by a transpose, so the mistake would be **silent**, whereas `tile`'s rank rejection surfaces as an explicit error. §6 generalises that into a stated boundary between two principles that genuinely conflict here — **match the ecosystem where a divergence would be silent; diverge only where it surfaces as an error that teaches** — reconciling RFC-078 §4.1's ecosystem-default rule with this project's standing preference for explicit over silent. Verified on review by calling the shipped functions from an external crate rather than reading the tests: `repeat` and `tile` not swapped, `meshgrid(len 3, len 2)` producing `[2, 3]` (an `ij` implementation would give `[3, 2]` and fail on shape alone), every error path returning `Shape`, and a 2,000,000-element `repeat` returning `Allocation` without attempting the allocation. **Review found three errors — all in the RFC and handoff, none in the implementation:** `MattenError::Axis` does not exist and every axis check in the crate reports through `Shape`; `crates/matten/README.md` has no public-API section to update; and core's `Cargo.toml` needs no `[[example]]` entry for an auto-discovered, feature-free example. All three were caught by the implementer reading the handoff *against the codebase* rather than executing it literally, and flagged rather than silently resolved — which is the verification duty added to the mid-capability role instructions on 2026-07-29 working as intended, on its first substantial outing. **Release-readiness check (first real exercise of the `3.39.0` addition):** unreleased published-crate change exists and is user-facing, but a release is **not** justified yet — `0.40.0` shipped hours earlier, and under §1.1 adoption is not a success measure, so there is no waiting audience. Recorded trigger: one further feature slice should prompt `0.41.0` rather than letting core API additions accumulate unreleased. |
| 3.42.0 | 2026-07-30 | Closed RFC-088: **negative indices in `slice_str`** — `"-1"` is the last element along an axis, `"0:-1"` everything but it — closing the deferral RFC-008 §4 made in `0.1.0` with no criterion to revisit it. Second theme against §1.1's planning baseline, and met earlier than anything else on the candidate list: `x[-1]` is among the first things a reader arriving from Python tries, and until now it was a parse error. **Unreleased**; the family stays at `0.40.0`. Scoped to `slice_str` alone: the builder's `IntoSliceRange` is a *sealed* trait implemented only over five standard `usize` range types, and adding `isize` impls would make every existing `range(1..3)` call ambiguous through integer-literal inference — a **source-breaking change to working downstream code**, which RFC-015's compatibility policy does not permit for a convenience feature. That asymmetry is recorded in RFC-088 §4 so a later reader does not mistake it for an oversight. Design is resolve-then-validate: `SliceSpec`'s index and range fields became signed, one `resolve_signed` helper computes `dim + i`, and RFC-008 §12.2's four bounds checks are the same code with resolved inputs. Out-of-range **errors rather than clamps**, diverging from Python — which clamps the range form — because matten already errors on positive out-of-range (`"0:100"` on size 3 is an error today) and one spec string must not be validated by two different rules depending on its sign; the divergence is *visible*, so it falls on the "diverge where it surfaces and teaches" side of RFC-087 §6's boundary rule, which is now doing work on its second consecutive RFC. Review verified the semantics by execution rather than by reading tests, including the axis-threading hazard the handoff singled out: on a `[3, 2]` tensor, `":,-3"` correctly errors naming *"axis 1 with size 2"*, where an implementation resolving every axis against axis 0's size would have accepted it and silently returned the wrong column. **One should-fix found and applied:** the implementer's exclusivity argument — that a negative index can only fail by resolving below zero, and a non-negative one only by exceeding the dimension — is true but did not enumerate a third failure shape, an *inverted* range whose bounds both resolve in range. That path fell through to a pre-existing message template printing only resolved values, so `"-1:-3"` and `"-1:0"` produced byte-identical text naming numbers the caller never wrote. Fixed per-bound, annotating only the bounds actually written negative, leaving the all-positive message byte-identical. Deliberately **not** widened into RFC-008 §13's general "errors must quote the original spec" gap, which the bounds messages have never met and which is unrelated to this change. RFC-088 §8 also records, without resolving, that `"0:0"` yields shape `[0]` while `Tensor::zeros(&[0])` rejects zero-sized dimensions — a pre-existing shape-model inconsistency that negative indices make easier to reach by accident, left for its own RFC. **Release-readiness check:** two unreleased core public-surface changes now stand behind `0.40.0` — RFC-087's eight functions and this — which is exactly the trigger recorded at `3.41.0`, so the answer flips to **yes, a release is justified**. |
| 3.43.0 | 2026-07-31 | Implemented RFC-089: prepared `0.41.0`, bumping the lock-step family version `0.40.0` -> `0.41.0` and releasing RFC-087 (`repeat`/`repeat_axis`/`tile`/`meshgrid`) and RFC-088 (negative `slice_str` indices) — the first two themes chosen against §1.1's planning baseline, and the first release triggered by the `3.39.0` release-readiness check answering "yes" after previously answering "not yet" at `3.41.0`. Re-measured rather than trusted the RFC's own figures: the retarget was **37 strings across 17 files**, matching RFC-089 §5 exactly when measured with the corrected `0\.40\b` pattern (Cargo.toml's single site resolved by the version bump itself, leaving 36/16 to edit). `docs/src/introduction.md` and `docs/src/reference/public-api-snapshot.md` both got content rewrites, not just number swaps, per RFC-089 §4 — the former now names RFC-087 and RFC-088's actual content instead of RFC-086's; the latter's framing sentence now names RFC-088 alongside RFC-087, and its rows were left untouched since RFC-088 changed no public item. CHANGELOG gained a `[0.41.0]` entry (Added/Changed/Version, no Maturity section since no label changed) naming the repeat-vs-tile contrast, meshgrid's `xy` convention, tile's rank rejection, and slice_str's builder-exclusion and no-clamp policy explicitly, per RFC-089 §6.2's over-claim list. `cargo metadata` confirmed all five crates at `0.41.0`; the only `.rs` change is `crates/matten/src/lib.rs`'s install-pin doc comment. Also corrected in this pass, found while rewriting the Status paragraph rather than named by RFC-089 itself: the header's lead sentence had claimed `0.40.0` "released, tagged, and published" while the same sentence's tail still called tagging "blocked" — a contradiction left behind when an earlier concurrent edit updated only the opening clause after the `0.40.0` tag-defect repair (recorded at `3.39.0`) without updating the rest of the sentence it was part of; the `v0.40.0` release-table row carried the identical stale "blocked" wording in its own last cell. Both are corrected here to state plainly that `0.40.0` is released, tagged, and published, with the defect repaired. **No tag created, nothing published** — both are separate owner actions, and there is no blocking precondition this time. |
| 3.44.0 | 2026-07-31 | Closed RFC-089's **preparation** phase: the lock-step family is at `0.41.0` in the tree, releasing RFC-087 (`repeat`, `repeat_axis`, `tile`, `meshgrid` and their `try_` forms) and RFC-088 (negative indices in `slice_str`) — the first two themes chosen against §1.1's planning baseline. **Not yet tagged or published**; both remain owner actions, and unlike `0.40.0` there is **no blocking precondition**, the orphaned-tag defect having been repaired during that release and re-verified here (100 of 100 tags resolving to ancestors of `origin/main`). The release exists because the release-readiness check added at `3.39.0` answered **not yet** at RFC-087's disposition — recording the trigger *one further feature slice should prompt `0.41.0`* rather than leaving it to memory — and then **yes** at RFC-088's. Both answers on consecutive uses, which is the evidence the check is a decision and not a formality, and the direct remedy for the accumulation RFC-086 §2 diagnosed. The version-string retarget was measured with `0\.40\b` per the correction recorded at `3.41.0`; that lesson paid immediately, the suffixed pattern finding only 35 of 37 and missing exactly the two bare-form sites — `docs/src/introduction.md` and `docs/src/reference/public-api-snapshot.md` — which also needed **content** updates rather than a number swap, the former still describing RFC-086's release. Implementation review required one correction, and it was **not in the implementation**: `ROADMAP.md`'s Status paragraph carried a clause attributing `matten_mlprep::train_test_split_seeded` (RFC-077) — `0.39.0`'s content — to `0.40.0`. The clause was correct at the previous commit and became false when a mechanical retarget moved its version number; the root cause was an earlier partial edit at `3.39.0` that replaced only the sentence's opening and left the remainder describing `0.39.0`, parking a stale version number inside a historical narrative. That occurrence sat in the Status *prose* rather than the history table, so it fell outside the protected set the RFC enumerated. Re-anchored to `0.40.0`'s actual content (RFC-082 through RFC-085), with `0.39.0`'s own content and its `matten-stats` publication divergence delegated to rows `3.25.0`/`3.26.0` rather than restated — shortening the paragraph to what is currently true rather than re-setting the trap. The same partial edit had also left the Status reading *"released, tagged, and published … but tagging is blocked"*, self-contradictory in one sentence; the implementer found and fixed that unprompted. Recorded as a small outstanding hygiene item, deliberately not actioned here: `docs/book/` is untracked **and** absent from `.gitignore`, so it shows as `??` and is a live hazard for any future `git add -A`. No code, API, or maturity change; `matten-stats` and every other label is unchanged from `0.40.0`, and the `[0.41.0]` CHANGELOG entry carries no `Maturity` section and nowhere claims NumPy compatibility, both `tile`'s rank rule and negative indices' non-clamping being deliberate divergences. |
| 3.45.0 | 2026-07-31 | Post-release alignment for `0.41.0` (documentation only; no code, version, manifest, tag, or publish change). **`0.41.0` is released, tagged, and published to crates.io — all five crates live at `0.41.0`, matching the planned scope exactly.** Verified against the registry after publication rather than assumed. **No post-release correction was required**, the second consecutive release for which that holds, against `0.39.0` which needed one because `matten-stats` was published despite RFC-079 §3 deferring it. The release publishes the **first two themes chosen against §1.1's planning baseline**: RFC-087's `repeat`/`repeat_axis`/`tile`/`meshgrid` and RFC-088's negative `slice_str` indices — both core public-surface additions, both selected on the baseline's first criterion of what a learner meets early and often, and neither reachable by anyone reading docs.rs until now. Pre-flight before the irreversible steps: 418 workspace tests, 116 doctests, fmt, clippy, six guard scripts, and a `cargo publish --dry-run` on core that packaged 128 files cleanly. The tag was placed on the **Prepare** commit (`84df18d`) per the convention `0.37.0`–`0.40.0` established, after confirming no file under `crates/` differs between it and `HEAD`, so the published artifacts are identical either way; it is GPG-signed, and 101 of 101 tags now resolve to ancestors of `origin/main` with the signed invariant intact. Publishing followed dependency order — `matten` first, then `matten-ndarray`, `matten-mlprep`, `matten-data`, `matten-stats` — and the release checklist's sequencing caveat did not materialise, each companion resolving immediately once core was live. **Release-cadence note:** this release exists because the readiness check added at `3.39.0` answered *not yet* at RFC-087's disposition while recording its own trigger, then *yes* at RFC-088's. Two consecutive releases have now been driven by that check rather than by someone remembering, which is the remedy RFC-086 §2 proposed for a project that had previously failed cadence in both directions. Also recorded: `docs/book/` was added to `.gitignore` (commit *"Ignore mdbook build output at docs/book"*), closing a hazard where mdbook's untracked, unignored output showed as `??` in every `git status` and sat one `git add -A` away from being committed; `docs/book.toml`, the tracked mdbook config beside it, was verified to remain tracked, the ignore pattern being directory-only. A consequence deliberately not actioned: the `&& rm -rf docs/book` step in twelve verification blocks across `rfcs/handoffs/` and RFC-076 is now redundant, but eleven of those are historical records and rewriting them for a cosmetic gain is not warranted. |
| 3.46.0 | 2026-07-31 | Closed RFC-090: **`histogram` added to `matten-stats`**, resolving the bin-selection policy RFC-040 §8 deferred in v0.21.2 — the oldest open question in the project, and the one every subsequent statistics RFC deferred to. The crate now exposes seven functions. **Unreleased**; the family stays at `0.41.0`. **The policy is that there is no automatic bin rule:** `bins` is a required argument, and no Sturges / Freedman–Diaconis / Scott / Doane / `"auto"` mode is implemented, because each is a statistical assumption wearing a default's clothing — Sturges assumes approximate normality, Freedman–Diaconis degenerates at zero IQR, Scott assumes normality outright — and a caller receiving fourteen bins from `histogram(&x)` has silently accepted one with nothing at the call site to show a choice was made. A required argument is the *absence* of a policy rather than one, the same shape RFC-082 chose for `batch_rows`. Matches NumPy on the closed last bin, since an open one would drop the maximum and leave `counts.sum()` silently below `x.len()`; diverges on constant input, erroring rather than inventing NumPy's data-independent `±0.5` range — both sides decided by RFC-087 §6's silent-versus-visible rule, now applied across three consecutive RFCs. **Amends RFC-078 §5's companion boundary** from `Tensor -> f64` to *"a summary is `f64` where scalar, a small owned struct where inherently vector-valued, and `matten-stats` never returns a `Tensor`"*, applied at all five sites across `lib.rs`, the crate README and `stats.md`. The last clause is load-bearing: the original rule existed to stop the crate drifting into a tensor-transform competitor to `matten-mlprep`, so the amendment deliberately does **not** unblock the matrix-wide and axis-wise forms RFC-083 §6 deferred, which do return a `Tensor`. **Review found one defect, of the class this project most consistently rejects:** with every input finite but the derived `hi - lo` overflowing to infinity, `histogram` returned `Ok` with `edges = [NaN, inf, inf, inf, …]`. The `NonFiniteValue` guard checks inputs, not the derived range, and — the reason it mattered — the sum invariant this RFC had specified as its load-bearing test still passed, so the required test suite was structurally blind to it. Fixed by rejecting a non-finite range, with `NonFiniteValue`'s message broadened to *"found in the input, or produced by a computation over it"* so it remains true in both cases. Three implementer judgment calls were confirmed correct on review, two of them exposing gaps in this project's own documents rather than the implementation: a second error variant `AllocationLimit` for the too-large-`bins` case, which RFC-090 §6 had named only negatively ("`Empty` is wrong here") without saying what to use; the genericisation of `ZeroVariance`'s `Display`, which said *"correlation is undefined…"* while already being reused verbatim by `skewness` and `kurtosis`; and the pinning of `edges[bins] = hi`, whose justification proved **understated** — testing showed the handoff's own formula `(range * bins) / bins` does not merely lose precision but can overflow to infinity before the division recovers, making the refinement a correctness fix rather than the tidy-up the handoff implied. |
| 3.47.0 | 2026-07-31 | Brought **§3.1 Candidate themes** back into line with the corpus. That section states that entries are removed as they are taken up, and three had not been: `repeat` / `tile` / `meshgrid` (taken up by RFC-087 and released in `0.41.0`), negative slice indices (RFC-088, same release), and histogram (RFC-090, unreleased). The list was recorded at `3.34.0` precisely so theme selection reads from a written list rather than from recall, which makes a stale entry worse than a missing one: it offers the owner work that is already done. The upkeep step belongs at each RFC's disposition and was missed twice before being caught here; it is now performed as part of closing an RFC that consumes a listed theme. Also updated the surviving `matten-stats` row for the matrix-wide and axis-wise covariance/correlation forms, which RFC-083 §6 deferred on the grounds that they would return a `Tensor` and so break the crate's then-stated `Tensor -> f64` shape. RFC-090 §5 replaced that phrasing, so the row's stated reason no longer quoted anything in the codebase; the entry is still blocked, and now more explicitly, since the amendment's "never a `Tensor`" clause was written to leave these forms outside it. The row now records that a future boundary RFC must argue against that clause rather than around the wording it replaced. Documentation only: no RFC state, code, API, version, or release change. |
| 3.48.0 | 2026-07-31 | Closed the `check-release-docs.sh` hygiene item recorded at `3.34.0` — and found it already discharged. That note said the script still carried for `matten-data` the over-broad blanket `experimental` grep that RFC-084's review C1 had removed for `matten-stats`. It does not: RFC-085 rewrote the whole block, and that work landed *after* the note was written, so the note was stale on arrival. This is the second stale-inventory finding in one day, and the sharper one, because unlike the §3.1 rows pruned at `3.47.0` it was acted on: it was the item recommended for this turn, and the recommendation was made from the note rather than from the file. Auditing the block instead turned up two live gaps of the same class, both proven reachable by deliberate injection before any fix was written. **First**, only one of the four companions had its `examples/*.rs` checked for stale maturity claims — `matten-data`, which got that coverage only because RFC-085's review C1 caught `csv_to_tensor.rs` still asserting *"matten-data is **Beta**"* long after the promotion. A line reading *"`matten-stats` is **Experimental**"* injected into the brand-new `examples/histogram.rs` passed the script cleanly. Enumerating sites one promotion at a time is precisely what rule 002 forbids, so all four companions are now covered by one loop and a future companion inherits the check by existing; the forbidden set stays per-crate, since `matten-stats` sits at production-ready candidate and a uniform ban would reject it for being correct. **Second**, `matten-stats` had no positive assertion that its `lib.rs` still declares a maturity at all — deleting the declaration outright passed, because the existing check only fires on a *wrong* label and an absent one is a different failure. `matten-data` has had that assertion since RFC-085; `matten-stats` now does too. Two defects in the pattern itself were fixed on the way. The shipped `matten-data` pattern was case-insensitive and so rejected *"is an experimental approach to schema inference"* — the exact sentence its own comment claimed to accept — now fixed by matching a label only as this project writes one, bolded or capitalised as a proper noun, with the multi-word "production-ready candidate" needing no such carve-out. And the first draft of the shared pattern appended `\b` after the closing `**`, which can never match since both sides are non-word characters: the bolded branch, the most common way a label is actually written, was dead while the bare branch kept working, so the check looked alive. It was caught only by probing each shape separately rather than trusting one passing probe. Final state: 17 probes, 10 rejections and 7 acceptances, all as intended; six guards pass. Guard-only change — no code, API, version, or release change. |
| 3.49.0 | 2026-07-31 | Consolidated every production-ready companion's maturity-label checking into one `check_production_ready_crate` function in `check-release-docs.sh`, replacing four hand-rolled variants that had drifted apart across two separate sections of the script — the crude original block near the top and the later per-crate RFC-057 / RFC-058+RFC-080 sections near the bottom, which is why the drift was easy to miss. **Three gaps, all proven reachable by deliberate injection before any fix was written.** `matten-ndarray`'s `lib.rs` check was **dead code**: it piped through `grep -v "//"`, and every doc-comment line in a Rust file begins `//!`, so no doc comment could ever trip it — a banner reading *"**Experimental (0.41.x family).**"* passed. Neither `matten-ndarray` nor `matten-mlprep` checked its README banner for `Experimental` at all. And neither had any **positive** assertion, so deleting the maturity declaration outright passed silently — the same absent-versus-wrong distinction found for `matten-stats` at `3.48.0`, which is now confirmed as a pattern rather than a one-off: three of the five crates had it. The function asserts four things per crate — banner and Status-line free of a superseded label, body prose free of a present-tense claim, and the declaration positively present in both README and `lib.rs` — so the assertions can no longer drift per crate and a future promotion inherits them by adding one call. The body-prose check exists because anchoring to the banner alone would have lost the whole-file coverage the deleted `\bBeta\b` blocks provided; reusing the present-tense claim pattern keeps that coverage while dropping the booby-trap those blanket bans carried, since they worked only while the file happened to contain no maturity history and would have fired on the first person to add some (RFC-084 review C1). The positive patterns accommodate both banner dialects in use: `matten-data` writes `**Production-ready** (RFC-085)` and the other two write `**Production-ready.**`. **Verified by 27 probes: 22 must-fail and 5 must-pass, all correct.** Fourteen of the 22 re-prove every case the deleted RFC-057, RFC-058/RFC-080 and RFC-085 blocks caught, so the consolidation is a strict superset rather than a trade; the 5 acceptances confirm promotion history, past-tense narration, adjectival prose and `matten-stats`' own correct candidate label all still pass. Net 69 lines removed, 53 added. Six guards pass. Guard-only change — no code, API, version, or release change. |
| 3.50.0 | 2026-07-31 | Audited every guard in the project for the defect class found at `3.48.0`–`3.49.0` — a check that cannot fail — rather than waiting to trip over the next one. Scope: all six `scripts/*.sh` guards and both `tools/matten-report/tests/` harnesses, with roughly 25 of `check-release-docs.sh`'s 39 assertions individually probed by injecting a real violation and confirming a non-zero exit. **Most of the estate is sound.** The three dependency guards fail correctly on a desynced pin, an unparseable pin, and a forbidden dependency, and their `(^|[[:space:]])<dep> v` matcher was confirmed not to collide on `matten-ndarray` when banning `ndarray`. `check-matten-data-scope.sh` (4/4) and `check-streaming-scope.sh` (3/3) reject every violation they claim to, and the latter's `awk` filter correctly still accepts a commented-out mention. `module-boundaries.sh` turns out to already implement the discipline proposed as a rule-002 amendment: built-in `expect_failure` self-tests that re-prove each rule on every run, against a throwaway fixture. It is the model the other guards should follow. **Two real deficits found and fixed.** First, the core stale-version check filtered with a bare `grep -v '#\['`, which drops any line mentioning `#[` *anywhere* rather than lines that *are* attributes — so a genuine stale string sharing a line with an attribute escaped, proven by injecting `//! #[derive] note: matten 0.41 is current.` and watching it pass. The exclusion is now anchored to the start of the matched source line, after the `path:lineno:` prefix. It is the same shape as the dead `matten-ndarray` check removed at `3.49.0`: an over-broad `grep -v` eating the very lines its check exists to catch, which makes this a genuine recurring pattern in this script rather than two coincidences. Second, three checks were wrapped in bare `if [ -d ... ]` / `if [ -f ... ]` tests, so a moved target turned into a *silently absent check*: renaming `docs/src/migration` made the entire overclaim block evaporate with the script still exiting 0, and removing `docs/src/benchmarks/results.md` did the same to the benchmark-ID freshness check. All three now report an error when their target is missing — a guard that disappears when its subject moves fails in exactly the way a guard that cannot fire does. One non-defect worth recording: the benchmark-ID check matches IDs as unanchored substrings, so an ID renamed by *suffixing* still counts as present. The first probe of it produced a false "DEAD" reading for that reason; re-probing by deleting the ID outright showed the check is live. Not fixed, because suffix-renaming a cited report ID is not a realistic drift mode, but noted so the next auditor does not re-derive it. The audit covered guards only; the specification corpus has not been swept for the equivalent problem — claims in RFCs and `docs/src/` that no longer describe the code — and that remains open. Guard-only change: no code, API, version, or release change. |
| 3.51.0 | 2026-07-31 | Audited every normative claim in `docs/src/` against the code, at the owner's direction, extending the `3.50.0` guard audit to the specification side. Method: all 89 Rust code fences across the 51 book pages were extracted and classified, then each class checked by the strongest mechanism available to it rather than by reading. **First finding is about the gate, not the content:** CI runs `mdbook build`, which renders the book but never compiles a line of it — `mdbook test` is run nowhere, so all 89 blocks were unverified. Compiling 67 of them against the real crates found **no defects**; 55 built clean and the other 12 are two deliberate book idioms (placeholder identifiers such as `start`/`end`/`t`, and semicolon-less expression listings of the form `v.sum()   // 10.0`), not errors. Twenty result-asserting claims were then *executed* rather than read, and 18 matched the code exactly — including every `Element` predicate and coercion on the dynamic page and every `get()` boundary case on the shape page — with two skipped as context-dependent. **The real defect was in the signature listings.** The reference pages document APIs as bare `Tensor::name(args) -> Ret` lines, which nothing had ever checked. Coercing all 26 to function pointers and compiling showed six were wrong: `repeat`, `try_repeat`, `repeat_axis`, `try_repeat_axis`, `tile` and `try_tile` were all documented as **associated functions** when every one takes `&self`, so a reader following the page wrote `Tensor::repeat(3)` and got a type error. The remaining twenty matched exactly, including the genuinely associated `concatenate`, `stack` and `meshgrid` in the same file — so this was drift in six specific entries, not a documentation convention. All six were RFC-087 functions, closed four RFCs ago and reviewed at the time without anyone checking the receiver. Fixed, and all 26 now coerce cleanly. Two other claim sets were verified clean: `contributing/architecture.md`'s root-export excerpt matches `lib.rs` exactly, omitting only the three `#[doc(hidden)]` slice-plumbing traits, which is correct; and `public-api-snapshot.md` mentions all 134 doc-visible public functions in core — the single apparent omission was an artifact of the audit's own regex, which could not see `range<R: IntoSliceRange>(r)` past the generic parameter. **A guard now enforces the receiver agreement** (rule 002: a guard, not a sweep), comparing every documented `Tensor::` signature against the first `pub fn` of that name in core and rejecting a mismatch in either direction, plus a documented method with no definition at all. Argument-type drift is explicitly out of scope: the full check needs a compiler, which is heavier than this script's contract. Proving the guard exposed a defect in its own first draft — under `set -euo pipefail`, an assignment from a command substitution takes the substitution's exit status, so the `grep` for a method that did not exist aborted the entire script at that line, silently skipping all 12 later sections and reporting only a bare non-zero exit. Caught by the third probe, fixed with `|| true`, and confirmed by counting executed sections (25, matching a clean run). Three probes now pass: missing `&self`, spurious `&self`, and an undefined method. Six guards, two harnesses, the full workspace test suite and all doctests pass. Documentation and guard change only — no code, API, version, or release change. |
