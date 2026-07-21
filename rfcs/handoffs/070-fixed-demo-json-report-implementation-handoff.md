# RFC-071 Fixed-Demo JSON Report Implementation Handoff

**Project:** `matten`
**Related RFCs:** RFC-063, RFC-065, RFC-068, RFC-069, RFC-070, RFC-071
**Document kind:** Local-tool implementation handoff
**Status:** Implemented and reviewed under RFC-071; committed in `d0ef169`; release-prep candidate only
**Date:** 2026-07-21

---

## 1. Summary

Prepare the first private JSON output slice for `tools/matten-report`, limited
to fixed `--demo` reports.

The RFC-070 JSON report-schema policy audit accepted JSON as useful only when it
is treated as private local-tool output first. RFC-071 records that private
prerequisite as the normative authority. This handoff therefore proposes a
narrow implementation boundary:

```text
add --format json only for fixed --demo reports
require --output for JSON, matching the HTML file-output policy
emit deterministic private schema_version 0 JSON
cover every supported fixed demo with exact JSON snapshots
keep input-mode JSON and public schema/API/crate work out of scope
```

This is not a public report contract. It is a local artifact format for review,
experimentation, and future policy evidence.

---

## 2. Current State

`tools/matten-report` currently supports:

```text
OutputFormat::Markdown
OutputFormat::Html
```

The supported fixed demo report kinds are:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
```

Markdown/plain text remains the default. HTML requires explicit `--output` and
is already supported for all fixed demos plus the reviewed data-readiness input
mode. There is currently no `--format json`, no JSON dependency, and no public
report model.

---

## 3. Implementation Scope

Allowed:

```text
add OutputFormat::Json privately inside tools/matten-report
accept --format json in parse_format
require --output for --format json
support JSON only for --demo data-readiness
support JSON only for --demo shape-flow
support JSON only for --demo dynamic-readiness
support JSON only for --demo mlprep-standardization
support JSON only for --demo educational-path
add private JSON envelope and payload helper structs inside tools/matten-report
add a local-tool JSON encoder dependency if review accepts it
add exact JSON snapshot tests for every fixed demo
add CLI rejection tests for unsupported JSON paths
update tools/matten-report README/help text and release-checklist smoke commands
update tracking docs to record implementation once reviewed
```

The implementation may use `serde` and `serde_json` directly in
`tools/matten-report/Cargo.toml` only. This tool is workspace-excluded and
`publish = false`; the dependency must not be added to workspace dependencies or
to published `matten` family crates.

Not authorized:

```text
input-mode JSON
public JSON schema
public Report enum
public renderer API
public matten-report crate
public matten-viz crate
workspace membership change
published-crate dependency change
SVG / Vega-Lite / notebook / browser runtime scope
JavaScript or external assets
expression tracing
autograd
core Tensor visualization APIs
new report family
raw CSV JSON output
general project scanning
project mutation
release prep
version bump
tag or publish action
generated JSON artifacts checked into the repository
determinant / inverse / broader linalg scope
```

---

## 4. CLI Policy

JSON should mirror the local-file artifact policy already used by HTML:

```text
matten-report --demo data-readiness --format json --output <report.json>
matten-report --demo shape-flow --format json --output <report.json>
matten-report --demo dynamic-readiness --format json --output <report.json>
matten-report --demo mlprep-standardization --format json --output <report.json>
matten-report --demo educational-path --format json --output <report.json>
```

Required behavior:

```text
Markdown remains default.
--format json without --output fails.
--format json with --input fails for now.
--format json with unknown --demo fails through the existing demo-kind checks.
--select remains rejected for demo mode.
```

Suggested error text:

```text
--format json requires --output <report.json>
--format json is only supported for --demo data-readiness, shape-flow, dynamic-readiness, mlprep-standardization, or educational-path
--format json is not supported for --input yet
```

The exact wording may differ, but tests must prove the negative paths are
intentional and user-readable.

---

## 5. Private JSON Envelope

Every JSON report should use the same deterministic private envelope:

```json
{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "shape-flow",
  "input_mode": "demo",
  "data": {}
}
```

Rules:

```text
schema_version is always 0 for this private unstable format.
schema_status is always "private-local".
tool is always "matten-report".
report_kind uses the existing fixed demo kind strings.
input_mode is always "demo" in this slice.
data is a family-specific object.
limits is omitted in this slice because user-controlled input JSON is not supported.
```

The implementation must not serialize current private report structs directly as
the contract. It should build explicit private JSON payload structs or values
whose field names are chosen for the schema, not for renderer convenience.

Do not include:

```text
timestamps
hostnames
environment variables
absolute paths
random IDs
build metadata
Cargo package version
```

---

## 6. Shared Value Shapes

Use small shared shapes where they make the JSON easier to review.

Recommended tensor preview shape:

```json
{
  "shape": [2, 3],
  "values": [1.0, 2.0, 3.0],
  "truncated": true,
  "shown_values": 3,
  "total_values": 6,
  "limit": 3
}
```

Recommended table shape:

```json
{
  "columns": ["name", "mean", "std"],
  "rows": [
    ["sales", 123.33333333333333, 20.548046676563253]
  ]
}
```

Recommended checkpoint shape:

```json
{
  "label": "after reshape",
  "shape": [2, 3],
  "values": [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
}
```

The exact field set can be refined during implementation, but the review target
is data-oriented JSON, not a Markdown/HTML section dump.

---

## 7. Family Payload Requirements

Each fixed demo payload should contain the semantic values currently used by its
Markdown and HTML renderers.

Minimum required payload coverage:

| Report kind | Required JSON content |
|---|---|
| `data-readiness` | selected columns, source column summary, missing-count summary, numeric tensor preview |
| `shape-flow` | broadcast, reshape, axis-reduction, and matmul checkpoints |
| `dynamic-readiness` | dynamic element examples, schema summary, mask/selection examples, strict and explicit conversion outcomes |
| `mlprep-standardization` | selected columns, before/after shapes, before/after values, means, population standard deviations |
| `educational-path` | learning checkpoints for tensor construction, shape observation, data transformation, and derived results |

Allowed omissions:

```text
human-only prose that exists only to teach in Markdown/HTML
CSS/layout labels
HTML section ordering metadata
Markdown heading strings that are not data labels
```

Required preservation:

```text
values needed to explain the same computation path
shape information for tensors and matrices
row-major preview ordering
finite numeric values as JSON numbers
```

---

## 8. Numeric And Null Policy

For this fixed-demo slice:

```text
finite f64 values are JSON numbers
missing values are explicit null only when the data concept is genuinely absent
NaN / Infinity / -Infinity must not be silently emitted as null
```

The current fixed demos are expected to produce finite numeric values. If an
implementation path can encounter non-finite values, it must either reject them
with a clear local-tool error or encode them explicitly as a tagged value such
as:

```json
{ "non_finite": "NaN" }
```

The exact tagged representation may be adjusted in implementation review, but
silent loss through `null` is not acceptable.

---

## 9. Testing Requirements

Required tests:

```text
parse --format json
reject --format json without --output
reject --format json with --input
render exact JSON snapshot for data-readiness demo
render exact JSON snapshot for shape-flow demo
render exact JSON snapshot for dynamic-readiness demo
render exact JSON snapshot for mlprep-standardization demo
render exact JSON snapshot for educational-path demo
prove JSON output is deterministic across repeated render calls
prove JSON string escaping is delegated to the encoder or covered by hostile string tests
```

Recommended command checks:

```text
cargo fmt --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
bash scripts/check-release-docs.sh
git diff --check
```

If the project standard command differs, use the existing local-tool check
pattern already used by prior `tools/matten-report` slices.

---

## 10. Documentation Requirements

Update only documentation needed for the new local-tool behavior:

```text
tools/matten-report/README.md
docs/src/contributing/release-checklist.md
rfcs/README.md after implementation review
ROADMAP.md after implementation review
```

Documentation must say:

```text
JSON is private local-tool output.
JSON requires --output.
JSON is fixed-demo-only in this slice.
Input-mode JSON is not supported yet.
No public JSON schema or compatibility promise exists.
```

---

## 11. Acceptance Criteria

Review can accept an implementation only if:

```text
all five fixed demos support --format json --output <path>
Markdown output remains byte-identical
HTML output remains byte-identical
input-mode JSON is rejected
the emitted JSON uses schema_version 0 and schema_status private-local
the emitted JSON is deterministic and exact-snapshot tested
no public API or public crate is introduced
no published crate gains a JSON/report dependency
no generated JSON artifacts are committed
local-tool dependency additions, if any, are confined to tools/matten-report
release/version/tag/publish work is absent
```

---

## 12. Follow-Up Boundaries

After this slice is implemented and reviewed, possible later work can be
decided separately:

```text
input-mode data-readiness JSON with bounded metadata
more machine-readable artifact checks
SVG / Vega-Lite local output policy
public report-schema RFC after private JSON proves useful
public matten-report / matten-viz crate-boundary RFC
```

None of those follow-ups are authorized by this handoff.

---

## 13. Review Questions

Review should decide:

```text
[ ] Is the fixed-demo-only boundary correct?
[ ] Is requiring --output for JSON correct?
[ ] Is schema_version 0 / schema_status private-local sufficient?
[ ] Is it acceptable to add serde/serde_json only to tools/matten-report?
[ ] Are the required payload contents enough to preserve the educational/report value?
[ ] Are direct private-struct serialization and HTML-section JSON correctly rejected?
[ ] Are input-mode JSON, public schemas, public crates, and release work correctly out of scope?
```
