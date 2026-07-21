# RFC-071: Private Fixed-Demo JSON Report Artifacts

**Status:** Implemented (d0ef169; targeted for 0.37.0)
**Target:** 0.37.0
**Theme:** Add private fixed-demo JSON artifacts to `tools/matten-report`
**Depends on:** RFC-063, RFC-065, RFC-068, RFC-069, RFC-070
**Related:** RFC-030, RFC-049, RFC-054

---

## 1. Summary

This RFC authorizes the private fixed-demo JSON report slice that was designed
and implemented through the RFC-070 readiness-audit handoff line.

The scope is deliberately narrow:

```text
tools/matten-report --demo <kind> --format json --output <path>
```

for the five fixed demos:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
```

The output is private local-tool JSON with `schema_version: 0` and
`schema_status: "private-local"`. It is an evidence artifact for future report
readiness work, not a public schema.

## 2. Background

RFC-070 remains the public visualization/report readiness audit. It asks whether
local `tools/matten-report` artifacts are mature enough to justify a future
public product surface.

The audit and follow-up handoffs found that a public schema or public report
crate is premature. They also found that deterministic private JSON for existing
fixed demos is useful as a prerequisite:

- it makes the report data shape inspectable without reading HTML;
- it provides stable local evidence for future schema policy;
- it keeps renderer and schema commitments private;
- it avoids adding dependencies to published crates;
- it lets the project defer input-mode JSON until bounds and non-finite policy
  are designed.

This RFC records that private prerequisite as the normative authority for the
0.37.0 candidate, so RFC-070 can remain audit-only for public surfaces.

## 3. Goals

1. Add private JSON output for existing fixed demos in `tools/matten-report`.
2. Keep Markdown/plain text as the default.
3. Require explicit `--output` for JSON, matching the local-file artifact policy.
4. Use deterministic private report envelopes.
5. Cover every fixed demo with tests.
6. Reject unsupported JSON paths explicitly.
7. Keep all dependencies confined to the workspace-excluded `publish = false`
   local report tool.
8. Use the slice as the 0.37.0 local-tool release milestone under lock-step
   family versioning.

## 4. Non-Goals

This RFC does not authorize:

```text
[ ] input-mode JSON
[ ] public JSON schema
[ ] public Report enum
[ ] public renderer API
[ ] public matten-report crate
[ ] public matten-viz crate
[ ] workspace membership change for tools/matten-report
[ ] dependency change in any published crate
[ ] core Tensor visualization API
[ ] Tensor::plot(), Tensor::show(), or Tensor::backward()
[ ] expression tracing
[ ] autograd
[ ] SVG output
[ ] Vega-Lite output
[ ] JavaScript, external assets, notebook, browser, dashboard, GUI, or server integration
[ ] generated JSON artifacts checked into the repository
[ ] raw CSV JSON output
[ ] project scanning or mutation
[ ] determinant, inverse, or broader linalg scope
[ ] tag or publish action
```

## 5. Design

### 5.1 CLI

Supported command shape:

```text
matten-report --demo <kind> --format json --output <path>
```

JSON must be explicit-file-only. It must not print JSON to stdout by default,
because the report tool already treats richer artifacts as deliberate local
files.

### 5.2 Report Envelope

The private JSON envelope should include stable identifying fields such as:

```text
schema_version: 0
schema_status: "private-local"
tool: "matten-report"
report_kind
input_mode: "demo"
data
```

The envelope must not include timestamps, hostnames, environment values,
absolute paths, random IDs, build metadata, or Cargo package versions.

### 5.3 Schema Status

`schema_version: 0` means the schema is private and unstable. It is suitable for
local snapshots and review, not downstream compatibility promises.

Future public schema work requires a separate RFC.

### 5.4 Dependency Boundary

`serde` and `serde_json` may be used inside `tools/matten-report` only.

They must not be added to published crate dependencies as part of this RFC.

## 6. Business And Release Decision

The project uses lock-step family versioning under RFC-030. Recent local-tool
visualization milestones have been released as public family checkpoints even
when no published crate API changes, because the repository treats the public
family version as the durable documentation, changelog, and milestone coordinate
for the whole matten project.

For 0.37.0, the accepted business decision is:

- publish a lock-step family checkpoint for the private fixed-demo JSON milestone;
- state clearly that public crates have no API/runtime/dependency change;
- keep private-tool features described as local artifacts rather than packaged
  public APIs;
- reconsider this release model before future private-tool-only milestones if
  the publish churn outweighs the value of a public project checkpoint.

## 7. Acceptance Criteria

- JSON is supported for the five fixed demos only.
- JSON requires `--output`.
- Unsupported JSON paths, including input-mode JSON, are rejected.
- Markdown and HTML behavior remains unchanged.
- Exact or deterministic JSON tests cover all five fixed demos.
- Published crates receive no dependency change.
- Tracked docs and release notes describe the scope as private local-tool JSON.
- RFC-070 remains open only for public visualization/report readiness.

## 8. Release Status

The implementation was reviewed and committed before release prep. The 0.37.0
release prep retargets lock-step family metadata and documentation to this
RFC-071 scope.

Tag and publish actions remain separate maintainer actions and must use the
bare SemVer tag format, for example `0.37.0`.
