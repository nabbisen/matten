# RFC-054 `matten-migrate` Readiness Audit

**Project:** `matten`
**Related RFC:** RFC-054: `matten-migrate` Assisted Migration Tool
**Document kind:** Readiness audit
**Status:** Reviewed GO; retained as RFC-054 reopening decision record
**Date:** 2026-07-11

---

## 1. Summary

RFC-054 can be reopened for a first-tool handoff, but not for direct
implementation yet.

The original deferred note required a stable migration-doc foundation before any
tool work. That foundation now mostly exists:

```text
RFC-050 production migration guide: done
RFC-051 bridge conversion contracts: done
RFC-052 target playbooks: done
RFC-053 readiness report template: done
worked readiness report: exists
```

The missing piece is not documentation maturity. The missing piece is an
accepted, narrow first-tool handoff that proves `matten-migrate` will stay
advisory, local-only, non-mutating, and honest about its limitations.

Audit recommendation:

```text
Open RFC-054 for first-tool handoff drafting.
Do not implement tools/matten-migrate until that handoff is reviewed and accepted.
Keep the first possible tool scope inspect/report/list-targets only.
Keep rewrite/apply/source mutation explicitly out of scope.
```

---

## 2. Trigger Audit

The deferred implementation note says to create a full developer handoff only
when at least one trigger is true.

| Trigger | Audit result | Evidence |
|---|---|---|
| RFC-050-053 are implemented and reviewed | Satisfied | `rfcs/README.md` lists RFC-050, RFC-051, RFC-052, and RFC-053 in `done/`. |
| Maintainers have used the manual report template on real projects | Not proven | One worked report exists for an example, but the repo does not show a real downstream project report. |
| At least two migration target playbooks are stable | Satisfied | Five playbooks are present: `ndarray`, `nalgebra`, Polars/Pandas, Candle, and NumPy. |
| Concrete request for automation | Partial | The maintainer asked when RFC-054 can be opened and requested this audit, but has not yet asked for a specific scanner/report command implementation. |

The first and third triggers are enough to justify a handoff. The partial
automation signal is not enough to skip handoff review.

---

## 3. Stabilized Inputs

The following inputs now exist and are stable enough to feed a first-tool
handoff:

```text
docs/src/migration/index.md
docs/src/migration/when-to-migrate.md
docs/src/migration/target-selection.md
docs/src/migration/common-pitfalls.md
docs/src/migration/bridge-contracts.md
docs/src/migration/bridge-crate-policy.md
docs/src/migration/readiness-checklist.md
docs/src/migration/readiness-report.md
docs/src/migration/examples/linear-regression-gd-readiness.md
docs/src/migration/playbooks/
```

These provide:

```text
target-selection vocabulary
production-pressure signals
manual report sections
bridge-contract boundaries
safe advisory disclaimers
per-target migration guidance
```

The first tool should reuse those names and disclaimers rather than inventing a
new decision model.

---

## 4. Remaining Risks

The original deferral risks still matter:

```text
overpromising automatic conversion
brittle source scanning
immature target recommendations
turning a documentation problem into a tooling problem
```

Those risks are manageable only if the first handoff forbids:

```text
rewrite/apply commands
Cargo.toml editing
source mutation
network or telemetry
ML-based transformation
compile-time correctness claims
automatic production-readiness claims
```

The first tool may inspect and report. It must not mutate.

---

## 5. Recommended First Handoff Scope

Draft a compact implementation handoff for:

```text
tools/matten-migrate
publish = false
workspace-excluded
local-only
advisory-only
```

Allowed first commands:

```text
matten-migrate inspect <path>
matten-migrate report <path>
matten-migrate list-targets
```

Potentially allowed only if the handoff specifies conservative behavior:

```text
matten-migrate explain-api <api-name>
```

Not allowed:

```text
matten-migrate rewrite
matten-migrate apply
automatic Cargo.toml edits
source rewrites
network lookups
telemetry
```

The handoff should define:

```text
detection model
input path policy
output report format
advisory wording
false-positive limitations
test fixtures
CI/release-checklist commands
dependency policy
```

---

## 6. Audit Verdict

```text
READY TO DRAFT FIRST-TOOL HANDOFF
NOT READY FOR DIRECT IMPLEMENTATION
```

Reason:

```text
The migration docs and playbooks are mature enough to guide a tool.
The tool boundary still needs explicit review before code exists.
```

Next action:

```text
Draft RFC-054 first-tool handoff for review.
```
