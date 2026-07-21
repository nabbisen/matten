# RFC-070 JSON Report Schema Policy Audit Handoff

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068, RFC-069, RFC-070
**Document kind:** Policy audit handoff
**Status:** Audit drafted for review; no implementation authorized
**Date:** 2026-07-17

---

## 1. Summary

Prepare the next RFC-070 planning slice: an audit of whether JSON report data
should become a reviewed policy boundary for `tools/matten-report`.

The RFC-070 readiness audit found that local Markdown/HTML artifacts are useful
but not ready for public `matten-report` / `matten-viz` crates, public renderer
APIs, or one public report schema. The private report-model extraction then
reduced local HTML-shell duplication without opening public scope.

This handoff asks for a policy audit only. It should answer whether JSON report
data is a useful prerequisite for future public-readiness work, and if so, what
compatibility, bounds, security, ownership, dependency, and review rules would
need to exist before any implementation.

---

## 2. Audit Questions

The audit should answer:

```text
Should JSON be considered at all, or should reports remain Markdown/HTML only?
If JSON is useful, is it local-tool-private, unstable experimental, or future public-contract material?
Should JSON mirror current report-family data structs, or define a separate reviewed schema?
Which fields are stable enough to name across report families?
Which fields are intentionally renderer-only and should not appear in JSON?
How should tensor previews, dynamic values, CSV schema summaries, and conversion errors be represented?
What display bounds are required before JSON can include user-controlled input?
What escaping/encoding/security rules apply to JSON generated from CSV paths, headers, and values?
What compatibility promise, if any, applies to JSON keys, ordering, numeric formatting, and null/error forms?
Should JSON require a new dependency, or stay blocked until std-only/manual output is justified?
Would JSON make public renderer APIs easier to review, or create premature API surface?
```

The audit should make a recommendation, but not implement it.

---

## 3. Evidence To Review

Primary code and policy evidence:

```text
tools/matten-report/src/main.rs
rfcs/proposed/070-public-visualization-report-readiness-audit.md
rfcs/handoffs/070-public-visualization-report-readiness-audit.md
rfcs/handoffs/070-private-report-model-extraction-handoff.md
rfcs/done/068-rich-local-visualization-artifacts.md
rfcs/done/069-input-mode-html-report-policy.md
ROADMAP.md
rfcs/README.md
```

The audit should specifically inventory:

```text
DataReadinessReportData
InputDataReadinessReportData
ShapeFlowReportData
DynamicReadinessReportData
MlprepStandardizationReportData
EducationalPathReportData
HTML escaping helpers
input-mode display bounds
exact Markdown/HTML snapshot tests
static/self-contained HTML safety tests
```

---

## 4. Required Policy Boundaries

The audit must keep these boundaries closed:

```text
no JSON implementation
no --format json
no public Report enum or public report schema
no public renderer API
no public matten-report crate
no public matten-viz crate
no workspace membership change
no new dependency
no public API snapshot
no generated JSON artifact checked in
no release prep
no version bump
no tag or publish action
no SVG / Vega-Lite / notebook / browser runtime scope
no JavaScript or external assets
no expression tracing
no autograd
no core Tensor visualization APIs
```

If the audit concludes JSON is worthwhile, it should still require a later
implementation handoff before any code changes.

---

## 5. Schema Policy Topics

The audit should cover these policy topics.

### 5.1 Ownership

Decide whether any future JSON schema would be:

```text
private local-tool output with no compatibility promise
reviewed local-tool output with snapshot compatibility
future public contract material
```

The default recommendation should remain private unless the audit proves a
stronger contract is maintainable.

### 5.2 Shape

Evaluate whether JSON should be:

```text
one cross-family report envelope
one schema per report family
one low-level section/block model
direct serialization of private report structs
```

Direct serialization of current private structs should be treated skeptically,
because it can freeze implementation details accidentally.

### 5.3 Compatibility

Define what compatibility would mean for:

```text
field names
field ordering
numeric formatting
tensor shape/value representation
dynamic Element representation
missing/null representation
conversion-error representation
bounded previews and truncation markers
report-family names
schema version markers
```

The audit may recommend no compatibility promise if JSON should remain private.

### 5.4 Security And Bounds

For user-controlled input mode, define policy for:

```text
file paths
CSV headers
CSV values
conversion errors
wide column lists
long fields
large tensor previews
non-finite numbers if they become possible
```

The policy should preserve the existing summary-only stance for input mode.

### 5.5 Dependencies

Decide whether a future implementation may use a JSON dependency, and under
which boundary:

```text
workspace-excluded local tool only
published crate dependency
optional feature
manual std-only JSON writer
```

No dependency is authorized by this handoff.

---

## 6. Suggested Audit Output

The audit should produce a new handoff or audit file, likely:

```text
rfcs/handoffs/070-json-report-schema-policy-audit.md
```

Expected sections:

```text
verdict
current report-family inventory
schema ownership recommendation
compatibility recommendation
security/bounds recommendation
dependency recommendation
implementation readiness verdict
next-step recommendation
explicit non-goals
review questions
```

---

## 7. Acceptance Criteria

```text
[ ] audit decides whether JSON is worthwhile now
[ ] audit distinguishes private local-tool JSON from public schema/API readiness
[ ] audit inventories all current report families
[ ] audit records what must not be serialized as stable public contract
[ ] audit defines compatibility and versioning posture, even if the posture is "none"
[ ] audit defines security and bounds policy for user-controlled input
[ ] audit records dependency policy and keeps new dependencies unauthorized
[ ] audit keeps public report/viz crates deferred
[ ] audit keeps core Tensor visualization APIs deferred
[ ] ROADMAP.md and RFC indices track the audit accurately
```

---

## 8. Verification

Minimum verification for this handoff draft:

```bash
bash scripts/check-release-docs.sh
git diff --check
```

If the later audit changes `docs/src/`, also run:

```bash
mdbook build docs
```

Remove generated `docs/book` afterward if created.

---

## 9. Follow-up Boundary

After this policy audit is reviewed, the next step should depend on the audit
verdict:

| Verdict | Follow-up |
|---|---|
| JSON not worthwhile | Close or pause JSON work; keep Markdown/HTML local |
| Private JSON useful | Draft a narrow local-tool implementation handoff |
| Public schema plausible but premature | Draft schema-readiness prerequisites |
| Public report/viz crates ready | Still require a separate crate-boundary RFC |

Do not implement JSON, public crates, public renderer APIs, or core
visualization APIs as part of this audit handoff.

---

## 10. Audit Record

The policy audit was drafted for review in
[`070-json-report-schema-policy-audit.md`](./070-json-report-schema-policy-audit.md).

The audit recommends private local-tool JSON as plausible future work, but only
behind a later fixed-demo implementation handoff. Public JSON schema, public
renderer APIs, public `matten-report` / `matten-viz` crates, input-mode JSON,
published-crate dependency changes, release prep, version bump, tag, publish,
and generated artifacts remain unauthorized.
