# RFC-070 JSON Report Schema Policy Audit

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068, RFC-069, RFC-070
**Document kind:** Policy audit and next-step recommendation
**Status:** Drafted for review; no implementation authorized
**Date:** 2026-07-21

---

## 1. Verdict

JSON report data is worthwhile to explore, but not as a public contract yet.

Recommended posture:

```text
private local-tool JSON output may be useful
public JSON schema is not ready
public report/viz crates remain deferred
core matten remains visualization-free
```

If work continues, the next slice should be a narrow implementation handoff for
private `tools/matten-report` JSON output. That handoff must define an explicit
private schema shape before code changes, and it must not serialize current
private report structs directly.

This audit does not authorize JSON implementation, `--format json`, public
schemas, public renderer APIs, public crates, dependency changes, release prep,
version bump, tag, publish, or generated artifacts.

---

## 2. Current Inventory

Current local report families in `tools/matten-report/src/main.rs`:

| Report family | Current data shape | Notes |
|---|---|---|
| `data-readiness` demo | `DataReadinessReportData` | Fixed demo CSV, selected columns, missing counts, success tensor preview |
| `data-readiness` input | `InputDataReadinessReportData` | User-controlled CSV path/headers plus bounded success or conversion-error summary |
| `shape-flow` | `ShapeFlowReportData` | Broadcast, reshape, axis reductions, matmul checkpoints |
| `dynamic-readiness` | `DynamicReadinessReportData` | Dynamic element display, schema summary, masks, strict/explicit conversion |
| `mlprep-standardization` | `MlprepStandardizationReportData` | Before/after shapes, values, means, population std values |
| `educational-path` | `EducationalPathReportData` | Teaching narrative plus multiple shape/data checkpoints |

Current format state:

```text
OutputFormat supports Markdown and Html only.
There is no --format json.
tools/matten-report/Cargo.toml has no JSON dependency.
All report data structs are private.
Markdown and HTML outputs are covered by exact snapshot tests.
HTML safety tests cover static/self-contained output.
Input-mode HTML uses display caps for columns, long text, errors, and tensor previews.
```

Current display bounds:

```text
MAX_DISPLAY_COLUMNS = 12
MAX_DISPLAY_CHARS = 120
MAX_ERROR_CHARS = 240
MAX_TENSOR_PREVIEW_VALUES = 12
```

---

## 3. Why JSON Is Useful

JSON could help the project separate report data from rendered Markdown/HTML
without opening a public crate immediately.

Potential value:

```text
forces explicit report-data naming
makes renderer inputs easier to reason about
supports future local artifact checks beyond string snapshots
can serve as an intermediate audit target before any public renderer API
can help reviewers identify which values are data and which are presentation
```

This value is real only if JSON is treated as a reviewed boundary. Dumping
private structs directly would create the appearance of a schema while freezing
implementation details accidentally.

---

## 4. Ownership Recommendation

Recommended ownership for the next slice:

```text
private local-tool output
no public API contract
no SemVer compatibility promise
snapshot-tested deterministic text output
schema_version present but marked private/unstable
```

A future public schema would require a separate RFC after the private JSON shape
has proven useful and maintainable.

Suggested envelope for future review:

```text
schema_version: 0
schema_status: "private-local"
tool: "matten-report"
report_kind: one of the existing KIND_* names
input_mode: "demo" or "csv"
limits: present when user-controlled content is bounded
data: family-specific object
```

Do not include timestamps, host paths beyond the existing explicit input label,
environment data, random IDs, or build metadata. Deterministic output matters
more than provenance in this local-tool scope.

---

## 5. Shape Recommendation

The next implementation should use a common private envelope plus
family-specific payloads.

Recommended:

```text
one private envelope shared by all JSON reports
one explicit payload schema per report family
small shared value shapes for tensor previews and bounded lists
```

Not recommended:

```text
direct serialization of current private report structs
one public Report enum
one public section/block renderer schema
JSON that mirrors current HTML section order as a contract
```

Reasoning:

```text
The current structs are computation conveniences.
The current HTML sections are presentation choices.
The stable concepts are report kind, input mode, tensor shape/value previews,
summary counts, conversion result, and bounded-display policy.
```

---

## 6. Compatibility Recommendation

For private local-tool JSON:

```text
No public compatibility promise.
Changes require review because local snapshots and downstream scripts may exist.
Generated text should be deterministic.
Exact JSON snapshot tests should cover each supported report path.
```

If public compatibility is ever proposed later, it must decide:

```text
field-name stability
object field ordering expectations for generated text
schema versioning and migration policy
numeric formatting and finite-number policy
tensor shape/value representation
dynamic Element representation
null/missing-value representation
conversion-error representation
bounded-preview metadata
report-kind naming stability
```

For now, use `schema_version: 0` to communicate private/unstable status.

---

## 7. Security And Bounds Recommendation

JSON should preserve the existing summary-only stance for user-controlled input.

Future JSON must not include:

```text
full raw CSV data
unbounded headers or values
unbounded file paths
unbounded conversion errors
unbounded tensor values
environment metadata
absolute paths beyond the user-visible input label already accepted by the tool
```

For user-controlled input, future JSON should include bounded strings and
machine-readable truncation metadata rather than only human prose markers:

```text
value
truncated: true/false
omitted_count where applicable
limit used for truncation
```

JSON string escaping should be delegated to a JSON encoder in any future
implementation. Manual string concatenation should be rejected unless the
implementation proves it handles all JSON escaping cases with focused tests.

Non-finite numeric values need an explicit policy before implementation. The
recommended private policy is:

```text
finite f64 values -> JSON numbers
NaN / Infinity / -Infinity -> explicit tagged string or structured non_finite value
never silently coerce non-finite values to null
```

The current report data is expected to be finite, but the policy should be in
place before JSON becomes an output format.

---

## 8. Dependency Recommendation

No dependency is authorized by this audit.

If JSON implementation is later accepted, a JSON encoder dependency is
acceptable only inside the workspace-excluded `tools/matten-report` local tool.
It must not become a published `matten` or companion dependency.

Recommended future dependency posture:

| Boundary | Recommendation |
|---|---|
| `tools/matten-report` local tool | May consider direct `serde` / `serde_json` use in a reviewed handoff |
| published core `matten` | No new JSON/report dependency |
| published companion crates | No new JSON/report dependency for this scope |
| public `matten-report` / `matten-viz` | Deferred; requires separate crate-boundary RFC |

Manual std-only JSON output is not preferred because it increases escaping risk.
It should be considered only if review rejects a local-tool JSON dependency.

---

## 9. Implementation Readiness

Private JSON implementation is plausible, but not ready to start without a
narrow implementation handoff.

Ready:

```text
private report data builders exist
format policy already centralizes output-format decisions
exact-output test culture is already in place
HTML/input-mode safety tests already define useful bounds
```

Not ready:

```text
no accepted JSON schema shape
no accepted non-finite numeric policy
no accepted JSON dependency policy
no accepted compatibility/versioning wording
no accepted input-mode JSON bounds representation
```

Recommended first implementation slice:

```text
private local-tool JSON for fixed --demo reports only
explicit --format json requiring --output
schema_version: 0 and schema_status: "private-local"
exact JSON snapshots for every supported fixed demo
no input-mode JSON until bounds metadata is reviewed
no public crate/API/dependency in published crates
```

Input-mode JSON can follow only after the fixed-demo schema proves maintainable.

---

## 10. Non-goals

This audit does not authorize:

```text
JSON implementation
--format json
input-mode JSON
public Report enum or public schema
public renderer API
public matten-report crate
public matten-viz crate
workspace membership change
published-crate dependency change
generated JSON artifacts checked into the repository
release prep
version bump
tag or publish action
SVG / Vega-Lite / notebook / browser runtime scope
JavaScript or external assets
expression tracing
autograd
core Tensor visualization APIs
determinant / inverse / broader linalg scope
```

---

## 11. Next Step

If review accepts this audit, draft a private fixed-demo JSON implementation
handoff. That handoff should decide the exact schema before code work starts.

Suggested future handoff:

```text
rfcs/handoffs/070-fixed-demo-json-report-implementation-handoff.md
```

The handoff should keep input-mode JSON, public schemas, public crates, SVG,
Vega-Lite, browser/notebook output, dependency changes in published crates, and
release prep out of scope.

---

## 12. Review Questions

Review should decide:

```text
[ ] Is private local-tool JSON worthwhile now?
[ ] Is public JSON schema correctly rejected for now?
[ ] Is the private envelope plus family-specific payload recommendation right?
[ ] Should fixed-demo JSON come before input-mode JSON?
[ ] Is a local-tool serde/serde_json dependency acceptable in a future implementation handoff?
[ ] Is the non-finite numeric policy adequate?
[ ] Are public report/viz crates and core visualization APIs still correctly deferred?
[ ] Are ROADMAP.md, rfcs/README.md, and the handoff index sufficient tracking surfaces?
```
