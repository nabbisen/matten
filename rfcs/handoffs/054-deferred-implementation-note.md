# Deferred Implementation Note — RFC-054 `matten-migrate` Assisted Migration Tool

**Project:** `matten`  
**Document kind:** Deferred implementation note  
**Scope:** RFC-054 only  
**Status:** Superseded for the first local advisory slice; retained as historical deferral record
**Recommended revisit:** satisfied for the first advisory tool slice; rewrite/apply remain deferred

---

## 1. Decision

This note originally deferred all `matten-migrate` implementation. That
deferral has now been superseded for the first local advisory tool slice only:

```text
tools/matten-migrate
inspect / report / list-targets
workspace-excluded
publish = false
local-only
non-mutating
```

The reopening and implementation records are:

```text
rfcs/handoffs/054-matten-migrate-readiness-audit.md
rfcs/handoffs/054-matten-migrate-first-tool-handoff.md
```

The original prohibition still applies to public crates, rewrite/apply flows,
automatic source conversion, network/telemetry behavior, and any stronger
correctness-oracle claims.

RFC-054 was accepted only as a future direction until the following were stable:

```text
[x] RFC-050 migration guide exists
[x] RFC-051 bridge conversion contracts exist
[x] RFC-052 target playbooks exist
[x] RFC-053 readiness report template exists
[x] at least one worked readiness report exists
[x] users or maintainers still want tool support after using the docs
```

---

## 2. Why defer?

The tool depends on concepts that must stabilize first:

```text
target-selection logic
bridge contract language
report format
production-pressure signals
safe wording around suggestions
```

Implementing the CLI before those are stable would risk:

```text
overpromising automatic conversion
creating brittle source scanning
locking in immature target recommendations
turning a documentation problem into a tooling problem
```

---

## 3. What is allowed now?

Allowed:

```text
[✓] mention RFC-054 as future direction
[✓] create a placeholder docs note
[✓] create manual migration readiness report template
[✓] collect tool requirements from users
[✓] maintain the reviewed local advisory `tools/matten-migrate` slice
```

Not allowed:

```text
[ ] create crates/matten-migrate
[ ] implement Cargo.toml editing
[ ] implement rewrite/apply commands
[ ] add telemetry/network behavior
```

---

## 4. Future full handoff trigger

The first local advisory handoff was created after these triggers became true.
Future tool expansion still needs a separate handoff/review gate.

Original trigger list:

```text
[x] RFC-050–053 are implemented and reviewed
[ ] maintainers have used the manual report template on real projects
[x] at least two migration target playbooks are stable
[x] there is a concrete request for automation
```

---

## 5. Future first tool scope

The first RFC-054 implementation started with advisory-only scope:

```bash
matten-migrate inspect .
matten-migrate report .
matten-migrate list-targets
```

Do not include:

```bash
matten-migrate rewrite
matten-migrate apply
```

unless a separate future RFC explicitly approves assisted rewriting.

---

## 6. Safety promises for future tool

Any future tool must be:

```text
local-only
no telemetry
no network calls
no source upload
advisory
non-mutating by default
```

The first version should generate:

```text
matten-migration-report.md
```

not rewritten source code.

---

## 7. Final instruction to developers

Treat RFC-054 as:

```text
first local advisory slice implemented
future expansion still requires explicit handoff/review
rewrite/apply still deferred
```

If further implementation pressure appears, return for a full handoff before
writing code.
