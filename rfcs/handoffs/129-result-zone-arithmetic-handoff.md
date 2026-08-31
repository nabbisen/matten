# Developer Handoff — RFC-129: Result-Zone Arithmetic

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/proposed/129-result-zone-arithmetic.md`
**Base:** after RFC-127 ships. Rides `0.47.0`, not a patch. See §2.

> **ACCEPTED 2026-09-01 and PRIORITIZED.** The owner authorized `0.47.0` early under RFC-094 §4.2(c)
> specifically to move this forward. **You may start.** It runs in parallel with RFC-127 — see §2.1.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Add `try_add` / `try_sub` / `try_mul` / `try_div`, with the operators delegating to them.

## 2. Why this is a minor, and why that matters to you

```text
RFC-094 §4.1   excluded: any new public API
```

Four new public functions cannot ride a patch. **This is the only reason the audit's
highest-*likelihood* finding ships after its Critical** — an ordinary `[2000,1000]` tensor trips it,
no hostile input needed.

**Practical consequence for you:** your diff will include `public-api-snapshot.md` rows, which
RFC-121's release explicitly forbade. Do not carry that instruction forward — see §5.

### 2.1 Parallel with RFC-127, and one trap

RFC-127 and this may run at the same time; their files are disjoint. But:

```text
crates/matten/src/ops/tensor_ops.rs    <- the operators. YOURS.
crates/matten/src/tensor/ops.rs        <- an RFC-127 allocation site. NOT yours.
```

**Two different files, near-identical paths.** The audit's own text uses both spellings. Confirm the
path before editing, and if you find yourself in `tensor/ops.rs`, stop.

## 3. The gap

```text
default max_elements = 1_048_576        a 1024x1024 matrix
a [2000,1000] tensor:  constructs, .abs() works, .sum_axis(0) works,
                       try_concatenate correctly returns Err
                       &big + &big   ->  PANIC, with no try_add anywhere
```

`Add`/`Sub`/`Mul`/`Div` are `std::ops` traits and **cannot** return `Result`. The operator panicking
is not the defect. **The defect is that no recoverable alternative exists** — in the library whose
stated proposition is legible, recoverable failure, and where nearly every other convenience API has a
`try_` twin.

## 4. The shape — copy `clip`/`try_clip`, do not invent

`crates/matten/src/elementwise.rs` already has this exact pattern. Follow it.

```text
try_add(&self, other: &Tensor) -> Result<Tensor, MattenError>      and sub/mul/div
impl Add for &Tensor  ->  delegates to try_add, panicking on Err
```

```text
THE OPERATOR'S PANIC MESSAGE MUST NOT CHANGE.
Its text is asserted by existing tests and quoted in the book. Preserving it is
what makes this purely additive.
VERIFY by running every existing arithmetic test UNMODIFIED. Editing one is a
signal the message moved — report it rather than updating the test.
```

**No new `MattenError` variant.** Broadcast incompatibility is `Broadcast`; budget excess is
`Allocation`. Both exist. A new variant for an existing condition fragments the error model this crate
is praised for. This is R2.

**Dynamic tensors:** `try_*` must return `MattenError::Unsupported`, matching `try_dot`/`try_matmul` —
not panic. Every numeric entry point already guards this; follow the same guard.

## 5. Documentation — and this inverts RFC-121

```text
public-api-snapshot.md   FOUR new rows. REQUIRED.
compatibility.md          a row per method, Supported, citing RFC-129
the arithmetic reference  state that the operators panic and name the try_ twin
```

> **RFC-121 forbade a snapshot row** because `0.46.1` changed no public item. **Here four items are
> added, so omitting them is the defect.** This is RFC-109's case, not RFC-103's. Carrying the recent
> instruction forward is the likeliest documentation error in this task.

## 6. Out of scope

```text
changing any operator's panic message      §4
a new MattenError variant                  §4
try_broadcast or a public broadcast API    larger, not proposed
threading MattenLimits through arithmetic  a real design question, deferred —
                                           today a caller cannot raise the budget
                                           for these ops at all
RFC-127's fixes                            ship first
the version bump                           the 0.47.0 release RFC owns it
a fifth try_ twin if you find a gap        REPORT it, do not add it here
```

## 7. Risks

```text
R1  Changing the operators' panic text (§4).
R2  Adding an error variant for a condition MattenError already covers (§4).
R3  Forgetting the dynamic guard, so try_add panics where try_dot returns
    Unsupported.
R4  Omitting the public-api-snapshot rows — RFC-121's instruction carried
    forward where RFC-109's applies (§5).
R5  Implementing try_* by duplicating broadcast logic instead of inverting the
    dependency. The operator delegates to try_, never the reverse.
R6  Shipping in a patch (§2).
```

## 8. Required tests

```text
T1  try_add/sub/mul/div return Ok with the same result the operator produces,
    for ordinary shapes
T2  a [2000,1000] pair returns Err rather than panicking
T3  broadcast-incompatible shapes return Err, not a panic
T4  a dynamic tensor returns Unsupported, not a panic
T5  the operators still panic, with byte-identical messages
T6  every pre-existing arithmetic test passes UNMODIFIED
```

## 9. Acceptance criteria

```text
[ ] the four try_ methods exist and return Result
[ ] the operators delegate; T6 holds
[ ] T1-T5 present and passing
[ ] no new MattenError variant
[ ] dynamic guard present on all four
[ ] public-api-snapshot.md gains four rows; compatibility.md and the arithmetic
    reference updated
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] no version bump, tag, or publish
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-129/matten-rfc129-result-zone-arithmetic-implementation-review-request-v0.1.md`

Quote the operator panic message before and after to show it is unchanged, list the four snapshot
rows, include guard/test/clippy output, deviations with reasoning, and anything you want answered.
