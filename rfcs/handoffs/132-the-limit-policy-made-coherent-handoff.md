# Developer Handoff — RFC-132 Stage 2: Applying the Boundary-Only Limit Model

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/accepted/132-the-limit-policy-made-coherent.md`
**Base:** **after RFC-127 ships as `0.46.2`.** Lands with RFC-129 in `0.47.0` — see §2.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Apply the limit model the owner chose: **Option A, boundary-only.** Stage 1 is decided; this is
execution.

## 2. This lands WITH RFC-129, not before or after

RFC-129 (`try_add`/`try_sub`/`try_mul`/`try_div`) was accepted before this decision, and one of its
tests contradicts it.

```text
RFC-129's original T2   a [2000,1000] pair returns Err
under Option A          that pair returns Ok
```

Both RFCs ship in `0.47.0`. **RFC-129's handoff has been amended.** If you are implementing them
separately, coordinate — landing RFC-129 alone ships a test asserting behaviour this task removes.

## 3. The rule you are implementing

> **Limits bound allocations sized by a value the CALLER SUPPLIED as data — a shape, a count, a
> parsed document. They do not bound allocations sized by data already in memory and already
> validated.**

**Learn this sentence rather than the site list below.** The list is derived from it, and the whole
point of the RFC is that the rule answers the question for operations that do not exist yet. If you
find a site the list does not mention, apply the rule and say which way you decided.

```text
LIMITS APPLY
  try_new / try_zeros / try_ones / try_full / try_reshape     caller-supplied shape
  the three _with_limits constructors                          unchanged, keep them
  from_json / load_json / serde Deserialize                    parsed document
  from_csv / load_csv / from_csv_path / Table / CsvBatchReader
  from_json_dynamic / from_csv_dynamic
  slice_str                                                    caller-supplied string
  repeat / repeat_axis / tile / meshgrid                        caller-supplied COUNT

LIMITS DO NOT APPLY
  arithmetic and its try_ twins
  reductions, axis reductions, statistics
  matmul / dot / outer / trace / norm
  slicing an existing tensor
  concatenate / stack
```

**`repeat`/`tile` are the subtle case** — they act on an existing tensor but the *multiplier* comes
from the caller, so the size does not come from the data. They stay guarded. If that reasoning does
not convince you, say so before removing their guards.

## 4. Change A — remove the checks that no longer apply

Every site in the second list. **This is a behaviour change**: operations that panicked now succeed.

```text
DO NOT simply delete the check and leave the error variant unused. If a
MattenError variant becomes unconstructible, say so in the review request —
that is a finding, not a tidy-up, and RFC-111 left exactly such a variant behind
in matten-ndarray.
```

## 5. Change B — `max_parse_bytes`, which becomes required

Under Option A this is **the** boundary control, and it is currently public, documented, and enforced
nowhere — its own rustdoc says so.

```text
enforce it in load_json, load_csv, from_csv_path and the string parsers
use a metadata pre-check AND Read::take — a size check alone is a TOCTOU race
then DELETE the "do not yet enforce this limit at runtime" note
```

**Do not leave the note while adding the enforcement, and do not delete the note without adding it.**
Either half alone leaves the documentation lying in a new direction.

## 6. Change C — the two sentences

```text
limits.rs   "the single source of truth for all allocation budgets" — rewrite to
            state §3's rule. It is false today and would be false in a new way
            after this change.
the broadcast panic message   advises "increase the limit", a remedy that does
            not exist. Under A that check is removed, so the message goes with
            it. GREP for other messages making the same offer and report what
            you find.
```

## 7. Required tests

```text
T1  a hostile shape is still rejected at try_new and from_json   (RFC-127's case)
T2  a large in-memory pair now SUCCEEDS under arithmetic — the positive
    assertion that the budget no longer applies there
T3  load_json / load_csv refuse a file exceeding max_parse_bytes
T4  repeat / tile with a huge count still ERROR
T5  the three _with_limits constructors still honour a custom budget
T6  every pre-existing test passes, EXCEPT those asserting a budget error on a
    non-boundary operation — list each one you changed and why
```

**T6 is the one to watch.** Any test you edit is a behaviour change you are shipping; the list of
edits is the change's real surface area.

## 8. Out of scope

```text
changing the DEFAULT budget values      a separate decision; 1_048_576 may well
                                        be wrong but that is not this task
per-Tensor or per-call limits           options B and C were not chosen
RFC-127's fixes                         ship first, in 0.46.2
performance work                        RFC-133
the version bump                        the 0.47.0 release RFC owns it
```

## 9. Risks

```text
R1  Removing a guard from a site whose size DOES come from the caller —
    repeat/tile are the trap (§3).
R2  Deleting a check and leaving an unconstructible error variant unreported (§4).
R3  Enforcing max_parse_bytes with a size check only, leaving a TOCTOU race (§5).
R4  Deleting the "does not yet enforce" note without adding the enforcement (§5).
R5  Landing without RFC-129 (§2).
R6  Editing tests to pass instead of listing them as behaviour changes (§7).
R7  Applying the site list mechanically instead of the rule, so a site the list
    misses gets no decision at all (§3).
```

## 10. Acceptance criteria

```text
[ ] §3's rule applied; any site not in the list decided BY THE RULE and reported
[ ] limits removed from every non-boundary site; unconstructible variants reported
[ ] max_parse_bytes enforced with metadata pre-check AND Read::take; note deleted
[ ] limits.rs's "single source of truth" sentence rewritten to state the rule
[ ] the broadcast message advising an unreachable remedy is gone; grep reported
[ ] T1-T6; T6's edited tests listed with reasons
[ ] the §2 probe from the RFC now produces ONE coherent behaviour, not three
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] no version bump, tag, or publish
```

## 11. Required review-request format

Write to:
`.git-exclude/review-request/RFC-132/matten-rfc132-limit-policy-stage2-implementation-review-request-v0.1.md`

Include the site list you derived and any site you decided by the rule rather than the list, the
before/after of the RFC §2 probe, every test you edited with its reason, the grep for remedy-offering
messages, and deviations with reasoning.
