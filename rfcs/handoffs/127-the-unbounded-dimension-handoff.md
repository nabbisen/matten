# Developer Handoff — RFC-127: The Unbounded Dimension

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/accepted/127-the-unbounded-dimension.md`
**Base:** `main` @ `88bc784`, clean tree, family at `0.46.1`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Close every process abort and the silent slice wrong-answer found by the external architect audit of
`0.46.1`. **Correctness fixes only — no new public API.**

## 2. Read this before anything else

**A 36-byte JSON document currently aborts the `matten` process, uncatchably.** This is the most
serious defect found in this project, it is reachable from untrusted input, and it is live on
crates.io right now.

```text
from_json(r#"{"shape":[400000000000,0],"data":[]}"#)  -> Ok, shape=[400000000000, 0], len=0
  then .sum_axis(1)  -> "memory allocation of 3200000000000 bytes failed" -> SIGABRT
                        catch_unwind does NOT recover
```

> **The cause is one missing bound, and it traces to RFC-111.** `checked_shape_len` guards only the
> *product*. A zero dimension makes the product `0`, so `checked_mul` can never overflow and **no
> individual dimension is ever bounded**. Before RFC-111 a zero dimension was rejected, which
> incidentally bounded every dimension. RFC-111 removed that rejection deliberately and correctly.
> Nothing downstream was revisited.

**RFC-111 stays.** Zero-sized dimensions are accepted on purpose after a three-stage campaign
(RFC-105/110/111) and slicing depends on them. The defect is the *absent bound*, not the accepted
zero. Re-rejecting zero to make the abort disappear is R1 and it is the likeliest wrong turn here.

## 3. Re-derive the defects first — four short probes

**Do this before editing.** They are the entire justification for the RFC, and you should see them
fail with your own eyes.

```text
E2  from_json + try_new on [400000000000, 0]      -> both Ok today
E3  .sum_axis(1) on that tensor                    -> process abort, catch_unwind does not save
E4  try_matmul([usize::MAX,0], [0,usize::MAX])     -> Ok, shape.product()=3.4e38, data.len()=1
E5  [2,3] tensor: slice().index(usize::MAX)        -> Ok, returns the LAST ROW [4,5,6]
    same tensor:  slice().index(9)                 -> Err, correctly
```

A scratch crate depending on `crates/matten` by path is enough; run E3 in a subprocess since it
aborts. **Report what you observed** — if any of the four does not reproduce, stop and say so.

## 4. Change A — bound each dimension (`shape.rs`)

The root cause, and the ~10 lines that close every abort in the report.

```text
in checked_shape_len, BEFORE the product loop:
    reject any single dim that exceeds the element budget
then the existing checked_mul loop, byte-identical
```

```text
USE the existing MattenError::Allocation variant, with an actionable message
naming the offending dimension. Same error class the guarded sites already produce.

DO NOT reject dim == 0.
DO NOT change the checked_mul loop.
```

**Both `validate_shape` and `MattenLimits::check_shape` call this helper**, so one change should cover
the ~18 sites that fan out from them. **Verify that yourself** rather than trusting it — if some
allocation path does not reach `checked_shape_len`, that is a finding and Change B's list grows.

## 5. Change B — the remaining allocation sites, but only the ones still reachable

The audit reports the limit applied at 8 of ~13 shape-derived allocation sites.

```text
1. APPLY CHANGE A FIRST.
2. THEN re-probe each cited site to see which are still reachable.
3. Guard only those, copying the three-line pattern from linalg.rs:180-183.
```

**A guard that can no longer fire is worse than no guard** — it reads as coverage and can never fail.
That is exactly what RFC-117's ninth guard was written to avoid.

**Derive the site list yourself and report your method.** The reviewer confirmed the pattern at the
cited lines but did **not** independently derive the denominator, and "~13" is the audit's
approximation. A discrepancy is worth more than the edit.

## 6. Change C — the sign flip (`slice.rs`)

Six `as isize` casts at `301, 302, 311, 320, 335, 336`. `usize::MAX as isize` is `-1`, and `-1` means
"from the end" under RFC-088's negative indexing.

```text
replace each with isize::try_from, returning the existing out-of-range Slice error
delete the comment at slice.rs:332-333
```

That comment tells the next reader the hazard is already handled. It is not. **A false reassurance is
worse than silence** — leaving it would be the same defect class the whole `0.46.1` release was about.

## 7. Change D — `Tensor::new`'s rustdoc

A `pub(crate)` helper sits between the doc block and the `pub fn new` it was written for, so Rust
bound the docs to the helper. The flagship constructor renders on docs.rs with no description and no
`# Panics`.

```text
move panic_if_dynamic OUT from between the doc block and pub fn new
add missing_docs = "deny" so it cannot recur
```

```text
IF the deny surfaces gaps beyond Tensor::new — the audit says there are none in
194 items — REPORT THE LIST AND STOP. Do not fix them.
This is a correctness patch. A widening diff is how a patch stops being one.
```

## 8. Change E — the invariant, asserted in debug only

`shape.iter().product() == data.len()` is the crate's one global invariant, and E4 shows a public
`Result` API can break it. ~31 `Tensor { .. }` sites assert it by convention only.

```text
add a debug-only assertion; call it from those construction sites
cfg(debug_assertions) ONLY
```

Under debug it turns all 772 tests and 133 doctests into invariant checks for free. **Do not make it a
release-mode check** — that is a behaviour change and exceeds patch scope (R3).

## 9. Required tests

```text
T1  the E2 shape is REJECTED at construction, from from_json AND try_new
T2  RFC-111's own zero-sized tests pass UNMODIFIED — editing one is a signal
    you re-rejected zero
T3  no operation on any constructible tensor aborts; E3's case now Err or valid
T4  try_matmul cannot return shape.product() != data.len()  (E4)
T5  slice().index(usize::MAX) -> Err; index(9) still Err; ordinary indices
    unchanged  (E5)
T6  every pre-existing test passes unmodified except where T2 explains it
```

## 10. Out of scope — a diff touching these is a defect

```text
try_add / try_sub / try_mul / try_div     NEW PUBLIC API. RFC-094 §4.1 excludes
                                           it from a patch. This is the single
                                           most tempting addition here and the
                                           one the RFC explicitly forbids (§9).
re-rejecting zero dimensions               RFC-111 stands
enforcing max_parse_bytes                  behaviour change -> minor
performance work (P-1, P-2)                changes summation order -> minor
proptest                                   its own RFC
CHANGELOG.md, Cargo.toml version, pins     the release RFC owns those
docs/src/**                                separate cycle
```

**On `try_*`:** the audit rates it the highest-*likelihood* finding in the report, and it is genuinely
urgent. It is excluded here purely because a patch cannot carry new public API. It has its own RFC.
Adding it here would breach the clause RFC-120 was written to keep honest.

## 11. Risks

```text
R1  Re-rejecting zero dimensions. Reverts RFC-111, breaks slicing. T2 catches it.
R2  Adding guards at sites Change A already closed — vacuous checks (§5).
R3  Release-mode invariant assertion (§8).
R4  Letting missing_docs = "deny" widen this into a doc sweep (§7).
R5  Adding try_* "while we are here" (§10).
R6  Trusting the audit's site list instead of deriving it (§5).
R7  Treating this as authorizing a release. It does not.
```

## 12. Acceptance criteria

```text
[ ] E2-E5 re-derived and reported BEFORE editing
[ ] degenerate shapes rejected at construction, both entry points
[ ] RFC-111's zero-sized tests pass unmodified
[ ] no constructible tensor can abort the process
[ ] try_matmul cannot return a shape/data mismatch
[ ] slice().index(usize::MAX) -> Err; slice.rs:332-333's comment deleted
[ ] Tensor::new documented; missing_docs = "deny"; further gaps REPORTED not fixed
[ ] debug-only invariant assertion, cfg(debug_assertions), at the construction sites
[ ] T1-T6 present and passing
[ ] NO new public API — assert against the public-api surface
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] no version bump, tag, or publish
```

## 13. Required evidence

```text
- the four probes BEFORE the fix, with their output
- the same four AFTER, showing Err or a valid result
- your derived allocation-site list and the method that produced it
- which sites Change A already closed, and which still needed a guard
- confirmation that RFC-111's zero-sized tests were not edited
- the missing_docs output after the deny — including any gap you did NOT fix
- guard, test and clippy output
```

## 14. Required review-request format

Write to:
`.git-exclude/review-request/RFC-127/matten-rfc127-the-unbounded-dimension-implementation-review-request-v0.1.md`

Include files changed with line counts, §13's evidence, deviations with reasoning, and anything you
want answered at review.
