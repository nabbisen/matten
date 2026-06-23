# RFC-049: Benchmarking, Complexity Metrics, and Positioning Report

**Status:** Proposed  
**Target Release:** v0.20.x planning / v0.21+ maturity hardening  
**Owner:** `matten` maintainers  
**Related:** RFC-030, RFC-033–048, RFC-038, RFC-040, RFC-041  
**Scope:** Benchmark harness, reproducible measurement policy, complexity metrics, positioning report  
**Crates Affected:** `matten`, `matten-ndarray`, `matten-mlprep`, `matten-data`  
**Non-API RFC:** This RFC does not add new public runtime APIs to core `matten`.

---

## 1. Summary

This RFC introduces a modest, reproducible benchmarking and positioning program for `matten`.

The purpose is not to prove that `matten` is faster than larger ecosystems. The purpose is to clarify `matten`'s position with evidence:

```text
execution time
memory behavior
example-code ELOC
dependency footprint
implementation simplicity
regression trends
```

The benchmark program should answer:

```text
What is matten good at?
Where is it intentionally simpler?
Where is it slower but acceptable?
Where would performance become a blocker?
How much code does a user write to solve small problems?
```

The benchmark program must not create a false product promise that `matten` is a replacement for:

```text
ndarray
nalgebra
NumPy
SciPy
Pandas
Candle
```

Instead, the benchmark report should show that `matten` is a small, approachable, Tensor-centered Rust numeric crate for PoC, learning, and small workflows.

---

## 2. Motivation

`matten` now has:

- stable core tensor functionality;
- companion crate boundaries;
- small data-to-Tensor workflows through `matten-data`;
- examples for famous small math / numerical-computing problems;
- future comfort API proposals.

As the project grows, users and maintainers need evidence for three questions:

1. Is `matten` fast enough for its intended workflows?
2. Are examples and APIs genuinely simpler for small tasks?
3. Are regressions visible before they become release-quality issues?

Benchmarking is therefore useful for positioning, maturity hardening, regression detection, documentation honesty, and release confidence.

But benchmarking is dangerous if it becomes a marketing contest. A simplistic "matten vs NumPy" or "matten vs ndarray" leaderboard would be misleading and would damage the project's identity.

This RFC defines a safe benchmarking program.

---

## 3. Product Position

`matten` should be benchmarked as:

> A small, Tensor-centered Rust numeric crate for readable PoC and small numerical workflows.

It should not be benchmarked as:

- a NumPy replacement;
- a SciPy replacement;
- a Pandas replacement;
- an ML tensor backend;
- a BLAS/LAPACK engine;
- a dataframe engine;
- a GPU framework.

The benchmark conclusion must be phrased in terms of tradeoffs:

```text
matten is small and approachable.
ndarray is broader and more mature for Rust N-D arrays.
nalgebra is stronger for linear algebra structures.
NumPy/SciPy/Pandas are mature Python scientific/data ecosystems.
Candle is for ML tensor/model workflows.
```

---

## 4. Goals

The benchmarking program SHOULD:

1. Establish a reproducible internal baseline for `matten`.
2. Measure representative core tensor operations.
3. Measure representative examples from RFC-043–048.
4. Measure selected companion workflows.
5. Compare selected Rust peer tasks with `ndarray` and `nalgebra`.
6. Optionally compare selected reference tasks with NumPy and Pandas.
7. Measure example-code ELOC for approachability.
8. Record memory behavior where feasible.
9. Produce a human-readable positioning report.
10. Avoid misleading claims.
11. Avoid hard performance gates until enough baseline history exists.

---

## 5. Non-goals

This RFC MUST NOT introduce:

- performance marketing claims;
- a claim that `matten` is faster than NumPy;
- a claim that `matten` replaces ndarray/nalgebra;
- SciPy clone benchmarks;
- Pandas dataframe benchmark suites;
- Candle / ML tensor backend benchmarks in Phase 1;
- GPU benchmarks;
- large CSV / streaming benchmarks;
- hard CI speed-fail thresholds in the initial release;
- public API changes merely to make benchmarks faster.

The benchmark program must not pressure the project into scope creep.

---

## 6. Benchmarking Principles

### 6.1 Benchmark for clarity, not victory

The benchmark program exists to clarify:

```text
where matten is good enough
where matten is intentionally simpler
where specialized crates are better
```

It does not exist to produce a single "winner".

### 6.2 Separate peer comparisons from reference comparisons

Rust peer comparisons:

```text
ndarray
nalgebra
```

Reference ecosystem comparisons:

```text
NumPy
Pandas
```

Deferred or task-specific references:

```text
SciPy
Candle
```

Use the word "reference" for Python ecosystem comparisons. Avoid "competitor" in reports.

### 6.3 Compare only comparable tasks

Do not compare Pandas to core `Tensor` operations. Pandas should appear only for `matten-data` table-to-Tensor workflows.

Do not compare SciPy linalg/optimization to `matten` unless the project has accepted the relevant scope, which it currently has not.

Do not compare Candle unless the task is specifically ML tensor/model workflow, which is not part of current core scope.

### 6.4 Report limitations honestly

Every report must state:

```text
Benchmarks are workload-specific.
Numbers depend on hardware, compiler, dependencies, and environment.
This report is for positioning and regression visibility, not universal ranking.
```

---

## 7. Metrics

### 7.1 Execution time

Measure wall-clock execution time for controlled workloads.

Recommended Rust harness:

```text
criterion
```

For scripts and cross-language reference tasks, use a lightweight runner that records:

```text
command
environment
warmup count
sample count
median
mean
stddev
min
max
```

Prefer median for interpretation.

### 7.2 Memory behavior

Memory measurement is useful but must be practical.

Possible metrics:

```text
peak RSS
allocated bytes, if instrumentation is available
allocation count, if instrumentation is available
input size
output size
```

Initial requirement:

```text
record peak RSS for scenario benchmarks if feasible
```

Do not block Phase 1 if allocation-level measurement is not ready.

### 7.3 Example ELOC

ELOC means effective lines of code.

Measure user-solution code size to solve the task.

Report two variants if practical:

```text
ELOC without comments/blank lines
ELOC without comments/blank lines/imports
```

Rules:

```text
exclude comments
exclude blank lines
report whether imports are included
do not include benchmark harness code
do not include generated code
do not include output formatting helpers unless user would need them
```

Purpose:

```text
approachability
readability
teaching cost
```

ELOC must not be treated as a universal quality metric.

### 7.4 Dependency footprint

Record:

```text
direct dependencies
transitive dependencies
feature set
binary size, optional
build time, optional
```

This is important because `matten`'s positioning includes small dependency scope.

### 7.5 Implementation ELOC

Optional.

If measured, implementation ELOC must be used cautiously. It is useful for internal maintainability, not cross-language comparison.

Do not compare Rust implementation ELOC directly with NumPy/Pandas/SciPy implementation size in a simplistic way.

---

## 8. Benchmark Phases

### Phase 1: Internal baseline

Purpose:

```text
Establish matten's own performance and memory baseline.
No competitor/reference comparison is required.
```

Core targets:

```text
Tensor construction
reshape / flatten
elementwise add
elementwise multiply
scalar add/mul
broadcast vector over matrix
sum
mean
sum_axis
mean_axis
matmul
slicing
dynamic try_numeric
```

Companion targets:

```text
matten-ndarray:
  Tensor -> ArrayD
  ArrayD -> Tensor
  roundtrip

matten-mlprep:
  standardize_columns
  minmax_scale_columns
  add_bias_column
  train_test_split

matten-data:
  csv_to_tensor
  schema_summary
  select_columns
  fill_missing
  try_numeric
```

Output:

```text
benchmarks/results/internal-baseline.json
benchmarks/reports/internal-baseline.md
```

Acceptance:

```text
[ ] core benchmark harness exists
[ ] companion benchmark harness exists where feasible
[ ] results are reproducible on one maintainer machine
[ ] report includes environment
[ ] no marketing claims
```

### Phase 2: Rust peer comparison

Compare selected tasks with:

```text
ndarray
nalgebra
```

Use `ndarray` for:

```text
N-D array construction
elementwise operations
reductions
broadcasting
small matrix-like operations if practical
```

Use `nalgebra` for:

```text
small fixed/dynamic matrices
matrix multiplication
small vector/matrix algorithms
```

Recommended first task set:

```text
vector distance / pairwise distance
cosine similarity
matrix multiplication
Markov chain weather model
tiny PageRank
linear regression gradient descent
heat equation 1D
```

Output:

```text
benchmarks/results/rust-peer-comparison.json
benchmarks/reports/rust-peer-comparison.md
```

Report wording:

```text
Rust peer comparison
```

Do not use:

```text
matten beats ndarray
matten loses to nalgebra
```

Interpret the results in terms of tradeoffs.

### Phase 3: Ecosystem reference comparison

Provide context against familiar ecosystems, not direct competition.

Recommended references:

```text
NumPy for small array/numeric examples
Pandas only for matten-data table-to-Tensor workflow
```

Do not include SciPy or Candle in Phase 3 unless a separate task-specific decision approves them.

NumPy reference tasks:

```text
cosine similarity
Markov chain
tiny PageRank
linear regression gradient descent
heat equation 1D
```

Pandas reference task:

```text
CSV -> select columns -> fill missing -> numeric matrix
```

Report wording:

```text
Reference comparison
```

Explicitly state:

```text
Python/NumPy/Pandas use different execution models and mature native kernels.
These comparisons are for user-context and code-shape understanding, not a direct replacement claim.
```

### Phase 4: Regression tracking

Use benchmarks to detect unexpected slowdowns.

Initial policy:

```text
No hard CI failure on performance.
Record results.
Compare trends.
Warn manually on suspicious regressions.
```

Future policy after at least several baseline runs:

```text
soft warning on >20% median regression
hard failure only for severe, stable, unexplained regressions
```

Hard thresholds require a future RFC or release-policy decision.

---

## 9. Benchmark Workloads

### 9.1 Micro workloads

Use several sizes:

```text
tiny:    10–100 elements
small:   1,000–10,000 elements
medium:  100,000+ elements, only where memory-safe
```

Do not begin with huge data.

Workloads:

```text
construct vector
construct matrix
reshape
flatten
elementwise add
elementwise multiply
scalar add
scalar multiply
broadcast row/vector over matrix
sum all
mean all
sum_axis
mean_axis
matmul 16x16
matmul 64x64
matmul 128x128
```

### 9.2 Applied example workloads

Align with the examples program:

```text
existing pairwise_distance
existing 26_cosine_similarity
30_magic_square_checker
31_fibonacci_matrix_power
32_graph_path_counting
33_markov_chain_weather
34_tiny_pagerank
35_linear_regression_gradient_descent
36_heat_equation_1d
37_kmeans_small
38_nearest_neighbor_classification
```

Deferred:

```text
39_finite_difference_derivative
40_trapezoidal_integration
41_adjacency_walks_extended, optional and not reserved
```

### 9.3 Companion workloads

`matten-ndarray`:

```text
to_arrayd
from_arrayd
roundtrip
```

`matten-mlprep`:

```text
standardize_columns
minmax_scale_columns
add_bias_column
train_test_split
```

`matten-data`:

```text
from_csv_str
schema_summary
select_columns
fill_missing
try_numeric
to_tensor
csv_to_tensor full workflow
```

---

## 10. Competitor / Reference Matrix

| Library | Role | Include phase | Suitable tasks | Do not use for |
|---|---|---:|---|---|
| `ndarray` | Rust peer | 2 | N-D arrays, reductions, broadcasting | dataframe, ML models |
| `nalgebra` | Rust peer | 2 | small matrix/vector linalg-style examples | N-D tensor workflows |
| NumPy | ecosystem reference | 3 | small array/numeric scripts | direct Rust-vs-Python ranking |
| Pandas | ecosystem reference | 3 | CSV/table-to-numeric matrix | core tensor operations |
| SciPy | deferred reference | later | only if accepted SciPy-like task exists | Phase 1–3 |
| Candle | deferred reference | later | ML tensor/model workflows | current core benchmarks |

---

## 11. Harness Design

### 11.1 Repository layout

Recommended:

```text
benchmarks/
  README.md
  Cargo.toml, optional if separate harness workspace is used
  core/
    construction.rs
    elementwise.rs
    reductions.rs
    matmul.rs
    scenario_examples.rs
  companions/
    ndarray_roundtrip.rs
    mlprep.rs
    data.rs
  peers/
    ndarray/
    nalgebra/
  references/
    numpy/
    pandas/
  scripts/
    run_internal.sh
    run_rust_peers.sh
    run_references.sh
    collect_eloc.py
    summarize.py
  results/
    .gitkeep
  reports/
    .gitkeep
```

If the project prefers Rust-native benches:

```text
benches/
  core_construction.rs
  core_elementwise.rs
  core_reductions.rs
  core_matmul.rs
  companions.rs
```

Both can coexist if clearly documented.

### 11.2 Criterion usage

For Rust microbenchmarks, use `criterion` unless maintainers choose a lighter approach.

Rules:

```text
pin benchmark input generation
avoid printing inside timed section
use black_box where appropriate
separate setup from measured body
record crate versions
```

### 11.3 Cross-language references

Reference comparisons may be run by scripts, not Cargo bench.

Record:

```text
Python version
NumPy version
Pandas version
OS
CPU
memory
command
```

Do not make Python dependencies required for ordinary Rust CI.

---

## 12. CI Policy

### 12.1 Initial CI

In Phase 1:

```bash
cargo bench --no-run
```

or equivalent compile-check should pass.

Do not run full benchmarks on every CI job unless fast.

### 12.2 Scheduled / manual benchmarks

Use manual or scheduled jobs for full benchmark runs.

Recommended:

```text
manual maintainer run before maturity decisions
optional scheduled weekly/monthly run
release-candidate run before v1 discussion
```

### 12.3 No hard gates initially

Initial benchmark jobs should not fail releases due to speed changes.

They may fail on:

```text
benchmark harness does not compile
report generator fails
invalid result schema
```

But not on "slower than previous run" until policy matures.

---

## 13. Report Format

Every report should include:

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

Good positioning language:

```text
matten is fast enough for small PoC workflows in these scenarios,
while keeping dependency and API scope small.
```

Avoid:

```text
matten beats NumPy
matten replaces ndarray
matten is faster than Pandas
```

---

## 14. ELOC Methodology

### 14.1 What counts

Count solution code that a normal user would write.

Include:

```text
task-specific function body
task-specific setup data
explicit algorithm loop
```

Exclude:

```text
comments
blank lines
benchmark harness
test harness
generated code
large hard-coded data tables, report separately if needed
```

Imports:

```text
report both with imports and without imports if feasible
```

### 14.2 ELOC report row

Example:

```text
Task: Tiny PageRank

matten:
  ELOC without imports: 42
  ELOC with imports: 45

ndarray:
  ELOC without imports: 39
  ELOC with imports: 43

NumPy:
  ELOC without imports: 18
  ELOC with imports: 20
```

### 14.3 Interpretation

ELOC is an approachability signal, not a performance signal.

Shorter is not always better if it hides important policy or error handling.

---

## 15. Memory Measurement Policy

### 15.1 Phase 1

Record memory only where practical.

Acceptable initial options:

```text
peak RSS from process wrapper
platform-specific measurement documented honestly
manual measurement recorded in report
```

### 15.2 Future

A future improvement may add:

```text
allocation counter
jemalloc/mimalloc stats
dhat/heaptrack notes
criterion custom measurements
```

Do not add allocator dependencies to core `matten` for benchmarking.

---

## 16. Versioning and Reproducibility

Benchmark reports must record:

```text
matten family version
git commit
rustc version
cargo profile
target triple
OS
CPU model
RAM
dependency versions
feature flags
compiler flags
Python version, if references used
NumPy/Pandas versions, if references used
```

Reports should also record whether the run used:

```text
debug
release
bench profile
native CPU flags
```

Default should be release/bench profile, not debug.

---

## 17. Documentation Updates

Add:

```text
docs/src/benchmarks/index.md
docs/src/benchmarks/methodology.md
docs/src/benchmarks/results.md
```

or equivalent under the existing docs structure.

README should not display benchmark numbers until the benchmark methodology is stable. Prefer linking to reports.

---

## 18. Security and Supply-Chain Considerations

Benchmarking should not add risk to core.

Rules:

```text
[ ] no benchmark dependency becomes a normal dependency of core `matten`
[ ] no Python dependency is required for normal Rust build/test
[ ] no network access during benchmarks
[ ] no external datasets downloaded at runtime
[ ] generated results do not include secrets or machine-specific private paths where avoidable
[ ] benchmark scripts are deterministic and inspectable
```

---

## 19. Acceptance Criteria

RFC-049 is implemented when:

```text
[ ] benchmark scope and non-goals are documented
[ ] internal baseline harness exists
[ ] selected core microbenchmarks compile and run
[ ] selected companion benchmarks compile and run where feasible
[ ] report template exists
[ ] ELOC methodology is documented
[ ] environment metadata is captured
[ ] no ordinary CI job depends on Python references
[ ] no benchmark adds runtime dependency to core matten
[ ] reports avoid replacement/marketing claims
```

Phase 2 acceptance:

```text
[ ] ndarray peer tasks implemented
[ ] nalgebra peer tasks implemented where appropriate
[ ] comparison report uses "peer comparison" language
[ ] versions/features documented
```

Phase 3 acceptance:

```text
[ ] NumPy reference tasks implemented, if approved
[ ] Pandas reference task limited to table-to-Tensor workflow
[ ] report uses "reference comparison" language
[ ] no SciPy/Candle scope creep
```

Phase 4 acceptance:

```text
[ ] at least three historical runs exist
[ ] regression policy proposed separately
[ ] no hard speed gate added without approval
```

---

## 20. Implementation Plan

### PR-049-1: Methodology docs

Add:

```text
benchmarks/README.md
docs/src/benchmarks/methodology.md
```

Include:

- purpose;
- non-goals;
- metrics;
- phases;
- ELOC rules;
- report format.

### PR-049-2: Internal Rust benchmark harness

Add core microbenchmarks:

```text
construction
elementwise
reductions
matmul
broadcasting
```

Acceptance:

```text
[ ] harness compiles
[ ] local run produces result
[ ] no normal dependency pollution
```

### PR-049-3: Scenario benchmarks

Add applied tasks:

```text
cosine similarity, using existing example where possible
Markov chain
PageRank
linear regression
heat equation
```

Acceptance:

```text
[ ] tasks align with examples
[ ] outputs / correctness checks exist outside timed section
```

### PR-049-4: Companion benchmarks

Add:

```text
ndarray roundtrip
mlprep standardize/train-test
matten-data csv_to_tensor
```

Acceptance:

```text
[ ] companion benchmarks compile under correct features
[ ] dynamic rejection benchmark optional, not required
```

### PR-049-5: ELOC tool

Add a simple script:

```text
benchmarks/scripts/collect_eloc.py
```

or Rust equivalent.

Acceptance:

```text
[ ] excludes blank lines/comments
[ ] documents import policy
[ ] output can be used in report
```

### PR-049-6: First positioning report

Add:

```text
benchmarks/reports/positioning-v0.1.md
```

Acceptance:

```text
[ ] includes environment
[ ] includes limitations
[ ] includes interpretation
[ ] avoids marketing claims
```

### PR-049-7: Rust peer comparison

Optional after baseline.

Add `ndarray` and `nalgebra` peer tasks in benchmark-only crates/scripts.

Acceptance:

```text
[ ] does not affect normal workspace dependencies
[ ] report uses peer-comparison framing
```

### PR-049-8: Ecosystem reference comparison

Optional after Rust peer comparison.

Add NumPy/Pandas scripts.

Acceptance:

```text
[ ] Python references are optional
[ ] no normal CI dependency
[ ] report uses reference-comparison framing
```

---

## 21. Open Questions

1. Should `criterion` be the default Rust benchmark harness, or should the project start with a lighter custom runner?
2. Should benchmark harness crates live inside the workspace or outside normal workspace members?
3. Should benchmark results be committed, or only reports?
4. Which examples should be included in the first scenario benchmark?
5. Which memory measurement method is acceptable on the maintainer's primary OS?
6. Should Phase 2 include `nalgebra` immediately, or only after linalg boundary RFC-041 is accepted?

Recommended defaults:

```text
1. Use criterion for Rust microbenchmarks.
2. Keep benchmarks inside repo but avoid normal dependency pollution.
3. Commit reports, not large raw result histories.
4. Start with cosine similarity, Markov chain, PageRank, linear regression, heat equation.
5. Start with documented peak RSS if feasible.
6. Include nalgebra only for small matrix/vector tasks, not N-D tasks.
```

---

## 22. Final Decision Request

Approve RFC-049 as the benchmark and positioning program for `matten`.

The expected result is not a performance contest. The expected result is a reproducible, honest evidence base for `matten`'s intended position:

```text
small
clear
Tensor-centered
Rust-native
good enough for PoC and small workflows
honest about when to use larger ecosystems
```
