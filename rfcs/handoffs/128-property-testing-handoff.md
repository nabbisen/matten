# Developer Handoff — RFC-128: Property Testing

**From:** High-capability model. **Date:** 2026-09-01.
**Design authority:** `rfcs/proposed/128-property-testing.md`
**Base:** after RFC-127 ships as `0.46.2`. See §1.

> **PENDING ACCEPTANCE.** RFC-128 is in `proposed/`. **Do not start** until the owner accepts it and
> it moves to `accepted/`. This handoff is written ahead so the package is ready, not to authorize work.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Add `proptest` as a **dev-dependency** and write the crate's invariants as properties.

**RFC-127 ships first.** This does not block it and must not delay it.

## 2. What makes this task different

Every other test you have written for this project asserts a case someone chose. **These assert what
must be true of cases nobody chose**, and the difference is the entire point:

> `shape.iter().product() == data.len()` is the crate's one global invariant. It is named in RFC-001
> as O1, asserted by convention at ~31 construction sites, and relied on by every
> `expect("valid by construction")` in the codebase. **Nobody ever wrote it down as a property.** When
> RFC-111 removed the bound that incidentally protected it, 772 tests, 133 doctests, nine guards and a
> clean clippy saw nothing — and a 36-byte JSON document could abort the process.

You are not adding coverage. You are adding a *kind* of check the project does not currently have.

## 3. The four properties

```text
P1  for any tensor from ANY public constructor or operation:
        shape.iter().product() == data.len()
    <- the property whose absence produced RFC-127's Critical

P2  for any valid shape and any in-range flat index:
        coord_to_flat(flat_to_coord(i)) == i

P3  for any two broadcast-compatible shapes: result is the pairwise max,
    right-aligned; for any incompatible pair: an Err, not a panic, not a
    wrong shape

P4  for any tensor and any index: the result is Err, or a tensor whose every
    element appears in the source
    <- the property whose absence produced RFC-127's silent last-row answer
```

**P1 and P4 are the two that matter.** If time is short, they come first.

## 4. The generator is the whole task

**A generator that only produces small friendly shapes proves nothing and will pass forever.** That is
the vacuous-guard failure in test form, and it is R1.

```text
MUST generate  zero dimensions            RFC-111 made them legal
               rank 0                     the scalar
               the maximum rank
               very large single dims      the RFC-127 case
               shapes whose product overflows usize

MUST bound     the DATA size, not the shape.
               Generate shapes freely. Construct only when the product is small.
               For the rest, assert the constructor REJECTS them — that is the
               property, not a skip.
```

**Demonstrate the generator produces those classes.** Print a sample, or assert coverage. "It should
generate them" is not evidence.

## 5. Prove the properties can fail — worth the extra step

Rule 002 §4 applied to a test suite. **A property accepted without seeing it fail is a property
nobody has verified detects anything.**

```text
if RFC-127 has NOT yet landed in your tree:
    write P1 and P4 first and confirm they FAIL against the current code
    then rebase onto RFC-127 and confirm they pass
if RFC-127 HAS landed:
    temporarily revert one of its guards, confirm the property fails, restore
```

Report both outputs either way.

## 6. The release question — answer it, do not assume it

RFC-094 §4.3's mechanical test will say this is releasable, because `crates/matten/Cargo.toml` and
`src/**/tests.rs` are under `crates/`.

```text
RUN:  cargo package --list -p matten
      -> does it include src/**/tests.rs ?

if NO   the change reaches no published package -> no release. Say so.
if YES  the test files ship, §4.3 is right, and a release RFC must follow.
```

**This is a real question with a real answer and I have not run it.** Report what you find.

## 7. Out of scope

```text
[dependencies]                    dev-dependencies ONLY. Adding proptest to
                                  [dependencies] changes every downstream user's
                                  graph and would need a release. This is R3.
cargo-fuzz, coverage gates        larger; cargo-fuzz needs nightly, and RFC-049's
                                  Phase 4 is explicitly unauthorized
any src/ change outside tests     this RFC adds tests, not fixes
RFC-127's fixes                   its own RFC, ships first
weakening a property until it passes   if a property fails, that is a FINDING
```

## 8. Risks

```text
R1  A generator that never produces a degenerate shape (§4).
R2  Unbounded generated DATA, OOMing CI (§4).
R3  proptest in [dependencies] rather than [dev-dependencies] (§7).
R4  Assuming no release is needed without running cargo package --list (§6).
R5  Tuning a failing property until it passes. Report it instead.
R6  Delaying RFC-127. This ships after.
```

## 9. Acceptance criteria

```text
[ ] proptest in [dev-dependencies] only — asserted
[ ] cargo tree -e normal shows no new normal dependency downstream
[ ] P1-P4 implemented
[ ] the generator demonstrably produces zero dims, rank 0, max rank, huge dims
[ ] generated data bounded; the suite cannot OOM
[ ] at least P1 and P4 PROVEN able to fail (§5), both outputs reported
[ ] added CI runtime measured and reported
[ ] cargo package --list -p matten run; the release question answered (§6)
[ ] no src/ change outside test modules
[ ] nine guards; cargo test --workspace --all-features
[ ] no version bump, tag, or publish
```

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-128/matten-rfc128-property-testing-implementation-review-request-v0.1.md`

Include the generator's demonstrated coverage, §5's failure proof, §6's packaging answer, the CI
runtime delta, deviations with reasoning, and anything you want answered at review.
