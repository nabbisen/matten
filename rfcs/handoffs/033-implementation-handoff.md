# RFC-033 Developer Handoff: `matten-data` Beta-Decision and Scope Lock

**Project:** `matten`  
**RFC:** RFC-033  
**Handoff Kind:** Strategic / Governance / Scaffold Gate  
**Implementation Level:** Policy-first; implementation only after acceptance  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-033 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-033 does not authorize broad coding by itself. It authorizes a decision gate and, if accepted, a controlled experimental `matten-data` scaffold.

The development team should treat this RFC as a scope lock:

```text
Allowed:
  small table-to-Tensor preparation companion

Forbidden:
  dataframe/query/large-data/ML-preprocessing scope
```

The first implementation task is not feature coding. It is repository alignment:

1. Add RFC-033 to `rfcs/proposed/`.
2. Update RFC index and roadmap references.
3. Add a `matten-data` decision note to docs.
4. If maintainers approve scaffold, create only minimal crate shell.
5. Mark maturity as `Experimental`.

### Crate decision

If scaffold is approved:

```text
crates/matten-data/
  Cargo.toml
  README.md
  src/lib.rs
```

Initial `src/lib.rs` may contain only crate docs, feature status, and empty module declarations.

### Dependency rule

Allowed direction:

```text
matten-data -> matten
```

Forbidden:

```text
matten -> matten-data
```

Core dependency-boundary CI must remain green.

---

## 2. Internal Design

No deep internal design is required for RFC-033. This is a policy and scope decision.

However, the scaffold should reserve module boundaries that later RFCs can fill:

```text
src/
  lib.rs
  error.rs       # from RFC-034/RFC-035
  table.rs       # from RFC-034
  schema.rs      # from RFC-034/RFC-035
  csv.rs         # from RFC-035
  numeric.rs     # from RFC-035
```

Do not expose these modules publicly until their RFCs are accepted.

---

## 3. Task Breakdown / PR Plan

### PR-033-1: RFC and roadmap registration

- Add `rfcs/proposed/033-matten-data-beta-decision-and-scope-lock.md`.
- Update `rfcs/README.md`.
- Update `ROADMAP.md` to point at RFC-033 for `matten-data` decision.
- Add note that RFC-032 is consumed elsewhere.

Acceptance:

```text
[ ] RFC appears in proposed list
[ ] roadmap uses RFC-033, not RFC-032
[ ] no implementation beyond docs
```

### PR-033-2: Scope guard documentation

- Add a short mdBook or docs page:
  - what `matten-data` is;
  - what it is not;
  - relationship to core `dynamic`;
  - relationship to Polars/DataFusion/Pandas.
- Avoid API promises not accepted by RFC-034/RFC-035.

Acceptance:

```text
[ ] docs say experimental
[ ] docs say not a dataframe
[ ] docs mention table-to-Tensor workflow only
```

### PR-033-3: Optional experimental crate scaffold

Only if maintainers approve.

- Add `crates/matten-data/Cargo.toml`.
- Add crate-level `README.md`.
- Add `src/lib.rs` with `#![forbid(unsafe_code)]`.
- Add to workspace members.
- Add CI `cargo check -p matten-data`.
- Do not implement table behavior yet unless RFC-034/RFC-035 are also accepted.

**Versioning (architect ruling, RFC-033–042 review Q10).** `matten-data` is
introduced under lock-step family versioning (RFC-030). Its `Cargo.toml` uses
`version.workspace = true`, so its package version is inherited from
`[workspace.package].version` (e.g. `0.20.0`) — **not** `0.1.0`. Maturity is
expressed by the Status label `Experimental` in its README/rustdoc, not by a
separate crate version.

**Boundary tooling.** Add `matten-data` to the `pub use matten` companion guard
crate list in `scripts/check-release-docs.sh` (RFC-032, architect Q1 tooling note).
The dependency-boundary script already forbids core depending on `matten-data`.

Acceptance:

```text
[ ] crate compiles
[ ] crate docs say Experimental
[ ] core dependency-boundary check passes
[ ] no dataframe-like API exists
```

---

## 4. Acceptance / QA Checklist

### Policy acceptance

```text
[ ] `matten-data` approved only as experimental
[ ] beta not claimed
[ ] dataframe/query scope forbidden
[ ] large CSV / streaming deferred to RFC-037
[ ] core dependency direction unchanged
```

### Repository QA

```bash
cargo fmt --all --check
bash scripts/check-core-dependency-boundary.sh
cargo check --workspace
cargo test --workspace --doc
```

### Documentation QA

```text
[ ] README / docs do not call `matten-data` a dataframe
[ ] docs mention use Polars/DataFusion/etc. for dataframe workloads
[ ] docs identify output goal as `matten::Tensor`
```

### Security / maintenance QA

```text
[ ] no new heavy dependency
[ ] no network/database/file side effects except future explicit CSV path APIs
[ ] no unsafe
[ ] no async runtime introduced
```

---

## 5. Do Not Implement Yet

Until RFC-034 and RFC-035 are accepted:

- no `Table::from_csv_str`;
- no `select_columns`;
- no `fill_missing`;
- no `try_numeric`;
- no `to_tensor`.

Until RFC-037 is replaced by a future streaming RFC:

- no streaming API.

Until RFC-042 is accepted or folded:

- no dataframe-like naming or examples.
