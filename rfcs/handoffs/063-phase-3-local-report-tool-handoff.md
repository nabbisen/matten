# RFC-063 Phase 3 Local Report Tool Handoff

**Project:** `matten`  
**Related RFC:** RFC-063: Visual Understanding and Reporting  
**Document kind:** Compact local-tool planning handoff  
**Status:** Implemented for the first `matten-data` readiness slice; future report families deferred  
**Scope:** Local-only `tools/matten-report` planning and first-slice authorization  

---

## 1. Scope

This handoff defines the implementation boundary for the first local reporting
tool slice accepted by review.

Proposed tool name:

```text
tools/matten-report
```

Allowed for the first implementation:

```text
publish = false
workspace-excluded
local command-line tool
plain text / Markdown output first
small deterministic demo inputs
explicit input paths
explicit output path or stdout
existing matten workspace crates
local helper code inside the tool
matten-data readiness report only
std::env::args parsing only
```

Not authorized in the first implementation:

```text
public API
published crate
workspace dependency changes for published crates
core matten dependency additions
Tensor::plot
Tensor::show
automatic expression tracing
lazy expression graph
autograd / backward
SVG output
HTML output
Vega-Lite JSON
Plotters or other plotting stacks
notebook integration
GUI / dashboard
telemetry
network access
automatic project mutation
shape report
dynamic readiness report
matten-mlprep standardization report
JSON output
CLI parser dependency
workspace membership
```

## 2. Tool Placement

Accepted placement:

```text
tools/matten-report/
```

Accepted Cargo policy:

```text
publish = false
excluded from workspace initially
```

Rationale:

```text
keeps published crates dependency-light
keeps reporting/tooling outside core matten
matches the benchmark/local-tool isolation pattern
allows experimentation without public API commitment
```

If implementation later needs the tool inside the workspace for CI ergonomics,
that must be called out explicitly in the implementation review and must not add
dependencies to published crates.

## 3. First Output Format

Initial output format should be Markdown/plain text only.

Allowed:

```text
stdout report
single .md output file
small aligned ASCII tables
short titled sections
fixed row/column order
```

Not allowed in the first implementation:

```text
HTML
SVG
Vega-Lite JSON
JSON
images
ANSI color
terminal-width-dependent layout
large generated artifacts
```

Hard output rules:

```text
no timestamps
no random data
no environment-specific absolute paths except the explicit input path if shown
no terminal-width-dependent wrapping
no ANSI color
no Unicode block charts
no unordered map iteration in output
```

Use fixed ordering:

```text
source column order
selected column order
stable error ordering
stable section order
```

## 4. First Report Targets

The first implementation must include only:

```text
matten-data readiness report
```

Deferred to later handoffs:

```text
shape report
dynamic readiness report
matten-mlprep standardization report
integration with migration reports
project scanning
```

### 4.1 `matten-data` Readiness Report

Should summarize:

```text
source columns
selected columns
columns intentionally left out
missing counts
numeric conversion result
Tensor shape and row-major values, if conversion succeeds
```

This must remain table-to-Tensor preparation, not dataframe reporting.

Required sections:

```text
# matten data-readiness report

## Input
path or demo label

## Source columns
column names in source order

## Selected columns
selected column names in report order

## Columns left out
source columns not selected

## Missing values
missing count per selected column

## Numeric conversion
strict conversion result: success or specific error

## Tensor preview
only if conversion succeeds: shape and row-major values, if small
```

Do not include:

```text
data quality score
clean / dirty judgment
recommendations to migrate
automatic fixes
statistics beyond simple missing/numeric readiness
group-by/join/pivot/query
large histogram
correlation/covariance
model-readiness claim
```

## 5. Input / Output Contract

The first implementation should use explicit CLI flags.

Accepted command shape:

```text
matten-report --demo data-readiness
matten-report --input examples/data/small.csv --kind data-readiness --select sales,cost
matten-report --input examples/data/small.csv --kind data-readiness --select sales,cost --output target/report.md
```

Accepted output destinations:

```text
stdout
explicit output file path
```

Selection policy:

```text
input mode requires --select for the first implementation
--select uses comma-separated column names in requested order
demo mode uses hard-coded selection
--kind remains required for input mode even though only data-readiness exists
```

Not allowed:

```text
modifying input files
rewriting project files
creating output next to input by default
walking the repository automatically
reading environment secrets
network calls
telemetry
```

If no input file is provided, a demo mode may use hard-coded data, but it must be
clearly labeled as demo input. Do not guess selected columns silently.

## 6. Dependency Policy

Allowed:

```text
standard library
existing matten workspace crates
```

First implementation:

```text
std::env::args only
no new dependency
```

Not allowed without later review:

```text
serde_json for visualization formats
Plotters
HTML template engines
terminal UI crates
clap / argh / pico-args or other CLI parser dependencies
dataframe libraries
ML frameworks
browser automation
```

Tool `Cargo.toml` should use path-only dependencies:

```toml
[package]
publish = false

[dependencies]
matten = { path = "../../crates/matten", default-features = false }
matten-data = { path = "../../crates/matten-data", features = ["csv"] }
```

Because the tool is workspace-excluded, do not rely on workspace-inherited
dependency declarations. Because the tool is `publish = false`, path-only local
dependencies are preferred over path-plus-version constraints; the local compile
and test gates catch API drift without creating a prerelease version-sync chore.
Lockfile policy must be explicit: either commit a tool lockfile if that becomes
the local-tool policy, or ignore it intentionally.

## 7. Security And Privacy Boundary

The tool is local-only.

It must not:

```text
use network APIs
read environment secrets
scan directories unless explicitly provided a path
mutate input files
rewrite project files
create files except the explicit --output path
create files by default
```

## 8. Documentation

If implemented, documentation should be minimal:

```text
tools/matten-report/README.md
docs/src/examples or docs/src/reference link only if useful
release checklist command if CI runs the tool
```

Do not promote it as a public product or library.

Preferred wording:

```text
local report tool
plain text report
data-readiness summary
```

Avoid:

```text
visualization framework
dashboard
plotting
production reporting
automatic analysis
data quality score
production report
```

## 9. CI And Verification

If implemented, suggested checks:

```bash
cargo fmt --check
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --output target/matten-report-demo.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input tools/matten-report/fixtures/small.csv \
  --kind data-readiness \
  --select sales,cost
bash scripts/check-release-docs.sh
```

If the tool is workspace-excluded, CI must call it by manifest path.

If the tool writes an output file during tests, write only under `target/` or a
temporary directory and remove any committed generated artifact from scope.

## 10. Acceptance Checklist

```text
[x] tool stays under tools/matten-report
[x] publish = false
[x] workspace-excluded
[x] no published crate dependency changes
[x] no public API changes
[x] first implementation includes only data-readiness report
[x] std::env::args only; no external CLI parser dependency
[x] --demo data-readiness is clearly labeled demo input
[x] input mode uses explicit --input, --kind data-readiness, and --select
[x] --output writes only to the provided path
[x] no files are created by default
[x] output is Markdown/plain text only
[x] no image/SVG/HTML/Vega-Lite output
[x] no JSON output
[x] no plotting/notebook/GUI scope
[x] no telemetry or network access
[x] no automatic project mutation
[x] explicit input/output behavior documented
[x] reports explain without judging data/model quality
[x] CI runs by manifest path if workspace-excluded
[x] no generated report artifacts checked in
[x] tool README states local-only / publish=false / not public API
[x] release-documentation guard passes
```

Observed implementation verification:

```bash
cargo fmt --check --manifest-path tools/matten-report/Cargo.toml
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --output target/matten-report-demo.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost
bash scripts/check-release-docs.sh
mdbook build . --dest-dir ../target/mdbook-phase3-tool-check
```

## 11. Deferred Work

```text
shape report
dynamic readiness report
matten-mlprep standardization report
SVG output
HTML output
Vega-Lite JSON
JSON output
CLI parser dependency
workspace membership
published crate
public report API
integration with migration reports
project scanning
```
