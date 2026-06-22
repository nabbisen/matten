# RFC-036 Developer Handoff: `matten-data` Examples, Documentation, and Release Gate

**Project:** `matten`  
**RFC:** RFC-036  
**Handoff Kind:** Documentation / QA Handoff  
**Implementation Level:** Examples and release checks required  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-036 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-036 is the release-quality gate for `matten-data`.

It ensures examples teach only accepted APIs and do not become hidden product scope.

---

## 2. Internal Design

No complex internal design is needed.

However, add a docs/check script section if release-doc checks exist.

Recommended script additions:

```text
scripts/check-release-docs.sh
  check matten-data README status
  check examples do not use forbidden dataframe terms
  check canonical imports
```

The script should avoid banning terms in Non-goals sections.

---

## 3. Task Breakdown / PR Plan

### PR-036-1: Required examples

Add:

```text
examples/data_00_quickstart.rs
examples/data_01_schema_summary.rs
examples/data_02_select_columns.rs
examples/data_03_missing_values.rs
examples/data_04_to_tensor.rs
examples/data_05_errors.rs
```

Acceptance:

```text
[ ] examples compile
[ ] examples are small
[ ] examples use accepted API only
[ ] no dataframe-like example story
```

### PR-036-2: README

Add README sections:

```text
status: Experimental
purpose
quickstart
scope
non-goals
API overview
error behavior
relationship to matten::Tensor
relationship to core dynamic
when to use Polars/DataFusion/Pandas instead
```

Acceptance:

```text
[ ] README says not a dataframe
[ ] README says experimental
[ ] quickstart shows table-to-Tensor only
```

### PR-036-3: mdBook / project docs

Add companion page:

```text
docs/src/companions/matten-data.md
```

Acceptance:

```text
[ ] install shown with matten + matten-data
[ ] canonical imports use matten::Tensor
[ ] limitations clear
```

### PR-036-4: CI integration

Add CI commands:

```bash
cargo check -p matten-data --examples
cargo test -p matten-data
cargo test -p matten-data --doc
```

Acceptance:

```text
[ ] examples checked in CI
[ ] docs tested
[ ] workspace remains green
```

### PR-036-5: Release-doc checks

Add checks for:

- status label;
- forbidden terms in examples;
- canonical imports;
- explicit missing/numeric conversion docs;
- alternative crate guidance.

Acceptance:

```text
[ ] check fails on use matten_data::Tensor
[ ] check fails on example names with groupby/join/pivot/query
[ ] check permits those words in non-goals docs
```

---

## 4. Acceptance / QA Checklist

### Example QA

```text
[ ] quickstart demonstrates full workflow
[ ] schema example is not analytics-heavy
[ ] select-columns example proves order
[ ] missing-values example proves no silent zero
[ ] to-tensor example proves shape
[ ] errors example shows boundary Result behavior
```

### Documentation QA

```text
[ ] status = Experimental
[ ] not a dataframe stated early
[ ] output Tensor shape documented
[ ] missing-value policy documented
[ ] numeric conversion policy documented
[ ] use Polars/DataFusion/Pandas instead guidance present
```

### Scope QA

Forbidden in examples:

```text
groupby
group_by
join
merge
pivot
query
rolling
dataframe
series
loc
iloc
```

Unless explicitly under non-goals docs.

### CI QA

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo check --workspace --examples --all-features
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-release-docs.sh
```

---

## 5. Release Decision

`matten-data` may ship as experimental only if:

```text
[ ] all examples pass
[ ] README has scope guard
[ ] release-doc checks pass
[ ] no dataframe examples exist
[ ] no streaming examples exist
[ ] no large-data claims exist
```

This RFC does not approve beta.
