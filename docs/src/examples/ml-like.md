# ML-like

Two small algorithms often associated with machine learning, written with `matten`
to show that a `Tensor` is enough for recognizable ML-shaped tasks. They use only the
default Phase-1 numeric API, small hard-coded inputs, and deterministic output.

The boundary is deliberate: these are **algorithm demonstrations, not an ML
framework**. There is no training loop abstraction, no model object, no autograd, and
no randomness — `k`, initial centroids, labels, and iteration counts are all fixed and
explicit. Both use a small local `argmin` helper, since core `matten` has no `argmin`
yet (a future RFC-038 candidate).

## Examples

### `37_kmeans_small.rs`

*Difficulty: Advanced-small.* Clusters six 2-D points into two groups with Lloyd's
algorithm: assign each point to the nearest centroid, then move each centroid to the
mean of its points. Deterministic initial centroids make the run reproducible; it
converges to the two obvious clusters.

```bash
cargo run --example 37_kmeans_small
```

### `38_nearest_neighbor_classification.rs`

*Difficulty: Beginner.* Classifies a query point by the label of its single nearest
training point (1-NN) over a labeled `[samples, features]` data matrix. No training
step, no fitted parameters — just a nearest-point search.

```bash
cargo run --example 38_nearest_neighbor_classification
```

## What this is not

These are single-file demonstrations of accepted APIs. They do not imply that
`matten` is an ML framework, a clustering/classification library, or a replacement for
a dedicated ML toolkit.
