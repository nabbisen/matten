# RFC-052 — Production Target Playbooks

**Project:** `matten`  
**Milestone:** v0.23+ planning  
**Status:** Accepted (architect ruling 2026-06-24); implementation planned for v0.23.x  
**Document type:** RFC  
**Primary audience:** users migrating from PoC to production  
**Depends on:** RFC-049 Benchmarking and Positioning, RFC-050 Production Migration Guide  
**Related:** RFC-051 Bridge Conversion Contracts, RFC-053 Migration Readiness Diagnostics  

---

## 1. Summary

This RFC defines production target playbooks for users who outgrow `matten`.

Each playbook explains:

```text
when to choose the target
what matten concepts map cleanly
what does not map cleanly
which examples demonstrate the migration
which bridge crate or manual path to use
what pitfalls to expect
```

Initial targets:

```text
ndarray
nalgebra
Polars
Candle
NumPy
Pandas
```

The first implementation should prioritize `ndarray` and `nalgebra`, because those align most directly with RFC-049 Phase 2 Rust peer comparison.

> **Peer-evidence status (2026-06-25).** The official RFC-049 Phase 2 Rust peer comparison is
> **accepted** (`benchmarks/reports/peer-comparison-v0.1.md`, Report ID
> `matten-rfc049-rust-peer-comparison-v0.1`). Playbooks **may** cite its results, but only
> task-scoped and with the same machine / small-fixed-size caveat — no ranking, no
> "faster than X", no universal migration mandate. Allowed shape: "In the accepted RFC-049
> Phase 2 peer comparison, dense `matmul` and matrix-vector tasks showed a larger gap than
> lighter vector-style tasks; if those kernels become production hot paths, consider moving
> that part of the workflow to `ndarray` or `nalgebra`." Forbidden: "ndarray/nalgebra are
> better than matten", "matten is too slow for production", "replace matten after PoC".

---

## 2. Motivation

Users do not only need conversion APIs. They need judgment.

A successful `matten` PoC may encounter different production pressures:

```text
performance
larger arrays
linear algebra accuracy/stability
dataframe/table workflows
ML tensor/model integration
Python ecosystem collaboration
```

Different pressures imply different targets.

A single "migrate to production" page would be too vague. Playbooks make the guidance concrete.

---

## 3. Goals

1. Provide target-specific migration guidance.
2. Explain when the target is appropriate.
3. Map common `matten` examples to target ecosystems.
4. Avoid universal "best library" claims.
5. Use RFC-049 data as positioning context, not marketing.
6. Keep bridge and conversion expectations honest.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] adding all target libraries as dependencies
[ ] implementing all bridge crates immediately
[ ] benchmarking every production target
[ ] claiming target libraries are always better
[ ] claiming matten can automatically rewrite code
[ ] adding dataframe or ML behavior to core matten
```

---

## 5. Playbook format

Each playbook should follow this structure:

```text
# Migrating from matten to <target>

## Choose this target when
## Do not choose this target when
## Concept mapping
## Example migrations
## Conversion path
## Common pitfalls
## Performance/positioning notes
## Minimal checklist
```

---

## 6. ndarray playbook

### Choose `ndarray` when

```text
you need production Rust N-D arrays
you need stronger ecosystem support for array operations
you need more mature performance behavior
you need array views/slicing/broadcasting depth beyond matten
you want to stay in Rust
```

### Do not choose `ndarray` when

```text
you need small fixed-size linear algebra
you need a dataframe
you need ML tensors/devices/models
you need Python-native collaboration
```

### Concept mapping

| `matten` | `ndarray` |
|---|---|
| `Tensor` numeric storage | `ArrayD<f64>` or dimension-specific arrays |
| `shape()` | `.shape()` |
| reshape | ndarray's current reshape APIs (e.g. `into_shape_with_order` / `to_shape`, per ownership/layout) |
| elementwise ops | array arithmetic |
| axis reductions | ndarray axis methods |
| `matten-ndarray` bridge | `to_arrayd` / `from_arrayd` |

### Candidate examples

```text
04_elementwise_ops
06_broadcasting
22_matrix_multiplication
27_axis_reductions
36_heat_equation_1d
```

### Notes

`ndarray` is likely the first production migration target for general N-D numeric workloads.
When writing the playbook, check the ndarray version used by `matten-ndarray` before copying
any reshape snippet, and prefer current APIs over deprecated ones.

---

## 7. nalgebra playbook

### Choose `nalgebra` when

```text
your data is naturally vectors/matrices
you need linear algebra APIs
you need small/mid matrix operations
you need geometry/transforms
you do not need N-D tensor semantics
```

### Do not choose `nalgebra` when

```text
your workload is general N-D arrays
your workload is table/dataframe-heavy
you need dynamic ingestion cleanup
you need ML tensor/device behavior
```

### Concept mapping

| `matten` | `nalgebra` |
|---|---|
| rank-1 Tensor | `DVector<f64>` |
| rank-2 Tensor | `DMatrix<f64>` |
| `dot` / vector ops | vector dot/norm operations |
| `matmul` | matrix multiplication |
| `outer` | vector outer product / matrix construction |

### Candidate examples

```text
20_dot_product
21_matrix_vector_product
22_matrix_multiplication
31_fibonacci_matrix_power
33_markov_chain_weather
34_tiny_pagerank
35_linear_regression_gradient_descent
```

### Notes

`nalgebra` should not be presented as a general tensor replacement.

---

## 8. Polars playbook

### Choose Polars when

```text
your problem is table/dataframe analytics
you need columnar operations
you need filtering/grouping/joining/aggregation
you need large data handling
```

### Do not choose Polars when

```text
you only need small numeric tensors
you need simple teachable PoC code
you need matrix/tensor algorithms
```

### Concept mapping

| `matten` / `matten-data` | Polars |
|---|---|
| `matten-data::Table` | DataFrame-like table |
| schema summary | schema / dtypes |
| select columns | column selection |
| fill missing | missing-value operations |
| `try_numeric` | explicit casting / numeric selection |

### Boundary warning

Polars guidance must not cause `matten-data` to become a dataframe library.

The playbook should say:

```text
If you need group-by/join/pivot/query, migrate to Polars.
Do not expect matten-data to grow those features.
```

---

## 9. Candle playbook

### Choose Candle when

```text
you need ML tensors
you need model inference/training workflows
you need device-aware tensors
you need integration with model weights
```

### Do not choose Candle when

```text
you only need small numeric PoCs
you need dataframe analytics
you need general Rust N-D arrays without ML/device concerns
```

### Concept mapping

| `matten` | Candle |
|---|---|
| numeric Tensor | ML tensor |
| shape | tensor dims |
| f64 data | likely dtype conversion needed |
| examples | ML-like examples only with caution |

### Candidate examples

```text
37_kmeans_small
38_nearest_neighbor_classification
35_linear_regression_gradient_descent
```

### Boundary warning

Do not imply `matten` is an ML framework.

---

## 10. NumPy playbook

### Choose NumPy when

```text
you are moving to Python
you need Python scientific ecosystem integration
you need mature vectorized operations
you collaborate with Python-heavy teams
```

### Do not choose NumPy when

```text
you need a pure Rust production stack
you want Rust compile-time packaging
you need minimal dependencies
```

### Notes

This playbook should be manual and conceptual unless a Python bridge is explicitly designed later.

---

## 11. Pandas playbook

### Choose Pandas when

```text
your workflow is Python table analytics
your data is row/column-oriented
you need joins/groupby/pivot/query
```

### Boundary warning

This is especially important for `matten-data`:

```text
Pandas is where dataframe workflows belong.
matten-data is only a small CSV/table-to-Tensor preparation helper.
```

---

## 12. Target selection decision tree

Add a decision tree:

```text
Need dataframe operations?
  yes -> Polars or Pandas
Need ML tensor/model workflow?
  yes -> Candle
Need Rust N-D arrays?
  yes -> ndarray
Need Rust matrix/vector linalg?
  yes -> nalgebra
Need Python scientific ecosystem?
  yes -> NumPy
Need small teachable PoC?
  stay with matten
```

---

## 13. Acceptance criteria

This RFC is implemented when:

```text
[ ] ndarray playbook exists.
[ ] nalgebra playbook exists.
[ ] Polars/Pandas boundary playbook exists or is stubbed with clear warning.
[ ] Candle playbook exists or is stubbed with clear warning.
[ ] target-selection decision tree exists.
[ ] at least three existing matten examples are mapped to target choices.
[ ] docs include no "faster than" or "replacement for" claims.
[ ] if official RFC-049 Phase 2 peer numbers are not yet accepted, each playbook's
    "Performance / positioning notes" section is marked **pending** and contains no numeric
    claims (no sandbox samples, no unfilled-template figures, no unofficial runs).
```

---

## 14. Suggested initial implementation

Phase A:

```text
ndarray playbook
nalgebra playbook
target-selection matrix
```

Phase B:

```text
Polars/Pandas table workflow playbook
Candle ML workflow playbook
```

Phase C:

```text
example-by-example migration notes after RFC-049 Phase 2 results
```
