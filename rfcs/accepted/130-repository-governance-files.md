# RFC-130: Repository Governance Files

**Status:** **Accepted** 2026-09-01 by the owner, who supplied the disclosure URL
`https://github.com/nabbisen/matten/security/advisories/new`. **BLOCKED on one owner action:**
private vulnerability reporting is currently **disabled** on the repository — measured 2026-09-01 via
`gh api repos/nabbisen/matten/private-vulnerability-reporting` → `{"enabled": false}` — so that URL
does not accept reports yet. Handoff: `rfcs/handoffs/130-repository-governance-files-handoff.md`.
Starts after RFC-127 **ships**. No version bump, tag, or publish.
**Target:** `SECURITY.md`, `CONTRIBUTING.md`, `crates/*/Cargo.toml` (manifest metadata)
**Theme:** Give a vulnerability somewhere to go, and make the contributor and project links findable
**Related:** RFC-001 (the threat model this serves), RFC-094 §4.1 (why the manifest keys are a patch),
internal audit F10, external audit D-3/D-4/D-16

---

## 1. Summary

```text
A  SECURITY.md at the repository root, with the owner's disclosure channel
B  CONTRIBUTING.md at the root, so GitHub surfaces it
C  homepage / documentation keys in the five published manifests
```

**A and B reach zero published packages — no release.** **C changes `crates/*/Cargo.toml` and is
therefore packaged metadata**, which needs a release; see §6, which is the one genuinely interesting
question in this RFC.

## 2. Why — two auditors, independently

`SECURITY.md`'s absence was found by the internal audit (2026-08-28, F10) and again by the external
architect (D-3), neither aware of the other. That is not a coincidence worth ignoring.

```text
five crates published to crates.io
RFC-001 is titled "Threat Model and Boundary Safety Policy"
its §2 motivation is untrusted boundary input
RFC-127 documents a live, uncatchable process abort reachable from untrusted JSON
...and there is nowhere to report it privately
```

**The last line is the point.** A project that maintains its own threat model, publishes five crates,
and currently ships a remotely-triggerable DoS has no channel to receive that report except a public
issue — which is the worst possible place for it.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | No `SECURITY.md` at the root or under `.github/` | `ls` — absent in both |
| E2 | No `CONTRIBUTING.md` anywhere in the repository | `find . -name 'CONTRIBUTING*'` → nothing |
| E3 | No `homepage` or `documentation` key in the workspace or any crate manifest | `grep` across `Cargo.toml` and `crates/*/Cargo.toml` |
| E4 | The disclosure channel, supplied by the owner 2026-09-01 | `https://github.com/nabbisen/matten/security/advisories/new` |
| E5 | The book is deployed and has a stable URL; crates.io currently links neither it nor a homepage | `docs.yaml`; crates.io listings |

## 4. Change A — `SECURITY.md`

```text
Reporting          E4's URL — GitHub private vulnerability reporting
Supported versions the current 0.4x family only; pre-1.0, no backports
What is in scope   the boundary surfaces RFC-001 names: JSON, CSV, filesystem
                   paths, caller-supplied shapes and indices, the slice
                   mini-language
What is NOT        performance, and anything the project states as out of scope
Expectations       say plainly that this is a small pre-1.0 project maintained
                   by one person; do not promise a response time nobody is
                   contracted to meet
```

```text
E4's URL requires PRIVATE VULNERABILITY REPORTING to be ENABLED on the
repository. That is an owner setting, not a file. VERIFY it is on before
the file promises it — a disclosure channel that 404s is worse than none,
which is the same defect class as max_parse_bytes.
```

**Do not invent a PGP key, a security email alias, or an embargo policy.** The owner supplied one
channel; use exactly that.

## 5. Change B — `CONTRIBUTING.md`

The project has extensive contributor documentation already — `docs/src/contributing/**`, including
the release checklist. **It is invisible to GitHub**, which looks for a root `CONTRIBUTING.md` to
surface in the PR and issue UI.

```text
KEEP IT SHORT AND POINT AT WHAT EXISTS.
Do not restate the release checklist, the RFC lifecycle, or the guard list —
they live in docs/src/contributing/ and rfcs/, and a copy will rot (RFC-107 §8).
```

Link the book's contributing section, `rfcs/README.md`, and the nine guard scripts. That is the whole
file.

## 6. Change C — the manifest keys, and why they are a release

```text
homepage      = the deployed book
documentation = docs.rs (or omit; cargo defaults to docs.rs for published crates)
```

**This changes `crates/*/Cargo.toml`, which is packaged metadata**, so under RFC-094 §4.3's mechanical
test it is releasable. Is it patch content?

```text
RFC-094 §4.1 as amended by RFC-120:
    "correctness fixes to already-published crate content — code, rustdoc,
     or a packaged README — and nothing else"
```

**Manifest metadata is none of those three.** Adding `homepage` is not a correctness fix; nothing is
wrong today, a link is merely absent. So by the amended §4.1, **Change C is not patch content** and
belongs in a minor.

> This is the second time the RFC-120 amendment has produced a clean answer to a question that would
> otherwise have been argued. Recorded because the amendment's value is exactly that.

**Therefore:** A and B land immediately with no release. **C waits for `0.47.0`** and should be folded
into that release's slice rather than triggering anything of its own.

If the owner would rather have the crates.io links sooner, the lever is authorizing `0.47.0` — the
same lever RFC-129 needs.

## 7. Scope

### Out of scope — a diff touching these is a defect

```text
any crates/*/src file                     no code changes here at all
enabling private vulnerability reporting  an owner repository SETTING (§4)
a security policy beyond the channel      no embargo terms, no PGP, no SLA
restating contributor docs in the root    §5
docs/src/**                               the documentation batch is its own RFC
RFC-127's fixes                           ship first
```

## 8. Risks

```text
R1  Shipping SECURITY.md before private reporting is enabled, so the URL 404s
    (§4). Verify the setting first, and if it is off, say so and hold the file.
R2  Promising a response time. Nobody is contracted to meet one.
R3  Restating contributor docs in CONTRIBUTING.md, creating a second copy that
    rots (§5).
R4  Putting Change C in a patch (§6).
R5  Inventing a second disclosure channel. Exactly E4's URL.
```

## 9. Acceptance criteria

```text
[ ] SECURITY.md at the ROOT, naming E4's URL and nothing else as the channel
[ ] private vulnerability reporting confirmed ENABLED, or the file held and the
    owner told
[ ] supported-versions statement present and honest about pre-1.0
[ ] no response-time promise
[ ] CONTRIBUTING.md at the root, short, linking rather than restating
[ ] Change C NOT included — deferred to 0.47.0 (§6), stated in the review request
[ ] git diff touches no crates/** path
[ ] cargo package --list unchanged for all five crates — asserted
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 10. What this does not fix

```text
- the vulnerability itself. RFC-127 does that, and should ship FIRST so the
  channel is not announced while the known defect is still live.
- the three tools' missing unsafe policy (internal audit F11)
- the v1.0 readiness audit, nine releases stale (internal audit F6)
```

**Sequencing note worth stating:** RFC-127 first, then this. Publishing a disclosure channel is an
invitation to look; it is better to have closed the known Critical before extending it.
