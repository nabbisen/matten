# RFC-063 Phase 2 Example Reports Handoff

**Project:** `matten`  
**Related RFC:** RFC-063: Visual Understanding and Reporting  
**Document kind:** Compact examples implementation handoff  
**Status:** Implemented for the canonical example set in `0.29.0-pre.2`; retained as implementation record  
**Scope:** Example-only terminal / Markdown-style summaries  

---

## 1. Scope

Implement the next RFC-063 step as runnable examples that print small,
deterministic, terminal-friendly summaries.

Allowed:

```text
local helper functions inside examples
plain text output
Markdown-like tables printed by examples
small hard-coded data
existing workspace crates only
CI smoke runs for canonical examples
```

Do not add:

```text
public APIs
crate dependencies
plotting dependencies
image assets
generated artifacts
SVG output
Vega-Lite JSON
HTML output
local tools
automatic expression tracing
lazy expression graphs
```

## 2. Canonical examples

Implement these first, in order:

```text
crates/matten/examples/57_visual_shape_axis_summary.rs
crates/matten/examples/dynamic_09_visual_readiness_summary.rs
crates/matten-data/examples/data_06_visual_readiness_summary.rs
```

These are canonical because they cover the accepted priority: shape/axis
understanding and data-readiness understanding, without adding APIs or tooling.
They are the full first implementation set; do not add more examples without a
separate review request.

Name-collision check before implementation:

```text
current core numeric examples end at 56_...
current dynamic examples end at dynamic_08_...
current matten-data examples end at data_05_...
```

If the current sequence changes before implementation, use the next available
number while preserving the same grouping.

### 2.1 Core shape / axis summary

`57_visual_shape_axis_summary.rs` should show:

```text
input shape
operation name
axis meaning when relevant
output shape
small output values
```

Required operations:

```text
broadcast add
reshape or flatten
mean_axis(0)
mean_axis(1)
matmul
```

Keep helper functions local to the example. The output must be stable enough for
a reviewer to compare by eye.

Recommended structure:

```text
== Broadcasting ==
input A shape: [2, 3]
input b shape: [3]
result shape: [2, 3]
meaning: b repeats across rows

== Axis reductions ==
mean_axis(0): collapse rows, keep columns -> shape [3]
mean_axis(1): collapse columns, keep rows -> shape [2]
```

Keep this as an example, not a full tutorial page.

### 2.2 Dynamic readiness summary

`dynamic_09_visual_readiness_summary.rs` should show:

```text
dynamic values
none / missing mask
numeric-convertible mask
strict conversion result
explicit policy conversion result
```

The example should follow the existing dynamic on-ramp pattern:

```text
cfg(feature = "dynamic") real example
cfg(not(feature = "dynamic")) fallback main
```

If it reads CSV or JSON, gate that path the same way existing dynamic examples
do. Prefer an in-memory construction if it keeps the example smaller.

Recommended policy comparison:

```text
strict conversion rejects None / Text
explicit policy uses none_as(0.0)
text parsing is shown only for text values that actually parse as f64
```

Do not imply labels such as `"active"` become numeric under a permissive policy.

### 2.3 `matten-data` readiness summary

`data_06_visual_readiness_summary.rs` should show:

```text
source columns
selected columns
columns intentionally left out
missing counts
numeric conversion result
Tensor shape and row-major values
```

If the example uses CSV constructors, add the matching `[[example]]` entry with
`required-features = ["csv"]` in `crates/matten-data/Cargo.toml`.

## 3. Optional follow-up

After the canonical examples pass review, a later small handoff or explicit owner
approval may add:

```text
crates/matten-mlprep/examples/visual_standardize_summary.rs
```

That example should compare before/after column mean and standard deviation for
`standardize_columns`. It is not part of the first Phase 2 implementation set.

## 4. Output rules

Reports explain; they do not judge.

Allowed:

```text
axis 0 collapses rows and returns one value per column
column "age" has one missing value
column "city" is left out before numeric conversion
shape [2, 3] matmul [3, 2] -> [2, 2]
```

Avoid:

```text
this dataset is clean
this operation is optimal
this model is good
this library is faster
this migration is required
```

Output should remain compact. Prefer a few short sections over a large report.

Canonical output must not include:

```text
timestamps
random data
environment-dependent paths
terminal-width-dependent layout
unordered map iteration
ANSI color escapes
Unicode blocks that render inconsistently
```

Prefer:

```text
plain ASCII
short titled sections
small aligned tables
fixed input values
fixed row/column order
explicit shape lines
```

## 5. Documentation updates

If the examples are implemented, update only the relevant example indexes:

```text
docs/src/examples/index.md
docs/src/examples/data.md
docs/src/tutorial/start-here.md
```

Update `docs/src/examples/companions.md` only if omitting the new companion
example would make the existing companion-example list confusing.

Do not introduce current user-facing "Phase 1" / "Phase 2" wording in docs; the
release guard treats that vocabulary as RFC/process-only.

Use wording such as:

```text
visual examples
readability examples
shape and readiness summaries
```

## 6. CI and release gates

When the examples are implemented, update smoke coverage:

```text
.github/workflows/test.yaml
docs/src/contributing/release-checklist.md
```

Canonical smoke commands:

```bash
cargo run -p matten --example 57_visual_shape_axis_summary
cargo check -p matten --example dynamic_09_visual_readiness_summary
cargo run -p matten --example dynamic_09_visual_readiness_summary --features dynamic
cargo run -p matten-data --example data_06_visual_readiness_summary
```

The `matten-data` smoke command is acceptable while `csv` remains a default
feature. If the smoke job runs with `--no-default-features`, use:

```bash
cargo run -p matten-data --features csv --example data_06_visual_readiness_summary
```

Do not add exact stdout snapshot tests unless the project first adopts a stable
example-output testing convention.

Suggested verification:

```bash
cargo fmt --check
cargo check --workspace --examples --all-features
cargo check -p matten --example dynamic_09_visual_readiness_summary
cargo run -p matten --example 57_visual_shape_axis_summary
cargo run -p matten --example dynamic_09_visual_readiness_summary --features dynamic
cargo run -p matten-data --example data_06_visual_readiness_summary
mdbook build . --dest-dir ../target/mdbook-phase2-check   # from docs/
bash scripts/check-release-docs.sh
```

## 7. Acceptance checklist

```text
[x] helper functions stay inside examples
[x] canonical examples are the only Phase 2 examples in the first implementation set
[x] no public API change
[x] no Cargo.toml dependency additions
[x] no plotting / notebook / GUI positioning
[x] no image assets or generated artifacts
[x] output is deterministic and small
[x] output contains no timestamps, randomness, environment paths, or terminal-width dependence
[x] canonical examples compile
[x] canonical examples run in CI smoke
[x] dynamic example compiles without the dynamic feature and runs with it
[x] data example has required-features = ["csv"] if it uses CSV constructors
[x] docs update only necessary index/link pages
[x] docs index the new examples without process-phase wording
[x] no user-facing "Phase 1" / "Phase 2" wording
[x] release-documentation guard passes
[x] mdBook builds
```

Observed implementation verification:

```bash
cargo fmt --check
cargo check --workspace --examples --all-features
cargo check -p matten --example dynamic_09_visual_readiness_summary
cargo run -p matten --example 57_visual_shape_axis_summary
cargo run -p matten --example dynamic_09_visual_readiness_summary --features dynamic
cargo run -p matten-data --example data_06_visual_readiness_summary
mdbook build . --dest-dir ../target/mdbook-phase2-check   # from docs/
bash scripts/check-release-docs.sh
```
