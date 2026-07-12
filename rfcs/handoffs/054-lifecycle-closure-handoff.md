# RFC-054 Lifecycle Closure Handoff

**Project:** `matten`
**Related RFC:** RFC-054: `matten-migrate` Assisted Migration Tool
**Document kind:** Lifecycle closure / status-resolution handoff
**Status:** Accepted; lifecycle closure implemented and prepared for implementation review
**Scope:** RFC bookkeeping and documentation-truth alignment only

---

## 1. Summary

RFC-054 has delivered the reviewed local advisory `matten-migrate` tool scope:

```text
inspect
report
list-targets
suggest --target
explain-api
check-bridges
```

The implemented tool remains:

```text
tools/matten-migrate
workspace-excluded
publish = false
local-only
advisory
heuristic
non-mutating except optional report file output
rewrite/apply explicitly rejected
no network
no telemetry
```

This handoff proposes a lifecycle decision, not new migration tooling:

```text
Close RFC-054 as implemented for the reviewed local advisory tool scope.
Extract rewrite/apply/source mutation and public-crate packaging to a future
separate RFC or explicit release-policy decision.
Move RFC-054 from rfcs/proposed/ to rfcs/done/ only after review acceptance.
```

The goal is to remove the stale "open because future automation exists"
ambiguity while preserving the prohibition on automatic migration behavior.

---

## 2. Pre-Closure State

Before this lifecycle-closure slice, RFC-054 remains in `rfcs/proposed/` with
this status shape:

```text
Accepted as future direction; local advisory inspect/report/suggest/
explain-api/check-bridges slices implemented; rewrite/apply and public crate
remain deferred.
```

The reviewed local advisory evidence now exists:

```text
tools/matten-migrate/Cargo.toml
tools/matten-migrate/README.md
tools/matten-migrate/src/main.rs
tools/matten-migrate/fixtures/
docs/src/migration/index.md
docs/src/migration/readiness-report.md
docs/src/contributing/release-checklist.md
rfcs/handoffs/054-deferred-implementation-note.md
rfcs/handoffs/054-matten-migrate-readiness-audit.md
rfcs/handoffs/054-matten-migrate-first-tool-handoff.md
rfcs/handoffs/054-matten-migrate-target-suggestion-handoff.md
rfcs/handoffs/054-matten-migrate-explain-api-handoff.md
rfcs/handoffs/054-matten-migrate-check-bridges-handoff.md
```

The remaining future ambitions are not required to keep RFC-054 open:

```text
rewrite/apply commands
source mutation
Cargo.toml editing or dependency injection
public `matten-migrate` crate
network or telemetry behavior
stronger correctness-oracle claims
JSON/HTML/richer output modes beyond the current reviewed CLI
```

---

## 3. Proposed Closure Boundary

If accepted, perform a docs/RFC bookkeeping slice only:

```text
rfcs/proposed/054-matten-migrate-assisted-migration-tool.md
  -> rfcs/done/054-matten-migrate-assisted-migration-tool.md
```

Update status/index wording to say:

```text
Implemented for the reviewed local advisory tool scope.
rewrite/apply, source mutation, public-crate packaging, and stronger migration
automation require a future separate RFC or explicit release-policy decision.
```

Do not implement:

```text
rewrite/apply commands
automatic source rewriting
Cargo.toml editing
dependency injection
public `crates/matten-migrate`
new bridge crates
new core `matten` APIs
network analysis
telemetry
ML-based or AST-based code transformation
new report formats or output modes
config file support unless separately reviewed
```

---

## 4. Required Checks Before Closure

Before moving RFC-054 to `done/`, verify:

```text
[ ] rfcs/README.md moves row 054 to Done and removes stale "Accepted as future direction" ambiguity
[ ] rfcs/README.md retargets row 054 from `./proposed/054-...` to `./done/054-...`
[ ] RFC-054 Status field matches its new `done/` location
[ ] RFC-054 no longer implies it stays in `proposed/` because rewrite/apply or a public crate remain
[ ] `rfcs/handoffs/050-054-review-request.md` is consciously left as a historical file-list entry if it still mentions `rfcs/proposed/054-...`
[ ] rewrite/apply/source mutation remain explicitly unauthorized and extracted to future RFC/policy ownership
[ ] public `matten-migrate` crate packaging remains explicitly unauthorized and extracted to future RFC/policy ownership
[ ] `tools/matten-migrate` remains documented as local-only, advisory, unpublished, and non-mutating
[ ] docs do not imply automatic migration, dependency editing, source rewriting, or correctness guarantees
[ ] docs/src/contributing/release-checklist.md continues to cover the current local tool commands
[ ] no source/runtime/API/dependency behavior changes
[ ] release-doc guard still passes
[ ] git diff --check passes
```

Recommended commands:

```text
bash scripts/check-release-docs.sh
git diff --check
```

No Rust build is required for a pure RFC/docs move unless implementation files
are touched.

---

## 5. Acceptance Criteria

The closure is accepted when:

```text
[ ] RFC-054 is no longer the only open proposed RFC solely because future automation exists
[ ] future rewrite/apply/source mutation requires a separate RFC or explicit release-policy decision
[ ] future public-crate packaging requires a separate RFC or explicit release-policy decision
[ ] reviewed local advisory artifacts remain discoverable
[ ] the tool is not reframed as an automatic migration engine, dependency editor, or correctness proof
[ ] no source/runtime/API/dependency behavior changes
```

---

## 6. Still Deferred

Still deferred after closure:

```text
rewrite/apply commands
source mutation
Cargo.toml editing
dependency injection
public `matten-migrate` crate
network or telemetry behavior
ML-based code transformation
AST-based code rewriting
automatic correctness claims
new bridge crates
new core `matten` APIs
JSON/HTML/richer output modes beyond the current reviewed CLI
config file support unless separately reviewed
```
