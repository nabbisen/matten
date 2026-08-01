# RFC-092: Adopt the 5-Folder RFC Lifecycle Variant

**Status:** **Implemented** 2026-08-01. `rfcs/accepted/` is live; RFC-000 carries a reciprocal
amendment note and a new `Accept` operation; `rfcs/README.md` and the org workflow document name
the `proposed/` → `accepted/` → `done/` transitions. Decided by the owner directly, so this RFC
records a decision rather than proposing one, and went `proposed/` → `done/` without passing
through the folder it creates — the last RFC that will. No version, no release
**Target:** Process change; no version, no release
**Theme:** Make "the maintainer signed off" a folder transition instead of a Status-field qualifier
**Amends:** RFC-000 §"Folder layout" and §"Operations"
**Related:** RFC-090, RFC-091, `.git-exclude/roles/ai-multi-agent-software-development-organization-and-workflow.md`

---

## 1. Summary

Add `rfcs/accepted/` and route RFCs through it between review and implementation.

This is not a new idea. RFC-000 already specifies both layouts and gives the test for choosing
between them; this RFC records that the test now resolves the other way for this project, and moves
the project accordingly.

## 2. Why — the 4-folder variant stopped matching how this project works

RFC-000's own criterion:

> Use this variant if "the maintainer signed off" is a meaningful event distinct from "the
> implementer finished." Skip it otherwise — `accepted/` will sit empty in projects where the two
> events collapse, and an empty folder is a maintenance burden with no payoff.

When RFC-000 was written the two events collapsed. They no longer do. The project runs a three-tier
organisation in which sign-off and implementation are performed by **different parties on different
days**, with a Developer Handoff document as the interface between them. Sign-off is the single most
load-bearing transition in that workflow, and it is the only one the folder structure cannot express.

**The cost is already visible in the corpus, not hypothetical.** RFC-000 states that the folder is
the source of truth and that a Status field disagreeing with it is the anti-pattern the policy exists
to prevent. Yet both recent RFCs were accepted while sitting in `proposed/`, and both had to carry a
hand-written qualifier reconciling the contradiction:

```text
RFC-090   "**Status:** `proposed/` by folder (not yet implemented); reviewed and
           accepted 2026-07-31 — implementation authorized under the handoff"

RFC-091   "**Status:** `proposed/` by folder (not yet implemented); reviewed and
           accepted 2026-08-01 — preparation authorized under the handoff"
```

Those qualifiers are the workaround. They are honest, and they are evidence: a policy whose central
rule needs a per-file disclaimer is being worked around rather than followed. Each one is also a
place where a future editor can let the Status and the folder drift apart, which is exactly the
failure RFC-000 names.

## 3. Decision

Adopt the 5-folder variant:

```
rfcs/
  proposed/    ← under review
  accepted/    ← review complete; implementer may start
  done/        ← shipped
  archive/     ← withdrawn or superseded (still unused)
  draft/       ← (optional, unused)
```

The maintainer's acceptance moves the file `proposed/` → `accepted/`. The implementer's completion
moves it `accepted/` → `done/`. Nothing else changes: the folder remains the source of truth, and the
Status field continues to mirror it — but now it can mirror it without a disclaimer.

## 4. Scope

### In scope

```text
create rfcs/accepted/ with a README stating what belongs there and what does not
amend RFC-000's folder-layout and operations sections, with a reciprocal note in RFC-000
add an Accepted section to rfcs/README.md
name the transition in the org workflow document
```

### Out of scope — a diff touching these is a defect

```text
moving any existing RFC. RFC-076 is proposed-and-deferred, NOT accepted; nothing
  currently sits in the accepted state, so accepted/ starts empty and correctly so
retroactively rewriting RFC-090's or RFC-091's Status qualifiers — they are accurate
  history of how those RFCs were actually handled
archive/ and draft/ — still unused, and this RFC does not change that
any code, version, release, or maturity change
```

## 5. The empty-folder objection, answered

RFC-000 warns that `accepted/` "will sit empty in projects where the two events collapse, and an
empty folder is a maintenance burden with no payoff." It will indeed start empty here, because no RFC
is currently between sign-off and implementation.

That is a statement about *this moment*, not about the project. Six RFCs passed through the accepted
state in the last week alone (RFC-085 through RFC-091); every one would have occupied the folder, and
every one instead occupied `proposed/` while claiming not to. The warning is aimed at projects where
the state never occurs — not at one where it occurs constantly and is currently between instances.

`accepted/README.md` keeps the directory tracked and tells a reader what the state means, so the
folder is self-describing rather than an unexplained empty box.

## 6. Acceptance criteria

```text
[ ] rfcs/accepted/ exists, tracked, with a README defining the state
[ ] RFC-000 carries a reciprocal amendment note pointing at this RFC, per its own
    supersession convention — RFC-000 is amended, NOT superseded, and stays authoritative
    on everything else
[ ] rfcs/README.md has an Accepted section, and its layout description matches reality
[ ] the org workflow document names the proposed/ -> accepted/ -> done/ transitions
[ ] no existing RFC moved; RFC-090 and RFC-091 Status text unchanged
[ ] all seven guards pass
```

## 7. Non-goals

```text
adopting draft/ — the drafting step happens in a single commit here and needs no folder
using archive/ — nothing has been withdrawn or superseded yet
changing who accepts an RFC (the owner) or who implements it
```
