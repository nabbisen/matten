# RFC-073: Private Input-Mode JSON Report Policy

**Status:** Implemented; reviewed (GO, no conditions) and committed; `0.38.0` release-prep complete and prepared for review; publish/tag not yet authorized
**Target:** `0.38.0` local-tool policy (release-prep candidate)
**Theme:** Bounded private JSON for user-provided data-readiness input
**Depends on:** RFC-001, RFC-037, RFC-069, RFC-070, RFC-071, RFC-072
**Related:** RFC-023, RFC-034, RFC-035, RFC-063, RFC-068

---

## 1. Summary

This RFC proposes the next policy decision for the workspace-excluded,
`publish = false` `tools/matten-report` binary:

```text
matten-report \
  --input <csv-path> \
  --kind data-readiness \
  --select <col1,col2> \
  --format json \
  --output <report.json>
```

The proposed output is a bounded, deterministic, private local artifact. It
extends the private schema-v0 approach from RFC-071 to the already-supported
data-readiness input path from RFC-069. It does not create a public schema,
public renderer API, published report crate, or raw CSV export.

The accepted policy and detailed handoff authorized one implementation
checkpoint. That checkpoint is complete and prepared for review. No public
schema or API, version, release preparation, tag, or publish action is
authorized.

## 2. Why This Is The Next Candidate

RFC-071 shipped private JSON for five fixed demos and deliberately deferred
input-mode JSON until user-controlled bounds and conversion-error policy were
designed. RFC-072 then separated report construction from format/family
rendering and closed the structural migration.

The remaining asymmetry is narrow:

| Input | Markdown | HTML | Private JSON |
|---|---|---|---|
| Five fixed demos | supported | supported | supported |
| CSV data-readiness | supported | supported | rejected |

Closing that asymmetry can help local automation and make data-readiness
outcomes inspectable without expanding core `Tensor`. It is lower-risk than a
public schema because the output remains schema version 0, local-tool-owned,
and explicitly compatibility-unstable.

User input makes this materially different from fixed-demo JSON. Paths,
headers, selected names, conversion-error text, column counts, and tensor data
can all be user-controlled or large. The feature therefore requires its own
policy rather than inheriting RFC-071 silently.

## 3. Current Contract And Ownership

The current input path accepts only `data-readiness`. It builds one private
`DataReadinessReportData` value before renderer dispatch. That model contains:

```text
input label
source columns
selected columns
left-out columns
missing counts
numeric conversion success { tensor shape, tensor values }
numeric conversion error { normalized message }
```

Current outcome classes are significant:

| Outcome | Current process behavior | Proposed JSON behavior |
|---|---|---|
| File open / CSV parse failure | command error | remain command error; no report artifact |
| Invalid or duplicate selection | command error | remain command error; no report artifact |
| Strict numeric conversion and tensor construction succeed | report success | JSON success artifact |
| Strict numeric conversion fails | reportable data-readiness outcome | JSON error-summary artifact |
| Numeric conversion succeeds but tensor construction fails | pre-write command error | remain command error; no new or changed destination |
| Output write fails | command error; destination state is not guaranteed | remain command error without adding atomicity claims |

The existing private JSON owner uses an explicit schema-v0 envelope and
family-specific payloads. It does not serialize report structs directly. Its
tensor preview is capped at 12 values and rejects non-finite numbers.

The existing input-mode HTML policy provides these reviewed display limits:

```text
columns shown:        12
characters per path/header: 120
conversion-error characters: 240
tensor values shown: 12
```

These values are the conservative baseline for RFC-073. A later detailed
handoff must assign an explicit private JSON policy owner and test parity with
the accepted HTML limits. It need not refactor the HTML renderer merely to
share constants.

## 4. Goals

1. Decide whether private input-mode JSON should be implemented for the one
   existing CSV data-readiness path.
2. Preserve Markdown as the default and require explicit `--output` for JSON.
3. Preserve summary-only reporting rather than exposing the raw CSV table.
4. Represent both successful and failed strict numeric conversion as bounded
   report outcomes.
5. Keep ingestion, parsing, selection, report/tensor construction, and
   output-I/O failures as command failures rather than JSON report documents.
6. Require deterministic schema-v0 output with explicit `input_mode: "csv"`.
7. Include machine-readable truncation metadata for user-controlled content.
8. Reuse the existing finite-number rejection policy and tensor-preview limit.
9. Keep dependencies, APIs, and runtime behavior of all published crates
   unchanged.
10. Require a reviewed detailed implementation handoff before code changes.

## 5. Non-Goals

This RFC does not authorize:

```text
[ ] implementation before RFC acceptance and a reviewed detailed handoff
[ ] JSON to stdout or implicit output-file selection
[ ] input-mode JSON for report kinds other than data-readiness
[ ] full raw CSV rows, cells, or table export
[ ] unbounded paths, headers, column lists, errors, or tensor values
[ ] a public or stable JSON schema
[ ] schema_version 1
[ ] a public Report enum or renderer API
[ ] a public matten-report or matten-viz crate
[ ] workspace membership change for tools/matten-report
[ ] a new dependency in any published crate
[ ] direct serialization of private report structs
[ ] timestamps, hostnames, environment values, canonicalized paths, random IDs, or build metadata
[ ] JSON documents for file-open, CSV-parse, invalid-selection, or output-write failures
[ ] a JSON report outcome for report/tensor-construction failure
[ ] SVG, Vega-Lite, JavaScript, external assets, notebook, browser, dashboard, GUI, or server scope
[ ] core Tensor visualization, expression tracing, or autograd
[ ] determinant, inverse, broader statistics, or other mathematics scope
[ ] streaming or large-CSV lifecycle changes
[ ] generated JSON artifacts checked into the repository
[ ] version bump, release preparation, tag, or publish action
```

## 6. Proposed Policy

### 6.1 Command And Output

The only candidate command is:

```text
matten-report --input <csv-path> --kind data-readiness \
  --select <columns> --format json --output <report.json>
```

Policy:

```text
explicit --format json
explicit --output
UTF-8 JSON file
one trailing newline
deterministic for the same input bytes, path spelling, and selection order
no stdout report content
existing process-error prefix and output-write behavior preserved
```

The tool must not canonicalize the input path or add host-derived provenance.
The displayed path is based on the spelling supplied by the user, then bounded
by the JSON presentation policy.

### 6.2 Private Envelope

The envelope should remain private schema version 0:

```json
{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "data-readiness",
  "input_mode": "csv",
  "limits": {},
  "data": {}
}
```

`input_mode` must no longer be unconditionally hardcoded to `"demo"` in the
shared renderer path. The implementation handoff must preserve all fixed-demo
bytes while adding the CSV mode explicitly.

`limits` records the effective bounds used for user-controlled fields. Its
exact field names require review and snapshot acceptance before implementation.

### 6.3 Bounded Values And Lists

JSON must not express truncation only through human prose. A bounded string
should carry enough metadata to distinguish complete and shortened values:

```text
value
truncated
shown_chars
total_chars
limit
```

A bounded list should carry:

```text
items
truncated
shown_items
total_items
limit
```

Every displayed path, header, selected name, left-out name, missing-count
column name, and conversion-error message must use the applicable bound. Counts
must be measured in Unicode scalar values, matching Rust `char`-based display
limits, not bytes.

The baseline limits are:

```text
MAX_DISPLAY_COLUMNS = 12
MAX_DISPLAY_CHARS = 120
MAX_ERROR_CHARS = 240
MAX_TENSOR_PREVIEW_VALUES = 12
```

The implementation must use structured omitted counts rather than adding a
synthetic `"... N more"` item to machine-readable lists.

### 6.4 Success Outcome

A success artifact requires both strict numeric conversion and tensor
construction to succeed. It may include:

```text
bounded input label
bounded source/selected/left-out column lists
bounded missing-count entries
numeric_conversion.status = "success"
tensor shape
bounded row-major tensor preview and its existing truncation metadata
```

The tensor preview must retain the RFC-071 policy:

```text
at most 12 finite f64 values
shape
shown_values
total_values
limit
truncated
NaN / Infinity / -Infinity rejected explicitly
```

### 6.5 Conversion-Error Outcome

A strict numeric conversion failure is report data, not a command failure. Its
JSON artifact should contain:

```text
the same bounded input/column/missing summary
numeric_conversion.status = "error"
a bounded normalized message with truncation metadata
no tensor field
```

The first slice should not expose a new public or quasi-public error taxonomy.
It should serialize the report-owned normalized message, not `MattenDataError`
or its internal fields directly. A future machine-readable error-code proposal
requires separate review if local automation proves the message insufficient.

### 6.6 Command Failures

These failures complete before `output::write` begins:

```text
input file cannot be opened
CSV cannot be parsed
selected column does not exist
selection is empty or contains duplicates
report construction fails after conversion, including zero-row tensor construction
JSON cannot be serialized
JSON representation policy rejects a value, including non-finite numbers
```

They must retain nonzero exit status, the established stderr error prefix, and
no report on stdout. Because writing has not begun, they must create no output
artifact and must leave an already-existing destination unchanged.

A failure that occurs inside the current `output::write` is different. The
writer uses `std::fs::write`, which is not an atomic replacement contract and
may create, truncate, or partially write the destination before returning an
error. For a write-time failure, this RFC requires only:

```text
nonzero exit status
the established stderr error prefix
no report on stdout
```

It does not promise that the destination is absent or unchanged. Atomic
replacement, rollback, and cleanup-on-failure are outside this RFC. A later
handoff must preserve this distinction in its process and filesystem
assertions and must not use failure injection to imply stronger output
semantics.

### 6.7 Encoding And Security

`serde_json` remains the encoder. Manual JSON escaping or concatenation is not
acceptable. The existing `serde` and `serde_json` dependencies remain confined
to the workspace-excluded local tool.

Tests must cover at least:

```text
quotes, backslashes, control characters, and non-ASCII in path/header/error text
long path and header values
more than 12 source/selected/left-out/missing-count entries
more than 12 tensor values
conversion success and conversion error
non-finite numeric rejection at the JSON policy boundary
```

The JSON must never include raw CSV rows or cells beyond values already present
in the bounded normalized conversion-error message and tensor preview.

## 7. Compatibility And Ownership

The artifact remains private and unstable:

```text
schema_version = 0
schema_status = "private-local"
no SemVer compatibility promise
no public Rust type or schema package
no downstream-consumer support commitment
```

Even without a public compatibility promise, changes require review because
exact snapshots and local scripts may exist. The implementation must use
explicit private JSON representation types owned by `render::json`, while
`report::data_readiness` remains format-neutral.

`app` continues to build one report-data value before selecting a renderer.
The JSON renderer must not read the CSV, select columns, or perform numeric
conversion independently.

## 8. Implementation Readiness Gate

RFC acceptance should authorize only a detailed implementation handoff. That
handoff must settle:

```text
exact envelope and payload field names/order
exact bounded-string/list representation and private JSON limit owner
success and conversion-error snapshots
fixed-demo byte-preservation strategy when input_mode becomes explicit
CLI/app routing changes and removal of the current rejection
pre-write unchanged-destination assertions and write-time routing assertions
header-only/zero-row tensor-construction failure with absent and existing destinations
test ownership under the RFC-072 module boundaries
process-boundary and module-boundary impact
CI and release-checklist smoke commands
whether the work merits a release, deferred until after implementation evidence
```

No code change may begin from this RFC alone.

## 9. Risks And Controls

| Risk | Control |
|---|---|
| Raw or excessive user data leaks into JSON | Summary-only allowlist plus structured bounds for every user-controlled field |
| Private output is mistaken for a stable schema | Version 0, `private-local`, no public types, explicit no-compatibility statement |
| HTML and JSON bounds drift | Record matching values and add parity assertions without requiring a cross-format refactor |
| Fixed-demo JSON changes accidentally | Require byte-identical fixed-demo snapshots and process anchors |
| Conversion failures become ambiguous process errors | Preserve report-owned success/error distinction explicitly |
| Tensor-construction failure is mistaken for a conversion-error artifact | Keep the reachable header-only/zero-row path as a pre-write command error and test both destination states |
| Pre-write failures alter an output path | Require no newly created artifact and an unchanged existing destination before `output::write` begins |
| Write-time failure is mistaken for atomic output | Require nonzero status/stderr routing but make no destination-state guarantee for the existing `fs::write` owner |
| JSON injection or invalid encoding | Use `serde_json`; prohibit manual escaping |
| Renderer recomputes report data | Preserve RFC-072 report-before-render ownership guard |
| Feature work turns into public product scope | Keep tool workspace-excluded, binary-only, and `publish = false` |
| Release churn follows automatically | Leave release decision outside RFC acceptance and implementation handoff |

## 10. Alternatives

### Keep Input-Mode JSON Rejected

This is the lowest-maintenance choice and remains valid if the bounded schema
does not justify its tests and policy cost. Markdown and HTML already serve
human inspection.

### Emit The Fixed-Demo Payload Unchanged

Rejected. It would expose unbounded user-controlled strings and lists, omit
conversion-error representation, and mislabel the input mode as `demo`.

### Serialize `DataReadinessReportData` Directly

Rejected. It would couple artifact shape to computation convenience, weaken
renderer ownership, and create accidental schema expectations.

### Open A Public Schema First

Rejected. RFC-070 closed public readiness without implementation, and private
schema-v0 evidence is still insufficient for a compatibility promise.

### Implement Broader Mathematics Or A Bridge Instead

Deferred as separate RFC candidates. They remain valuable, but introduce core
mathematical semantics or ecosystem dependency commitments unrelated to this
bounded local-tool gap.

## 11. Review Questions

Review should decide:

```text
[ ] Is private data-readiness input-mode JSON valuable enough to implement?
[ ] Is explicit-file-only output still the correct JSON policy?
[ ] Are success and strict-conversion-error artifacts both required?
[ ] Should ingestion/parse/selection/report-construction/representation failures remain process errors with no new or changed artifact?
[ ] Should header-only/zero-row tensor construction be tested with absent and existing destinations?
[ ] Should write-time failures remain process errors without any destination-state guarantee?
[ ] Are 12 columns, 120 path/header chars, 240 error chars, and 12 tensor values the right baseline?
[ ] Is structured truncation metadata sufficient and appropriately private?
[ ] Should normalized error text remain the first-slice machine representation instead of a new error-code taxonomy?
[ ] Does input_mode = "csv" correctly extend the private envelope without changing fixed-demo bytes?
[ ] Are serde_json encoding and finite-number rejection the right retained policies?
[ ] Are report/render/app ownership boundaries consistent with RFC-072?
[ ] Are public schemas/APIs/crates, broader formats, core visualization, mathematics, bridges, release work, and publishing still correctly deferred?
[ ] Should acceptance authorize only a detailed implementation handoff?
```

## 12. Acceptance Criteria

RFC acceptance requires:

```text
[ ] one narrow command shape and report kind
[ ] explicit summary-only field allowlist
[ ] exact outcome taxonomy: report outcome versus command failure
[ ] machine-readable bounds and truncation policy
[ ] retained finite-number and deterministic-output policy
[ ] private schema-v0 and ownership boundary
[ ] fixed-demo behavior-preservation requirement
[ ] implementation-handoff gate before coding
[ ] explicit no-public-surface and no-release authorization
```
