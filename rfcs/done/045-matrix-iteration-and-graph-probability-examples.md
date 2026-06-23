# RFC-045: Matrix Iteration and Graph/Probability Examples

**Status:** Implemented (v0.20.4) — matrix-iteration examples 33–34 (Markov chain, tiny PageRank); optional 41 adjacency-walks remains a conditional candidate.  
**Target Release:** v0.20.x / v0.21.0  
**Related:** RFC-043, RFC-044, RFC-038  
**Scope:** Intermediate examples based on repeated matrix/vector iteration

---

## 1. Summary

This RFC adds intermediate examples that show `matten` solving recognizable matrix-iteration problems.

Initial examples:

```text
examples/33_markov_chain_weather.rs
examples/34_tiny_pagerank.rs
```

Optional candidate, not reserved:

```text
41_adjacency_walks_extended.rs

Add only if the Phase 0 inventory shows it teaches a distinct concept beyond
32_graph_path_counting.rs (e.g. comparing multiple powers side by side, walk
growth over k, directed vs undirected contrast, self-loop effect, or
reachability). If it only repeats A^2 / A^3 path counting on a larger graph,
do not add it.
```

These examples teach the power of matrix multiplication and repeated updates without adding new framework scope.

---

## 2. Motivation

After beginner examples, users should see `Tensor` as useful for real small models:

- probabilities over states;
- graph transitions;
- iterative ranking;
- convergence-like behavior.

These are mathematically famous and practical, but still small enough for one-file examples.

---

## 3. Example 33: Markov Chain Weather Model

### Problem

Given transition probabilities between sunny and rainy weather, compute the distribution after several days.

Example transition matrix:

```text
Sunny -> Sunny: 0.8
Sunny -> Rainy: 0.2
Rainy -> Sunny: 0.4
Rainy -> Rainy: 0.6
```

### APIs demonstrated

- vector state representation;
- transition matrix;
- matrix-vector multiplication;
- iteration;
- probability interpretation.

### Implementation note

Use a clear convention:

```text
state vector row or column
transition matrix orientation
```

Do not mix conventions.

### Acceptance

```text
[ ] prints initial distribution
[ ] prints distribution after N steps
[ ] rows/columns sum to expected probability total
[ ] comments explain transition orientation
```

---

## 4. Example 34: Tiny PageRank

### Problem

Rank a tiny set of pages by link structure.

Famous idea:

```text
rank_next = damping * transition * rank + teleport
```

### APIs demonstrated

- adjacency matrix;
- column or row normalization;
- repeated matrix-vector update;
- scalar/matrix arithmetic;
- convergence-like iteration.

### Implementation note

Keep the graph tiny, 4 or 5 pages.

Do not try to implement a full PageRank library.

### Acceptance

```text
[ ] tiny hard-coded graph
[ ] damping factor documented
[ ] fixed number of iterations
[ ] deterministic output
[ ] no web/network input
```

---

## 5. Example (OPTIONAL): Adjacency Walks Extended

**Optional, deferred decision (architect ruling, RFC-043–048 review Q1).** Add this
only if it is not redundant with `32_graph_path_counting`. If added, it takes the
next free number in the additive band; otherwise it is dropped.

### Problem

Extend beginner graph path counting with a slightly more complex graph and multiple powers.

### APIs demonstrated

- matrix power loop;
- interpreting `A^2`, `A^3`;
- optional formatting helper.

### Acceptance

```text
[ ] graph remains tiny
[ ] output matrices are small
[ ] comments explain walks
```

This example should be skipped if `32_graph_path_counting` already teaches the concept sufficiently.

---

## 6. Documentation Requirements

Add docs page:

```text
docs/src/examples/matrix-iteration.md
```

The page should explain:

```text
These are small teaching examples.
They are not graph-framework or probability-library implementations.
```

---

## 7. QA Checklist

```text
[ ] examples compile
[ ] examples run deterministically
[ ] probability examples preserve approximate sum
[ ] PageRank example has no external input
[ ] examples use small matrices only
```

CI:

```bash
cargo check -p matten --examples --all-features
cargo run -p matten --example 33_markov_chain_weather
cargo run -p matten --example 34_tiny_pagerank
```

---

## 8. Non-goals

- No graph library.
- No Markov-chain package.
- No PageRank crate.
- No sparse matrix.
- No web crawling.
- No convergence-proof machinery.
