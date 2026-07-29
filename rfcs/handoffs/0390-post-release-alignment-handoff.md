# `0.39.0` Post-Release Alignment Handoff

**Project:** `matten`
**Related:** RFC-077, RFC-078, RFC-079
**Document kind:** Post-release truth-alignment handoff
**Status:** Drafted for review; no release action authorized
**Date:** 2026-07-28

---

## 1. Purpose

`0.39.0` is tagged and published, but the repository does not say so. The last commit is
`041c115 Prepare 0.39.0 RFC-079 pre-v1 feature release`; every prior release in this line closed with an
alignment commit and this one has not. This handoff closes that gap.

**Documentation only.** No code, no version change, no publish action.

## 2. Step 0 — determine what was actually published (blocking)

This cannot be assumed, and the rest of the work branches on it.

The `0.39.0` release was prepared with `matten-stats` **excluded** from the publish list, per the owner's
RFC-079 §3 decision to defer its first publication pending an external read of RFC-078 §4.1's `ddof = 1`
divergence. ROADMAP `3.25.0` records that exclusion as fact. But `matten-stats` carried no `publish = false`
key, so nothing mechanically prevented it from going out.

**Determine, without running any publish command:**

```text
does crates.io/crates/matten-stats exist at 0.39.0?
```

Check the crates.io web page or `cargo search matten-stats`. **Do not** run `cargo publish`, with or
without `--dry-run` — the crate's whole status is "deliberately withheld," and publish tooling is the wrong
instrument for answering a question about state.

Record the answer explicitly in the review request. Everything below has a **Case A** (not published) and
**Case B** (published) branch.

## 3. Common work — both cases

### 3.1 RFC-079 status

`rfcs/proposed/079-0390-pre-v1-feature-release.md` → `rfcs/done/`, with a status recording: accepted after
review (one required correction applied), implemented, **released as `0.39.0`, tagged and published**.

### 3.2 ROADMAP

- **Status line** — `0.39.0` is released, tagged, and published; name what shipped
  (`train_test_split_seeded`) and, per §4/§5 below, `matten-stats`'s actual state.
- **Release table** — a `v0.39.0` row. Follow the existing `v0.38.0` row's shape.
- **History row** — the release execution, the publish outcome, and (Case B) the correction.

### 3.3 Indexes

`rfcs/README.md`: move RFC-079 from proposed to done; update the pre-v1 feature-work theme row to say
`0.39.0` shipped.

## 4. Case A — `matten-stats` was NOT published

The record is already accurate; only the release itself needs recording.

```text
ROADMAP Status: state that matten-stats remains unpublished pending the external
  read of RFC-078 §4.1, and that its 0.39.0 version exists only in-repository
RFC-078 status: unchanged — implemented and released in-repo, not yet on crates.io
no correction needed to ROADMAP 3.25.0 or RFC-079 §3
```

Optionally, record the deferral in `docs/src/contributing/release-checklist.md`'s publish-ordering section
(around line 237) so the next person publishing sees it at the moment it applies. One line, removed when
the crate first publishes. This is a judgement call, not a requirement.

## 5. Case B — `matten-stats` WAS published

Then several statements in the repository are now false and must be corrected, not quietly left.

```text
[ ] ROADMAP 3.25.0 says "matten-stats is explicitly excluded from that step's
    publish list" — do NOT edit that history row; it accurately records the
    decision as it stood. Add a NEW history row recording that the publish
    nonetheless included matten-stats, and why the earlier statement no longer
    describes reality.
[ ] RFC-079 §3's deferral is now moot — the crate name is claimed and the
    ddof = 1 choice is shipped. Update RFC-079's status to say so.
[ ] RFC-078's status: change from released-in-repo to published at 0.39.0.
[ ] CHANGELOG [0.39.0]: it currently makes zero mention of matten-stats, which
    was correct for a release that excluded it. If the crate shipped, the entry
    misdescribes the release. Add a corrective note — either amend the [0.39.0]
    entry or add a dated clarification — stating that matten-stats 0.39.0 was
    published, at Experimental maturity, with the ddof = 1 divergence.
[ ] The external ddof read is no longer a gate on first publication. It now
    informs whether a FUTURE change is warranted. Restate it that way rather
    than deleting it — the open question did not disappear, its consequence changed.
```

**Do not rewrite history rows to make the record look consistent.** The project's convention, established
across RFC-074/075, is that a superseded decision stays recorded and a new entry explains what changed.

## 6. Scope

### Out of scope — a diff touching these is a defect

```text
any crates/*/src/*.rs change
any Cargo.toml or Cargo.lock change (including publish keys)
any version change — 0.39.0 stays
any maturity-label change
any new tag or publish action
RFC-076 updates
```

## 7. Verification

```bash
bash scripts/check-release-docs.sh      # includes the ROADMAP header/history parity guard
git diff --check
git diff --name-only                    # expect ONLY .md files
git status --porcelain                  # no Cargo.toml / Cargo.lock / .rs
```

The parity guard will fail if the ROADMAP header is bumped without appending its history row — the exact
defect it was written for.

## 8. What the review request must report

```text
[ ] the Step 0 answer, and how it was determined (NOT via publish tooling)
[ ] which case was executed, A or B
[ ] for Case B: each of the five corrections, with the superseded history row left intact
[ ] ROADMAP header == last history row, dated
[ ] git diff --name-only showing documentation files only
[ ] confirmation no version, manifest, tag, or publish change occurred
```

## 9. Known pitfalls

1. **Assuming the publish outcome instead of checking it.** Step 0 exists because the record and reality
   may disagree.
2. **Using `cargo publish --dry-run` to determine state.** Wrong instrument; check crates.io directly.
3. **Editing ROADMAP `3.25.0`** to make it retroactively correct. Add a new row instead.
4. **Deleting the external-ddof-read requirement** in Case B. Its consequence changed; the question did not
   go away.
5. **Bumping the ROADMAP header without a history row** — guard-caught, but check anyway.

## 10. Review stop

Acceptance makes this a commit point. It authorizes no release, version, tag, or publish action.
