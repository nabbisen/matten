# RFC-042: Pandas-Inspired Scope Guard for `matten-data`

**Status:** Implemented (v0.21.3) — three-check anti-scope guard (example file names / public-API identifiers / README scope statement) shipped as scripts/check-matten-data-scope.sh, wired into CI.
**Target Release:** v0.20+ / v0.21+  
**Related:** RFC-033, RFC-034, RFC-035, RFC-036  
**Scope:** Explicit anti-scope guard for Pandas/dataframe expectations

---

## 1. Summary

This RFC defines a strong scope guard for `matten-data`.

It may be folded into RFC-033 if maintainers prefer fewer RFCs, but keeping it separate makes the risk visible.

Core rule:

```text
matten-data may borrow the idea of named columns and table preparation.
matten-data must not become a dataframe library.
```

---

## 2. Motivation

Users familiar with Pandas may see `Table` and expect:

- joins;
- group-by;
- pivot;
- indexing;
- filtering expressions;
- missing-value analytics;
- datetime handling;
- rolling windows;
- rich IO;
- large data.

If `matten-data` tries to satisfy those expectations, it will fail and damage the `matten` identity.

This RFC defines what to reject early.

---

## 3. Allowed Pandas-Inspired Ideas

Allowed:

```text
column names
schema summary
select columns by name
missing-value visibility
explicit fill missing
explicit numeric conversion
table-to-Tensor output
```

These are preparation features, not analytics features.

---

## 4. Forbidden Pandas-Like Features

Forbidden in `matten-data` v0.20/v0.21:

```text
DataFrame
Series
Index
MultiIndex
join
merge
concat rows by key
groupby
aggregate by group
pivot
melt
stack/unstack table operations
query
eval
loc/iloc clone
rolling
expanding
resample
datetime index
categorical dtype
large IO framework
SQL-like expressions
lazy execution
```

Do not add aliases that imply these features.

---

## 5. Naming Policy

Avoid names that imply Pandas compatibility.

Preferred:

```text
Table
SchemaSummary
select_columns
fill_missing
try_numeric
to_tensor
```

Avoid:

```text
DataFrame
Series
groupby
merge
pivot
loc
iloc
query
```

`Table` is acceptable because it is generic and modest.

---

## 6. Documentation Policy

README must include:

```text
matten-data is not a dataframe library.
```

It should also say:

```text
Use Polars, DataFusion, Pandas, or another dataframe/query tool for joins,
group-by, pivot, lazy queries, or large data.
```

Do not market `matten-data` as:

```text
Pandas for Rust
DataFrame for matten
lightweight Polars
```

---

## 7. Example Scope Guard

Examples must not include:

```text
group_by_sales_region
join_customers_orders
pivot_monthly_sales
rolling_average
query_filter
```

If similar user stories are needed, they should be written as "use another crate" documentation, not `matten-data` examples.

---

## 8. Release-Docs Guard

The scope guard must be **precise**, not a broad substring scan over example bodies
(architect ruling, RFC-033–042 review Q3). A naive scan fails on legitimate code —
e.g. `index` is a common loop/indexing variable, `join` appears in `Path::join` /
`str.join()`, and `loc` is a substring of `local`/`location`/`block`.

Use three separate checks:

**(1) Example file-name guard.** Reject dataframe-story terms in example *file names*
(not source bodies):

```text
groupby  group_by  join  merge  pivot  query  rolling
dataframe  data_frame  series  loc  iloc
```

So an example file named `join_customers_orders.rs` fails; an example that merely
calls `path.join(...)` does not.

**(2) Public API identifier guard.** Reject dataframe-shaped public definitions in
companion source (`crates/matten-data/src`), matched as definitions, not arbitrary
text:

```text
pub struct DataFrame / pub enum DataFrame / pub type DataFrame
pub struct Series    / pub enum Series
pub fn groupby / group_by / join / merge / pivot / query / loc / iloc
```

**(3) Documentation context.** Forbidden words are allowed in explicit non-goal
sections (e.g. "Non-goals", "Not a dataframe", "When to use Polars/DataFusion
instead"). Either avoid automated body scanning, or scan only headings/examples/API
snippets, or exclude those non-goal sections.

Do **not** body-scan for broad/common terms (`index`, `join`, `loc`, `query`).

Acceptance for the guard:

```text
[ ] existing core examples using `index` continue to pass
[ ] Path::join / str.join usage does not fail
[ ] a public `DataFrame`/`Series` type would fail
[ ] a public groupby/join/pivot/query API would fail
[ ] an example file named join_customers_orders.rs would fail
[ ] non-goal docs can mention join/group-by/pivot
```

---

## 9. Security / Maintenance Rationale

Dataframe APIs increase risk because they introduce:

- complex semantics;
- hidden performance costs;
- higher user expectations;
- larger dependency pressure;
- more parser/expression surface;
- more edge cases around missing data and types.

Avoiding them keeps `matten-data` maintainable and easier to audit.

---

## 10. Acceptance Criteria

```text
[ ] README says not a dataframe library
[ ] no public type named DataFrame or Series
[ ] no join/groupby/pivot/query APIs
[ ] examples avoid dataframe-like stories
[ ] release-doc check guards example scope
[ ] docs recommend external tools for dataframe workloads
```

---

## 11. Future Reconsideration

A future RFC may reconsider specific table operations only if:

- there is strong user evidence;
- the operation remains small;
- the operation does not require index/query semantics;
- it does not introduce heavy dependencies;
- it does not change `matten-data`'s identity.

Default answer should remain no.

---

## Architect Rulings — v0.21 Boundary Review (2026-06-23)

Both questions accepted. RFC-042 stays standalone; the three-check release-docs guard
is authorized (target **v0.21.3**, or any earlier patch — it is mechanical).

**Q12 — Keep RFC-042 standalone** (even though it references RFC-033): the
dataframe-creep risk warrants a visible, named, enforceable artifact.

**Q13 — Authorize the three-check guard** exactly as §8 specifies: (1) example
file-name guard, (2) public-API identifier guard over `crates/matten-data/src`,
(3) non-goal documentation context handling. Do **not** body-scan broad terms
(`index`, `join`, `loc`, `query`).

- **Must fail on:** a public `DataFrame`/`Series` type; a public
  `groupby`/`group_by`/`join`/`merge`/`pivot`/`query` API; an example file name
  implying dataframe operations.
- **Must not fail on:** an ordinary variable named `index`; `Path::join`; forbidden
  words inside documentation non-goal sections; words like `local`/`location`/`block`.
