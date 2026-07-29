# RFC-077: Seeded Train/Test Split for `matten-mlprep`

**Status:** Implemented — reviewed (GO, no conditions, `matten-rfc077-implementation-review-v0.1.md`), committed `4c554a4`; pre-v1 additive API on the `0.38.x` line; no release authorized
**Target:** Pre-`1.0.0` feature work on the `0.x` line; release family undecided
**Theme:** Implement the seeded shuffled split RFC-024 §6 already specified, closing the one
documented caveat holding `matten-mlprep` at production-ready candidate
**Depends on:** RFC-022, RFC-024, RFC-028, RFC-030, RFC-032, RFC-058
**Related:** RFC-067, RFC-074, RFC-076

---

## 1. Summary

Add one public function to `matten-mlprep`:

```rust
pub fn train_test_split_seeded(
    x: &Tensor,
    train_ratio: f64,
    seed: u64,
) -> Result<(Tensor, Tensor), MattenMlprepError>;
```

RFC-024 §6 specified this signature, the RNG policy, and the testing requirement years of releases ago
and left it as "planned." This RFC implements it unchanged.

**This is deliberately pre-v1 work.** The `0.x` line carries no compatibility promise, so an additive API
can be shaped, used, and corrected before `1.0.0` freezes it. Adding it *after* 1.0 would lock the
signature on first release.

## 2. Motivation

Two things resolve at once:

1. **A real functional gap.** The only split today is ordered — `first floor(n*ratio)` rows to train. Any
   dataset with ordered classes or time-correlated rows produces a biased split, and the current
   documentation's advice ("shuffle the rows yourself first") pushes a correctness-sensitive operation onto
   the user with no reproducibility guarantee.
2. **The one caveat blocking a maturity decision.** RFC-076 §5's family maturity table records
   `matten-mlprep` at *production-ready candidate* with exactly one cited caveat:
   *"`train_test_split` is ordered-only, no shuffle/seed."* Closing it removes the stated obstacle — though
   the promotion itself remains a separate decision (§7).

## 3. Design

### 3.1 Signature and semantics

Exactly RFC-024 §6's signature. Semantics:

```text
1. validate: rank-2, train_ratio finite and in (0.0, 1.0), n_train != 0
2. build the row index vector [0, 1, ..., rows-1]
3. shuffle it with Fisher-Yates driven by a SplitMix64 stream seeded from `seed`
4. first n_train shuffled indices -> train rows; the remainder -> test rows
5. gather rows in shuffled order into the output tensors
```

`n_train = floor(rows * train_ratio)`, identical to the ordered split, so the *sizes* of the two outputs
match `train_test_split` exactly for the same inputs. Only row selection and order differ.

### 3.2 RNG — SplitMix64, hand-rolled, no dependency

RFC-024 §6 pre-decided this and the decision is adopted unchanged:

> "The RNG must be dependency-light and documented. A tiny deterministic PRNG such as SplitMix64 is
> acceptable if exact reproducibility is tested and documented. Pulling `rand` is not allowed without a new
> dependency review."

SplitMix64 is ~10 lines, has no dependencies, and is fully specified by its constants. **No new dependency
is introduced**, so no dependency review is triggered and `matten-mlprep`'s manifest keeps its single
`matten` dependency.

Reproducibility is a contract, not an implementation detail: the same `(x, train_ratio, seed)` must produce
byte-identical outputs on every platform and every future release of this crate. That means the PRNG
constants and the shuffle direction are part of the public contract once released, and §6 records that.

### 3.3 Errors — unchanged surface

No new `MattenMlprepError` variant. Reuses five of the enum's existing six variants, exactly as
`train_test_split` does: `ExpectedMatrix`, `InvalidRatio`, `EmptySplit`, `DynamicTensor` (under the
`dynamic` feature), and `Matten` (wrapping a rejected `Tensor::try_new`, the same path
`train_test_split` uses for its own output construction). `ZeroVariance` is scaling-only and
unused here. Shuffling itself introduces no new failure mode — it permutes indices that are known
valid; `Matten` is a defensive path expected to be unreachable given validated shapes, not a
consequence of shuffling.

## 4. Scope

### In scope

```text
train_test_split_seeded in crates/matten-mlprep/src/split.rs
a private SplitMix64 helper (module-private; not exported)
unit tests incl. exact-reproducibility and permutation-integrity coverage
one example, following the existing mlprep_* naming convention
doc-comment correction on train_test_split (it currently says the seeded
  variant is "planned but not in this release")
README ## Public API block entry
```

### Out of scope

```text
promoting matten-mlprep to production-ready (separate decision, §7)
any new dependency, including rand
stratified, grouped, or time-series splits
changing train_test_split's existing behaviour
a shuffle utility exposed on its own
any core matten change
version bump, release prep, tag, publish
```

## 5. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | **Additive** — one new function; nothing changed or removed |
| Runtime behavior | None for existing APIs; `train_test_split` is untouched |
| Feature flags | None |
| Dependencies | **None** — SplitMix64 is hand-rolled |
| MSRV | None (`1.85`) |
| Maturity labels | None — promotion is separate (§7) |
| SemVer | Additive; on `0.x` this is a minor bump when released |

## 6. Reproducibility contract

Once released, these become part of the crate's observable behaviour and may not change without a
documented breaking-change decision:

```text
the SplitMix64 constants and advance/mix sequence
the Fisher-Yates iteration direction and index-draw method
the mapping from `seed` to the first PRNG state
n_train = floor(rows * train_ratio), matching the ordered split
```

The implementation must therefore lock at least one exact expected permutation in a test, so an accidental
change to any of the above fails rather than silently reshuffling users' data.

## 7. Relationship to the `matten-mlprep` maturity question

RFC-058 deferred full-production-ready to "a separate future review," and RFC-076 §5 carries the candidate
label into any future 1.0 family. This RFC **removes the cited caveat but does not promote the crate** —
that remains its own decision, and should be taken on its own evidence rather than triggered automatically
by one API landing.

## 8. Acceptance criteria

```text
[ ] signature matches RFC-024 §6 exactly
[ ] no new dependency; matten-mlprep still depends only on matten
[ ] no new error variant; five existing ones are reused (`ExpectedMatrix`, `InvalidRatio`,
    `EmptySplit`, `DynamicTensor`, `Matten`)
[ ] identical (x, ratio, seed) produces identical output, asserted against a locked permutation
[ ] different seeds produce different permutations for a large-enough input
[ ] train/test row counts match train_test_split for the same (x, ratio)
[ ] train ∪ test is exactly the input row multiset — no row lost or duplicated
[ ] existing train_test_split behaviour is byte-identical (its own tests unchanged and passing)
[ ] the stale "planned but not in this release" doc comment is corrected
[ ] no core matten change; no version bump, release prep, tag, or publish
```

## 9. Non-goals

```text
[ ] promoting matten-mlprep
[ ] adding rand or any other dependency
[ ] stratified / grouped / time-series splitting
[ ] exposing a general shuffle helper
[ ] altering train_test_split
[ ] any 1.0 release activity — RFC-076 remains proposed and unexecuted
```

## 10. Follow-up

A release containing this API is a separate decision. If taken, it is a `0.x` minor bump under RFC-030
lock-step versioning, and — unlike the last eight releases — it changes a **published** crate, so RFC-075
§3.1's local-tool-only justification requirement does not apply.
