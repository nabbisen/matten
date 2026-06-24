# Compact Implementation Handoff — RFC-050–053 Production Migration and Bridge Support

**Project:** `matten`  
**Document kind:** Compact implementation handoff  
**Scope:** RFC-050, RFC-051, RFC-052, RFC-053  
**Status:** Ready for implementation planning  
**Explicitly excluded:** RFC-054 implementation  
**Recommended target:** v0.23.0 / v0.23.x  
**Primary audience:** documentation implementers, bridge-crate maintainers, release maintainers  

---

## 0. Executive Summary

This handoff implements the first stage of the “family-car to super-car bridge” direction.

`matten` should remain:

```text
small
approachable
Tensor-centered
good for PoC, learning, and small serious workflows
dependency-light
```

When users outgrow it, `matten` should help them migrate intentionally to production-oriented ecosystems:

```text
ndarray
nalgebra
Polars
Candle
NumPy
Pandas
```

The implementation should be mostly documentation, templates, and bridge-contract policy.

Do **not** add heavy dependencies to core `matten`.

Do **not** implement `matten-migrate` yet.

---

## 1. Implementation Principle

The implementation must preserve this layered architecture:

```text
core matten:
  owns Tensor
  stays small
  has no heavy target-library dependencies
  does not become a dataframe / ML / linalg backend

bridge crates:
  own dependency-specific conversion
  document copy/shape/value/error contracts
  do not re-export core Tensor

docs:
  explain when to stay with matten
  explain when and how to migrate
  provide target playbooks
  provide report templates

tools:
  deferred until docs and bridge contracts stabilize
```

The correct user promise is:

```text
matten helps users know when and how to leave matten.
```

The wrong promise is:

```text
matten automatically converts every PoC into production code.
```

---

## 2. Scope

### In scope for this compact handoff

```text
[✓] migration guide pages
[✓] target-selection matrix
[✓] target playbooks
[✓] bridge conversion contract template
[✓] matten-ndarray contract audit
[✓] migration readiness report template
[✓] one worked migration readiness example
[✓] release-doc checks / review checklist against overclaims
```

### Out of scope

```text
[ ] new bridge crates
[ ] matten-nalgebra implementation
[ ] matten-polars implementation
[ ] matten-candle implementation
[ ] matten-migrate CLI
[ ] automatic source rewriting
[ ] new core matten API
[ ] new core matten dependencies
[ ] Phase 3 Python benchmark implementation
```

---

## 3. Recommended PR Plan

## PR-1 — RFC-050 Migration Guide Foundation

Goal:

```text
Create the main production migration guide and define project language.
```

Files to add:

```text
docs/src/migration/index.md
docs/src/migration/when-to-migrate.md
docs/src/migration/target-selection.md
docs/src/migration/common-pitfalls.md
```

Files to update:

```text
docs/src/SUMMARY.md
README.md, if it links to major guide sections
docs/src/introduction.md, if appropriate
```

Content requirements:

```text
[ ] explain "family car to super-car" metaphor without being gimmicky
[ ] define when to stay with matten
[ ] define when to migrate
[ ] include target-selection matrix
[ ] state that outgrowing matten is a successful PoC outcome
[ ] state that core matten remains dependency-light
[ ] state that migration support is not automatic conversion
```

---

## PR-2 — RFC-052 Production Target Playbooks

Goal:

```text
Add target-specific migration guidance.
```

Files to add:

```text
docs/src/migration/playbooks/index.md
docs/src/migration/playbooks/ndarray.md
docs/src/migration/playbooks/nalgebra.md
docs/src/migration/playbooks/polars-and-pandas.md
docs/src/migration/playbooks/candle.md
docs/src/migration/playbooks/python-numpy.md
```

Each playbook must include:

```text
## Choose this target when
## Do not choose this target when
## Concept mapping
## Example migrations
## Conversion path
## Common pitfalls
## Performance / positioning notes
## Minimal checklist
```

Target-specific rules:

```text
ndarray:
  general Rust N-D array production path

nalgebra:
  small/mid dense vector/matrix and linalg path

Polars/Pandas:
  dataframe/table analytics path;
  explicitly say matten-data will not grow groupby/join/pivot/query

Candle:
  ML tensor/model workflow path;
  do not imply matten is an ML framework

NumPy:
  Python scientific ecosystem path;
  manual/conceptual unless future bridge/tooling is designed
```

---

## PR-3 — RFC-051 Bridge Conversion Contract Template

Goal:

```text
Define a standard conversion contract for bridge crates and audit matten-ndarray.
```

Files to add:

```text
docs/src/migration/bridge-contracts.md
docs/src/migration/bridge-crate-policy.md
```

Files to update:

```text
crates/matten-ndarray/README.md
docs/src/companions.md
docs/src/SUMMARY.md
```

Contract table must include:

```text
source type
target type
direction
copy/view behavior
shape/rank policy
memory order policy
dynamic tensor policy
NaN policy
missing-value policy
integer/text/bool policy
error behavior
performance caveat
examples
```

For `matten-ndarray`, expected contract:

```text
Tensor -> ndarray::ArrayD<f64>
ArrayD<f64> -> Tensor
copies both directions
numeric tensors only
rejects dynamic tensors
preserves shape
preserves logical row-major element order
```

Do not implement `matten-nalgebra` in this PR. Only document future direction.

---

## PR-4 — RFC-053 Migration Readiness Diagnostics and Report Format

Goal:

```text
Add a manual migration readiness report template and one worked example.
```

Files to add:

```text
docs/src/migration/readiness-report.md
docs/src/migration/readiness-checklist.md
docs/src/migration/examples/linear-regression-gd-readiness.md
```

The report template must include:

```text
# matten Migration Readiness Report

## Summary
## Current matten usage
## Production pressure signals
## Recommended target(s)
## Direct conversion candidates
## Manual redesign areas
## Bridge crates / tools
## Risks
## Next steps
```

The checklist must assess:

```text
data size pressure
runtime pressure
axis-reduction pressure
linear algebra pressure
dataframe pressure
ML/device pressure
dynamic ingestion pressure
dependency policy
target ecosystem preference
team language preference
```

Worked example recommendation:

```text
35_linear_regression_gradient_descent
```

Alternative worked example:

```text
36_heat_equation_1d
```

Do not implement a CLI.

---

## 4. File Map

Suggested final docs tree:

```text
docs/src/migration/
  index.md
  when-to-migrate.md
  target-selection.md
  common-pitfalls.md
  bridge-contracts.md
  bridge-crate-policy.md
  readiness-report.md
  readiness-checklist.md
  playbooks/
    index.md
    ndarray.md
    nalgebra.md
    polars-and-pandas.md
    candle.md
    python-numpy.md
  examples/
    linear-regression-gd-readiness.md
```

Update:

```text
docs/src/SUMMARY.md
docs/src/companions.md
docs/src/benchmarks/index.md, only if linking positioning to migration
README.md, only for top-level guide link if desired
```

Do not update:

```text
core matten APIs
runtime behavior
companion APIs, except docs
benchmark harness, except links if needed
```

---

## 5. RFC-050 Acceptance Criteria

RFC-050 is done when:

```text
[ ] migration guide index exists
[ ] "when to stay / when to migrate" page exists
[ ] target-selection matrix exists
[ ] at least three example-based migration notes or references exist
[ ] docs state core matten remains dependency-light
[ ] docs state migration support does not make matten a production super-crate
[ ] docs avoid "faster than X" claims
[ ] docs avoid "drop-in replacement" claims
[ ] no core dependency is added
```

Required key wording:

```text
Outgrowing matten is a successful PoC outcome.
```

---

## 6. RFC-051 Acceptance Criteria

RFC-051 is done when:

```text
[ ] bridge conversion contract template exists
[ ] bridge crate policy page exists
[ ] matten-ndarray README includes a conversion contract table
[ ] docs explain bridge crates must not re-export Tensor
[ ] docs explain bridge crates own target-specific dependencies
[ ] future bridge crate checklist exists
[ ] no new bridge crate is created unless separately approved
[ ] no target-library dependency is added to core matten
```

---

## 7. RFC-052 Acceptance Criteria

RFC-052 is done when:

```text
[ ] ndarray playbook exists
[ ] nalgebra playbook exists
[ ] Polars/Pandas playbook or boundary page exists
[ ] Candle playbook or careful stub exists
[ ] NumPy/Python playbook exists
[ ] target-selection decision tree exists
[ ] at least three matten examples are mapped to target choices
[ ] no playbook claims matten replaces the target
[ ] no playbook claims the target is always better than matten
[ ] if official RFC-049 Phase 2 peer numbers are not yet accepted, each playbook's
    performance/positioning section is marked pending and includes no numeric claims
```

Minimum example mapping set:

```text
22_matrix_multiplication -> ndarray / nalgebra
27_axis_reductions -> ndarray
35_linear_regression_gradient_descent -> ndarray / nalgebra
50_rowwise_scoring -> ndarray or stay with matten
data_00_quickstart -> Polars/Pandas if dataframe operations are needed
```

---

## 8. RFC-053 Acceptance Criteria

RFC-053 is done when:

```text
[ ] migration readiness report template exists
[ ] readiness checklist exists
[ ] checklist maps pressure signals to target playbooks
[ ] one worked report example exists
[ ] report is advisory, not automatic conversion
[ ] no CLI is introduced
[ ] no source-code scanning tool is introduced
```

Required advisory disclaimer:

```text
This report is advisory. It does not prove production readiness,
does not guarantee a target library is better, and does not perform automatic conversion.
```

---

## 9. Release-Doc and Scope Guards

Add or extend release-doc checks if practical.

Guard categories:

```text
[ ] no "matten is faster than" in migration docs
[ ] no "drop-in replacement" in migration docs
[ ] no "automatic conversion" claim except in RFC-054 future/deferred context
[ ] no new target-library dependency in core matten
[ ] no dataframe feature promises in matten-data docs
```

A simple grep-based guard is acceptable over:

```text
docs/src/migration
```

for forbidden phrases.

Allow historical RFC text if necessary, but current docs should be strict.

---

## 10. QA Checklist

Before merge:

```text
[ ] mdBook builds locally
[ ] links in SUMMARY.md are correct
[ ] all new pages have clear titles
[ ] examples referenced by name exist
[ ] migration docs do not cite sandbox benchmark numbers
[ ] migration docs do not use unofficial peer-comparison numbers
[ ] migration docs use RFC-049 accepted internal/official results only
[ ] no core Cargo.toml dependency change
[ ] no companion Cargo.toml dependency change unless docs-only update needs none
[ ] release-doc guard passes
```

If official Phase 2 peer numbers are not yet available:

```text
[ ] migration docs say peer comparison is pending
```

If official Phase 2 peer numbers are available:

```text
[ ] migration docs use them only as task-scoped positioning
[ ] no ranking language
```

---

## 11. Documentation Tone

Use:

```text
consider
tradeoff
production pressure
migration path
bridge
manual checklist
target-specific
```

Avoid:

```text
upgrade
competitor
winner
loser
drop-in
automatic
replace
always
never
```

Example good sentence:

```text
If dense matrix/vector kernels become production hot paths, consider moving that
part of the workflow to ndarray or nalgebra while keeping matten as the PoC reference.
```

Example bad sentence:

```text
For production, replace matten with nalgebra.
```

---

## 12. Deferred Items

Do not implement now:

```text
matten-migrate CLI
matten-nalgebra bridge crate
matten-polars bridge crate
matten-candle bridge crate
automatic rewriting
source scanner
Cargo.toml modifier
Phase 3 Python benchmark
```

Each needs a separate handoff and explicit approval.

---

## 13. Completion Definition

This compact handoff is complete when:

```text
[ ] RFC-050 docs are merged
[ ] RFC-052 playbooks are merged
[ ] RFC-051 bridge contract template and matten-ndarray audit are merged
[ ] RFC-053 report template and one worked example are merged
[ ] RFC-054 remains explicitly deferred
[ ] release-doc checks protect the main scope promises
```

Suggested versioning:

```text
v0.23.0:
  migration guide + playbooks

v0.23.x:
  bridge contracts + readiness report

future:
  tool or new bridge crate handoffs
```
