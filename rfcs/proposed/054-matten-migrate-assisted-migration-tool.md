# RFC-054 — `matten-migrate` Assisted Migration Tool

**Project:** `matten`  
**Milestone:** Future / post-guide / post-bridge stabilization  
**Status:** Accepted as future direction (architect ruling 2026-06-24; deferral confirmed in deep review 2026-06-27); implementation deferred (see handoff deferred note)  
**Document type:** RFC  
**Primary audience:** tool authors, maintainers, advanced users  
**Depends on:** RFC-050 Production Migration Guide, RFC-051 Bridge Conversion Contracts, RFC-052 Production Target Playbooks, RFC-053 Migration Readiness Diagnostics  
**Related:** RFC-049 Benchmarking and Positioning  

---

## 1. Summary

This RFC proposes a future `matten-migrate` command-line tool.

The first version should be advisory, not an automatic code rewriter.

Its job is to help users answer:

```text
What matten APIs am I using?
Which production target might fit?
Which bridge crate or guide should I read?
What parts of my code are easy to migrate?
What parts require manual redesign?
```

It may generate a migration readiness report.

It must not promise automatic production conversion.

---

## 2. Motivation

After migration guides and bridge contracts exist, users may still need help applying them to their project.

A small CLI can make migration guidance practical:

```text
scan a project
identify matten usage
summarize likely migration targets
link to relevant playbooks
generate a Markdown report
```

This is useful without taking on the risk of automatic rewriting.

---

## 3. Goals

1. Provide local-only project inspection.
2. Generate RFC-053-style migration readiness reports.
3. Detect common `matten` API usage patterns.
4. Suggest target playbooks.
5. Suggest bridge crates where applicable.
6. Avoid hidden network or telemetry behavior.
7. Avoid automatic rewriting in v1 of the tool.

---

## 4. Non-goals

This RFC does not authorize:

```text
[ ] automatic Rust source rewriting in first version
[ ] guaranteed conversion correctness
[ ] compilation of transformed code
[ ] dependency injection into Cargo.toml without user confirmation
[ ] network-based analysis
[ ] ML-based code transformation
[ ] migration to every possible target
[ ] changing core matten APIs
```

---

## 5. Tool name and packaging

Suggested name:

```text
matten-migrate
```

Packaging options:

```text
crates/matten-migrate
```

or:

```text
tools/matten-migrate
```

Recommendation:

```text
tools/matten-migrate initially, publish=false,
until the report format and user value stabilize.
```

This avoids creating a public tool crate too early. Initial implementation, if ever
approved, should follow the **workspace-excluded, `publish = false`** tooling pattern used
by `benchmarks/` (kept out of the published-crate graph; covered for free by
`scripts/check-published-dependency-isolation.sh`). It must not become a published crate in
`crates/` until its value and stability are proven by a later full handoff.

---

## 6. Commands

Initial commands:

```bash
matten-migrate inspect .
matten-migrate report .
matten-migrate suggest --target ndarray .
matten-migrate suggest --target nalgebra .
```

Possible later commands:

```bash
matten-migrate list-targets
matten-migrate explain-api Tensor::matmul
matten-migrate check-bridges
```

Do not include in first version:

```bash
matten-migrate rewrite
matten-migrate apply
```

unless a later RFC explicitly authorizes automatic rewriting.

---

## 7. Detection model

First version may use simple, conservative detection:

```text
Cargo.toml dependencies
feature flags
source grep for known matten APIs
example file names
manual configuration file
```

It does not need a full Rust AST parser initially.

Possible detected APIs:

```text
Tensor::new / try_new / from_vec
reshape / flatten
sum / mean / axis reductions
matmul / dot / outer
norm / trace
var / std
concatenate / stack
dynamic APIs
matten-data APIs
matten-ndarray conversions
```

---

## 8. Output report

Generate Markdown:

```text
matten-migration-report.md
```

Include:

```text
summary
detected APIs
detected crates/features
likely production pressures
recommended target playbooks
bridge crate options
manual redesign warnings
next steps
```

---

## 9. Suggestion rules

Examples:

```text
Detected:
  matten-data + select_columns + fill_missing + try_numeric

Suggest:
  If table operations remain simple, stay with matten-data.
  If joins/groupby/pivot/query are needed, read the Polars/Pandas playbook.
```

```text
Detected:
  many matmul / dot / outer calls on rank-2 tensors

Suggest:
  read nalgebra playbook; consider matten-nalgebra bridge when available.
```

```text
Detected:
  axis reductions and N-D shapes

Suggest:
  read ndarray playbook; consider matten-ndarray bridge.
```

---

## 10. Safety and honesty rules

The tool must say:

```text
This is an advisory report.
It does not prove your project is production-ready.
It does not guarantee that a target library is better.
It does not perform automatic conversion.
```

No hidden telemetry.

No network calls.

No source upload.

---

## 11. Configuration

Optional config file:

```toml
# matten-migrate.toml
preferred_language = "rust"
preferred_targets = ["ndarray", "nalgebra"]
avoid_targets = ["python"]
notes = "production target must stay pure Rust"
```

This helps avoid bad suggestions.

---

## 12. Acceptance criteria

This RFC is implemented when:

```text
[ ] Tool can inspect Cargo.toml and source tree locally.
[ ] Tool can generate RFC-053-style Markdown report.
[ ] Tool suggests at least ndarray/nalgebra/Polars/Pandas/Candle playbooks.
[ ] Tool has no network behavior.
[ ] Tool makes no automatic code changes.
[ ] Tool is publish=false initially.
[ ] Docs clearly label it experimental/advisory.
```

---

## 13. Deferred future: assisted rewrite

A future RFC may consider limited assisted rewriting, but only after:

```text
migration guides are stable
bridge crates are stable
report format is accepted
users have requested automation
```

Possible safe rewrite candidates:

```text
add bridge crate dependency suggestion
replace direct shape comments with mapping notes
generate side-by-side example file
```

Risky rewrite candidates:

```text
rewrite numeric algorithms automatically
rewrite indexing semantics
rewrite dataframe pipelines
rewrite ML workflows
```

Those should remain deferred.

---

## 14. Risks

| Risk | Mitigation |
|---|---|
| Users trust suggestions too much | Strong advisory wording. |
| Tool becomes a code-rewriter too early | No rewrite command in v1. |
| Source privacy concerns | Local-only, no telemetry. |
| Poor suggestions | Allow config and conservative rules. |
| Maintenance burden | Start publish=false and small. |

---

## 15. Relationship to other RFCs

```text
RFC-050:
  defines migration strategy

RFC-051:
  defines bridge conversion contracts

RFC-052:
  defines target playbooks

RFC-053:
  defines report format

RFC-054:
  optionally automates report generation
```

This RFC must not be implemented before RFC-050 through RFC-053 have usable documentation.
