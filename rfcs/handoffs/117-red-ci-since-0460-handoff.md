# Developer Handoff — RFC-117: CI Has Been Red Since Before `0.46.0`

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/117-red-ci-since-0460.md`
**Base:** `main` @ `071f4a7`, clean tree.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Fix `process-boundary.sh`'s two `header_only` cases, add the missing `matten-data` bullet to
`[0.46.0]`, and add a ninth guard that runs the tools' shell tests locally.

## 2. Context you are owed

**CI has been red since before `0.46.0` was tagged** — four runs, same failure. The reviewer
authorized the push, tag and publish without checking the workflow on the commit just pushed. The
break is RFC-111's, the miss is the reviewer's, and **the shipped crates are correct** — the
regression is in a test.

Nothing you did is implicated. This is a defect fix plus the guard that should have caught it.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Failure is `header_only_absent: expected a nonzero exit` | reproduced locally; identical in CI runs `31301270368`, `31301275565`, `31301318598`, `31313280917` |
| E2 | Cause: RFC-111 removed core's zero-sized rejection, so a 0-row table now yields an empty tensor and the tool succeeds | `crates/matten-data/src/tests.rs:288-294`, the sibling test RFC-111 already inverted |
| E3 | `module-boundaries.sh` and its `--self-test` **pass** | run locally |
| E4 | No `scripts/*.sh` runs the tools' shell tests | `grep -rln "process-boundary" scripts/` → nothing |
| E5 | `tools/` is workspace-excluded, so `cargo test --workspace` never reaches it | root `Cargo.toml` `exclude` |
| E6 | `test.yaml:203-205` is the only thing that runs them | direct read |
| E7 | `[0.46.0]` mentions `matten-data` **zero** times | `sed -n '/## \[0.46.0\]/,/## \[0.45.0\]/p' CHANGELOG.md \| grep -c matten-data` |
| E8 | The two cases are `header_only_absent` (`:187`) and `header_only_existing` (`:199`) | direct read |

Re-derive before editing. **Report any discrepancy first.**

## 4. Change A — and the second case is NOT just an inversion

### 4.1 `header_only_absent` (`:187`)

The tool now succeeds and writes the JSON output. Replace `assert_process_error` with the
success-shape assertions the neighbouring cases already use (`assert_status … 0`, empty stdout/stderr,
output file exists, `assert_fingerprint`).

**Capture the fingerprint, do not compute it.** Byte count and SHA-256 must come from the actual
output — every other fingerprint in this file was captured the same way.

### 4.2 `header_only_existing` (`:199`) — its purpose no longer has a fixture

This case does **not** test the header-only input. It tests that **a pre-write failure leaves an
existing output file untouched** — it writes a sentinel, runs a case expected to fail, and asserts the
sentinel survived.

`header_only.csv` was the fixture that produced a pre-write failure. **It no longer fails**, so
inverting this case would delete the property it protects rather than update it.

```text
WRONG   invert it to assert successful overwrite — the pre-write-failure property
        then has NO test anywhere
RIGHT   repoint it at an input that still fails BEFORE writing, keeping the
        sentinel assertion intact
```

Candidates to evaluate — **pick one and justify it in the review request**:

```text
a nonexistent --input path        fails at read, before any write
--format html with no --output    an argument error (see the html_requires_output
                                  case at :140), but it has no --output to preserve,
                                  so it may not fit
fixtures/missing.csv, non_finite.csv   check whether either still errors
```

If **no** remaining input produces a pre-write failure, say so — that is a finding worth more than
the fix, because it would mean the property is now untestable through this tool's CLI.

**Do not change `matten-report`'s source.** The behaviour is correct.

## 5. Change B — the `[0.46.0]` bullet

Add one bullet to the **existing** `[0.46.0]` entry's `Changed` section. Do not restructure it; it was
reviewed and is otherwise accurate.

What it must say: `matten-data`'s `NumericTable::to_tensor()` on a table with zero data rows returned
`Err` in `0.45.0` and returns `Ok` with an empty tensor in `0.46.0`, following from core accepting
zero-sized dimensions. A header-only CSV is the ordinary way to reach it.

**This is correcting the record, not rewriting it** — the behaviour is in `0.46.0`; the entry simply
failed to mention it.

## 6. Change C — the ninth guard

```text
scripts/check-tool-tests.sh
```

It must run the same set as `test.yaml:203-205`.

**Derive the list from `test.yaml` rather than hand-copying it.** A hand-copied list is the next thing
to drift, and this whole RFC exists because of drift between what CI runs and what anything local
runs. If deriving is impractical, hand-copy **and** add a comment naming the risk — then say so in the
review request.

**Prove it can fail** (rule 002 §4): revert one assertion, confirm the guard rejects it, restore,
confirm it passes. State both outcomes.

## 7. Acceptance criteria

```text
[ ] process-boundary.sh passes; BOTH header_only cases handled (§4)
[ ] header_only_existing still protects the pre-write-failure property, or the
    absence of any such input is reported as a finding (§4.2)
[ ] fingerprints CAPTURED, not computed
[ ] module-boundaries.sh and --self-test unmodified and still passing
[ ] matten-report's source unchanged — assert via git diff --stat
[ ] [0.46.0] gains one matten-data bullet; nothing else in CHANGELOG changes
[ ] scripts/check-tool-tests.sh exists, matches test.yaml's set, and is PROVEN
    to fail on a reverted assertion
[ ] all NINE guards pass
[ ] no crate, no .rs file touched
[ ] no version bump, tag, or publish
```

## 8. Risks

```text
R1  Making matten-report error again. The behaviour is correct; the test is stale.
R2  Inverting header_only_existing and silently deleting the pre-write-failure
    property (§4.2). The likeliest defect here.
R3  Computing a fingerprint instead of capturing it.
R4  A hand-copied guard list that drifts from test.yaml (§6).
R5  Restructuring the [0.46.0] entry rather than adding one bullet.
R6  Fixing only the first case — the script stops at the first failure, so the
    second is untested until the first passes.
```

## 9. Required review-request format

Write to:
`.git-exclude/review-request/RFC-117/matten-rfc117-red-ci-since-0460-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, which input you
chose for `header_only_existing` and why, how each fingerprint was captured, the guard's
proven-failure output, all nine guards, and anything you want answered at review.
