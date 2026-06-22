# RFC-042 Developer Handoff: Pandas-Inspired Scope Guard for `matten-data`

**Project:** `matten`  
**RFC:** RFC-042  
**Handoff Kind:** Governance / QA Handoff  
**Implementation Level:** Scope guard and release checks  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-042 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-042 is a scope guard. It may be implemented as docs, release-doc checks, naming rules, and review criteria.

The main rule:

```text
matten-data may use named columns and table preparation.
matten-data must not become a dataframe library.
```

---

## 2. Internal Design

No runtime internal design is needed.

Implement guardrails in:

```text
README
mdBook docs
release-doc script
PR review checklist
example naming policy
```

---

## 3. Task Breakdown / PR Plan

### PR-042-1: Naming policy docs

Document preferred names:

```text
Table
SchemaSummary
select_columns
fill_missing
try_numeric
to_tensor
```

Document forbidden names:

```text
DataFrame
Series
Index
loc
iloc
groupby
merge
pivot
query
```

Acceptance:

```text
[ ] docs include naming policy
[ ] no public DataFrame/Series types
```

### PR-042-2: README scope guard

Add early statement:

```text
matten-data is not a dataframe library.
```

Acceptance:

```text
[ ] statement appears near top of README
[ ] external tool guidance present
```

### PR-042-3: Example guard

Update release-doc script with a **precise** scope guard (RFC-042 §8): reject
dataframe/story terms in example *file names* and dataframe-shaped *public API
identifiers* in companion source; do not reject ordinary implementation variables
or legitimate Rust operations. Do not body-scan examples for broad terms like
`index`, `join`, `loc`, or `query`. Allow forbidden words in explicit non-goal docs.

Acceptance:

```text
[ ] groupby/join/pivot/query examples fail check
[ ] existing examples using `index` and `Path::join`/`str.join` continue to pass
[ ] non-goals docs can mention the terms
```

### PR-042-4: PR review checklist

Add checklist item:

```text
Does this PR add dataframe-like semantics?
```

Acceptance:

```text
[ ] checklist exists
[ ] reviewer guidance clear
```

---

## 4. Acceptance / QA Checklist

```text
[ ] no public type named DataFrame
[ ] no public type named Series
[ ] no join/merge/groupby/pivot/query API
[ ] no dataframe-like examples
[ ] docs recommend external tools for dataframe workloads
[ ] release-doc guard in place
```

CI:

```bash
bash scripts/check-release-docs.sh
cargo check --workspace --examples --all-features
```

---

## 5. Do Not Implement

- dataframe aliases;
- indexing model;
- query DSL;
- group-by;
- joins;
- pivot;
- time-series operations;
- lazy engine.

---

## 6. Escalation Rule

If a developer believes a forbidden API is necessary, they must write a new RFC. The default review answer should be no unless the proposal proves:

```text
[ ] still small
[ ] no heavy dependency
[ ] no index/query engine
[ ] no confusion with Pandas/Polars/DataFusion
[ ] clear user value beyond existing crates
```
