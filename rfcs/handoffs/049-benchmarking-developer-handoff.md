# RFC-049 Benchmarking Program — Developer Handoff

**Project:** `matten`  
**Related RFC:** RFC-049: Benchmarking, Complexity Metrics, and Positioning Report  
**Document Kind:** Detailed Implementation Handoff / PR Plan / QA Checklist  
**Status:** Draft for developer execution  
**Target:** v0.20.x planning follow-up / v0.21+ maturity hardening  
**Scope:** Benchmark harness, benchmark documentation, ELOC tooling, reporting, and optional peer/reference comparisons  

---

## 0. Executive Summary

Implement RFC-049 as a **benchmarking and positioning program**, not as a performance-marketing project.

The goal is to create reproducible evidence for:

```text
execution time
memory behavior where practical
example-code ELOC
dependency footprint
regression visibility
honest positioning
```

The goal is **not** to claim that `matten` is faster than or replaces:

```text
ndarray
nalgebra
NumPy
SciPy
Pandas
Candle
```

The first implementation should focus on:

```text
1. methodology docs
2. internal Rust benchmark harness
3. selected scenario benchmarks
4. selected companion benchmarks
5. ELOC script
6. first positioning report template
```

Peer/reference comparisons can follow after the internal baseline is stable.

---

## 1. Implementation Defaults

RFC-049 leaves several practical questions open. Use the following defaults unless maintainers decide otherwise.

### 1.1 Rust benchmark harness

Use:

```text
criterion
```

Reason:

- common Rust benchmark ecosystem;
- stable enough for project-level benchmarking;
- supports statistical measurements;
- familiar to Rust developers.

Do **not** add Criterion as a normal dependency of core `matten`.

### 1.2 Repository location

Recommended initial layout:

```text
benchmarks/
  README.md
  Cargo.toml
  src/
    lib.rs
    common.rs
    workloads/
      core.rs
      scenarios.rs
      companions.rs
  benches/
    core_construction.rs
    core_elementwise.rs
    core_reductions.rs
    core_matmul.rs
    scenarios.rs
    companions.rs
  scripts/
    collect_eloc.py
    summarize.py
    run_internal.sh
    run_rust_peers.sh
    run_references.sh
  reports/
    README.md
    positioning-v0.1.md
  results/
    README.md
```

Use a dedicated benchmark package:

```toml
[package]
name = "matten-benchmarks"
version.workspace = true
edition.workspace = true
publish = false
```

This keeps benchmark dependencies isolated from normal `matten` users.

### 1.3 Workspace membership

Preferred:

```text
Add `benchmarks` as a workspace member only if CI and release scripts can tolerate
the extra dev-only package.
```

If adding as a workspace member causes release friction, keep it as a standalone manifest and run it explicitly:

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
cargo bench --manifest-path benchmarks/Cargo.toml
```

Default recommendation for first implementation:

```text
Use `benchmarks/Cargo.toml` as a standalone package not published to crates.io.
Do not rely on normal `cargo test --workspace` to exercise it.
```

This minimizes risk to the existing workspace gates.

### 1.4 Raw results policy

Commit:

```text
benchmark methodology docs
report templates
small sample reports
scripts
harness code
```

Do not commit:

```text
large raw Criterion output histories
machine-specific bulky result directories
```

If committing raw JSON summaries, keep them small and curated:

```text
benchmarks/results/internal-baseline.sample.json
```

### 1.5 Memory measurement

Phase 1 default:

```text
Document memory methodology.
Add optional peak-RSS collection if easy on the maintainer environment.
Do not block Phase 1 on allocator-level measurements.
```

---

## 2. Non-negotiable Rules

The implementation must preserve these rules.

### 2.1 No core dependency pollution

Core `matten` must not gain normal dependencies on:

```text
criterion
ndarray
nalgebra
numpy
pandas
any benchmark-only crate
```

The dependency-boundary script must still pass.

### 2.2 No network

Benchmark scripts must not download datasets or fetch remote resources.

All benchmark data must be:

```text
generated locally
small
deterministic
checked into the repo only if appropriate
```

### 2.3 No marketing claims

Reports must not say:

```text
matten beats NumPy
matten replaces ndarray
matten is faster than Pandas
```

Use tradeoff language:

```text
matten is fast enough for this small PoC workflow
matten uses fewer dependencies for this task
ndarray/nalgebra/NumPy/Pandas are better choices for broader ecosystem needs
```

### 2.4 No hard performance gates initially

CI may fail if:

```text
benchmark harness does not compile
scripts fail syntactically
report schema is invalid
```

CI must not fail because:

```text
a benchmark is slower than previous run
```

until a later regression-threshold policy is accepted.

---

## 3. Proposed File Layout

Add:

```text
benchmarks/
  README.md
  Cargo.toml
  src/
    lib.rs
    common.rs
    workloads/
      mod.rs
      core.rs
      scenarios.rs
      companions.rs
  benches/
    core_construction.rs
    core_elementwise.rs
    core_reductions.rs
    core_matmul.rs
    scenarios.rs
    companions.rs
  scripts/
    collect_eloc.py
    summarize.py
    run_internal.sh
  reports/
    README.md
    positioning-v0.1.md
  results/
    README.md
```

Optional later:

```text
benchmarks/peers/
  ndarray/
  nalgebra/

benchmarks/references/
  numpy/
  pandas/

benchmarks/scripts/
  run_rust_peers.sh
  run_references.sh
```

Docs:

```text
docs/src/benchmarks/index.md
docs/src/benchmarks/methodology.md
docs/src/benchmarks/reports.md
```

Update docs navigation if the project uses mdBook `SUMMARY.md`.

---

## 4. Benchmark Package Manifest

Suggested `benchmarks/Cargo.toml`:

```toml
[package]
name = "matten-benchmarks"
version.workspace = true
edition.workspace = true
publish = false

[dependencies]
matten = { path = "../crates/matten", features = ["dynamic"] }
matten-ndarray = { path = "../crates/matten-ndarray" }
matten-mlprep = { path = "../crates/matten-mlprep" }
matten-data = { path = "../crates/matten-data" }

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "core_construction"
harness = false

[[bench]]
name = "core_elementwise"
harness = false

[[bench]]
name = "core_reductions"
harness = false

[[bench]]
name = "core_matmul"
harness = false

[[bench]]
name = "scenarios"
harness = false

[[bench]]
name = "companions"
harness = false
```

Adjust paths if repository layout differs.

If `version.workspace` / `edition.workspace` is not available from this package due to standalone manifest constraints, use explicit values matching the workspace.

---

## 5. Common Benchmark Helpers

Create:

```text
benchmarks/src/common.rs
```

Suggested responsibilities:

```rust
pub fn vector_f64(len: usize) -> Vec<f64>;
pub fn matrix_f64(rows: usize, cols: usize) -> Vec<f64>;
pub fn tiny_matrix_f64(rows: usize, cols: usize) -> Vec<f64>;
pub fn assert_close(a: f64, b: f64, tolerance: f64);
pub fn black_box_tensor<T>(value: T) -> T;
```

Keep helpers small and deterministic.

Do not use randomness in Phase 1.

If randomness is needed later, use fixed seed and document it.

---

## 6. Workload Module Design

### 6.1 Core workloads

Create:

```text
benchmarks/src/workloads/core.rs
```

Functions should separate setup from operation.

Example pattern:

```rust
pub fn make_vector_tensor(len: usize) -> Tensor {
    Tensor::from_vec(vec![1.0; len], vec![len]).expect("valid shape")
}

pub fn elementwise_add_workload(len: usize) -> Tensor {
    let a = make_vector_tensor(len);
    let b = make_vector_tensor(len);
    &a + &b
}
```

The actual API names must match the current repo.

Do not put `println!` inside measured bodies.

### 6.2 Scenario workloads

Create:

```text
benchmarks/src/workloads/scenarios.rs
```

Initial scenarios:

```text
cosine similarity, using existing example logic where possible
Markov chain weather
tiny PageRank
linear regression gradient descent
heat equation 1D
```

Rules:

```text
correctness checks outside timed section
small deterministic inputs
fixed iteration counts
no output formatting in measured section
```

### 6.3 Companion workloads

Create:

```text
benchmarks/src/workloads/companions.rs
```

Initial companion workloads:

```text
ndarray roundtrip
mlprep standardize_columns
mlprep train_test_split
matten-data csv_to_tensor
```

Rules:

```text
do not benchmark file I/O unless that is the explicit workload
prefer from_csv_str for stable data-ingestion baseline
separate parsing from conversion if useful
```

---

## 7. Criterion Bench Files

### 7.1 `core_construction.rs`

Benchmark:

```text
construct vector 100
construct vector 10_000
construct matrix 100x100
```

### 7.2 `core_elementwise.rs`

Benchmark:

```text
elementwise add
elementwise multiply
scalar add
scalar multiply
broadcast vector over matrix
```

Sizes:

```text
tiny: 100
small: 10_000
medium: 100_000, only if fast enough
```

### 7.3 `core_reductions.rs`

Benchmark:

```text
sum
mean
sum_axis
mean_axis
```

Use matrix shapes:

```text
100x10
1000x10
100x100
```

### 7.4 `core_matmul.rs`

Benchmark:

```text
matmul 16x16
matmul 64x64
matmul 128x128
```

Do not begin with huge matrix sizes.

### 7.5 `scenarios.rs`

Benchmark:

```text
cosine similarity
Markov chain N iterations
PageRank N iterations
linear regression N iterations
heat equation N steps
```

Use fixed N values.

Recommended first N:

```text
Markov chain: 100 steps
PageRank: 50 iterations
linear regression: 200 iterations
heat equation: 200 steps
```

Tune if runtime is too high.

### 7.6 `companions.rs`

Benchmark:

```text
matten-ndarray to_arrayd
matten-ndarray from_arrayd
matten-ndarray roundtrip
matten-mlprep standardize_columns
matten-mlprep train_test_split
matten-data from_csv_str
matten-data csv_to_tensor workflow
```

---

## 8. ELOC Tool

Create:

```text
benchmarks/scripts/collect_eloc.py
```

or a Rust equivalent if the project prefers no Python helper.

Python is acceptable if:

```text
it is optional
it is not required for normal Rust CI
it has no third-party dependency
```

### 8.1 Input

The script should accept file paths:

```bash
python benchmarks/scripts/collect_eloc.py \
  crates/matten/examples/26_cosine_similarity.rs \
  crates/matten/examples/33_markov_chain_weather.rs
```

### 8.2 Counting rules

Count:

```text
non-empty
non-comment
solution lines
```

Ignore:

```text
blank lines
line comments
doc comments
module-level explanatory comments
```

Optional flags:

```text
--exclude-imports
--json
--markdown
```

### 8.3 Output

Markdown example:

```text
| File | ELOC | ELOC excluding imports |
|---|---:|---:|
| 26_cosine_similarity.rs | 34 | 31 |
| 33_markov_chain_weather.rs | 48 | 45 |
```

JSON example:

```json
{
  "files": [
    {
      "path": "crates/matten/examples/26_cosine_similarity.rs",
      "eloc": 34,
      "eloc_excluding_imports": 31
    }
  ]
}
```

### 8.4 Acceptance

```text
[ ] works without third-party Python packages
[ ] documented rules
[ ] stable output
[ ] does not fail on Rust doc comments
[ ] can exclude imports
```

---

## 9. Report Template

Create:

```text
benchmarks/reports/positioning-v0.1.md
```

Initial content can be a template with placeholders.

Required sections:

```text
1. Purpose
2. Environment
3. Versions
4. What is measured
5. What is intentionally not measured
6. Internal baseline
7. Rust peer comparison, if applicable
8. Reference comparison, if applicable
9. Example ELOC
10. Memory notes
11. Interpretation
12. Positioning conclusion
13. Known limitations
```

### 9.1 Environment block

Template:

```text
OS:
CPU:
RAM:
rustc:
cargo:
target:
profile:
matten version:
git commit:
date:
```

Python reference section only if used:

```text
python:
numpy:
pandas:
```

### 9.2 Interpretation rules

Include this text or equivalent:

```text
These results are workload-specific and environment-specific.
They are intended for positioning and regression visibility.
They are not universal rankings and must not be read as replacement claims.
```

---

## 10. Documentation Updates

Add:

```text
docs/src/benchmarks/index.md
docs/src/benchmarks/methodology.md
docs/src/benchmarks/reports.md
```

### 10.1 `index.md`

Explain:

```text
why benchmarks exist
what they measure
what they do not claim
where reports live
```

### 10.2 `methodology.md`

Explain:

```text
execution time
memory measurement
ELOC
dependency footprint
peer vs reference comparisons
```

### 10.3 `reports.md`

Link to:

```text
benchmarks/reports/positioning-v0.1.md
```

or embed summaries if docs tooling supports it.

---

## 11. PR Plan

## PR-049-1: Methodology docs and benchmark skeleton

### Scope

Add:

```text
benchmarks/README.md
benchmarks/Cargo.toml
benchmarks/reports/README.md
benchmarks/results/README.md
docs/src/benchmarks/index.md
docs/src/benchmarks/methodology.md
docs/src/benchmarks/reports.md
```

### Acceptance

```text
[ ] benchmark scope documented
[ ] non-goals documented
[ ] no benchmark numbers displayed as claims
[ ] no core dependency change
[ ] docs navigation updated
```

### Suggested checks

```bash
cargo check --manifest-path benchmarks/Cargo.toml
bash scripts/check-core-dependency-boundary.sh
```

---

## PR-049-2: Internal microbenchmark harness

### Scope

Add:

```text
benchmarks/src/common.rs
benchmarks/src/workloads/core.rs
benchmarks/benches/core_construction.rs
benchmarks/benches/core_elementwise.rs
benchmarks/benches/core_reductions.rs
benchmarks/benches/core_matmul.rs
```

### Acceptance

```text
[ ] criterion benches compile
[ ] setup outside timed loop where practical
[ ] deterministic inputs
[ ] no printing in timed body
[ ] no large default workloads
```

### Suggested checks

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
```

Optional local run:

```bash
cargo bench --manifest-path benchmarks/Cargo.toml core_construction
```

---

## PR-049-3: Scenario benchmark harness

### Scope

Add:

```text
benchmarks/src/workloads/scenarios.rs
benchmarks/benches/scenarios.rs
```

Initial scenarios:

```text
existing cosine similarity
33_markov_chain_weather-style workload
34_tiny_pagerank-style workload
35_linear_regression_gradient_descent-style workload
36_heat_equation_1d-style workload
```

Use scenario logic directly, not by shelling out to examples.

### Acceptance

```text
[ ] each scenario has correctness check outside timed body
[ ] fixed iteration counts
[ ] no random data
[ ] no output formatting in measured body
```

---

## PR-049-4: Companion benchmark harness

### Scope

Add:

```text
benchmarks/src/workloads/companions.rs
benchmarks/benches/companions.rs
```

Initial workloads:

```text
matten-ndarray to_arrayd
matten-ndarray from_arrayd
matten-ndarray roundtrip
matten-mlprep standardize_columns
matten-mlprep train_test_split
matten-data from_csv_str
matten-data csv_to_tensor flow
```

### Acceptance

```text
[ ] companion benchmarks compile
[ ] no file I/O unless explicit
[ ] from_csv_str used for stable CSV parsing benchmark
[ ] no core dependency pollution
```

---

## PR-049-5: ELOC script

### Scope

Add:

```text
benchmarks/scripts/collect_eloc.py
```

or equivalent Rust tool.

### Acceptance

```text
[ ] counts non-comment, non-blank lines
[ ] can exclude imports
[ ] markdown output
[ ] json output optional
[ ] no external Python package required
```

### Suggested checks

```bash
python benchmarks/scripts/collect_eloc.py --help
python benchmarks/scripts/collect_eloc.py crates/matten/examples/26_cosine_similarity.rs
```

---

## PR-049-6: First internal positioning report

### Scope

Add:

```text
benchmarks/reports/positioning-v0.1.md
```

Fill with:

```text
methodology
environment
initial internal baseline table, if measured
ELOC table, if measured
limitations
positioning interpretation
```

### Acceptance

```text
[ ] no winner/loser language
[ ] no ecosystem replacement claim
[ ] all numbers include environment
[ ] limitations clearly stated
```

---

## PR-049-7: Rust peer comparison, optional after baseline

### Scope

Add benchmark-only peer implementations for:

```text
ndarray
nalgebra
```

Tasks:

```text
cosine similarity
matrix multiplication
Markov chain
PageRank
linear regression
heat equation
```

### Acceptance

```text
[ ] peer dependencies are benchmark-only
[ ] comparison tasks are equivalent enough
[ ] versions and features recorded
[ ] report says Rust peer comparison
```

---

## PR-049-8: Ecosystem reference comparison, optional

### Scope

Add optional scripts for:

```text
NumPy
Pandas
```

NumPy tasks:

```text
cosine similarity
Markov chain
PageRank
linear regression
heat equation
```

Pandas task:

```text
CSV -> select columns -> fill missing -> numeric matrix
```

### Acceptance

```text
[ ] Python is not required for ordinary Rust CI
[ ] no network access
[ ] versions recorded
[ ] report says reference comparison
[ ] no SciPy/Candle added
```

---

## 12. CI Integration

### 12.1 Required CI check

Add a compile-only benchmark check if runtime is acceptable:

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
```

If Criterion compile time is too high for normal CI, run it only in a benchmark workflow or manually.

### 12.2 Optional benchmark workflow

Create:

```text
.github/workflows/benchmarks.yaml
```

Trigger:

```yaml
on:
  workflow_dispatch:
  schedule:
    - cron: "0 0 * * 0"
```

Initial job:

```text
cargo bench --manifest-path benchmarks/Cargo.toml -- --noplot
```

Do not make scheduled benchmark failure block normal release unless harness fails to compile.

### 12.3 Artifacts

Optional workflow artifact:

```text
target/criterion
benchmarks/reports/generated
```

Avoid committing large artifacts automatically.

---

## 13. QA Checklist

Before merging RFC-049 implementation:

```text
[ ] core dependency-boundary check still passes
[ ] no benchmark-only dependency in crates/matten/Cargo.toml
[ ] benchmarks compile
[ ] docs explain purpose and non-goals
[ ] ELOC methodology documented
[ ] report template exists
[ ] no Python required for normal Rust build/test
[ ] no network access
[ ] no external datasets
[ ] no hard performance gate
[ ] no marketing claims
```

Run:

```bash
cargo fmt --all --check
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-release-docs.sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
```

If `benchmarks` is not a workspace member, also run:

```bash
cargo clippy --manifest-path benchmarks/Cargo.toml --all-targets -- -D warnings
```

---

## 14. Review Checklist

Reviewers should ask:

```text
[ ] Is the benchmark measuring an accepted `matten` scope?
[ ] Is setup excluded from the measured body where appropriate?
[ ] Is output formatting excluded from the measured body?
[ ] Are versions and features recorded?
[ ] Are benchmark-only dependencies isolated?
[ ] Does the report avoid replacement claims?
[ ] Is ELOC measured consistently?
[ ] Does the benchmark use deterministic data?
[ ] Is memory methodology honest?
[ ] Is the result useful for positioning or regression tracking?
```

---

## 15. Risks and Mitigations

### Risk: benchmark dependency pollution

Mitigation:

```text
benchmark package only
publish = false
dependency-boundary check
no core Cargo.toml changes
```

### Risk: misleading comparisons

Mitigation:

```text
peer vs reference terminology
report limitations
no winner/loser language
review checklist
```

### Risk: flaky performance numbers

Mitigation:

```text
no hard speed gate initially
median-focused reporting
manual interpretation
environment metadata
```

### Risk: over-scoping into SciPy/Pandas/Candle territory

Mitigation:

```text
Phase 1 internal only
Phase 2 Rust peers only
Phase 3 NumPy/Pandas reference only
SciPy/Candle deferred
```

### Risk: benchmark maintenance burden

Mitigation:

```text
small workload set
scenario alignment with examples
no huge datasets
no heavy results history
```

---

## 16. Definition of Done

RFC-049 initial implementation is done when:

```text
[ ] methodology docs exist
[ ] benchmark harness compiles
[ ] internal microbenchmarks exist
[ ] scenario benchmarks exist
[ ] companion benchmarks exist where feasible
[ ] ELOC script exists
[ ] first report template exists
[ ] no normal dependency pollution
[ ] no marketing claims
[ ] CI can compile-check benchmarks or has a manual benchmark workflow
```

Full RFC-049 program is mature when:

```text
[ ] internal baseline has multiple historical runs
[ ] Rust peer comparison is available
[ ] optional ecosystem reference comparison is available
[ ] reports include interpretation and limitations
[ ] regression policy is proposed from actual history
```

---

## 17. Final Notes to Developers

Treat benchmarks as a mirror, not a scoreboard.

Good benchmark outcome:

```text
matten is clearly positioned.
users understand tradeoffs.
maintainers can see regressions.
scope stays honest.
```

Bad benchmark outcome:

```text
matten starts chasing NumPy/Pandas/SciPy/Candle.
benchmark numbers become marketing.
benchmark dependencies leak into core.
CI becomes flaky due to performance variance.
```

Keep the first implementation small and boring. That is the right shape for `matten`.
