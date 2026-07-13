# RFC-066 — v1.0 Readiness Audit and Release Decision Gate

**Project:** `matten`
**Status:** Implemented (v0.31.0); audit-only, no v1.0 release authorization
**Document type:** RFC
**Primary audience:** maintainers, release reviewers, public-API reviewers
**Scope:** Audit and decision process only; no version bump or release authorization
**Related:** RFC-015, RFC-030, RFC-057, RFC-058, RFC-059, RFC-063, RFC-065

---

## 1. Summary

This RFC proposes a deliberate v1.0 readiness audit for the `matten` family.

It does **not** authorize a `1.0.0` release.

The audit should answer:

```text
Is the current matten family ready to be considered for v1.0?
If not, what exactly blocks it?
If yes, what explicit maintainer decision is still required?
```

The audit must produce evidence before any version decision:

```text
public API review
public API snapshot evidence
panic/Result boundary review
serde/canonical-format review
documented limitations and non-goals review
companion maturity impact review
release-gate evidence
```

Only after that audit is reviewed may maintainers decide whether to prepare a
separate v1.0 release RFC.

---

## 2. Motivation

The companion maturity line is now settled enough to justify a readiness review:

```text
matten-ndarray: production-ready
matten-mlprep: production-ready candidate
matten-data: production-ready candidate
```

The project also completed major pre-v1 groundwork:

```text
core fallible reduction surface
production migration guide
benchmark/positioning reports
visual understanding and educational docs
local advisory reporting/migration tools
documentation-governance cleanup
RFC-049 and RFC-054 lifecycle closures
```

However, project docs still correctly say:

```text
v1.0.0 requires explicit maintainer confirmation
```

This RFC creates the audit path between "the project feels mature" and "the
maintainer may choose a v1.0 release." It prevents accidental v1.0 drift.

---

## 3. Goals

1. Define a reviewable v1.0 readiness audit.
2. Keep v1.0 release authorization separate from the audit.
3. Identify blocking vs non-blocking findings.
4. Check whether lock-step family versioning makes companion candidate status a blocker.
5. Verify the public API snapshot against the crate surface.
6. Verify the panic/Result split is stable and documented.
7. Verify the serde/JSON/CSV compatibility story is stable enough for v1.0 discussion.
8. Verify limitations, non-goals, and deferred items are clear.
9. Verify release gates are adequate for a v1.0 decision.
10. Reconcile both documented v1.0 gates: compatibility policy and release checklist.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] version bump to 1.0.0
[ ] cargo publish
[ ] tag creation
[ ] public API changes
[ ] companion maturity promotion
[ ] shuffled/seeded train_test_split
[ ] Display implementation
[ ] mutation API
[ ] zero-sized tensors
[ ] streaming / large CSV
[ ] public matten-report, matten-viz, or matten-migrate crates
[ ] rewrite/apply migration automation
[ ] dependency changes
```

If the audit recommends any of these, that recommendation needs a separate RFC
or explicit release-policy decision.

---

## 5. Audit Questions

### 5.1 Public API

Review:

```text
docs/src/reference/public-api-snapshot.md
crates/matten/src/lib.rs
crates/*/src/lib.rs
crate READMEs' Public API sections
```

Questions:

```text
[ ] Does the snapshot list every user-facing core export?
[ ] Are #[doc(hidden)] exports still compiler-visibility plumbing only?
[ ] Are companion public API snapshots or snapshot-equivalent README/rustdoc blocks current?
[ ] Are non_exhaustive error enums documented where future variants are possible?
[ ] Are deprecated or accidental exports absent?
```

### 5.2 Panic / Result Boundary

Review:

```text
docs/src/reference/compatibility.md
docs/src/reference/error-model.md
docs/src/reference/boundary.md
docs/src/reference/public-api-snapshot.md
```

Questions:

```text
[ ] Is the panic zone vs Result zone final enough to freeze for v1 discussion?
[ ] Are boundary APIs non-panicking on ordinary external input?
[ ] Are panic APIs clearly local/trusted convenience APIs?
[ ] Do dynamic rejection paths match documented Unsupported behavior?
```

### 5.3 Serde and Format Stability

Review:

```text
docs/src/reference/boundary.md
rfcs/done/009-serde-json-csv-and-boundary-integration.md
crates/matten/examples/10_json_roundtrip.rs
crates/matten/examples/11_csv_numeric_loading.rs
```

Questions:

```text
[ ] Is the JSON object format stable enough to describe as canonical?
[ ] Are nested-array parsing and object-format parsing boundaries documented?
[ ] Is CSV documented as ingestion, not canonical tensor serialization?
[ ] Are feature flags (`serde`, `json`, `csv`) stable enough for v1 discussion?
```

### 5.4 Deferred Items and Non-goals

Review:

```text
docs/src/reference/compatibility.md
docs/src/philosophy.md
docs/src/migration/
rfcs/README.md
ROADMAP.md
```

Questions:

```text
[ ] Are deferred items explicit enough not to block v1 by ambiguity?
[ ] Are Display, mutation, zero-sized tensors, streaming, batched matmul, and serious linalg still deliberate non-goals/deferred items?
[ ] Are visual/reporting public crates explicitly future-owned?
[ ] Is migration automation explicitly future-owned?
```

### 5.5 Companion Maturity and Lock-step Versioning

Review:

```text
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
README.md
crates/*/README.md
```

Questions:

```text
[ ] Does lock-step family versioning mean all crates must reach production-ready before v1?
[ ] Or may v1.0 include production-ready-candidate companions with explicit labels?
[ ] Is `matten-mlprep`'s ordered-only split a blocker?
[ ] Is `matten-data`'s candidate status and large/streaming CSV deferral a blocker?
[ ] Is `matten-ndarray` production-ready status still current?
```

The audit must make this a maintainer decision point, not hide it.

### 5.6 Release Checklist v1.0 Gate

Review:

```text
docs/src/contributing/release-checklist.md
docs/src/tutorial/start-here.md
docs/src/examples/
docs/src/reference/error-model.md
docs/src/reference/dynamic.md
rfcs/done/022-companion-crate-boundary-policy.md
```

Questions:

```text
[ ] Is the core public API stable enough for v1 discussion?
[ ] Is the dynamic on-ramp story clear and documented?
[ ] Are examples strong, scoped, current, and not smuggling in deferred scope?
[ ] Are diagnostics reliable enough for the documented panic/Result boundary?
[ ] Is the companion-crate boundary documented and still consistent with RFC-022?
[ ] Is the feature matrix clean across default, no-default, and opt-in feature profiles?
```

This section reconciles the separate v1.0.0 gate in the release checklist with
the compatibility-policy v1.0 requirements. The audit must cover both.

---

## 6. Required Output

The implementation of this RFC should create a reviewed readiness report, not a
release branch.

Recommended path:

```text
docs/design/v1-readiness-audit.md
```

That report is outside the mdBook unless a later review decides part of it
belongs in user-facing docs.

Required sections:

```text
summary
decision boundary
public API review
panic/Result boundary review
serde/format review
companion maturity review
deferred-item review
release-gate review
blocking findings
non-blocking findings
recommendation
```

Possible recommendations:

```text
not ready; list blockers
ready for maintainer decision; no release authorized yet
ready only after specific follow-up RFCs
```

---

## 7. Recommended Verification

The audit implementation should run the strongest practical local gates:

```bash
cargo fmt --all --check
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-matten-data-scope.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-release-docs.sh
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
```

If a gate cannot be run locally, record why.

The audit may also recommend using `cargo public-api`, but adding that tool as a
project dependency or release gate requires a separate decision.

---

## 8. Acceptance Criteria

This RFC is accepted when reviewers agree that:

```text
[ ] the audit does not authorize v1.0
[ ] the audit questions cover the existing documented v1.0 requirements in compatibility policy and release checklist
[ ] lock-step family / companion maturity is treated as an explicit decision point
[ ] deferred items are evaluated without forcing implementation
[ ] the required report shape is concrete enough for implementation review
```

This RFC is implemented when:

```text
[ ] docs/design/v1-readiness-audit.md or an accepted equivalent exists
[ ] the report is reviewed
[ ] blocking and non-blocking findings are clearly separated
[ ] the report states whether maintainers may consider a separate v1.0 release RFC
[ ] no version bump, tag, publish, API change, dependency change, or release authorization occurs
```

---

## 9. Sequencing

Suggested sequence:

```text
1. Review and accept this RFC.
2. Create a compact implementation handoff for the audit report.
3. Implement the audit report.
4. Review the audit report.
5. Only then decide whether to draft a separate v1.0 release RFC.
```

The audit may conclude that v1.0 should wait. That is a valid outcome.
