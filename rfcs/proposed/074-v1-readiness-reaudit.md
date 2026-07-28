# RFC-074: v1.0 Readiness Re-Audit

**Status:** Proposed; audit report complete (`docs/design/v1-readiness-audit.md`) and prepared for review; no v1.0 release authorized
**Target:** Post-0.38 v1.0 readiness re-audit; no version bump or release authorization
**Theme:** Re-run the RFC-066 v1.0 readiness audit, which is stale by exactly the
length of eight consumer-invisible releases
**Depends on:** RFC-002, RFC-015, RFC-030, RFC-057, RFC-058, RFC-059, RFC-066, RFC-067
**Related:** RFC-040, RFC-041, RFC-026, RFC-037, RFC-054, RFC-063 through RFC-073

---

## 1. Summary

This RFC proposes an audit-only re-run of the RFC-066 v1.0 readiness audit.

It does **not** authorize a `1.0.0` release, a version bump, a maturity
promotion, a public API change, a new dependency, release preparation, a tag,
or a publish action.

The audit should answer:

```text
Is the published surface (Tensor contract, error model, boundary APIs) v1-stable?
Which of stats / linalg / streaming is a v1 blocker versus explicitly post-v1?
Should matten-mlprep and matten-data move from production-ready candidate to
  production-ready (RFC-058/059/067)?
Does RFC-030 lock-step versioning still serve the family, given eight
  consumer-invisible releases?
Is a 1.0 commitment appropriate, or is a documented "0.x indefinitely" stance
  more honest?
What is the compatibility promise at 1.0, and what does it cost to maintain?
```

Expected initial posture, matching RFC-066's and RFC-070's precedent:

```text
Prefer "not ready" unless the audit can name a concrete, stable, testable
1.0 surface.
```

---

## 2. Background And Motivation

RFC-066 (*v1.0 Readiness Audit and Release Decision Gate*) is `Implemented
(v0.31.0)`. That is the same release at which published-crate change stopped.
Every conclusion RFC-066 reached predates the entire visualization line
(RFC-063 through RFC-073) and three companion-maturity decisions
(RFC-057/058/059).

A post-`0.38.0` measurement shows why this matters:

```text
git diff --name-only 0.31.0..0.38.0 -- 'crates/*/src/'
  -> crates/matten/src/lib.rs        (the only file)

git diff 0.31.0..0.38.0 -- crates/matten/src/lib.rs   (non-doc lines)
  -> none; every changed line is a //! doc comment (a version string inside a
     ```toml snippet)
```

Across `0.31.0` -> `0.38.0` — eight releases — the published crates
(`matten`, `matten-ndarray`, `matten-mlprep`, `matten-data`) have had exactly
zero functional change. Total `crates/` churn across that span is 9
insertions / 9 deletions, all version strings in docs and READMEs. The
CHANGELOG carries 31 "No public API" disclaimers over the same span.

Every individual release in that span was honest: each CHANGELOG entry
correctly states that no published-crate API, runtime, dependency, MSRV, or
maturity changed. RFC-071's accepted rationale — that private-tool milestones
may be released as lock-step family checkpoints when the changelog says so —
was reasonable in isolation. Applied eight times consecutively, it has
quietly become the project's default release mode. That is a consequence of
RFC-030 lock-step versioning doing exactly what it was designed to do, not a
violation of any accepted RFC.

The backlog left by RFC-070's remaining-themes table has eleven entries
(streaming, stats, linalg, two bridges, benchmark hard gates, companion
promotions, public report/viz, …). Picking one by intuition risks starting
another multi-release line that consumers never see. A readiness audit
produces a ranked, evidence-based answer to "what does the published surface
still need?" instead.

---

## 3. Goals

1. Re-measure v1.0 readiness against RFC-066's original findings, item by
   item, and record which have been met, which have gone stale, and which
   were overtaken by later RFCs (057-059, 067).
2. Decide whether the public `Tensor` contract, error model, and boundary
   APIs are v1-stable.
3. Classify each deferred mathematics/streaming theme (broader stats, broader
   linalg, streaming/large-CSV) as a v1 blocker or an explicit post-v1 item.
4. Decide whether `matten-mlprep` and `matten-data` should be recommended for
   promotion from `production-ready candidate` to `production-ready`
   (RFC-058/059/067), or whether the candidate label should persist into any
   future v1.0 family per RFC-067's resolved policy.
5. Assess whether RFC-030 lock-step versioning still serves the family, given
   eight consecutive consumer-invisible releases, and recommend whether that
   policy needs revisiting, an amendment, or no change.
6. Recommend whether a 1.0 commitment is appropriate now, or whether a
   documented "0.x indefinitely" stance is the more honest position.
7. If a 1.0 surface is named, describe what compatibility promise it would
   carry and what it would cost to maintain.
8. Produce a ranked, evidence-based recommendation for the next implementation
   RFC, so the backlog is resolved by measurement rather than by guess.
9. Keep this audit separate from release preparation and public API changes.

---

## 4. Non-Goals

This RFC does not authorize:

```text
[ ] implementation of any kind
[ ] a 1.0.0 release
[ ] any version bump
[ ] any maturity-label promotion or demotion
[ ] any public API change
[ ] any new dependency in any published crate
[ ] release preparation, tag, or publish action
[ ] any tools/matten-report or tools/matten-migrate scope
[ ] any RFC-030 lock-step versioning policy change (the audit may
    *recommend* one, but adopting it requires a separate RFC)
[ ] broader statistics, linear algebra, or streaming implementation
[ ] bridge crates (matten-nalgebra, matten-candle)
[ ] public matten-report / matten-viz crates (RFC-070 remains the closed
    authority on that question)
```

If the audit recommends any of these, that recommendation still requires a
separate future RFC or reviewed handoff before implementation.

---

## 5. Audit Questions

### 5.1 Public API Stability

Review:

```text
crates/matten/src/lib.rs (grep "^pub use")
docs/src/reference/public-api-snapshot.md
docs/src/reference/compatibility.md
docs/src/reference/error-model.md
docs/src/reference/boundary.md
```

Questions:

```text
[ ] Is the current `Tensor` contract (construction, shape, broadcasting,
    slicing, reductions, matmul) v1-stable as documented?
[ ] Is the panic-zone/Result-zone split (compatibility.md) final, or does it
    need to change before v1.0?
[ ] Does the public API snapshot still match the shipped MattenError enum
    and root exports exactly?
[ ] Is the `dynamic` feature's public surface (Element, NumericPolicy)
    v1-stable, or does it need its own pre-v1 review?
[ ] Has `cargo public-api` been run and approved as RFC-066/the release
    checklist require?
```

### 5.2 Deferred Mathematics And Streaming Scope

Review:

```text
rfcs/done/040-small-statistics-boundary-core-vs-companion.md
rfcs/done/041-linear-algebra-boundary-core-lite-vs-external-crates.md
rfcs/done/026-large-csv-and-streaming-data-policy.md
rfcs/done/037-deferred-streaming-and-large-csv-policy.md
```

Questions:

```text
[ ] Is broader statistics (covariance, correlation, quantile, histogram,
    z-score) a v1 blocker, a post-v1 companion candidate, or explicitly out
    of scope?
[ ] Is broader linear algebra (inverse, determinant, decomposition) a v1
    blocker or explicitly out of scope for core's "lite" positioning?
[ ] Is streaming/large-CSV a v1 blocker, or does matten's documented scope
    (PoC, learning, small workflows) make it permanently out of scope?
[ ] Does leaving these deferred at v1.0 require any change to the
    documented non-goals in README.md / ROADMAP.md?
```

### 5.3 Companion Maturity Promotion

Review:

```text
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
rfcs/done/067-v1-family-maturity-policy.md
crates/matten-mlprep/README.md
crates/matten-data/README.md
```

Questions:

```text
[ ] Has anything changed since RFC-058/059 that would justify promoting
    matten-mlprep or matten-data to production-ready?
[ ] If not, does RFC-067's policy (a candidate-labeled companion is not
    automatically a v1.0 blocker) still hold, or does the audit recommend
    revisiting it?
[ ] Does a lock-step v1.0 family including candidate-labeled companions
    need any documentation change beyond what RFC-067 already requires?
```

### 5.4 Lock-Step Versioning Policy

Review:

```text
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/071-private-fixed-demo-json-report-artifacts.md
CHANGELOG.md (0.31.0 through 0.38.0 entries)
```

Questions:

```text
[ ] Given eight consecutive consumer-invisible releases, does RFC-030
    lock-step versioning still serve users, or has it become a source of
    false-signal version bumps?
[ ] Is the RFC-071 rationale ("private-tool milestones may be released as
    lock-step family checkpoints") still sound as a *repeatable default*, or
    should it require an explicit justification each time going forward?
[ ] Would a policy change (e.g., requiring published-crate content for a
    minor bump, or a separate local-tool version track) better serve users
    without abandoning lock-step versioning's compatibility benefits?
[ ] Is this a recommendation-only finding for this audit, deferring any
    actual policy change to a separate RFC?
```

### 5.5 The 1.0 Commitment Itself

Review:

```text
rfcs/done/066-v1-readiness-audit-and-release-decision-gate.md
rfcs/done/067-v1-family-maturity-policy.md
docs/src/reference/compatibility.md (v1.0 requirements section)
```

Questions:

```text
[ ] Given the findings above, is a 1.0 commitment appropriate now?
[ ] If not, is a documented "0.x indefinitely" stance more honest than an
    implicit, perpetually-deferred v1.0 target?
[ ] If yes, what is the compatibility promise at 1.0 (which zones become
    frozen, which remain extensible under `#[non_exhaustive]`), and what
    does maintaining that promise cost going forward?
[ ] What explicit maintainer decision, if any, remains before a separate
    future v1.0 release RFC could be drafted?
```

---

## 6. Expected Audit Outputs

The audit should produce:

```text
[ ] readiness verdict: not ready / conditionally ready / ready for a 1.0 plan
[ ] a prerequisite list, if not ready
[ ] a per-companion maturity recommendation (promote / hold, with reasons)
[ ] a versioning-policy recommendation (keep RFC-030 as-is / amend / defer)
[ ] an explicit deferral list (what stays out of v1.0 regardless of verdict)
[ ] a recommended next RFC, ranked against the rest of the RFC-070
    remaining-themes backlog
```

The audit should prefer "not ready yet" unless it can name a concrete,
narrow, stable, testable 1.0 surface.

---

## 7. Initial Risk Assessment

| Risk | Treatment |
|---|---|
| Treating audit findings as release authorization | This RFC and its audit output authorize no version bump, tag, or publish; a separate RFC is required |
| Re-litigating RFC-067's MD-1 resolution without new evidence | Require the audit to cite what, if anything, changed since RFC-067 before recommending a policy change |
| Scope creep into implementing any named gap during the audit | Keep the audit read-only; findings become candidate RFCs, not code |
| Recommending a lock-step versioning change without weighing its compatibility benefit | Require the audit to state the benefit RFC-030 still provides, not just the cost observed in §2 |
| Understating companion promotion questions already settled by RFC-058/059 | Require the audit to cite those RFCs' original blocking reasons and confirm whether each still holds |

---

## 8. Review Questions

Review should decide:

```text
[ ] Is RFC-074 correctly scoped as audit-only, with no implementation or
    release authorized?
[ ] Are the six audit questions in §1/§5 sufficient to reach a readiness
    verdict, or is a question missing?
[ ] Is the lock-step versioning question (§5.4) correctly treated as
    first-class rather than a footnote?
[ ] Are the non-goals broad enough to prevent accidental implementation
    scope, including any matten-report scope?
[ ] Should the expected posture remain "not ready" unless the audit proves
    otherwise, matching RFC-066/RFC-070 precedent?
[ ] Are ROADMAP.md and rfcs/README.md the only tracking surfaces needed for
    this proposal?
```
