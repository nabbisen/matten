# RFC Implementation Handoffs

This directory holds **implementation handoffs** for proposed RFCs. A handoff
translates an accepted RFC's design into developer-executable work: module layout,
PR boundaries, task breakdown, acceptance checklists, edge cases, and CI commands.

The RFC remains the design authority; a handoff never overrides it. Handoffs are
working documents for the implementation team and may be revised as work proceeds.

## Current handoffs (v0.20+ proposed set, RFC-033–042)

| Handoff | RFC |
|---|---|
| `033-implementation-handoff.md` | RFC-033 `matten-data` Beta-Decision and Scope Lock |
| `034-implementation-handoff.md` | RFC-034 `matten-data` Table Model and Public API Boundary |
| `035-implementation-handoff.md` | RFC-035 CSV Ingestion, Schema Summary, Missing Values, and Numeric Conversion |
| `036-implementation-handoff.md` | RFC-036 `matten-data` Examples, Documentation, and Release Gate |
| `037-implementation-handoff.md` | RFC-037 Deferred Streaming and Large CSV Policy |
| `038-implementation-handoff.md` | RFC-038 Core Numeric Comfort APIs |
| `039-implementation-handoff.md` | RFC-039 Shape Composition API Boundary |
| `040-implementation-handoff.md` | RFC-040 Small Statistics Boundary — Core vs Companion |
| `041-implementation-handoff.md` | RFC-041 Linear Algebra Boundary — Core Lite vs External Crates |
| `042-implementation-handoff.md` | RFC-042 Pandas-Inspired Scope Guard for `matten-data` |

These RFCs are **proposed** (design only). No implementation is authorized by their
presence here; each requires per-RFC acceptance before coding begins.
