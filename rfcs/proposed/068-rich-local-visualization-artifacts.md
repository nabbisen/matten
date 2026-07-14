# RFC-068: Rich Local Visualization Artifacts

**Status:** Proposed; Phase 1 local HTML educational artifact implemented; shared educational report data refactor implemented for review
**Target:** v0.32.0 local visualization-artifact phase
**Theme:** Richer visual understanding through local, deterministic report artifacts
**Depends on:** RFC-022, RFC-049, RFC-063, RFC-064, RFC-065
**Related:** RFC-050, RFC-053, RFC-054, RFC-066, RFC-067

---

## 1. Summary

This RFC proposes the next visualization phase after RFC-063 and RFC-065.

The first approved direction should be richer local artifacts from the existing
`tools/matten-report` pattern, not public visualization crates and not core
rendering APIs.

The initial implementation slice should add one self-contained HTML artifact for
the existing learner-oriented educational path report:

```text
tools/matten-report --demo educational-path --format html --output <path>
```

The existing Markdown/plain-text output remains the default. The HTML artifact is
local-only, deterministic, static, and file-based. It must use no JavaScript, no
network, no external assets, no telemetry, no generated checked-in artifacts, and
no dependency in any published crate.

This RFC does not authorize `Tensor::plot`, `Tensor::show`, expression tracing,
autograd, `matten-viz`, a public `matten-report` crate, SVG output, Vega-Lite
JSON, notebook integration, or any plotting dependency in core `matten`.

---

## 2. Reviewer Overview

This review is about the next smallest useful visualization step.

RFC-063 established visual understanding through Markdown/ASCII docs, canonical
examples, and a local Markdown/plain-text `tools/matten-report` tool. RFC-065
then made the learner-facing path coherent and added the `educational-path`
demo.

The question for RFC-068 is:

```text
Should matten-report gain one richer local artifact format, starting with a
self-contained educational HTML report, while keeping all public crates and core
APIs unchanged?
```

The review should confirm:

```text
[ ] the first slice is local-tool-only and reviewable
[ ] the HTML artifact provides more visual understanding than plain Markdown
[ ] no public API, dependency, feature flag, runtime behavior, or maturity label changes
[ ] no public report/viz crate is authorized
[ ] the file-output, asset, security, and maintenance boundaries are explicit
```

---

## 3. Background

The project already has a visual-understanding base:

```text
RFC-063: visual docs, visual-summary examples, local matten-report demos
RFC-065: educational positioning, worked learner docs, educational-path report
```

Those RFCs deliberately kept visualization out of core `matten`:

```text
no Tensor::plot()
no public visualization API
no public matten-viz crate
no plotting dependency in core matten
no expression tracing
no autograd
```

That boundary remains correct. The useful next step is not a public renderer. It
is a better local artifact for the concepts already proven in Markdown/plain
text.

HTML is a reasonable first richer artifact because it can improve scannability
with headings, tables, simple blocks, and print-friendly styling without adding a
plotting stack or committing generated binary assets.

---

## 4. Goals

1. Make the educational path easier to inspect visually.
2. Reuse the existing `tools/matten-report` local-tool pattern.
3. Keep Markdown/plain text as the default output.
4. Add exactly one richer output format in the first slice: static HTML for the
   existing `educational-path` demo.
5. Keep generated artifacts out of version control.
6. Keep all published crates dependency-light and unchanged.
7. Define output/security boundaries before implementation.
8. Leave public report/viz crates for a future RFC only after local value is
   proven.

---

## 5. Non-goals

This RFC does not authorize:

```text
[ ] Tensor::plot(), Tensor::show(), Tensor::trace(), or Tensor::backward()
[ ] automatic expression tracing
[ ] lazy expression graphs
[ ] autograd
[ ] public matten-viz crate
[ ] public matten-report crate
[ ] plotting dependency in core matten
[ ] SVG output
[ ] Vega-Lite JSON output
[ ] JSON report output
[ ] browser UI, GUI, dashboard, or notebook requirement
[ ] JavaScript in generated reports
[ ] external CSS, fonts, images, or network assets
[ ] telemetry or network access
[ ] project scanning or source-code analysis
[ ] project mutation
[ ] visualization of large data sets
[ ] dataframe, ML-framework, or symbolic-math scope
[ ] performance-ranking charts
```

---

## 6. First Slice

### 6.1 Output format

Add a local HTML output option to `tools/matten-report` for the existing
`educational-path` demo.

Accepted command shape:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --format html --output target/matten-report-educational-path.html
```

The existing Markdown/plain-text command remains valid and remains the default:

```bash
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
```

The first slice should not require changing existing Markdown exact-output tests
except where parser help text or CLI usage text must list the new option.

### 6.2 HTML constraints

The generated HTML must be:

```text
self-contained
static
deterministic
UTF-8
print-friendly
small enough for review
free of JavaScript
free of external assets
free of network references
```

Allowed visual structure:

```text
semantic headings
tables
pre/code blocks
simple CSS embedded in <style>
small inline shape cards built from HTML/CSS
simple axis/readiness labels
```

Not allowed in the first slice:

```text
SVG
canvas
JavaScript
external CSS
external fonts
external images
data URLs
terminal-color/ANSI assumptions
browser automation tests
```

### 6.3 File-output policy

HTML output must be explicit-file output in the first slice. Do not print HTML
to stdout unless a later review accepts that behavior.

Required behavior:

```text
--format html without --output returns a clear error
--format markdown keeps current stdout/default behavior
--format html writes only the requested file
parent directories are not silently created unless existing tool policy already does so
errors are readable and never panic for ordinary CLI misuse
```

### 6.4 Escaping and safety

The first slice should avoid user-controlled HTML input by staying demo-only.
Still, any HTML writer should use an explicit escape helper for text values so
the code path is safe before future input modes are considered.

No input-mode HTML report is approved by this RFC.

---

## 7. Later Phases

Later phases require separate handoffs or RFC amendments.

Candidate follow-ups:

```text
HTML output for shape-flow
HTML output for dynamic-readiness
HTML output for data-readiness
HTML output for mlprep-standardization
small static SVG diagrams
Vega-Lite JSON from local tools
public matten-report crate
public matten-viz crate
```

The expected order is:

```text
1. prove one local HTML artifact
2. extend to one or two existing report families if useful
3. consider static SVG or Vega-Lite only after HTML/Markdown value is clear
4. consider public crates only after stable user value and API boundaries exist
```

Public crates require a new RFC covering:

```text
public API boundary
dependency policy
security and file-output behavior
maintenance owner
docs and release gates
versioning and maturity label
```

---

## 8. Dependency Policy

The first slice should use the standard library and existing workspace crates
only. No dependency should be added to core `matten` or any published crate.

Adding a dependency to `tools/matten-report` is not accepted for the first HTML
slice unless review finds a concrete reason. The preferred first implementation
is a small local HTML renderer over the existing fixed report data.

If a later slice proposes `serde_json`, an SVG writer, Plotters, or any other
format/rendering dependency, it must justify:

```text
why std-only output is insufficient
why the dependency is local-tool-only
how it is kept out of published crates
how release gates catch dependency drift
```

---

## 9. Relationship To Existing Boundaries

### Core `matten`

Core remains computation-only. It should not gain rendering methods, renderer
traits, expression graph hooks, or visualization dependencies.

### `tools/matten-report`

`tools/matten-report` remains:

```text
workspace-excluded
publish = false
local-only
deterministic
explicit about output files
not a public API
```

### RFC-054 / migration tooling

This work must not blur into source scanning or project mutation. `matten-report`
explains fixed demos and small user-selected report inputs only where explicitly
authorized; `matten-migrate` remains advisory and separate.

### RFC-063 / RFC-065

RFC-068 builds on the proven visual and educational content from RFC-063 and
RFC-065. It does not reopen their rejected scopes.

---

## 10. Acceptance Criteria

This RFC is accepted when:

```text
[ ] the first slice is limited to local HTML output for educational-path
[ ] Markdown/plain text remains the default
[ ] --format html requires explicit --output
[ ] no public API or published crate changes are authorized
[ ] no new dependency is authorized for the first slice
[ ] no SVG/Vega-Lite/JSON/notebook/browser scope is authorized
[ ] no expression tracing or autograd scope is authorized
[ ] output safety and escaping requirements are recorded
[ ] the implementation handoff is small enough for direct review
[ ] rfcs/README.md and ROADMAP.md track RFC-068 as the active proposed RFC
```

---

## 11. Open Questions

1. Should HTML stdout ever be allowed, or should HTML always require `--output`?
2. Should later HTML reports share one renderer data model, or stay independent
   fixed report functions until duplication becomes painful?
   Current answer for the next slice: extract a private shared data model for
   the existing educational-path Markdown and HTML reports before adding another
   HTML report family. See
   `rfcs/handoffs/068-shared-educational-report-model-handoff.md`.
3. Should static SVG be the next richer artifact after HTML, or should the
   project avoid SVG until a public visualization crate is seriously considered?
