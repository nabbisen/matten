# RFC-072: Behavior-Preserving `matten-report` Modularization

**Status:** Proposed; Slice 0/1 and Phase 2 reviewed and committed; Phase 3 shared formatting plus data-readiness Markdown extraction prepared for review; later renderer units unauthorized
**Target:** Post-0.37 maintainability work; release family undecided
**Theme:** Split the local report tool into explicit internal ownership boundaries
**Depends on:** RFC-063, RFC-065, RFC-068, RFC-069, RFC-070, RFC-071
**Related:** RFC-013, RFC-022, RFC-030, RFC-043, RFC-054

---

## 1. Summary

This RFC proposes a behavior-preserving modularization of the workspace-excluded
`tools/matten-report` binary.

The tool has accumulated five fixed report families, one CSV input path, three
output formats, output-policy validation, display/security helpers, private JSON
payloads, and 59 tests in one 5,023-line `src/main.rs`. That shape now obscures
ownership and exceeds the project's strong 500-ELOC split threshold by an order
of magnitude.

The proposed work changes internal source organization only:

```text
thin binary entry point
internal request/configuration model
CLI parsing and policy validation
application orchestration
report-family data builders
format-owned Markdown, HTML, and JSON renderers
output I/O
tests placed beside their owning modules
```

No user-visible behavior, report content, public API, dependency, package
boundary, or release scope changes under this RFC.

## 2. Background And Evidence

RFC-063 through RFC-069 established the local educational/reporting tool. RFC-071
added private fixed-demo JSON in `0.37.0`. RFC-070 then closed public
visualization readiness without authorizing public crates or APIs and selected
behavior-preserving modularization as the next design-first theme.

Current evidence from `tools/matten-report/src/main.rs`:

```text
5,023 physical lines
59 top-level functions
59 unit tests in one inline #[cfg(test)] module beginning at line 2,641
CLI parsing, validation, dispatch, report construction, three renderers,
formatting helpers, filesystem output, and tests in one compilation unit
```

Current dependencies remain appropriate for the local tool:

```text
matten
matten-data
matten-mlprep
serde
serde_json
```

The problem is internal ownership, not missing functionality. The refactor must
not become a vehicle for new report formats or public abstractions.

## 3. Requirements

### 3.1 Functional Preservation

The modularized tool must preserve:

1. Every currently accepted CLI command and argument combination.
2. Every currently rejected command and its user-readable error text.
3. Markdown as the default output format.
4. Explicit `--output` requirements for HTML and JSON.
5. Fixed-demo Markdown, HTML, and private JSON output byte-for-byte.
6. Data-readiness input-mode Markdown and HTML output byte-for-byte.
7. Input-mode JSON rejection.
8. Static, self-contained HTML with escaping and no JavaScript or external assets.
9. Display bounds for paths, columns, errors, and tensor previews.
10. Deterministic private JSON with `schema_version: 0`,
    `schema_status: "private-local"`, finite-number rejection, and trailing newline.
11. The rule that files are created only when `--output` is supplied.
12. Existing process exit behavior and `matten-report error: ...` prefix.
13. Process-level routing: help and Markdown to stdout, failures to stderr,
    explicit-file artifacts to the requested file without stdout leakage.

### 3.2 Structural Requirements

The implementation must:

1. Keep `tools/matten-report` workspace-excluded and `publish = false`.
2. Keep a binary-only package; do not add `src/lib.rs` or a library target.
3. Keep all internal cross-module visibility private or `pub(crate)`.
4. Establish an acyclic dependency direction between internal modules.
5. Separate report-data construction from Markdown/HTML/JSON rendering.
6. Keep private JSON payload types renderer-owned rather than treating them as
   the canonical report model.
7. Place tests in separate files under `src/`, following the project's
   `foo.rs` plus `foo/tests.rs` convention.
8. Target at most 300 ELOC per Rust file and require explicit justification for
   any production or test Rust file above 500 ELOC.
9. Move code mechanically where possible and avoid opportunistic rewrites.
10. Preserve the existing Cargo dependency set and feature selections.
11. Establish process-boundary preservation evidence against the monolithic
    baseline before moving the entry point, CLI, or output code.

## 4. Goals

1. Make ownership legible to maintainers.
2. Reduce the blast radius of later changes.
3. Make report-family data reusable across private renderers without making it public.
4. Make format policies and security helpers easy to locate and review.
5. Split the test suite along the same boundaries as production code.
6. Create a stable internal structure before any later feature RFC is selected.
7. Keep the refactor reviewable through behavior-preserving migration slices.

## 5. Non-Goals

This RFC does not authorize:

```text
[ ] new CLI flags, commands, report kinds, or input kinds
[ ] input-mode JSON
[ ] SVG, Vega-Lite, notebook, browser-runtime, dashboard, GUI, or server output
[ ] public JSON schema or schema_version change
[ ] public Report enum, report model, renderer API, or reusable library API
[ ] public matten-report or matten-viz crate
[ ] src/lib.rs or a library target
[ ] workspace membership change
[ ] dependency additions, removals, or version changes
[ ] changes to published matten-family crates
[ ] changes to Markdown, HTML, JSON, help, or error text
[ ] replacement of current CLI parsing with a framework
[ ] replacement of Box<dyn Error> or String errors as a separate redesign
[ ] generated report artifacts or new golden-output files checked into the repository
[ ] performance optimization or benchmark claims
[ ] expression tracing, computation graphs, autograd, or Tensor visualization APIs
[ ] determinant, inverse, broader statistics/linalg, or bridge implementation
[ ] version bump, release preparation, tag, or publish action
```

## 6. External Design Contract

The CLI remains exactly the current local-tool interface.

| Input mode | Markdown | HTML | JSON |
|---|---|---|---|
| Five fixed demos | stdout or explicit file | explicit file only | explicit file only; private schema v0 |
| CSV `data-readiness` | stdout or explicit file | explicit file only; bounded and escaped | rejected |
| Other CSV report kinds | rejected | rejected | rejected |

The five fixed demo identifiers remain:

```text
data-readiness
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
```

This RFC creates no public Rust contract. Internal names and file locations are
reviewed maintainability decisions, not SemVer API.

### 6.1 Process Contract

Function-level snapshots are necessary but insufficient because they do not
observe `main`, process exit status, or stdout/stderr/file routing. Before any
source movement, the implementation must add and run a dependency-free
process-boundary harness against the current monolithic binary.

The harness must assert at least:

| Case | Required evidence |
|---|---|
| `--help` | exit 0, exact stdout including final newline, empty stderr |
| Invalid invocation | exit 1, empty stdout, exact stderr including `matten-report error: ` and final newline |
| Markdown stdout | exit 0, exact report stdout, empty stderr |
| Explicit-file HTML or JSON | exit 0, exact file bytes, empty stdout and stderr |
| Filesystem write failure | nonzero exit, empty stdout, stderr retains the error prefix/routing and final newline |

The filesystem error detail may be platform-specific; its exact OS text need
not become a new compatibility contract. The stable prefix, routing, nonzero
status, and newline are required.

The detailed handoff should prefer a dependency-free exact shell harness wired
into CI and the release checklist. Another mechanism is acceptable only if it
executes the built binary and observes the same process-level evidence. Calling
private Rust functions directly does not satisfy this gate. The expected bytes
should be embedded in the harness rather than added as generated report files.

## 7. Internal Architecture

### 7.1 Target Module Tree

The implementation handoff may refine filenames, but it must preserve these
ownership boundaries:

```text
tools/matten-report/src/
  main.rs                  process entry point only
  app.rs                   application orchestration and dispatch
  app/
    tests.rs
  request.rs               Config, Input, OutputFormat, ReportKind
  cli.rs                   argument parsing, usage text, format policy
  cli/
    tests.rs
  output.rs                stdout / explicit-file writing
  report.rs                report-family module root and shared private shapes
  report/
    data_readiness.rs      fixed and input-mode report-data builders
    shape_flow.rs
    dynamic_readiness.rs
    mlprep_standardization.rs
    educational_path.rs
    tests.rs               builder invariants; split further if needed
  render.rs                renderer dispatch only
  render/
    common.rs              format-neutral display helpers only
    markdown.rs            Markdown renderer root
    markdown/              family renderers and separated tests
    html.rs                HTML renderer root
    html/                  document shell, family renderers, separated tests
    json.rs                private schema-v0 renderer root
    json/                  payload types, family mappings, separated tests
```

Format directories may use per-family files when a single renderer file would
exceed the line-count policy. Tests must not be kept in a new monolithic root
`tests.rs` merely to move the existing inline block unchanged.

### 7.2 Ownership

`request` owns the internal vocabulary needed by more than one layer:

```text
Config
Input
OutputFormat
ReportKind
```

`cli` owns parsing, usage text, supported combinations, and exact policy error
messages. It may construct `request` values but must not render reports or read
CSV files.

`report` owns family-specific data structures and builders. It may depend on
`matten`, `matten-data`, and `matten-mlprep`. It must not depend on CLI parsing,
filesystem output, HTML, Markdown, or JSON payload types.

`render` owns representation. Markdown and HTML consume private report data.
JSON owns its private schema-v0 envelope and payload mapping; JSON payload types
must not migrate into `report` as if they were canonical domain types.

`output` owns the final stdout/file choice and uses only the standard library.
It must not choose a renderer or alter rendered bytes.

`app` is the only orchestration layer. It translates a validated request into
input loading, report-data construction, renderer selection, and output writing.

### 7.3 Dependency Direction

Allowed direction:

```text
main -> app
app -> cli, request, report, render, output
cli -> request
report -> matten / matten-data / matten-mlprep
render -> request, report, serde / serde_json
output -> std
```

Forbidden direction:

```text
report -> cli
report -> render
report -> output
render -> cli
render -> output
output -> report or render
published crate -> tools/matten-report
```

No internal module is public outside the binary crate.

### 7.4 Data And Error Policy

Report-family data remains private and family-specific. This RFC does not seek a
single universal report enum or schema. Shared internal shapes are allowed only
where they remove real duplication without erasing report meaning.

The implementation should preserve current `Result<_, Box<dyn Error>>` and
CLI `Result<_, String>` behavior unless a mechanical signature adjustment is
required for module boundaries. A new error taxonomy would be separate design
work and must not be bundled into this refactor.

## 8. Migration Strategy

Implementation should be divided into reviewable, behavior-neutral slices.

### Slice 0: Process-Boundary Baseline

Before moving Rust source, add the process harness defined in Section 6.1, run
it against the current monolithic binary, and wire it into the local-tool CI and
release-checklist gates.

Required checkpoint:

```text
the harness fails when exit status, stdout/stderr routing, error prefix, or file bytes drift
all five required process cases pass against the pre-refactor binary
no production Rust source has moved
```

### Slice 1: Entry, Request, CLI, And Output Boundaries

Extract the process entry point, internal request types, argument parsing,
format-policy validation, usage text, and stdout/file writing. Move CLI tests to
their owned test file. Report construction and rendering may temporarily remain
behind private functions in the existing module.

Required checkpoint:

```text
all 59 existing tests still pass
CLI help and error assertions remain exact
no report snapshot changes
the process-boundary harness remains green after entry/CLI/output movement
```

### Slice 2: Report-Family Data Builders

Extract private data structures and builders for all five fixed families plus
data-readiness input mode. Renderers continue to consume the same values. The
detailed handoff must divide this phase into family-sized sub-slices; it must
not move every family builder as one review unit.

Recommended sub-slice order:

```text
shape-flow
dynamic-readiness
mlprep-standardization
educational-path
data-readiness fixed demo and input mode
```

The handoff may change the order when dependency evidence justifies it, but
each family sub-slice receives the checkpoint below before the next begins.

Required checkpoint:

```text
no renderer byte changes
no JSON payload type promoted to report-domain ownership
builder tests live outside production files
```

### Slice 3: Format-Owned Renderers

Extract Markdown, HTML, and JSON dispatch and family renderers. Keep the HTML
document shell, escaping, display bounds, and JSON finite-value/private-envelope
policies in clearly named renderer-owned modules. The detailed handoff must
subdivide this phase by format and then by shared shell/envelope or report
family; moving all renderers in one review unit is not acceptable.

Recommended shape:

```text
Markdown family sub-slices
HTML document/escaping helpers, then HTML family sub-slices
JSON envelope/preview helpers, then JSON family sub-slices
```

Required checkpoint:

```text
all exact Markdown/HTML/JSON snapshots remain byte-identical
static/self-contained HTML assertions remain intact
private JSON determinism remains intact
```

### Slice 4: Test Placement And Structural Closure

Finish splitting oversized test files, remove dead forwarding helpers, verify
module dependency direction, and document any file that remains above the
line-count target.

Required checkpoint:

```text
main.rs is a thin process entry point
no production or test Rust file exceeds 500 ELOC without reviewed justification
no monolithic replacement test file is created
all current behavior and CI smoke commands remain unchanged
the process-boundary harness remains part of CI and the release checklist
```

The implementation handoff must define exact file moves and may combine later
slices only when review concludes the combined diff remains mechanically
reviewable. Slice 0 is the exception: it must be captured and observed against
the monolithic baseline as a distinct checkpoint before Slice 1 or any other
Rust source movement.

## 9. Test And Verification Contract

The current 59-test suite is the minimum preservation baseline. The
implementation may add structural tests, but it must not delete or weaken
behavior assertions merely because functions move.

The process-boundary harness from Section 6.1 is an additional mandatory gate,
not part of the existing 59-test count. It must first pass against the current
monolith and then remain green through every migration sub-slice.

Required verification after implementation and one final formatting pass:

```bash
cargo fmt --manifest-path tools/matten-report/Cargo.toml --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
bash tools/matten-report/tests/process-boundary.sh
bash scripts/check-release-docs.sh
git diff --check
```

The implementation handoff must also retain the existing report-tool smoke
commands from `.github/workflows/test.yaml` and
`docs/src/contributing/release-checklist.md`. Snapshot tests remain source-owned
test fixtures; this RFC does not authorize checked-in generated report files.

## 10. Risks And Controls

| Risk | Control |
|---|---|
| Accidental output drift during code movement | Preserve exact snapshots and process-boundary bytes at every slice |
| Exit/routing drift hidden by private-function tests | Establish the process harness before movement and retain it in CI/release gates |
| Visibility expansion for convenience | Permit only private and `pub(crate)` items; no library target |
| Cyclic or vague module ownership | Enforce the dependency direction in Section 7.3 |
| Premature universal report abstraction | Keep family models private and JSON payloads renderer-owned |
| New monoliths replacing `main.rs` | Apply 300/500-ELOC guidance symmetrically to production and test files |
| Opportunistic behavior changes | Treat changed help, errors, output, dependencies, or features as out of scope |
| Test loss while relocating inline tests | Preserve all 59 named tests before structural cleanup |
| Public-product implication | Keep package workspace-excluded, unpublished, and binary-only |

## 11. Acceptance Criteria

RFC acceptance should confirm:

```text
[ ] behavior-preserving modularization is justified before more feature work
[ ] the external CLI/output contract is complete
[ ] the target ownership boundaries are understandable and cycle-free
[ ] report data remains separate from format-owned rendering
[ ] private JSON payloads do not become a public or canonical report schema
[ ] the migration slices are reviewable and preserve the 59-test baseline
[ ] process exit status, stdout/stderr routing, exact representative bytes, error prefix, and file output have a pre-movement executable gate
[ ] builder and renderer phases are subdivided by family/format in the detailed handoff
[ ] test placement follows the Rust CLI project rules
[ ] line-count targets apply to production and test Rust files
[ ] no code movement begins until a detailed implementation handoff is reviewed
[ ] no new feature, public API, dependency, version, or release action is authorized
```

## 12. Follow-Up Order

If this RFC is accepted:

1. Draft and review a detailed implementation handoff.
2. Implement only the reviewed behavior-preserving modularization slices.
3. Obtain implementation review and commit the completed refactor.
4. Then choose a separate next RFC among private input-mode JSON, broader
   mathematics, or a new ecosystem bridge.
