# RFC-075: v1.0 Release Decision

**Status:** Implemented — reviewed and accepted (`matten-rfc075-v1-release-decision-review-v0.1.md`, GO); MD-2 resolved, serde format declared stable, RFC-067 family maturity table recorded; no v1.0 release authorized
**Target:** Post-RFC-074 v1.0 prerequisite closure; no version bump or release authorization
**Theme:** Resolve RFC-074 MD-2, declare the serde canonical format stable, and
record the RFC-067 family maturity table, closing every non-code prerequisite
Path B named
**Depends on:** RFC-009, RFC-030, RFC-058, RFC-059, RFC-066, RFC-067, RFC-071, RFC-074
**Related:** RFC-015, RFC-057

---

## 1. Summary

This RFC is the maintainer-decision document RFC-074 recommended as "Path B"
(pursue v1.0 deliberately). It resolves the one open finding no audit or
review could resolve on its own — **MD-2** — and closes the remaining
non-code prerequisites RFC-074 listed for a future v1.0 release RFC:

```text
resolve MD-2 (RFC-030 lock-step versioning's unfired RFC-071 §6
  reconsideration trigger, and whether the project is heading toward 1.0
  or "0.x indefinitely");
declare the JSON canonical serde format explicitly stable;
record the RFC-067 family maturity table, deciding matten-mlprep and
  matten-data's inclusion in a future v1.0 family at their current
  production-ready-candidate label.
```

NF-1 and NF-2, RFC-074's other two prerequisites, are already closed (README
Public API blocks added; `cargo public-api` documented as a manual
release-checklist step).

**This RFC does not authorize a v1.0 release, version bump, tag, publish,
API change, dependency change, or companion maturity promotion.** Per
RFC-074 H3, resolving these prerequisites is a separate step from proposing
release preparation; a further, separate v1.0 release-prep RFC is required
before any release action, and must run the full gate set RFC-074's
documentation-only re-audit explicitly did not run.

## 2. Motivation

RFC-074 found the technical/API surface **conditionally ready** — every
RFC-066 gate cleared, more strongly evidenced than at the original audit
because the public API has had zero functional change across eight releases.
But it declined to recommend starting release preparation, because three
process items remained open and one new maintainer-decision finding (MD-2)
existed that no audit could resolve on the maintainer's behalf.

The RFC-074 review (`matten-rfc074-v1-readiness-reaudit-review-v0.1.md`)
independently confirmed the audit's findings and reasoned that Path B is
substantially cheaper here than for a typical project: a 1.0 compatibility
freeze constrains future API changes, but this project has already
demonstrated that freeze in practice for eight releases. The choice of Path B
over Path A was made explicitly by the project owner, not defaulted into.

This RFC is the vehicle for making that choice concrete and closing the
prerequisites it implies, without smuggling in an actual release decision.

## 3. Decision

### 3.1 MD-2 Resolution

MD-2 asked: should RFC-030 lock-step versioning continue to release a family
checkpoint regardless of content for private-tool-only milestones, or should
RFC-071 §6's reconsideration trigger now fire?

**Resolution: keep RFC-030 lock-step versioning unchanged, decided
consciously, with one added process requirement:**

```text
Any future release whose entire scope is local-tool-only (workspace-excluded,
publish = false crates such as tools/matten-report or tools/matten-migrate)
must include an explicit one-line justification in its CHANGELOG entry
stating why a lock-step family checkpoint is being released despite no
published-crate change, rather than relying on the RFC-071 precedent
silently.
```

This requirement is recorded operationally in
`docs/src/contributing/release-checklist.md` §7 (CHANGELOG) and in
`CHANGELOG.md`'s conventions blockquote, not only here — RFC-071 §6's
trigger lived in exactly one place (an RFC that moved to `rfcs/done/`) and
was bypassed silently across all eight releases this RFC exists to address.
Stating the rule once, in an RFC alone, reproduces that failure mode; the
RFC and the operational location must point at each other so neither drifts
silently.

This is deliberately the lighter of the two options RFC-074 raised (the
other being a separate unpublished coordinate for local-tool milestones).
RFC-030's core rationale — the crates only ship together, and a matched
version number is the simplest compatibility contract when a release changes
something — is unaffected by this finding. The problem was narrower: a
self-imposed reconsideration trigger went unchecked eight times, not that
the versioning model itself is wrong. A one-line CHANGELOG justification
requirement is enough to make the reconsideration trigger structurally hard
to bypass silently again, without redesigning the versioning model over a
process gap.

This resolution stands independently of any v1.0 decision. It applies to the
next local-tool-only release regardless of whether v1.0 release preparation
ever begins.

### 3.2 Serde Canonical Format Declaration

**Decision: the JSON canonical object form is declared stable as of this
RFC.**

```json
{"shape": [...], "data": [...]}
```

Covered by this declaration:

```text
the canonical object form's field names, field order is not significant
  (standard JSON object semantics), and the shape/data type contract;
Serialize/Deserialize for Tensor using this form (feature "serde" or "json");
round-trip fidelity for any numeric Tensor.
```

Explicitly **not** covered, and remaining free to change without a
compatibility break to the canonical form:

```text
the rank-1/rank-2 nested-array convenience input forms accepted by
  from_json (these remain convenience parsing, not the canonical output form);
CSV, which remains documented as ingestion, not canonical tensor
  serialization (RFC-009, RFC-074 §"Serde/Format Review");
Serialize's behavior for dynamic tensors (returns a serde error; unaffected
  by this declaration, which covers numeric tensors only).
```

Basis for this declaration: `crates/matten/src/ser.rs` has had zero
functional churn since `0.31.0` (RFC-074), and the canonical form has been
documented identically (`docs/src/reference/boundary.md`) across that entire
span. This declaration formalizes an already-stable fact; it does not change
behavior.

### 3.3 RFC-067 Family Maturity Table

Per RFC-067, any future v1.0 release RFC must include this table. Recording
it here lets a future release-prep RFC cite a settled answer rather than
re-litigating it.

| Crate | Current version | Maturity label | Public API stability | Candidate caveat | v1.0 family inclusion |
|---|---|---|---|---|---|
| `matten` | `0.38.x` family | stable (v0.x) | Stable; zero churn `0.31.0`→`0.38.0` (RFC-074) | none | **Include** |
| `matten-ndarray` | `0.38.x` family | production-ready (RFC-057) | Stable; zero churn | none | **Include** |
| `matten-mlprep` | `0.38.x` family | production-ready candidate (RFC-058) | Stable; zero churn | `train_test_split` is ordered-only, no shuffle/seed (RFC-024 §6, still planned not implemented); explicit, documented, not hidden API churn | **Include, at candidate label** |
| `matten-data` | `0.38.x` family | production-ready candidate (RFC-059) | Stable; zero churn | CSV-only ingestion; not a dataframe engine; no streaming (RFC-042 scope lock, RFC-037); explicit, documented, not hidden API churn | **Include, at candidate label** |

Per-crate RFC-067 checklist:

```text
matten-mlprep:
  Is the public API stable enough for v1.0? Yes (RFC-074 zero-churn evidence).
  Is the candidate label an acceptable documented scope caveat, not hidden
    churn? Yes — the ordered-split limitation is stated in the crate README
    and RFC-024 §6.
  Should it need a separate promotion RFC before v1.0? No — RFC-058 already
    deferred full-production-ready to "a separate future review" as a
    distinct, later decision; this RFC does not spend that review to gate
    v1.0 inclusion.

matten-data:
  Is the public API stable enough for v1.0? Yes (RFC-074 zero-churn evidence).
  Is the candidate label an acceptable documented scope caveat, not hidden
    churn? Yes — the "not a dataframe library" scope lock (RFC-042) is
    CI-enforced and stated in the crate README.
  Should it need a separate promotion RFC before v1.0? No, for the same
    reason as matten-mlprep.
```

Both companions may enter a future v1.0 family at their current
`production-ready candidate` label. Neither is promoted by this RFC; no
maturity label changes here.

Two release-prep tasks this table surfaces for a future v1.0 release RFC to
execute (not authorized here):

```text
crate READMEs currently say "pre-1.0; pin the minor version" — a v1.0
  release RFC must update this wording so it does not read as stale after
  an actual 1.0 release;
the v1.0 release notes must state each candidate label explicitly, per
  RFC-067's "no wording implies... silently promotes" requirement.
```

## 4. Rationale

RFC-074 explicitly declined to resolve MD-2 itself, correctly treating "0.x
indefinitely vs. heading to 1.0" as a decision only the maintainer can make —
the same posture RFC-066 took toward MD-1, resolved separately by RFC-067.
This RFC follows that precedent exactly: one focused decision document,
scoped to policy and prerequisite closure, not bundled with release
preparation.

The MD-2 resolution (§3.1) is deliberately the smaller intervention. RFC-074
and its review both note that RFC-030's core rationale is sound; the failure
was a specific, narrow one (a named reconsideration trigger never fired). A
process requirement that makes the next occurrence visible in the CHANGELOG
is proportionate. A larger versioning-model change was considered and
rejected as unnecessary for the same reason RFC-030 was adopted in the first
place: the crates still only ship together.

The serde declaration (§3.2) and family maturity table (§3.3) formalize
already-true facts rather than deciding new ones — consistent with RFC-074's
finding that the technical surface is already stable. Declaring them
explicitly is what `compatibility.md`'s v1.0 gate list requires; the facts
underneath were already established by zero-churn evidence.

## 5. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | None |
| Runtime behavior | None |
| Feature flags | None |
| Dependencies | None |
| Version | None — no bump |
| Release process | Adds a CHANGELOG justification requirement for future local-tool-only releases (§3.1); adds a serde-stability declaration reference point (§3.2); adds a settled family maturity table for a future v1.0 release RFC to cite (§3.3) |
| Maturity labels | No change — `matten-mlprep`/`matten-data` remain `production-ready candidate` |

## 6. Acceptance Criteria

This RFC is accepted when reviewers agree that:

```text
[ ] it resolves RFC-074 MD-2 explicitly, with a concrete, checkable process
    requirement rather than a vague intention;
[ ] the MD-2 resolution does not redesign RFC-030 beyond what the finding
    actually supports;
[ ] the serde canonical-format declaration states exactly what is and is
    not covered;
[ ] the RFC-067 family maturity table is complete and per-crate checklist
    items are answered, not deferred;
[ ] it does not authorize a v1.0 release, version bump, tag, publish, API
    change, dependency change, or maturity promotion;
[ ] it clearly hands off to a separate future v1.0 release-prep RFC rather
    than proposing release preparation itself.
```

## 7. Non-goals

This RFC does not:

```text
[ ] authorize a v1.0 release
[ ] bump any crate version
[ ] create a tag or publish anything
[ ] change any public API
[ ] change any dependency
[ ] promote matten-mlprep or matten-data
[ ] run cargo public-api and approve a snapshot (a future release-prep RFC's task)
[ ] update crate README "pre-1.0" wording (a future release-prep RFC's task)
[ ] redesign RFC-030 lock-step versioning beyond the §3.1 process requirement
[ ] decide anything about tools/matten-report, tools/matten-migrate, or any
    other backlog theme from RFC-070's remaining-themes table
```

## 8. Follow-up Work

If this RFC is accepted, a **separate** future v1.0 release-prep RFC may be
proposed. Per RFC-074 H2 step 5, it must run the full gate set the re-audit
explicitly did not run before any release action:

```text
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
the complete feature matrix (docs/src/contributing/release-checklist.md §2)
MSRV build/test (cargo +1.85.0 build / test)
cargo public-api, taken and approved, for matten and every companion
cargo package --workspace
```

That release-prep RFC must also update the "pre-1.0" wording this RFC's §3.3
identified, state each candidate label explicitly in release notes per
RFC-067, and perform the actual version bump, tag, and publish sequence —
none of which this RFC authorizes.
