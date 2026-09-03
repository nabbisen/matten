# Developer Handoff — RFC-130: Repository Governance Files

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/proposed/130-repository-governance-files.md`
**Base:** **after RFC-127 ships.** See §2 — the sequencing is not incidental.

> **UNBLOCKED 2026-09-03. You may start.**
>
> ```text
> gh api repos/nabbisen/matten/private-vulnerability-reporting  ->  {"enabled": true}
> ```
>
> The owner enabled private vulnerability reporting, so `https://github.com/nabbisen/matten/security/advisories/new`
> now accepts reports. **§3's precondition is satisfied** — re-derive it yourself before writing the
> file rather than trusting this banner, since a setting can be turned off again.
>
> RFC-127 has also shipped as `0.46.2`, satisfying §2's sequencing condition.

## 1. Task title

Add `SECURITY.md` and `CONTRIBUTING.md` at the repository root. **The manifest keys (Change C) are
NOT yours** — see §6.

## 2. Sequence with RFC-127, and why

**SATISFIED 2026-09-03 — `0.46.2` shipped, and with it RFC-127's fix.** This section is kept as the
record of why the sequencing mattered; the condition it imposes is met and does not block you.

**Publishing a disclosure channel is an invitation to look.** RFC-127 closed a live, uncatchable
process abort reachable from untrusted JSON, which **was** on crates.io in every version from
`0.17.0` until `0.46.2` fixed it.

```text
RFC-127 ships as 0.46.2  ->  THEN this
```

It is better to have closed the known Critical before extending an invitation to find others. If the
owner directs otherwise that is their call, but do not reorder it on your own judgement.

## 3. Change A — `SECURITY.md`

**The disclosure channel, supplied by the owner and the only one to use:**

```text
https://github.com/nabbisen/matten/security/advisories/new
```

```text
BEFORE WRITING THE FILE: confirm PRIVATE VULNERABILITY REPORTING is ENABLED on
the repository. It is an owner SETTING, not a file, and that URL 404s when it is
off.

A disclosure channel that 404s is worse than no channel — it is the same defect
class as max_parse_bytes: a documented control that does nothing. If it is off,
WRITE NOTHING and tell the owner.
```

Contents:

```text
Reporting           that URL, and nothing else
Supported versions  the current 0.4x family only; pre-1.0, no backports
In scope            the boundary surfaces RFC-001 names — JSON, CSV, filesystem
                    paths, caller-supplied shapes and indices, the slice
                    mini-language
Out of scope        performance, and anything the project states as out of scope
Expectations        say plainly this is a small pre-1.0 project maintained by one
                    person
```

```text
DO NOT invent a PGP key, a security@ alias, an embargo window, or a response-time
promise. Nobody is contracted to meet one, and a missed SLA in a security policy
is worse than an absent one.
```

## 4. Change B — `CONTRIBUTING.md`

The project already has substantial contributor documentation under `docs/src/contributing/**`,
including the release checklist. **GitHub cannot see it** — it looks for a root `CONTRIBUTING.md` to
surface in the PR and issue UI.

```text
KEEP IT SHORT. LINK, DO NOT RESTATE.
```

Point at the book's contributing section, `rfcs/README.md`, and `scripts/` (the nine guards). That is
the file.

```text
DO NOT copy the release checklist, the RFC lifecycle, or the guard list into it.
A second copy rots — RFC-107 §8 risk 1, and this project has found that defect
repeatedly. If you catch yourself explaining how to cut a release, stop.
```

## 5. Why these two need no release

```text
SECURITY.md, CONTRIBUTING.md    root-level -> 0 of 5 published packages
```

**Assert it** with `cargo package --list` for all five crates rather than assuming.

## 6. Change C is NOT in this task

```text
homepage / documentation keys in crates/*/Cargo.toml
```

Those are **packaged manifest metadata**, and under RFC-094 §4.1 as amended by RFC-120, patch content
is *"correctness fixes to already-published crate content — code, rustdoc, or a packaged README"*.
Manifest metadata is none of the three, and adding a `homepage` is not a correctness fix — nothing is
wrong, a link is merely absent.

**So Change C is minor content and waits for `0.47.0`.** Do not include it. State in your review
request that you deliberately excluded it and why.

## 7. Out of scope

```text
any crates/** file                        including the manifests (§6)
enabling private vulnerability reporting  an owner repository SETTING (§3)
a security policy beyond the channel      no PGP, no embargo, no SLA (§3)
restating contributor docs                §4
docs/src/**                               RFC-131's territory
RFC-127's fixes                           ship first
```

## 8. Risks

```text
R1  Shipping SECURITY.md before private reporting is enabled (§3).
R2  Promising a response time (§3).
R3  Restating contributor docs, creating a copy that rots (§4).
R4  Including Change C (§6).
R5  Inventing a second disclosure channel.
R6  Reordering ahead of RFC-127 (§2).
```

## 9. Acceptance criteria

```text
[ ] private vulnerability reporting CONFIRMED enabled, or nothing written and the
    owner told
[ ] SECURITY.md at the ROOT, naming only the owner's URL
[ ] supported-versions statement, honest about pre-1.0
[ ] no response-time promise, no PGP, no embargo terms
[ ] CONTRIBUTING.md at the root, short, linking rather than restating
[ ] Change C deliberately EXCLUDED, and said so in the review request
[ ] git diff touches no crates/** path
[ ] cargo package --list unchanged for all five crates — asserted
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-130/matten-rfc130-repository-governance-files-implementation-review-request-v0.1.md`

Confirm the reporting setting was checked and how, quote the supported-versions statement, confirm
Change C's exclusion, include guard output and the packaging assertion.
