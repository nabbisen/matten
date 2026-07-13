# RFC-067: v1.0 Family Maturity Policy

**Status:** Implemented (repository policy; no v1.0 release authorization)
**Target:** v1.0 decision policy; no release authorization
**Theme:** Resolve RFC-066 MD-1 for lock-step v1.0 family composition
**Depends on:** RFC-030, RFC-057, RFC-058, RFC-059, RFC-066

---

## 1. Summary

This RFC resolves the RFC-066 maintainer-decision finding MD-1:

```text
Can a lock-step v1.0 family include production-ready-candidate companions
with explicit labels, or must all family crates become production-ready first?
```

Proposed decision:

```text
A candidate-labeled companion is not automatically a v1.0 blocker.
```

The lock-step v1.0 family may include a companion whose maturity label remains
`production-ready candidate` if all of the following are true:

```text
the crate's public API is stable enough for v1.0 discussion;
the candidate reason is an explicit scope or workflow caveat, not hidden API churn;
the root README, crate README, release notes, and v1.0 release RFC state the label;
the v1.0 release RFC confirms the crate is included intentionally;
no wording implies that v1.0 silently promotes the crate to production-ready.
```

This RFC does not authorize a v1.0 release, version bump, tag, publish, API
change, dependency change, or companion maturity promotion.

## 2. Motivation

RFC-030 deliberately split version and maturity:

```text
Version = compatibility.
Maturity = Status label.
```

That split has worked across the v0.x family. The current family status is:

```text
matten          stable (v0.x)
matten-ndarray production-ready
matten-mlprep  production-ready candidate
matten-data    production-ready candidate
```

RFC-066 found the project close enough for v1.0 discussion after BF-1 remediation,
but it left MD-1 open. A v1.0 release decision needs an explicit rule so the
project does not either:

```text
silently promote candidate companions by version number; or
silently block v1.0 just because a companion has an honest candidate label.
```

## 3. Decision

Adopt this v1.0 family maturity policy:

```text
Production-ready candidate status is compatible with a lock-step v1.0 family
only when the candidate label is explicit and the v1.0 release RFC confirms the
crate's API stability and documented caveats.
```

The v1.0 release RFC must include a family maturity table:

```text
crate
version
maturity label
public API stability assessment
candidate caveat, if any
release decision for inclusion
```

For each candidate-labeled companion, the v1.0 release RFC must answer:

```text
Is the public API stable enough for v1.0?
Is the candidate label due to an acceptable documented scope caveat?
Does the crate README avoid stale "pre-1.0" wording after a v1.0 release?
Does the release note make the candidate label visible?
Should the crate remain in the lock-step v1.0 family, or does it need a
separate maturity RFC first?
```

If the answer is not clear, the crate needs a follow-up maturity RFC before
v1.0 release preparation.

## 4. Current Application

This RFC does not itself decide a v1.0 release. If accepted, it says only that
the current candidate labels are not automatic blockers.

Current expected v1.0 release-RFC checks:

```text
matten-mlprep:
  candidate caveat: ordered-only train_test_split, no shuffle.
  v1.0 release RFC must confirm whether this is acceptable within scope.

matten-data:
  candidate caveat: small CSV/table-to-Tensor on-ramp; not dataframe, not
  streaming or large CSV.
  v1.0 release RFC must confirm whether this is acceptable within scope.
```

The v1.0 release RFC may still decide that either companion needs a production-
ready promotion RFC first. This policy prevents only an automatic block.

## 5. Rationale

The alternative "all crates must be production-ready before any lock-step v1.0"
is simple, but it re-couples version and maturity in the strongest possible way.
That weakens the RFC-030 model and can turn honest scope labels into release
deadlocks.

The alternative "candidate companions are always allowed" is too loose. A v1.0
version is user-visible and should not hide instability behind a label.

This RFC chooses the middle rule:

```text
candidate label allowed, but not silent;
v1.0 family inclusion allowed, but not automatic;
API stability required, scope caveats disclosed.
```

This keeps the family version useful as a compatibility signal while preserving
the Status label as the maturity signal.

## 6. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | None |
| Runtime behavior | None |
| Feature flags | None |
| Dependencies | None |
| Release process | Adds an explicit family maturity table to any future v1.0 release RFC |
| Maturity labels | No automatic change |

## 7. Acceptance Criteria

This RFC is accepted when reviewers agree that:

```text
[ ] it resolves RFC-066 MD-1 explicitly;
[ ] it does not authorize v1.0 release preparation;
[ ] it preserves RFC-030's version-vs-maturity split;
[ ] it prevents candidate labels from being hidden in a v1.0 release;
[ ] it requires any future v1.0 release RFC to decide candidate inclusion
    crate by crate;
[ ] it does not promote matten-mlprep or matten-data.
```

## 8. Non-goals

This RFC does not:

```text
authorize v1.0 release preparation;
change any crate version;
publish or tag anything;
promote matten-mlprep or matten-data;
change public APIs;
change dependencies;
add or remove crates from the workspace;
settle the final v1.0 release decision.
```

## 9. Follow-up Work

The v1.0 audit and release-prep documentation record:

```text
MD-1 is resolved by RFC-067.
Any future v1.0 release RFC must include the family maturity table and
candidate-inclusion checks required here.
```

That follow-up does not authorize release preparation by itself.
