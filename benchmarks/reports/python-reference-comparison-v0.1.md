# Python reference comparison — v0.1

**Report ID:** `matten-rfc049-python-reference-comparison-v0.1`
**Status:** RFC-049 Phase 3 code-shape-first implementation report, refreshed for rereview. Runtime
context is intentionally omitted. ELOC is measured against minimal `matten` solution snippets, not
the longer didactic examples.

> **This is a reference comparison for code shape, ELOC, and dependency footprint. It is not a
> performance comparison, not a ranking, and not a "faster than NumPy/Pandas" claim.**

## Scope

Phase 3 places `matten` examples next to small NumPy/Pandas reference scripts so readers can see
what the code shape and dependency shape look like. It does not add public Rust APIs, does not add
Python to ordinary CI, and does not change any published crate dependency graph.

Runtime numbers are omitted in this first report. If runtime context is added later, it must be
collected for both sides on the same machine and labeled as environment-specific context only.

## How to regenerate

Setup may contact PyPI:

```bash
python3 -m venv .venv-bench-python
. .venv-bench-python/bin/activate
python3 -m pip install -r benchmarks/python/requirements.txt
```

Reference runs themselves must not access the network:

```bash
python3 benchmarks/python/run_references.py --environment
python3 benchmarks/python/run_references.py --all
```

## Environment

Captured with `python3 benchmarks/python/run_references.py --environment`.

| Field | Value |
|---|---|
| Python | 3.14.6 |
| Platform | Linux-7.1.3-1-cachyos-x86_64-with-glibc2.43 |
| Machine | x86_64 |
| NumPy pin | 2.3.5 |
| NumPy installed | 2.3.5 |
| Pandas pin | 2.3.3 |
| Pandas installed | 2.3.3 |
| Working directory | `/home/nabbisen/Desktop/matten/matten-git` |

Installed versions and transitive dependency counts were filled from a local venv using
`benchmarks/python/requirements.txt`.

## Task mapping

| Python reference task | Minimal `matten` ELOC snippet | Existing didactic example |
|---|---|---|
| Cosine similarity | `benchmarks/python/matten/cosine_similarity.rs` | `scenario/cosine_similarity`; `crates/matten/examples/26_cosine_similarity.rs` |
| Markov chain step | `benchmarks/python/matten/markov_chain.rs` | `scenario/markov_step`; `crates/matten/examples/33_markov_chain_weather.rs` |
| Tiny PageRank step | `benchmarks/python/matten/tiny_pagerank.rs` | `scenario/pagerank_step`; `crates/matten/examples/34_tiny_pagerank.rs` |
| Linear regression GD step | `benchmarks/python/matten/linear_regression_gd.rs` | `scenario/linreg_gd_step`; `crates/matten/examples/35_linear_regression_gradient_descent.rs` |
| Heat equation 1D step | `benchmarks/python/matten/heat_equation_1d.rs` | `scenario/heat_step`; `crates/matten/examples/36_heat_equation_1d.rs` |
| CSV to numeric matrix | `benchmarks/python/matten/csv_to_matrix.rs` | `matten-data` quickstart; `crates/matten-data/examples/data_00_quickstart.rs` |

## ELOC and dependency footprint

Generated with `python3 benchmarks/python/run_references.py --all`. The `matten` ELOC column counts
minimal task-equivalent snippets under `benchmarks/python/matten/`, not the didactic examples.

`ELOC` is shown as `without imports / with imports`.

| Task | Python ELOC | matten ELOC | Dependencies | Import check | Code-shape note |
|---|---:|---:|---|---|---|
| Cosine similarity | 8/9 | 9/10 | numpy pin 2.3.5; installed 2.3.5; transitive 0 | ok | NumPy keeps vector math compact through array primitives and linalg helpers. |
| Markov chain step | 10/11 | 21/22 | numpy pin 2.3.5; installed 2.3.5; transitive 0 | ok | Both versions expose the transition-matrix shape directly. |
| Tiny PageRank step | 12/13 | 29/30 | numpy pin 2.3.5; installed 2.3.5; transitive 0 | ok | NumPy expresses the dense update tersely; matten keeps the same shape story in Rust. |
| Linear regression GD step | 10/11 | 21/22 | numpy pin 2.3.5; installed 2.3.5; transitive 0 | ok | NumPy is compact for matrix algebra; matten shows the loop in one Rust binary. |
| Heat equation 1D step | 10/11 | 21/22 | numpy pin 2.3.5; installed 2.3.5; transitive 0 | ok | Both versions are dense-operator references, not optimized stencil kernels. |
| CSV to numeric matrix | 10/12 | 12/13 | pandas pin 2.3.3; installed 2.3.3; transitive 6 | ok | Pandas owns broader dataframe ergonomics; matten-data stays explicit and narrow. |

## Interpretation

The useful signal is code shape, not speed. NumPy and Pandas are mature Python ecosystems with
broad native capabilities. `matten` keeps the same small workflows in a Rust-first, `Tensor`-centered
shape without claiming to replace those ecosystems.

The NumPy rows are expected to be compact because dense vector/matrix manipulation is exactly
NumPy's native surface. The corresponding `matten` ELOC rows use minimal task-equivalent snippets;
the longer didactic examples remain linked separately for teaching context.

The Pandas row is intentionally scoped to table cleanup. Pandas owns much broader dataframe
semantics; `matten-data` is deliberately narrower and keeps each table-to-Tensor step explicit.

## Limitations

- Dependency counts come from installed package metadata and exclude optional extras.
- No runtime measurements are included.
- ELOC is an approachability signal only; it is not a universal quality metric.
- Python reference scripts are small, deterministic examples. They are not broad ecosystem
  benchmark suites.
