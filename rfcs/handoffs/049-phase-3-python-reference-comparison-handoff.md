# RFC-049 Phase 3 Handoff: Python Reference Comparison

**Project:** `matten`
**Related RFC:** RFC-049: Benchmarking, Complexity Metrics, and Positioning Report
**Document kind:** Implementation handoff / implementation record
**Status:** Implemented and accepted by review on 2026-07-11
**Target:** Post-0.29.0 planning / possible 0.30.x evidence slice
**Scope:** Optional NumPy/Pandas reference scripts, code-shape/dependency-footprint evidence,
and a curated reference-comparison report

---

## 1. Summary

RFC-049 Phase 1 and Phase 2 are complete: the benchmark harness has an internal Rust
baseline and an accepted Rust peer comparison against `ndarray` / `nalgebra`.

This handoff defines the next optional slice: **Phase 3 Python reference comparison**.
It adds script-driven NumPy/Pandas references for user context only. The center of
gravity is **code shape, ELOC, and dependency footprint**. Runtime, if collected, is
only a heavily caveated same-machine context note. This slice must not turn the
benchmark program into a leaderboard or create a claim that `matten` replaces
Python scientific/data tooling.

The intended output is a manually filled curated report, not a normal release gate.

## 2. Why This Slice

Users comparing `matten` to familiar workflows often think in Python first:

- NumPy for small numeric arrays and linear algebra-shaped examples;
- Pandas for CSV/table cleanup before numerical work.

Phase 3 can make those comparisons legible without changing product scope. The
report should help readers understand code shape, example-code ELOC, dependency
footprint, and, optionally, rough runtime context on one maintainer machine class.

## 3. Boundaries

Allowed:

- Add optional Python scripts under `benchmarks/`.
- Use NumPy for small array/numeric reference tasks.
- Use Pandas only for the `matten-data` table-to-Tensor workflow.
- Add a curated Markdown report under `benchmarks/reports/`.
- Update benchmark docs to say Phase 3 exists only if the implementation is accepted.

Forbidden:

- No SciPy, Candle, GPU, Polars, DataFusion, or large-data benchmark suites.
- No hard CI performance thresholds.
- No normal Rust CI dependency on Python, NumPy, or Pandas.
- No network fetches or downloaded datasets.
- No public Rust API, feature, or published crate dependency change.
- No wording like "`matten` beats NumPy" or "`matten` is faster than Pandas".
- No runtime table unless the `matten` and Python rows are collected on the same
  machine and clearly labeled as context, not ranking.

## 4. Proposed Repository Shape

Add only benchmark-local files:

```text
benchmarks/
  python/
    README.md
    requirements.txt
    run_references.py
    tasks/
      numpy_cosine_similarity.py
      numpy_markov_chain.py
      numpy_tiny_pagerank.py
      numpy_linear_regression_gd.py
      numpy_heat_equation_1d.py
      pandas_csv_to_matrix.py
  reports/
    python-reference-comparison-v0.1.md
```

Keep this outside the Cargo workspace behavior. The existing `benchmarks/` package
is already workspace-excluded and `publish = false`; Python files remain local to
that benchmark area.

This handoff intentionally supersedes RFC-049 §11.1's aspirational
`benchmarks/references/{numpy,pandas}/` plus `benchmarks/scripts/run_references.py`
layout for Phase 3. The actual shipped benchmark harness already uses a flatter,
workspace-excluded `benchmarks/` package layout; `benchmarks/python/` keeps the
optional Python material visibly separate without changing Cargo behavior.

## 5. Task Set

NumPy reference tasks:

| Task | Existing `matten` counterpart | Reference intent |
|---|---|---|
| cosine similarity | `scenario/cosine_similarity` | Small vector math corresponding to existing scenario coverage |
| Markov chain | `scenario/markov_step` | Matrix iteration and normalization context |
| tiny PageRank | `scenario/pagerank_step` | Repeated small linear algebra-shaped updates |
| linear regression gradient descent | `scenario/linreg_gd_step` | Small ML-like numeric loop, not an ML framework claim |
| heat equation 1D | `scenario/heat_step` | Simple numerical-method iteration |

Pandas reference task:

| Task | `matten` family counterpart | Reference intent |
|---|---|---|
| CSV -> select columns -> fill missing -> numeric matrix | `matten-data` table cleanup example/workload | `matten-data` table cleanup context only |

Pandas must not be compared to core tensor arithmetic. It is allowed only for the
table-to-numeric-matrix workflow.

If runtime context is collected for the Pandas task, the implementation must also
collect a same-environment `matten-data` counterpart in the same report. That can
be an existing benchmark-local workload if one exists, or a small benchmark-local
workload added under `benchmarks/`; it must not require public API changes.

## 6. Required Deliverables

For every NumPy/Pandas task, the implementation must capture:

- **Example ELOC:** executable lines for the reference script and the corresponding
  `matten` example/workload, using the RFC-049 ELOC methodology.
- **Dependency footprint:** direct dependency list, transitive dependency count
  where practical, and relevant feature/setup notes.
- **Code-shape notes:** short, factual observations about what each ecosystem makes
  concise or verbose.

Runtime context is optional. If included:

- collect the `matten` and Python rows on the same machine, same commit, and same
  report run;
- label results as environment-specific context only;
- do not sort or present them as a winner/loser ranking.

## 7. Script Behavior

The runner should:

- run one task at a time or all tasks;
- print deterministic-format Markdown or plain-text rows suitable for pasting into
  the report;
- record Python, NumPy, and Pandas versions;
- record OS/CPU/RAM in the same spirit as the existing benchmark environment capture;
- print ELOC and dependency-footprint rows;
- use generated in-repo data only;
- avoid network access during benchmark/reference runs;
- exit nonzero on invalid task names or missing optional dependencies with a clear message.

The output format should be deterministic; timing values, if collected, are not.
Do not commit machine-specific generated output automatically. The curated report
is filled manually from one maintainer run, following the Phase 2 precedent.

The scripts may use `time.perf_counter()` and repeated loops for optional runtime
context. This is sufficient for context reporting; Phase 3 does not need a full
Python benchmarking framework.

`requirements.txt` must use exact pins (`==`). Installing those requirements may
contact PyPI during environment setup; benchmark/reference runs themselves must
not access the network. The report must record the installed Python, NumPy, and
Pandas versions and they must match the setup file unless the report explicitly
explains a local override.

## 8. Report Wording

Use this framing:

```text
Reference comparison
```

The report must state:

```text
Python/NumPy/Pandas use different execution models and mature native kernels.
These comparisons are for user-context and code-shape understanding, not direct
replacement or ranking claims.
```

Avoid:

```text
matten beats NumPy
matten loses to Pandas
faster/slower ranking table
```

Prefer:

```text
For this small workflow on this maintainer environment...
The code shape differs...
The ELOC and dependency footprint differ...
Use NumPy/Pandas when the surrounding workflow belongs in Python data tooling.
```

Required report sections:

- environment and versions;
- task mapping;
- ELOC comparison;
- dependency-footprint comparison;
- optional runtime context, if collected;
- interpretation with the "reference comparison" framing.

## 9. Documentation Updates

If implemented, update:

- `benchmarks/README.md`:
  - add the optional Phase 3 command;
  - keep Python references out of ordinary CI;
  - keep Phase 4 deferred.
- `docs/src/benchmarks/index.md`:
  - move Phase 3 from deferred to implemented/accepted only after review acceptance.
- `docs/src/benchmarks/results.md`:
  - add a short reader-facing link to the curated Python reference report.
- `docs/src/benchmarks/methodology.md`:
  - document Python reference constraints, ELOC/dependency-footprint capture, and
    version/environment capture.
- RFC-049 status:
  - record Phase 3 status while keeping Phase 4 deferred.

## 10. Verification

Required Rust/doc gates:

```bash
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-release-docs.sh
mdbook build docs --dest-dir ../target/mdbook-rfc049-phase3-check
git diff --check
```

Required benchmark-local checks:

```bash
cargo bench --manifest-path benchmarks/Cargo.toml --no-run
```

Required Python checks, only when Python dependencies are installed:

```bash
python3 benchmarks/python/run_references.py --help
python3 benchmarks/python/run_references.py --task numpy-cosine-similarity
python3 benchmarks/python/run_references.py --task pandas-csv-to-matrix
python3 benchmarks/python/run_references.py --all
```

If NumPy/Pandas are not installed, the implementation review must say so clearly
and show the runner's missing-dependency behavior. Do not vendor dependencies into
the repository.

## 11. Acceptance Criteria

- [ ] Python scripts are optional and local to `benchmarks/`.
- [ ] No published crate dependency graph changes.
- [ ] No normal CI dependency on Python, NumPy, or Pandas.
- [ ] No network access during benchmark/reference runs and no downloaded datasets.
- [ ] `requirements.txt` pins exact Python package versions with `==`.
- [ ] NumPy task set matches RFC-049 Phase 3.
- [ ] Pandas is limited to the table-to-numeric-matrix workflow.
- [ ] Every NumPy task maps 1:1 to an existing measured `scenario/*` counterpart.
- [ ] If Pandas runtime context is included, a same-environment `matten-data`
      counterpart is included.
- [ ] ELOC and dependency footprint are required report rows for each task.
- [ ] Version/environment capture is documented in the report.
- [ ] Any runtime context is same-machine, heavily caveated, and not ranked.
- [ ] Report uses "reference comparison" framing.
- [ ] No SciPy/Candle/GPU/large-data scope creep.
- [ ] Phase 4 regression thresholds remain deferred.

## 12. Non-Goals

- No benchmark performance gates.
- No release-blocking Python dependency.
- No new `matten` API.
- No new companion crate.
- No `matten-report` or `matten-viz` publication work.
- No attempt to benchmark broad Pandas dataframe operations.

## 13. Resolved Review Decisions

- Phase 3 should proceed only as a **code-shape-first** reference slice. Runtime
  cannot be the main deliverable.
- The first accepted report should be manually filled from a maintainer run, as
  with Phase 2.
- Python setup should use exact pinned `requirements.txt` entries. A heavier
  lockfile/tool-specific environment is not required for the first optional
  reference slice.

## 14. Resolved Implementation Choices

1. Phase 3 proceeds now as a code-shape-first reference slice.
2. The RFC-049 task set remains unchanged.
3. The first implementation stops at ELOC, dependency footprint, and code-shape notes; runtime
   context is omitted.
