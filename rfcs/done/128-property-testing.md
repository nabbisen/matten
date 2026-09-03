# RFC-128: Property Testing

**Status:** **Implemented** 2026-09-03 in commit *"Add property tests for the shape/data invariant,
index round-trip, broadcasting, and slice bounds (RFC-128)"* (`56bef1d`), reviewed and approved with
**no corrections**. **Unreleased** — §7's packaging question was answered (test files DO ship), but
adding tests is not patch content under RFC-094 §4.1, so it rides `0.47.0` with RFC-129 and RFC-132
rather than needing a release of its own. Handoff: `rfcs/handoffs/128-property-testing-handoff.md`.
**Target:** `crates/matten/Cargo.toml` (`[dev-dependencies]`), `crates/matten/src/**/tests.rs`,
`.github/workflows/test.yaml`
**Theme:** Write the invariants down as properties, so the next RFC-111 cannot remove one silently
**Related:** RFC-127 (the defect this would have caught), RFC-111, RFC-013 (property testing recorded
as aspirational), RFC-094 §4.3 (no release)

---

## 1. Summary

```text
Add proptest as a [dev-dependencies] entry and write the crate's invariants as
properties: the shape/data invariant, the index round-trip, broadcasting, and
slice bounds.

NO published dependency change. dev-dependencies do not enter the dependency
graph of anyone who depends on matten.
NO crates/ CONTENT change beyond test files and one manifest section.
```

**No release.** A `[dev-dependencies]` entry and test files change nothing a user downloads — see §7.

## 2. Why this, and why now

The external architect's single strongest process claim, and it is checkable:

> *"Findings #1, #3, and #6 would each have been caught by a three-line property test."*

**RFC-127's Critical is the proof.** `shape.iter().product() == data.len()` is the crate's one global
invariant. It is asserted by convention at ~31 construction sites, documented in RFC-001 as O1, and
relied on by every `expect("valid by construction")` in the codebase. **It was never written down as
a property**, so when RFC-111 removed the bound that incidentally protected it, 772 example-based
tests and nine guards saw nothing.

```text
772 tests pass.  Nine guards pass.  Clippy is clean.  The defect is live on crates.io.
```

That gap is not a criticism of the suite's thoroughness — it is a limit of the *technique*. An
example test asserts what someone thought to write down. A property test asserts what must always be
true, against inputs nobody thought of.

**RFC-013 already recorded property testing as aspirational and explicitly not a gate.** This RFC
proposes making it real, and the justification is no longer aspirational: it is one Critical, live.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | No `proptest`/`quickcheck`/`arbitrary`/`cargo-fuzz` in any manifest | `grep` across all `Cargo.toml` — zero hits |
| E2 | The suite is entirely example-based: 772 tests, 133 doctests, ~96 executed examples | audit Step 0, reconfirmed by `cargo test --workspace --all-features` |
| E3 | O1 is stated as an obligation but asserted nowhere centrally | RFC-001 §O1; ~31 `Tensor { .. }` literals |
| E4 | RFC-127's Critical breaks O1 through a public `Result` API | probe: `shape.product()=3.4e38`, `data.len()=1` |
| E5 | `dev-dependencies` do not propagate to downstream consumers | Cargo's dependency model; verify with `cargo tree -e normal` |
| E6 | RFC-013 recorded property testing as aspirational, not a gate | RFC-013 |

**Re-derive E5 before relying on it** — it is the whole argument that this needs no release.

## 4. The properties

Four, in priority order. **The first is the one that matters.**

```text
P1  O1, the global invariant
    for any tensor produced by ANY public constructor or operation:
        shape.iter().product() == data.len()
    This is the property whose absence produced RFC-127.

P2  index round-trip
    for any valid shape and any flat index in range:
        coord_to_flat(flat_to_coord(i)) == i

P3  broadcasting algebra
    for any two broadcast-compatible shapes:
        the result shape is the pairwise max, right-aligned
    and for any INcompatible pair, the operation errors rather than panicking
    or producing a wrong shape

P4  slice bounds
    for any tensor and any index:
        the result is either Err, or a tensor whose every element appears in
        the source
    This is the property whose absence produced RFC-127's F-5.
```

### 4.1 Generate shapes that actually probe the boundary

**A generator that only produces small friendly shapes proves nothing.** The defect space is at the
edges:

```text
include  zero dimensions (RFC-111 made them legal)
include  rank 0 (the scalar) and the maximum rank
include  very large single dimensions — the RFC-127 case
include  shapes whose product overflows usize
```

**Bound the generated *data* size, not the generated shape**, or the property suite will itself try
to allocate. Generate the shape freely; construct only when the product is small enough, and assert
that the constructor **rejects** the rest rather than skipping it.

## 5. Where the properties live, and CI

```text
crates/matten/src/**/tests.rs    alongside the existing unit tests, same modules
.github/workflows/test.yaml      the properties run in the existing test job;
                                 no new job unless runtime demands one
```

**Report the added CI runtime.** If it is material, say so and propose a case count rather than
silently slowing every push.

## 6. What this RFC does not claim

```text
- it does not replace the example suite. Examples document intent; properties
  check invariants. Both.
- it does not make the crate correct. It makes one class of defect visible.
- it does not fix RFC-127. That ships first, and independently. If the
  properties are written first they SHOULD fail — see §8.
```

## 7. Why no release

```text
git diff --name-only <last-tag>..HEAD -- crates/   will be NON-EMPTY
```

That is RFC-094 §4.3's mechanical test, and it will say "releasable". **It is wrong here, and this is
the interesting case**: the changed paths are `crates/matten/Cargo.toml`'s `[dev-dependencies]`
section and `tests.rs` files.

```text
cargo package --list -p matten     -> does it include src/**/tests.rs?
                                      -> DERIVE THIS. If tests.rs files ARE packaged,
                                         they ship, and §4.3's test is right and this
                                         needs a release after all.
```

**Settle it by running `cargo package --list`, not by assuming.** The RFC-120 amendment made §4.1
cover "code, rustdoc, or a packaged README" — a packaged test file would be code. Report the answer
either way; it decides whether a release RFC follows.

## 8. Sequencing

```text
RFC-127 ships first, as 0.46.2. This RFC follows.
```

**Optionally, and it is worth doing:** write P1 and P4 *before* RFC-127's fix and confirm they
**fail**. That is rule 002 §4 applied to a test suite — a property accepted without seeing it fail is
a property nobody has verified detects anything. If you do this, report both outputs.

## 9. Scope

### Out of scope — a diff touching these is a defect

```text
any src/ change other than test modules     this RFC adds tests, not fixes
any [dependencies] entry                     dev-dependencies ONLY (§7)
fuzzing (cargo-fuzz), coverage gates         larger, separate; cargo-fuzz needs
                                             nightly and RFC-049's Phase 4 is
                                             unauthorized
RFC-127's fixes                              its own RFC, ships first
CHANGELOG, version, pins                     no release (§7, pending its check)
```

## 10. Risks

```text
R1  A generator that never produces a degenerate shape. The properties then pass
    forever and prove nothing — the vacuous-guard failure in test form (§4.1).
R2  Generating unbounded DATA and OOMing CI. Bound the data, not the shape.
R3  Adding proptest to [dependencies] instead of [dev-dependencies]. That would
    change every downstream user's dependency graph and would need a release.
R4  Assuming no release is needed without running cargo package --list (§7).
R5  Weakening a property until it passes. If a property fails, that is a finding
    — report it, do not tune the property.
R6  Letting this block RFC-127. It ships first (§8).
```

## 11. Acceptance criteria

```text
[ ] proptest in [dev-dependencies] ONLY — asserted
[ ] cargo tree -e normal shows no new normal dependency for downstream users
[ ] P1-P4 implemented
[ ] the shape generator produces zero dims, rank 0, max rank, and huge dims —
    demonstrated, not asserted
[ ] generated data is bounded; the suite cannot OOM
[ ] added CI runtime measured and reported
[ ] cargo package --list -p matten checked; the release question answered either way
[ ] no src/ change outside test modules
[ ] nine guards; cargo test --workspace --all-features
[ ] no version bump, tag, or publish
```

## 12. What this does not fix

```text
- the defects themselves — RFC-127
- fuzzing proper, and coverage measurement
- the absence of a differential suite against ndarray, which the audit rates the
  highest-signal test available for a crate selling NumPy-like semantics
```

And it does not guarantee the next invariant gets written down. **A property suite only checks the
invariants someone names.** RFC-111 removed a bound nobody had named; the discipline this RFC needs
alongside it is that a semantic change asks *"which invariant did this rest on?"* — which is a review
question, not a tooling one.
