# RFC-050 — Production Migration Guide and Bridge Strategy

**Project:** `matten`  
**Milestone:** v0.23+ planning  
**Status:** Implemented (v0.23.0); architect-accepted (deep review 2026-06-27)  
**Document type:** RFC  
**Primary audience:** maintainers, library users, companion-crate developers  
**Depends on:** RFC-049 Benchmarking and Positioning, RFC-025 Bridge Crate Policy, RFC-032 Companion Dependency and Import Convention  
**Related:** RFC-051, RFC-052, RFC-053, RFC-054  

---

## 1. Summary

This RFC establishes the official `matten` production-migration strategy.

`matten` should remain the **family car**: approachable, small, teachable, and useful for PoC, learning, laboratory workflows, and small production-adjacent tasks.

When users outgrow `matten`, the project should help them move to the **super-car** ecosystem — heavier production-grade libraries such as `ndarray`, `nalgebra`, Polars, Candle, NumPy, or Pandas — without trying to become those libraries.

The strategy is:

```text
prototype in matten
understand the workflow
identify production pressure
choose the right target ecosystem
migrate intentionally
```

This RFC authorizes documentation and bridge strategy, not heavy runtime dependencies in core `matten`.

---

## 2. Motivation

`matten` is intentionally positioned as a friendly Rust numeric crate for PoC workflows. That is a strength, but successful PoCs often lead to production development.

After a user succeeds with `matten`, they may ask:

```text
How do I scale this?
How do I make this faster?
How do I integrate with production data systems?
How do I move to a serious linear algebra backend?
How do I move to ML tensors or model workflows?
How do I keep the PoC logic understandable during migration?
```

If `matten` does not answer those questions, users may either:

```text
overextend matten beyond its intended role
rewrite their project from scratch with poor guidance
misunderstand which production library fits which pressure
```

The project should provide a responsible exit ramp.

---

## 3. Goals

1. Define a documented migration story from `matten` to heavier ecosystems.
2. Help users identify when to stay with `matten` and when to migrate.
3. Preserve `matten`'s small core by placing conversion work in bridge crates and documentation.
4. Provide migration playbooks and examples before tool automation.
5. Make migration guidance evidence-based, informed by RFC-049 positioning results.
6. Avoid marketing claims such as "`matten` is faster than X" or "`matten` replaces Y."
7. Keep the user promise honest: `matten` helps users leave `matten` when appropriate.

---

## 4. Non-goals

This RFC does **not** authorize:

```text
[ ] adding ndarray / nalgebra / Polars / Candle dependencies to core matten
[ ] making matten a universal compatibility layer
[ ] making matten a dataframe library
[ ] making matten an ML framework
[ ] promising automatic code conversion
[ ] promising performance equivalence with production libraries
[ ] adding BLAS/LAPACK/GPU/sparse/decomposition features to core
[ ] adding query/groupby/join/pivot semantics to matten-data
```

---

## 5. User-facing positioning

The project should document this positioning:

```text
Use matten when:
  you are prototyping
  you want clear local numeric code
  you need small/medium tensors
  you value low dependency burden
  you want teachable examples
  you are exploring algorithm shape

Migrate from matten when:
  array sizes become large
  axis reductions or matrix operations dominate runtime
  you need specialized linear algebra
  you need sparse arrays
  you need GPU acceleration
  you need dataframe/table analytics
  you need ML tensor/model workflows
  you need production data pipelines
  you need Python ecosystem integration
```

The wording should avoid implying failure:

```text
Outgrowing matten is a successful PoC outcome.
```

---

## 6. Architecture policy

Migration support must follow a layered policy:

```text
core matten:
  no heavy production-library dependencies
  no migration-specific dependency expansion
  may expose stable, simple conversion primitives if they are dependency-free

bridge crates:
  conversion to/from specific ecosystems
  dependency-specific behavior
  compatibility examples
  feature-specific limitations

docs:
  migration guides
  API mapping tables
  target selection guide
  workflow examples

tools:
  optional later-stage diagnostics and suggestions
  no blind automatic rewrite in the first version
```

---

## 7. Deliverables

### 7.1 Migration guide

Add a top-level guide:

```text
docs/src/migration/index.md
```

Suggested pages:

```text
docs/src/migration/when-to-migrate.md
docs/src/migration/target-selection.md
docs/src/migration/api-mapping.md
docs/src/migration/common-pitfalls.md
docs/src/migration/from-examples.md
```

### 7.2 Target matrix

Provide a target matrix:

| Need | Recommended target |
|---|---|
| Rust N-D arrays | `ndarray` |
| Small/mid linear algebra | `nalgebra` |
| Dataframe analytics | Polars |
| ML tensors/models | Candle |
| Python array workflows | NumPy |
| Python table workflows | Pandas |
| Keep simple Rust PoC | stay with `matten` |

### 7.3 Example mapping

Use existing examples as migration anchors:

```text
25_normalize_vector
26_cosine_similarity
33_markov_chain_weather
34_tiny_pagerank
35_linear_regression_gradient_descent
36_heat_equation_1d
50_rowwise_scoring
51_standardize_columns
52_minmax_scaling
```

For each selected example, document:

```text
matten version
recommended production target
what changes
what stays conceptually the same
what is not automatically portable
```

---

## 8. Relationship to RFC-049

RFC-049 benchmarking/positioning informs this RFC but does not become marketing material.

Allowed use:

```text
matten's internal baseline suggests which operations may matter for migration.
Peer comparison helps users understand tradeoffs for small comparable tasks.
```

Forbidden use:

```text
matten is faster than X
matten replaces X
X is always better than matten
```

---

## 9. Documentation tone

Migration docs should be calm and honest.

Use:

```text
"when to consider"
"tradeoff"
"production pressure"
"bridge path"
"manual migration checklist"
```

Avoid:

```text
"upgrade from inferior matten"
"competitor"
"drop-in replacement"
"automatic conversion guaranteed"
```

---

## 10. Acceptance criteria

This RFC is implemented when:

```text
[ ] Migration guide index exists.
[ ] "When to stay / when to migrate" page exists.
[ ] Target-selection matrix exists.
[ ] At least three example-based migration notes exist.
[ ] Docs explicitly state core matten remains dependency-light.
[ ] Docs explicitly state migration support does not make matten a production super-crate.
[ ] No core dependency is added.
[ ] Release-doc guard or checklist prevents migration docs from making "faster than X" claims.
```

---

## 11. Risks

| Risk | Mitigation |
|---|---|
| Users think matten is deprecated | State that migration is for successful growth, not failure. |
| Scope creep into core | Put conversions in bridge crates and docs. |
| Marketing claims | Use RFC-049 disclaimers and release-doc checks. |
| Too many targets at once | Start with `ndarray` and `nalgebra`; add Polars/Candle/Python as docs mature. |
| Tooling overpromise | RFC-054 is explicitly later-stage and conservative. |

---

## 12. Recommended sequencing

```text
1. Implement RFC-050 docs.
2. Implement RFC-052 target playbooks for ndarray/nalgebra first.
3. Implement RFC-051 bridge conversion contracts.
4. Use RFC-049 Phase 2 evidence to refine guidance.
5. Consider RFC-053 diagnostics.
6. Consider RFC-054 tool only after docs and bridges stabilize.
```
