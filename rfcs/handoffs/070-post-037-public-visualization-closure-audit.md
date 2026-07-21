# RFC-070 Post-0.37 Public Visualization Closure Audit

**Project:** `matten`
**Related RFCs:** RFC-022, RFC-030, RFC-063, RFC-065, RFC-068, RFC-069, RFC-070, RFC-071
**Document kind:** Closure audit and next-theme recommendation
**Status:** Accepted; lifecycle alignment prepared; no public implementation authorized
**Date:** 2026-07-21

---

## 1. Purpose

Decide whether the evidence available after the `0.37.0` release makes
`matten-report` or `matten-viz` ready to become a public product surface, and
whether RFC-070 should remain open.

This audit is design and lifecycle work only. It does not authorize source-code
reorganization, new output modes, a public crate, a public API, dependency
changes, version changes, release preparation, tags, or publishing.

## 2. Evidence Since The First Audit

The first RFC-070 readiness audit found that local reporting was useful but not
ready for a public API. Subsequent reviewed work added two kinds of evidence:

1. A private shared HTML document shell reduced renderer duplication while
   preserving family-specific private report models.
2. RFC-071 added deterministic private JSON for all five fixed demos, using
   `schema_version: 0` and `schema_status: "private-local"`, released in
   `0.37.0`.

Current local-tool behavior includes:

```text
Markdown output for fixed demos and data-readiness input mode
static HTML output for fixed demos and data-readiness input mode
private JSON output for the five fixed demos
explicit-file-only HTML and JSON artifacts
bounded and escaped user-controlled input-mode HTML
exact deterministic report snapshots
```

The current implementation remains a workspace-excluded, `publish = false`
binary. `tools/matten-report/src/main.rs` is 5,023 lines and owns CLI parsing,
format policy, input handling, report data, Markdown/HTML/JSON rendering, file
output, and tests.

## 3. Readiness Reassessment

| Boundary | Post-0.37 evidence | Verdict |
|---|---|---|
| Local educational reports | Multiple released Markdown/HTML/JSON paths | Proven as local tooling |
| Report data model | Private family-specific structs and private JSON payloads | Not stable as a public model |
| Renderer API | Private functions in one binary | Not ready for a public API |
| JSON schema | Explicitly private schema version 0 | Not a public compatibility contract |
| Crate boundary | Workspace-excluded local tool depending on three family crates | Public dependency direction unresolved |
| Maintenance shape | 5,023-line mixed-responsibility source file | Modularization needed before feature expansion |
| Core `Tensor` boundary | No visualization API or computation graph | Keep closed |

Private JSON improves inspectability, but it deliberately avoids the stability
promise needed by downstream consumers. The evidence therefore strengthens the
local-tool case without establishing public-crate readiness.

## 4. Closure Verdict

Recommended verdict:

```text
RFC-070 completed its audit purpose.
Do not create public matten-report or matten-viz crates now.
Do not expose a public report model, renderer API, or JSON schema now.
Keep visualization and reporting outside core Tensor.
Close RFC-070 after this closure audit is accepted.
```

Closing RFC-070 means placing its RFC file in `rfcs/done/` and identifying its
status as implemented specifically as an audit decision. It does not mean a
public visualization feature was implemented.

## 5. Next Ordered Theme

After RFC-070 closure, the next theme should be a new RFC for
behavior-preserving `matten-report` modularization.

The new RFC should define, before implementation:

```text
requirements and explicit non-goals
module ownership for CLI/config, domain report data, renderers, input/I/O, and tests
allowed dependency directions between those modules
whether report-family modules own their payloads or share internal primitives
how Markdown, HTML, and JSON exact-output behavior is preserved
how CLI errors, escaping, display bounds, and explicit-file policies are preserved
an incremental file-movement sequence with reviewable checkpoints
line-count and test-placement targets under the Rust CLI project rules
verification gates for the workspace-excluded tool
```

The modularization RFC must remain behavior-neutral unless review explicitly
authorizes a small correction. It must not use refactoring as authority for a
public API or output-format expansion.

## 6. Theme After Modularization

The following are candidates, not an ordered implementation queue:

| Candidate | Required authority | Main unresolved question |
|---|---|---|
| Private input-mode JSON | New RFC | Bounds, non-finite values, failure payloads, and private schema evolution |
| Broader mathematics | New RFC | Exact scope: determinant/linalg boundary or statistics policy |
| New ecosystem bridge | Per-crate RFC | Start with `nalgebra` or justify another target; define conversion contract and dependency isolation |

Selection should occur only after modularization is reviewed and completed.
None of these candidates is authorized by RFC-070 closure.

## 7. Explicit Deferrals

This audit does not authorize:

```text
public matten-report or matten-viz crate
public report model, renderer API, or JSON schema
input-mode JSON
SVG or Vega-Lite output
notebook, browser runtime, dashboard, GUI, or server integration
Tensor::plot(), Tensor::show(), or Tensor::backward()
expression tracing, computation graphs, or autograd
determinant, inverse, decompositions, or broader statistics
nalgebra, candle, streaming, or large-CSV implementation
dependency changes in published crates
source-code modularization before its RFC and handoff are reviewed
version bump, release preparation, tag, or publish action
```

## 8. Lifecycle Actions After Acceptance

The accepted audit authorized this lifecycle alignment, now prepared for its
own review:

1. Move RFC-070 into `rfcs/done/` from its proposed state.
2. Mark RFC-070 implemented as an audit decision after `0.37.0`.
3. Update `rfcs/README.md`, `ROADMAP.md`, the handoff index, and inbound links
   to record closure.
4. Draft the next numbered RFC for `matten-report` modularization.
5. Do not begin modularization until that RFC and its implementation handoff
   pass review.

## 9. Review Questions

Review should decide:

```text
[ ] Does private fixed-demo JSON strengthen local tooling without proving a public schema?
[ ] Is RFC-070 complete as an audit even though no public crate was authorized?
[ ] Should RFC-070 move to done only after this audit is accepted?
[ ] Is behavior-preserving matten-report modularization the correct next RFC-first theme?
[ ] Are private input-mode JSON, broader mathematics, and ecosystem bridges correctly deferred until after modularization?
[ ] Are the modularization design questions sufficient for the next RFC draft?
[ ] Does the audit avoid authorizing code, public API, dependencies, or release work?
```
