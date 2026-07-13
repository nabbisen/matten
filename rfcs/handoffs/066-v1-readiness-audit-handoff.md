# RFC-066 v1.0 Readiness Audit Handoff

**Project:** `matten`
**Related RFC:** RFC-066: v1.0 Readiness Audit and Release Decision Gate
**Document kind:** Compact implementation handoff
**Status:** Draft; request review before implementing the audit report
**Scope:** Audit report only; no v1.0 release authorization

---

## 1. Summary

Implement RFC-066 by producing a reviewed v1.0 readiness audit report:

```text
docs/design/v1-readiness-audit.md
```

The implementation is documentation/audit work only. It must not:

```text
version bump to 1.0.0
publish crates
create tags
change public APIs
change dependencies
promote companion maturity labels
authorize a v1.0 release
```

The report should determine whether the project is:

```text
not ready; list blockers
ready for maintainer decision; no release authorized yet
ready only after specific follow-up RFCs
```

---

## 2. Required Inputs

Read these before writing the report:

```text
rfcs/proposed/066-v1-readiness-audit-and-release-decision-gate.md
docs/src/reference/compatibility.md
docs/src/contributing/release-checklist.md
docs/src/reference/public-api-snapshot.md
docs/src/reference/error-model.md
docs/src/reference/boundary.md
docs/src/reference/dynamic.md
docs/src/philosophy.md
docs/src/migration/
docs/src/examples/
docs/src/tutorial/start-here.md
rfcs/done/009-serde-json-csv-and-boundary-integration.md
rfcs/done/015-public-api-stabilization-and-compatibility-policy.md
rfcs/done/022-companion-crate-boundary-policy.md
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
rfcs/done/063-visual-understanding-and-reporting.md
rfcs/done/065-educational-visualization-and-tensor-learning-path.md
README.md
crates/matten/src/lib.rs
crates/*/src/lib.rs
crates/*/README.md
```

The implementation may inspect source/tests/examples as needed, but the report
must cite tracked paths only.

---

## 3. Required Report Shape

Create:

```text
docs/design/v1-readiness-audit.md
```

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

The decision boundary must repeat that the report does not authorize:

```text
v1.0 release
version bump
tag
publish
API change
dependency change
companion promotion
new public tooling crate
```

---

## 4. Audit Work Items

### 4.1 Public API

Compare:

```text
docs/src/reference/public-api-snapshot.md
crates/matten/src/lib.rs
crates/*/src/lib.rs
crate README Public API / snapshot-equivalent sections
```

Record:

```text
[ ] user-facing exports match the snapshot
[ ] #[doc(hidden)] exports remain compiler-visibility plumbing
[ ] companion API blocks are current enough to serve as snapshot-equivalents
[ ] non_exhaustive error enums are documented
[ ] accidental public exports are absent or listed as findings
```

### 4.2 Panic / Result Boundary

Review:

```text
docs/src/reference/compatibility.md
docs/src/reference/error-model.md
docs/src/reference/boundary.md
docs/src/reference/public-api-snapshot.md
```

Record whether the panic/Result split is stable enough for v1 discussion, and
whether any mismatch is blocking.

### 4.3 Serde and Format Stability

Review:

```text
docs/src/reference/boundary.md
rfcs/done/009-serde-json-csv-and-boundary-integration.md
crates/matten/examples/10_json_roundtrip.rs
crates/matten/examples/11_csv_numeric_loading.rs
```

Record whether JSON object format is canonical enough for v1 discussion, and
whether CSV is correctly framed as ingestion rather than canonical tensor
serialization.

### 4.4 Deferred Items and Non-goals

Review:

```text
docs/src/reference/compatibility.md
docs/src/philosophy.md
docs/src/migration/
rfcs/README.md
ROADMAP.md
```

Record whether deferred items are explicit enough not to block v1 by ambiguity.
Do not convert deferred items into implementation work.

### 4.5 Companion Maturity and Lock-step Versioning

Review:

```text
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
README.md
crates/*/README.md
```

Record an explicit maintainer decision point:

```text
Can a lock-step v1.0 family include production-ready-candidate companions
with explicit labels, or must all family crates become production-ready first?
```

Do not answer this silently. If the audit cannot resolve it, list it as a
blocking or maintainer-decision finding.

### 4.6 Release Checklist v1.0 Gate

Review:

```text
docs/src/contributing/release-checklist.md
docs/src/tutorial/start-here.md
docs/src/examples/
docs/src/reference/error-model.md
docs/src/reference/dynamic.md
rfcs/done/022-companion-crate-boundary-policy.md
```

Cover every release-checklist v1.0 gate:

```text
stable core public API
clear dynamic on-ramp story
strong, scoped examples
reliable diagnostics
documented companion-crate boundary
clean feature matrix
```

---

## 5. Verification

Recommended local commands:

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
git diff --check
```

If a gate is not run, record why in the report or implementation review request.

Do not add `cargo public-api` as a project dependency or required gate in this
slice. The report may recommend it as a future decision.

---

## 6. Acceptance Criteria

The handoff is accepted when reviewers agree that:

```text
[ ] the report scope is audit-only and non-authorizing
[ ] every RFC-066 audit question is represented
[ ] both documented v1.0 gates are represented
[ ] companion maturity is an explicit decision point
[ ] deferred items are reviewed without forcing implementation
[ ] verification expectations are realistic and recordable
```

The implementation is accepted when:

```text
[ ] docs/design/v1-readiness-audit.md exists
[ ] blocking and non-blocking findings are separated
[ ] the report states whether maintainers may consider a separate v1.0 release RFC
[ ] no version bump, tag, publish, API change, dependency change, companion promotion, or release authorization occurs
```

---

## 7. Non-goals

```text
[ ] no v1.0 release preparation
[ ] no Cargo.toml version changes
[ ] no CHANGELOG release entry
[ ] no tag or publish command
[ ] no public API changes
[ ] no dependency changes
[ ] no companion maturity-label changes
[ ] no source/runtime behavior changes
[ ] no new public report/viz/migrate crate
```
