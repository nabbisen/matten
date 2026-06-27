# RFC-053 — Migration Readiness Diagnostics and Report Format

**Project:** `matten`  
**Milestone:** v0.24+ planning  
**Status:** Implemented (v0.23.4); architect-accepted (deep review 2026-06-27)  
**Document type:** RFC  
**Primary audience:** maintainers, tool authors, advanced users  
**Depends on:** RFC-049 Benchmarking and Positioning, RFC-050 Production Migration Guide, RFC-052 Production Target Playbooks  
**Related:** RFC-054 `matten-migrate` Assisted Migration Tool  

---

## 1. Summary

This RFC defines a migration readiness diagnostic report for `matten` projects.

Before building an automatic conversion tool, the project should first offer a structured way to assess:

```text
what the user's matten code does
which production pressures are present
which migration targets are plausible
which parts are directly portable
which parts require manual design
```

This RFC is documentation/report-first. It does not authorize automatic code rewriting.

---

## 2. Motivation

Migration is not only a mechanical conversion problem.

A `matten` project may use:

```text
simple numeric Tensor operations
dynamic ingestion cleanup
matten-data table preparation
examples-derived algorithms
manual loops
shape composition
linalg-lite helpers
statistics helpers
```

The right target depends on usage patterns.

Users need a readiness report before they need a code transformation.

---

## 3. Goals

1. Define a migration readiness checklist.
2. Define a report format.
3. Identify production pressure signals.
4. Recommend migration targets based on evidence.
5. Prepare groundwork for a later CLI assistant.
6. Avoid automatic rewriting in this stage.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] parsing arbitrary Rust code perfectly
[ ] automatic rewrite to ndarray/nalgebra/Polars/Candle
[ ] changing matten APIs
[ ] collecting private code or telemetry
[ ] running external network services
[ ] producing performance claims
```

---

## 5. Readiness dimensions

The report should assess:

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

---

## 6. Report format

Suggested file:

```text
matten-migration-report.md
```

Template:

```markdown
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

---

## 7. Inputs

Initial report can be manual.

Inputs:

```text
Cargo.toml dependency list
examples used as reference
rough tensor shapes
known performance pain
whether dynamic/matten-data is used
whether production target is Rust/Python/ML/dataframe
```

A later tool may inspect source files, but the first version should not require static analysis.

---

## 8. Signals and recommendations

### Signal: large N-D arrays

Recommendation:

```text
consider ndarray
```

### Signal: rank-1/rank-2 linalg-heavy code

Recommendation:

```text
consider nalgebra
```

### Signal: joins/groupby/pivot/query

Recommendation:

```text
consider Polars or Pandas
```

### Signal: model/tensor/device workflow

Recommendation:

```text
consider Candle
```

### Signal: dynamic cleanup followed by numeric conversion

Recommendation:

```text
keep matten-data/dynamic for ingestion,
then bridge numeric tensors to target library
```

### Signal: axis reductions dominate

Recommendation:

```text
review RFC-049 baseline and consider ndarray migration if this is hot path
```

---

## 9. Privacy and local-first policy

Any diagnostic tool derived from this RFC must be local-first.

```text
No telemetry.
No uploaded source code.
No network calls.
No hidden external analysis.
```

This aligns with the user's broader local/private tooling preferences.

---

## 10. Acceptance criteria

This RFC is implemented when:

```text
[ ] A migration readiness report template exists.
[ ] A manual checklist exists.
[ ] The checklist maps signals to target playbooks.
[ ] At least one worked example report exists.
[ ] Docs state that the report is advisory, not an automatic conversion.
[ ] No code rewriting tool is introduced yet.
```

---

## 11. Worked example candidates

Use one or more of:

```text
35_linear_regression_gradient_descent
36_heat_equation_1d
51_standardize_columns
data_00_quickstart
```

Example output:

```text
This workflow is numeric-array heavy and uses repeated matrix/vector operations.
Recommended target: ndarray first; nalgebra if the representation is strictly 2D linalg.
```

---

## 12. Relationship to RFC-054

RFC-054 may automate parts of this report later.

This RFC must come first because:

```text
assessment should be stable before tooling
manual report teaches the right mental model
automation without a report format would overpromise
```

---

## 13. Open questions

1. Should the first diagnostic be a Markdown template only?
2. Should a simple `cargo xtask` generate dependency/feature summaries?
3. Should the project provide example reports for each target ecosystem?
4. Should migration reports be part of RFC-049 benchmark reports or separate?
