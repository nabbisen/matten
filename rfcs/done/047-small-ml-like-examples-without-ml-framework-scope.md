# RFC-047: Small ML-Like Examples Without ML-Framework Scope

**Status:** Implemented (v0.20.8) — ML-like examples 37–38 (k-means, nearest-neighbor) with an algorithm-demo (not ML-framework) boundary.  
**Target Release:** v0.21.0  
**Related:** RFC-038, RFC-040, RFC-048  
**Scope:** Demonstrate small algorithms often associated with ML while preserving non-ML-framework identity

---

## 1. Summary

This RFC adds small ML-like teaching examples:

```text
examples/37_kmeans_small.rs
examples/38_nearest_neighbor_classification.rs
```

These examples demonstrate numerical algorithms using `Tensor`, but they must not imply that `matten` is an ML framework.

---

## 2. Motivation

Many users evaluating a tensor crate want to see ML-adjacent tasks.

The safe approach is:

```text
show small algorithms from scratch
do not add ML abstractions
do not add training framework
do not add datasets
do not add autograd
```

---

## 3. Example 37: K-Means Small

### Problem

Cluster a tiny set of 2D points into two groups.

### Math idea

Repeat:

```text
assign each point to nearest center
recompute centers as mean of assigned points
```

### APIs demonstrated

- matrix as point set;
- distance computation;
- reductions / mean;
- simple iteration;
- optional `argmin` from RFC-038.

### Implementation note

Before `argmin`, use local helper function for nearest center.

Do not add a reusable `KMeans` type.

### Acceptance

```text
[ ] tiny hard-coded dataset
[ ] fixed initial centers
[ ] fixed iteration count
[ ] deterministic output
[ ] comments say this is an algorithm demo
```

---

## 4. Example 38: Nearest Neighbor Classification

### Problem

Classify a point by the label of the nearest training point.

### Math idea

Compute distance from query point to each training point, choose nearest label.

### APIs demonstrated

- vector/matrix distance;
- row iteration;
- argmin-like logic;
- simple label association outside Tensor.

### Implementation note

Labels should stay as a small Rust array, not a Tensor, unless there is a clear reason.

### Acceptance

```text
[ ] tiny dataset
[ ] one or two query points
[ ] deterministic prediction
[ ] no dataset loader
[ ] no classifier abstraction
```

---

## 5. Relationship to `matten-mlprep`

If examples use standardization or train/test split, put them in `matten-mlprep` examples or reference RFC-048.

Do not duplicate companion-specific examples in core.

---

## 6. Documentation Requirements

Add docs page:

```text
docs/src/examples/ml-like.md
```

It must say:

```text
These are algorithm demonstrations. `matten` is not an ML framework.
```

---

## 7. QA Checklist

```text
[ ] examples compile
[ ] examples deterministic
[ ] no random initialization unless fixed
[ ] no autograd
[ ] no model type
[ ] no training framework language
[ ] no external datasets
```

CI:

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 37_kmeans_small
cargo run -p matten --example 38_nearest_neighbor_classification
```

---

## 8. Non-goals

- No model training API.
- No optimizer.
- No neural network.
- No autograd.
- No dataset management.
- No benchmarking.
- No production ML claims.
