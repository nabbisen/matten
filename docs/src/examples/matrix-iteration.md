# Matrix iteration

Intermediate examples built on repeated matrix/vector multiplication. They show
how an iterative process — a probability distribution evolving over time, or a
ranking settling to a fixed point — is just `Tensor::matmul` applied in a loop.

Like the rest of the applied band, these use only the default Phase-1 numeric API,
small hard-coded inputs, and deterministic output. They are teaching examples, not
a graph or probability library.

## Examples

### `33_markov_chain_weather.rs`

*Difficulty: Intermediate.* Models a two-state (Sunny / Rainy) weather process with
a row-stochastic transition matrix `P`. Each day applies `v_next = v · P` via
vector × matrix `matmul`, and the distribution converges to the stationary
`π = [5/6, 1/6]`.

```bash
cargo run --example 33_markov_chain_weather
```

### `34_tiny_pagerank.rs`

*Difficulty: Intermediate.* Ranks the nodes of a tiny directed graph with PageRank.
A column-stochastic link matrix `M` is power-iterated with damping
(`r_next[i] = (1 - d)/N + d·(M·r)[i]`) using matrix × vector `matmul`; the
best-connected node wins, and the link-less node keeps only its teleport share.

```bash
cargo run --example 34_tiny_pagerank
```

## What this is not

These are single-file demonstrations of accepted APIs. They do not imply a graph
framework, a probability toolkit, or a production PageRank implementation.
