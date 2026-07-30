# RFC-085 `matten-data` Promotion: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/085-promote-matten-data-production-ready.md`
**Document kind:** Detailed implementation handoff
**Status:** Inherits RFC-085's state — **implemented and reviewed 2026-07-30**. `matten-data` is
production-ready; RFC-059 §6's deferred review is closed
**Date:** 2026-07-30

---

## 1. Purpose

Promote `matten-data` **production-ready candidate → production-ready**, closing the review RFC-059 §6
deferred.

Label, documentation and guard changes only. **No code, API, test, example, or behaviour change.** No
release, no version bump; version stays `0.39.0`.

Unlike RFC-084, there is no CI gap to close first — `matten-data` already has a dedicated job and all
nine examples in the smoke runs. This is a single part.

## 2. Preconditions

```text
RFC-085 and this handoff accepted
working tree clean; version stays 0.39.0
scripts/check-release-docs.sh currently ASSERTS "production-ready candidate" for matten-data
  (line 102). Expect it to fail mid-change — that is the guard working. It gets inverted, not
  disabled.
```

## 3. The guard — two separate jobs in one block

### 3.1 Invert the label assertion

The `matten-data` block currently checks for the *candidate* label. Move it to *production-ready*,
keeping the four-check shape it already has (negative label check, positive check, shared-docs check).

**Careful with substring matching.** `"production-ready candidate"` **contains** `"production-ready"`,
so a naive `grep -qi "production-ready"` positive check passes even when the README still says
*candidate*. The negative check must reject the candidate label explicitly, e.g. a pattern anchored on
the README's lead banner and the `lib.rs` Status line. **Prove this specific trap is handled**: leave
the README saying "production-ready candidate" and confirm the guard **fails**.

This is the single most likely way to ship a guard that cannot detect the exact staleness it exists
to catch.

### 3.2 Fix the line-89 blanket grep — the RFC-084 C1 cleanup

```bash
# scripts/check-release-docs.sh:89
if grep -rIni "experimental" "$DATA/README.md" "$DATA/src/lib.rs" docs/src/examples/data.md "$DATA/examples"
```

This bans the **word** "experimental", case-insensitively, across four whole paths. It is the same
over-broad shape the RFC-084 review (finding C1) required narrowing for `matten-stats`, where it had
already forced three documentation sites to be rewritten less informatively. It is harmless today only
because `matten-data` was never at the `Experimental` label — so nothing legitimate needs the word yet.

Narrow it to a present-tense label claim, exactly as C1 required. **Prove both directions:**

```text
a past-tense or unrelated sentence containing "experimental" must PASS
a present-tense stale label must FAIL
```

Recorded as a candidate theme in `ROADMAP.md` §3.1; this slice closes it because it is touching that
block anyway.

## 4. Label sites

**Derive the list by sweep-and-classify. Do not trust this section as complete.**

```bash
grep -rniE "matten-data.{0,90}(production-ready candidate|candidate)" \
  --include="*.md" --include="*.rs" --include="*.sh" --include="*.toml" . | grep -v "^./.git-exclude"
```

Then classify each hit **live current claim** (change) or **historical record** (leave). Run it again
after editing and confirm every survivor is deliberate.

Known live sites, as a cross-check only:

```text
crates/matten-data/README.md            maturity line (~91), plus the §5 streaming-scope note
crates/matten-data/src/lib.rs           Status section, plus the §5 streaming-scope note
README.md                               root crate table row (~30)
docs/src/reference/compatibility.md     CURRENT-status sites only — this file deliberately carries
                                        per-family history; do not rewrite the historical entries
                                        recording earlier promotions
docs/src/examples/{data,companions,index}.md
docs/src/contributing/release-checklist.md
scripts/check-release-docs.sh           §3
ROADMAP.md                              Status prose + §3.1 candidate-theme rows + header + history row
rfcs/README.md                          remaining-themes row + RFC-085's own entry
```

**`ROADMAP.md` §3.1 needs two edits**, since this slice consumes two of its recorded candidates:
the `matten-data → full production-ready` advancement row, and the `check-release-docs.sh:89`
hygiene row.

### 4.1 Must not be touched

```text
rfcs/done/**            incl. RFC-059 §6's deferral ruling — a correct record of its position.
                        RFC-085 §2 answers it in argument; it does not get edited away
rfcs/handoffs/**
CHANGELOG.md
docs/design/history/**
ROADMAP.md history rows (append a new one; do not rewrite existing rows)
```

## 5. The streaming-scope note (RFC-085 §5)

The promotion covers the crate, and the `streaming` feature is stable **in what it does** while its
**scope may still grow** (RFC-082 §5 defers nine items). Say so in three places — crate rustdoc,
`crates/matten-data/README.md`, and `docs/src/reference/compatibility.md` — so a reader does not infer
that "production-ready" promises async, resumable, or parallel streaming.

Keep it short and factual. Do not restate RFC-082 §5's whole deferral list; point at it.

## 6. Required verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
bash scripts/check-release-docs.sh          # MODIFIED here — must pass with the new label
bash scripts/check-matten-data-scope.sh     # RFC-042 lock — must pass UNMODIFIED
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

Three deliberate-failure proofs are required, not one. A check that cannot fail is worthless, and this
project has shipped that mistake before:

```text
PROOF 1  README left at "production-ready candidate"     -> guard MUST fail  (§3.1's substring trap)
PROOF 2  a past-tense/unrelated "experimental" sentence  -> guard MUST pass  (§3.2)
PROOF 3  a present-tense stale label                     -> guard MUST fail  (§3.2)
```

Report all three outcomes, with the commands.

Scope confirmation:

```bash
git diff --name-only | grep -E '^(rfcs/done|rfcs/handoffs|docs/design/history)/|^CHANGELOG.md'
# expect EMPTY
git diff --name-only -- crates/matten crates/matten-mlprep crates/matten-ndarray crates/matten-stats
# expect EMPTY
git diff --name-only -- crates/matten-data/src crates/matten-data/tests crates/matten-data/examples
# expect ONLY crates/matten-data/src/lib.rs, and its diff must be //! doc lines only
grep -m1 '^version' Cargo.toml     # still 0.39.0
```

## 7. Known pitfalls

1. **The substring trap** (§3.1) — `"production-ready candidate"` contains `"production-ready"`. A
   naive positive check passes on stale text. Proof 1 exists for this.
2. **Deleting the guard block** instead of inverting it.
3. **Rewriting `compatibility.md`'s per-family history.** That file intentionally records earlier
   promotions; only current-status sites move.
4. **Editing RFC-059's deferral ruling** in `rfcs/done/`. It is history.
5. **Trusting §4's site list** instead of sweeping. Three prior RFCs shipped incomplete lists.
6. **Touching `matten-data`'s code, tests, or examples.** Labels, docs and one guard only.
7. **Weakening the RFC-042 scope lock**, or reading "production-ready" as licence to add dataframe
   features. It is not — the scope lock is unchanged and its guard must pass unmodified.
8. **Implying the `streaming` feature's scope is finished** (§5).
9. **Bumping the version**, or treating this as v1.0 progress. RFC-076 stays deferred.

## 8. What the review request must report

```text
[ ] all three deliberate-failure proofs, with commands and outcomes
[ ] the guard diff: inverted label assertion + narrowed line-89 grep
[ ] the full sweep output, every hit classified live vs historical
[ ] the streaming-scope note in all three locations
[ ] confirmation rfcs/done/, rfcs/handoffs/, CHANGELOG.md, docs/design/history/ absent from the diff
[ ] confirmation no matten-data source, test, or example changed except lib.rs doc comments
[ ] RFC-042 scope guard passing UNMODIFIED
[ ] both ROADMAP §3.1 rows consumed (advancement + hygiene)
[ ] full gate set incl. MSRV and mdbook; version still 0.39.0
```

## 9. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, publish, further
promotion, or any RFC-076 execution.
