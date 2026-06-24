# Deferred Implementation Note — RFC-054 `matten-migrate` Assisted Migration Tool

**Project:** `matten`  
**Document kind:** Deferred implementation note  
**Scope:** RFC-054 only  
**Status:** Do not implement yet  
**Recommended revisit:** after RFC-050–053 are implemented and reviewed  

---

## 1. Decision

Do **not** implement `matten-migrate` now.

RFC-054 is accepted only as a future direction until the following are stable:

```text
[ ] RFC-050 migration guide exists
[ ] RFC-051 bridge conversion contracts exist
[ ] RFC-052 target playbooks exist
[ ] RFC-053 readiness report template exists
[ ] at least one worked readiness report exists
[ ] users or maintainers still want tool support after using the docs
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
```

Not allowed:

```text
[ ] create tools/matten-migrate
[ ] create crates/matten-migrate
[ ] implement source scanning
[ ] implement Cargo.toml editing
[ ] implement rewrite/apply commands
[ ] add telemetry/network behavior
```

---

## 4. Future full handoff trigger

Create a full developer handoff for RFC-054 only when at least one of these is true:

```text
[ ] RFC-050–053 are implemented and reviewed
[ ] maintainers have used the manual report template on real projects
[ ] at least two migration target playbooks are stable
[ ] there is a concrete request for automation
```

---

## 5. Future first tool scope

When RFC-054 eventually starts, first scope should be advisory only:

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
future approved direction
not current implementation work
```

If implementation pressure appears, return for a full handoff before writing code.
