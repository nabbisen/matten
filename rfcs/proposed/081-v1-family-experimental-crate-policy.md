# RFC-081: `Experimental` Crates in a Lock-Step v1.0 Family

**Status:** Proposed; policy decision only, no v1.0 release authorized
**Target:** Post-`0.39.0` policy decision; unblocks RFC-076's rehabilitation
**Theme:** Answer the question RFC-067 did not reach — may a lock-step v1.0 family include a crate
labelled `Experimental`? — and refresh RFC-076's stale inventory
**Depends on:** RFC-022, RFC-029, RFC-030, RFC-040, RFC-057, RFC-058, RFC-059, RFC-067, RFC-078, RFC-080
**Related:** RFC-066, RFC-074, RFC-075, RFC-076, RFC-079

---

## 1. Summary

RFC-067 resolved MD-1 — whether a lock-step v1.0 family may include a `production-ready candidate`
companion — and answered yes, under five stated conditions. It never addressed `Experimental`, because at
the time no family crate carried that label.

One does now. `matten-stats` shipped in `0.39.0` at `Experimental` (RFC-078, RFC-040 §9), and is live on
crates.io. So a v1.0 release RFC must answer a question no accepted RFC has:

```text
May a lock-step v1.0 family contain a crate labelled Experimental?
```

**Proposed decision: no — not under the current lock-step model.** An `Experimental` crate must either be
promoted, or be removed from the lock-step family, before that family reaches `1.0.0`. §3 states the rule
and the two exits; §5 explains why this differs from RFC-067's answer for `candidate`.

This RFC also refreshes RFC-076's now-stale inventory (§6), which is the concrete reason it is being
written now.

**No v1.0 release, version bump, tag, publish, API change, dependency change, or maturity promotion is
authorized by this RFC.**

## 2. Motivation

`rfcs/proposed/076-v1-release-preparation.md` is the only document in `proposed/`, and it has been
accumulating staleness since it was accepted and deferred:

```text
1. it assumes a four-crate family                     (matten-stats made it five)
2. it lists matten-mlprep as production-ready candidate (RFC-080 promoted it)
3. its RFC-067 maturity table has no matten-stats row
4. it cannot answer whether an Experimental crate may ship in a 1.0 family
```

Items 1-3 are inventory. **Item 4 is a policy question**, and RFC-076 cannot resolve it for itself — the
same structural reason RFC-066 could not resolve MD-1 and RFC-074 could not resolve MD-2. Both were settled
by a separate decision RFC (RFC-067, RFC-075). This RFC follows that pattern.

Leaving RFC-076 to decay is the alternative, and it is the one this project has already named as a failure
mode: RFC-074's MD-2 finding was precisely about a decision going unmade while its context drifted.

## 3. Decision

**A lock-step v1.0 family may not include a crate labelled `Experimental`.**

Before the family reaches `1.0.0`, each `Experimental` crate must take one of two exits:

```text
EXIT A — promote it
  The crate advances at least to production-ready candidate, via its own
  promotion RFC, audited against the RFC-057 bar exactly as RFC-058, RFC-059,
  and RFC-080 were. RFC-067's five conditions then govern its inclusion.

EXIT B — remove it from the lock-step family
  The crate leaves lock-step versioning and publishes on its own cadence, or is
  withdrawn from crates.io, or is moved out of the workspace. This requires its
  own RFC amending RFC-030's membership, and is the honest choice for a crate
  the project wants to keep iterating on freely.
```

**The two exits are not equal-cost, and this RFC should not present them as though they were.**
`matten-stats 0.39.0` is already published on crates.io under lock-step versioning (RFC-078,
RFC-079's post-release alignment). Exit A is a normal promotion RFC, the same shape as RFC-058,
RFC-059, and RFC-080 — cheap, precedented, reversible in the sense that the crate keeps shipping
under the same name and version line throughout. Exit B, taken *now*, means one of: publishing
the crate on a divergent version line (contradicting the matched-version contract every existing
`matten`/`matten-stats` install pin already implies), or withdrawing a crate that has already
shipped. Neither is cheap, and withdrawal is not fully reversible — a withdrawn version stays
withdrawn on crates.io's history even if later versions resume.

**Exit A is therefore the expected path. Exit B remains available, but whoever takes it should
read this paragraph first, not discover the cost mid-execution.**

A v1.0 release RFC must record which exit each `Experimental` crate took, and may not proceed while any
remains unresolved.

### 3.1 What this does not change

```text
RFC-067's answer for production-ready candidate stands unchanged — candidates
  may ship in a 1.0 family under its five conditions
no crate is promoted or demoted by this RFC
matten-stats keeps its Experimental label today; 0.39.x is not a 1.0 family
RFC-030 lock-step versioning is unchanged for the 0.x line
```

## 4. Current application

| Crate | Label | Status against this rule |
|---|---|---|
| `matten` | stable | unaffected |
| `matten-ndarray` | production-ready (RFC-057) | unaffected |
| `matten-mlprep` | production-ready (RFC-080) | unaffected |
| `matten-data` | production-ready candidate (RFC-059) | permitted by RFC-067's five conditions |
| `matten-stats` | **Experimental** (RFC-078) | **must take Exit A or Exit B before any 1.0** |

`matten-stats` is the only crate this rule binds, and it binds nothing today — `0.39.x` is not a v1.0
family. The obligation attaches only when a v1.0 release is actually proposed.

## 5. Rationale — why `Experimental` differs from `candidate`

RFC-067 permitted candidates because the candidate label denotes *"an explicit scope or workflow caveat,
not hidden API churn"* — the crate's surface is settled; only its recommendation breadth is qualified. That
reasoning genuinely does not extend to `Experimental`:

- **`Experimental` denotes an unsettled surface.** RFC-040 §9 assigned it to `matten-stats` precisely
  because the crate had no usage history. RFC-078 §1 said so explicitly: a new crate's API benefits most
  from being changeable.
- **`1.0.0` is a compatibility commitment.** Shipping a crate whose label announces instability, inside a
  family whose version number announces stability, is a contradiction users cannot reasonably resolve.
  Lock-step versioning (RFC-030) makes it worse: `matten-stats 1.0.0` would carry the family's SemVer
  promise while its own documentation says the surface may move.
- **The `ddof` case is a live example, not a hypothetical.** `matten-stats`'s sample-vs-population divergence
  from core has never been reviewed by anyone who did not propose it, and it is now shipped. Under
  `Experimental` on `0.x`, changing it later is permitted and honestly signalled. Under a `1.0.0` family
  label, it would be frozen by the same act that declared the family stable.
- **The ladder would lose meaning.** RFC-058 §2 and RFC-080 §5 both rest on ladder discipline — crates
  advance one rung at a time, on evidence. Admitting `Experimental` directly into a 1.0 family would let a
  crate skip every rung by being adjacent to stable siblings.

**The counter-argument, stated fairly:** lock-step versioning already decouples version from maturity by
design (RFC-030: "maturity is expressed by per-crate Status labels, not by separate version numbers"), and
RFC-067 accepted exactly that decoupling for candidates. One could argue `Experimental` is just a further
step along the same axis, adequately handled by documentation.

This RFC rejects that, on the grounds that the decoupling has a limit and `Experimental` is past it: a
candidate says *"this is finished, but we recommend it narrowly"*; `Experimental` says *"this is not
finished."* A 1.0 family can honestly contain the first. It cannot honestly contain the second.

## 6. RFC-076 inventory refresh (mechanical consequences)

With §3 decided, RFC-076's four staleness items resolve as:

| # | Stale item | Correction |
|---|---|---|
| 1 | four-crate family | five crates: add `matten-stats` to `cargo package --workspace` expectations, `Cargo.lock` bump scope, and publish order (it depends only on `matten`, joining the independent companion group) |
| 2 | `matten-mlprep` listed as candidate | now **production-ready** (RFC-080) |
| 3 | RFC-067 maturity table has no `matten-stats` row | add it, at `Experimental`, flagged as requiring §3's Exit A or B |
| 4 | `Experimental`-in-1.0 unanswered | answered by §3; RFC-076 must record which exit `matten-stats` takes before it can proceed |

**This RFC does not edit RFC-076.** The refresh is specified here and applied by the accompanying handoff,
so the decision and its application stay reviewable separately.

## 7. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | None |
| Runtime behavior | None |
| Dependencies / features / MSRV | None |
| Version | None — no bump |
| Maturity labels | **None** — no crate promoted or demoted |
| Release process | Adds a precondition to any future v1.0 release RFC (§3) |
| RFC-030 lock-step | Unchanged for `0.x`; §3 Exit B would require a separate amending RFC if taken |

## 8. Acceptance criteria

```text
[ ] the rule is stated as a decision, with both exits concrete and checkable
[ ] the rule does not disturb RFC-067's answer for production-ready candidate
[ ] no crate is promoted, demoted, or relabelled
[ ] the counter-argument is stated fairly and answered, not omitted
[ ] RFC-076's four staleness items are enumerated with their corrections
[ ] no v1.0 release, version bump, tag, publish, or API change is authorized
```

## 9. Non-goals

```text
[ ] authorizing a v1.0 release
[ ] promoting matten-stats, or deciding which exit it takes
[ ] editing RFC-076 (the handoff does that)
[ ] changing RFC-030 lock-step versioning for the 0.x line
[ ] revisiting RFC-067's candidate rule
[ ] the external ddof read — separate, and now about a future change rather than a first publish
```

## 10. Follow-up

If accepted, the accompanying handoff applies §6's refresh to RFC-076. `matten-stats`'s exit choice is a
later decision, and the honest sequencing is that it should be made on the crate's own evidence — usage,
the external `ddof` read, and whether its surface has settled — rather than because a v1.0 release is
wanted.
