# RFC-120: Patch Releases Cover Published Documentation

**Status:** **Accepted** 2026-08-28 by the owner. Not yet implemented. **Governance editing, so no
Developer Handoff** (see `accepted/README.md`) — the high-capability model performs it. No version,
no release.
**Target:** `rfcs/done/094-release-cadence-policy.md` §4.1 and §4.2
**Amends:** RFC-094 §4.1 (patch contents) and §4.2 (adds the timing-versus-form rule it never had)
**Theme:** Close the gap that routes a published-documentation correction into a minor release a
month later
**Related:** RFC-094, RFC-095 (the amendment precedent), RFC-119, RFC-000

---

## 1. Summary

```text
RFC-094 §4.1 defines a patch's contents as "correctness fixes to already-published
crate CODE". A false statement in published rustdoc or a packaged README is not
code. It is also not excluded by §4.3, which excludes only documentation that
"does not reach crates.io".

It therefore falls through §4.1 and §4.3, and is caught by §4.2(b) — the 28-day
anti-rot floor — because "unreleased" is defined by the crates/ test and a
packaged README lives in crates/.

The literal policy sends a one-line correction into a MINOR release a month
later. This RFC makes it a patch, immediately, which is what §4.1's own
rationale already says it should be.
```

**No `crates/` change, so no release.** This RFC edits one RFC document.

## 2. This is a gap, not a stale statement — and the difference decides the mechanism

The distinction matters enough to state plainly, because it is what separates this RFC from RFC-119.

```text
RFC-119's targets   statements that BECAME FALSE. lib.rs:19 was accurate until
                    RFC-102 shipped slicing. Fix: correct the text.

RFC-093 §6          a rule that WAS WRONG — RFC-095 found it "drew the line in
                    the wrong place in both directions". Fix: amend.

RFC-094 §4.1        NEITHER. Accurate when written, still accurate: patches do
                    contain correctness fixes to published code. It is SILENT on
                    published documentation, because when it was written the
                    working model was "docs = the book = does not ship".
```

A gap is not repaired by correcting a sentence, because no sentence is wrong. It is repaired by a
**decision** about a case the policy does not cover — which is why this is an RFC and not a line in
RFC-119.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | §4.1 defines patch contents as *"correctness fixes to already-published crate code, and nothing else"* | `rfcs/done/094-release-cadence-policy.md:77` |
| E2 | §4.3 excludes *"documentation … that does not reach crates.io"* — a conditional exclusion, not a blanket one | `094:105-107` |
| E3 | §4.3's test is declared *"mechanical, not editorial"*: `git diff --name-only <last-tag>..HEAD -- crates/` empty means nothing to release | `094:116-118` |
| E4 | §4.2(b) fires on *"28 days … with anything unreleased"*, and §4.2 defines **triggers only — it has no contents clause at all** | `094:88-92`; the asymmetry with §4.1 is visible in the source |
| E5 | Published rustdoc and packaged READMEs are inside `crates/` and inside the published packages | `cargo package --list -p matten` contains `src/lib.rs`, `README.md`; `-p matten-stats` contains `README.md` |
| E6 | The root `README.md` is in **zero** of the five packages — the boundary is real, not notional | `cargo package --list` across all five |
| E7 | The precedent for declining a release used the package-membership test, not the word *"documentation"* | §4.3: `0.42.1` declined because the file was *"in zero of the five published packages"* |
| E8 | RFC-095 amended RFC-093 §6 **while RFC-093 was already in `done/`** | RFC-093 closed 2026-08-02 (`7cbb46a`); RFC-095 closed 2026-08-03 (`7151e4e`) |
| E9 | That amendment is recorded as a blockquote inside RFC-093 quoting the original wording verbatim and stating the replacement is *narrower* | `rfcs/done/093-*.md` §6 |
| E10 | RFC-119 supplies four live instances at once — `lib.rs:19`, `crates/matten-stats/README.md:146`, `stats.rs:7`, and an example | `.git-exclude/reviewed/matten-project-audit-2026-08-28-v0.1.md` F1, F3, F9, F2 |

## 4. The routing defect, traced

Take RFC-119's `crates/matten-stats/README.md:146` — a false capability claim on crates.io — and
apply RFC-094 as written:

```text
§4.3  is it releasable?     git diff -- crates/ is NON-EMPTY, and the file is in
                            a published package (E5). -> YES, releasable.
§4.1  is it a patch?        contents are "crate CODE" (E1). A README is not code.
                            -> NO.
§4.2  is it a minor?        no contents clause exists (E4); trigger (b) counts
                            "anything unreleased", which by E3's mechanical test
                            includes this file. -> YES, after 28 days.
```

**A one-line correction becomes a minor release a month later.** That contradicts §4.1's own stated
rationale — *"a user hitting a wrong answer should not wait for an unrelated feature to be ready"* —
which describes a reader of a false capability claim exactly as well as it describes a wrong number.

There is a second, smaller edge. §4.1 says *"and nothing else."* Under the current wording, bundling
RFC-119's documentation corrections with its example fix violates that phrase even though the example
fix independently justifies the release.

## 5. Change A — §4.1's contents clause

```text
FROM  contents: correctness fixes to already-published crate code, and nothing else
TO    contents: correctness fixes to already-published crate content — code,
                rustdoc, or a packaged README — and nothing else
```

`excluded:` is unchanged. No new public API and no behaviour change remain excluded, and a
documentation correction is neither.

**This is narrower in effect than it looks.** It does not admit documentation generally — §4.3's
exclusion of documentation that does not reach crates.io stands untouched, and E6 shows that boundary
already separates the root `README.md` from the packaged ones. The test remains
`cargo package --list`, exactly as in the `0.42.1` precedent (E7). What changes is that §4.1 now
*matches* that test instead of contradicting it.

## 6. Change B — §4.2 gains the contents clause it never had

§4.2 defines triggers and nothing else (E4). Add one sentence:

```text
The triggers determine WHEN a release happens, not WHAT KIND. If everything
unreleased when a trigger fires is a correction under §4.1, the release is a
patch.
```

Without this, (b) still routes a doc-only backlog into a minor after 28 days even once §4.1 admits
it as patch content. Change A alone fixes the common case — a correction shipped immediately — and
leaves the timer case broken.

## 7. How the amendment is recorded

**Follow RFC-095's precedent exactly (E8, E9).** RFC-094 is in `done/`; its text is a record and a
silent edit destroys it.

```text
- a blockquote in §4.1 and in §4.2, each headed
  "**Amended by RFC-120 (<date>).**"
- the ORIGINAL wording quoted verbatim inside it
- the replacement stated
- a sentence saying what the change does and does not widen (§5)
```

**Do not delete or rewrite the original lines in place.** RFC-000's anti-pattern is a document whose
stated state disagrees with its real one; an amendment that hides what it replaced is that anti-pattern
applied to content instead of status.

## 8. What this does not change

```text
the three triggers (a)/(b)/(c) themselves — unchanged
§4.3's exclusion of non-shipping documentation — unchanged, and load-bearing
§4.3's mechanical test — unchanged; this RFC makes §4.1 agree with it
the 28-day figure — untouched
RFC-030 lock-step versioning — untouched
the owner's separate authorization of every tag and every publish — untouched
whether RFC-119's corrections ship — Change A of RFC-119 justifies a release
   under the CURRENT wording; this RFC changes the justification's breadth,
   not its existence
```

## 9. Scope

### Out of scope — a diff touching these is a defect

```text
any crates/ file, any .rs, any Cargo.toml     -> no release, by construction
CHANGELOG.md                                  -> nothing ships here
the 0.46.1 release itself                     -> RFC-121
RFC-119's five corrections                    -> RFC-119, already accepted
RFC-094 §5, §6, §7, §8                        -> untouched
ROADMAP.md                                    -> the disposition record is
                                                 written at closure, not here
```

## 10. Risks

```text
R1  Editing §4.1/§4.2 in place instead of amending by blockquote (§7). This is
    the likeliest defect and it destroys the record RFC-095 was careful to keep.
R2  Widening §4.1 to "documentation" generally, dropping the packaged-crate
    boundary. §4.3's exclusion must survive verbatim; E6 shows the boundary is
    real and already separates the root README from the packaged ones.
R3  Making Change A without Change B, leaving (b) to route a doc-only backlog
    into a minor after 28 days (§6).
R4  Treating this RFC as authorizing the 0.46.1 release. It does not.
R5  Renumbering or reflowing RFC-094's surrounding text while editing. The diff
    should show insertions only.
```

## 11. Acceptance criteria

```text
[ ] §4.1's contents clause amended per §5, by blockquote, original quoted verbatim
[ ] §4.2 gains the timing-vs-form sentence per §6, by blockquote
[ ] both blockquotes headed "Amended by RFC-120 (<date>)"
[ ] §4.3 unchanged — verify by diff, not by reading
[ ] the three triggers unchanged; the 28-day figure unchanged
[ ] git diff shows INSERTIONS ONLY in 094; no removed line
[ ] git diff touches NO crates/, NO .rs, NO Cargo.toml, NO CHANGELOG.md
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, no tag, no publish
```

## 12. What this does not fix

A policy that is read only when someone consults it. RFC-118 §9 made the same admission about the
release checklist, and it is equally true here: this amendment makes the right answer *findable*, not
automatic. The mechanical alternative — deriving release form from `cargo package --list` in a
script — is a larger change and is deliberately not attempted.

It also does not address why four published statements went false without anyone noticing. That is
the audit's open finding, and no cadence policy can close it.
