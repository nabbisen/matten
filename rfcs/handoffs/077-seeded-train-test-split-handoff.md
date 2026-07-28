# RFC-077 Seeded Train/Test Split: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-077 (design authority)
**Document kind:** Detailed implementation handoff
**Status:** Drafted for review; implementation unauthorized until RFC-077 and this handoff are accepted
**Date:** 2026-07-28

---

## 1. Purpose

Implement `train_test_split_seeded` in `matten-mlprep` as one reviewable slice. RFC-077 is the design
authority; where they differ, RFC-077 wins.

**No release.** This ends at a reviewed implementation commit on the `0.x` line.

## 2. Preconditions

```text
RFC-077 and this handoff accepted
working tree clean
0.38.0 remains the current version — do NOT bump
RFC-076 (v1.0 prep) stays proposed and unexecuted; v1 is deferred
```

## 3. Files

```text
crates/matten-mlprep/src/split.rs        the new fn + private SplitMix64 + tests
crates/matten-mlprep/README.md           ## Public API block entry
crates/matten-mlprep/Cargo.toml          ONE new [[example]] target only — no dependency
crates/matten-mlprep/examples/train_test_split_seeded.rs   new example
```

**`Cargo.toml` must gain no `[dependencies]` line.** If you find yourself adding one, stop — RFC-024 §6
forbids it without a separate dependency review.

## 4. Implementation

### 4.1 SplitMix64 (module-private)

```rust
/// SplitMix64 — a tiny, dependency-free deterministic PRNG (RFC-024 §6).
///
/// The constants and advance order are part of the reproducibility contract
/// (RFC-077 §6): changing them changes every user's split for a given seed.
struct SplitMix64(u64);

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `[0, bound)`. `bound` must be non-zero.
    fn next_below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}
```

`next_below` uses modulo, which carries a negligible bias for `bound` far below `u64::MAX`. That is
acceptable here and **must be stated in the doc comment** rather than left implicit — row counts are tiny
relative to `u64`, and rejection sampling would complicate the reproducibility contract for no practical
gain. Do not silently "improve" this later; it is contract-bearing.

### 4.2 The function

Mirror `train_test_split`'s structure exactly — same validation order, same helpers, same error variants:

```rust
pub fn train_test_split_seeded(
    x: &Tensor,
    train_ratio: f64,
    seed: u64,
) -> Result<(Tensor, Tensor), MattenMlprepError> {
    let (rows, cols) = matrix_dims(x)?;

    if !train_ratio.is_finite() || train_ratio <= 0.0 || train_ratio >= 1.0 {
        return Err(MattenMlprepError::InvalidRatio(train_ratio));
    }

    let n_train = (rows as f64 * train_ratio).floor() as usize;
    if n_train == 0 {
        return Err(MattenMlprepError::EmptySplit { rows, train_ratio });
    }

    // Fisher-Yates over row indices, descending. Direction is contract-bearing.
    let mut order: Vec<usize> = (0..rows).collect();
    let mut rng = SplitMix64::new(seed);
    for i in (1..rows).rev() {
        let j = rng.next_below(i + 1);
        order.swap(i, j);
    }

    // gather rows in shuffled order
    let data = x.as_slice();
    let mut gather = |idx: &[usize]| -> Vec<f64> {
        let mut out = Vec::with_capacity(idx.len() * cols);
        for &r in idx {
            out.extend_from_slice(&data[r * cols..(r + 1) * cols]);
        }
        out
    };

    let train = Tensor::try_new(gather(&order[..n_train]), &[n_train, cols])
        .map_err(MattenMlprepError::Matten)?;
    let test = Tensor::try_new(gather(&order[n_train..]), &[rows - n_train, cols])
        .map_err(MattenMlprepError::Matten)?;
    Ok((train, test))
}
```

### 4.3 Doc comment

Write a doc comment in the file's existing voice, and include:

- the `n_train` formula, stated as identical to the ordered split;
- an explicit reproducibility guarantee — same `(x, ratio, seed)` → same output, always;
- a pointer to `train_test_split` for the ordered case;
- the `# Errors` list (the same four variants);
- a runnable doctest asserting exact output for a fixed seed.

### 4.4 Correct the stale comment

`crates/matten-mlprep/src/split.rs` currently tells users the seeded variant does not exist:

> "If you need a randomized split, shuffle the rows yourself first (a seeded variant is planned but not in
> this release; see RFC-024 §6)."

Replace with a pointer to `train_test_split_seeded`. Leaving it is a documentation defect the moment this
lands.

## 5. Tests — required

Place beside the implementation, matching the crate's existing test placement.

```text
[ ] reproducibility: same (x, ratio, seed) twice -> byte-identical train and test
[ ] LOCKED PERMUTATION: for a fixed small input and fixed seed, assert the exact
    expected row order. This is the test that makes RFC-077 §6's contract real —
    it fails if the constants, direction, or seeding change
[ ] seed sensitivity: two different seeds produce different orders for a
    large-enough input (use >= 8 rows so a coincidental match is implausible)
[ ] size parity: for the same (x, ratio), the train/test row counts equal
    train_test_split's
[ ] permutation integrity: train ∪ test, sorted, equals the input row multiset —
    no row lost, duplicated, or corrupted across the row boundary
[ ] shuffling is a permutation of ROWS, not of values within a row: verify each
    output row matches some complete input row (catches an off-by-one in gather)
[ ] error parity: rank-1/rank-3 -> ExpectedMatrix; ratio 0.0/1.0/NaN/inf ->
    InvalidRatio; ratio too small for the row count -> EmptySplit
[ ] dynamic tensor (feature = "dynamic") -> DynamicTensor
[ ] Matten path: the map_err(MattenMlprepError::Matten) conversion on both
    Tensor::try_new calls exists and type-checks. This path is defensive and
    expected to be unreachable given validated shapes (n_train and rows -
    n_train are both derived from a validated, non-empty row count) -- do not
    assume it away silently; a reachability note in the doc comment is
    sufficient if no test can force it
[ ] existing train_test_split tests unchanged and passing
```

The row-integrity and row-not-value tests matter most: an incorrect `gather` stride corrupts data silently
while still producing correctly-shaped tensors.

## 6. Example

`crates/matten-mlprep/examples/train_test_split_seeded.rs`, registered in `Cargo.toml` as
`name = "mlprep_train_test_split_seeded"` — the namespaced prefix is required to avoid a target-name
collision with core `matten` examples when the workspace builds.

Keep it in the style of `examples/train_test_split.rs`: small, printed output, no dependencies. Show that
re-running with the same seed reproduces the split.

## 7. Documentation

```text
crates/matten-mlprep/README.md   add train_test_split_seeded to the ## Public API block
```

Do **not** touch `CHANGELOG.md`, version metadata, `compatibility.md`, or `public-api-snapshot.md` — the
snapshot documents core `matten`, and release notes belong to a release slice, not this one.

## 8. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo test -p matten-mlprep --all-features
cargo test -p matten-mlprep --no-default-features
cargo run -p matten-mlprep --example mlprep_train_test_split_seeded
cargo +1.85.0 build && cargo +1.85.0 test --all-features
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
git diff --check
```

Plus, to prove nothing outside the intended scope moved:

```bash
git diff --name-only                          # expect only the 4 files in §3
git diff -- crates/matten-mlprep/Cargo.toml   # expect ONLY a new [[example]] block
git diff -- crates/matten/                    # expect empty
bash tools/matten-report/tests/process-boundary.sh   # anchors untouched
```

## 9. What the review request must report

```text
[ ] the locked-permutation test, with the exact asserted order shown
[ ] confirmation of no new dependency (the Cargo.toml diff)
[ ] confirmation of no new error variant
[ ] size-parity and permutation-integrity results
[ ] the corrected train_test_split doc comment
[ ] full gate set results, incl. MSRV
[ ] git diff --name-only output
[ ] confirmation that no version bump, CHANGELOG, or release metadata changed
```

## 10. Known pitfalls

1. **Gather stride bugs** — `data[r*cols..(r+1)*cols]` is the whole row; getting it wrong yields
   right-shaped, wrong-valued tensors. The row-integrity test is the guard.
2. **Shuffling values instead of rows** — permute the *index* vector, never the data slice.
3. **Adding `rand`** — forbidden by RFC-024 §6 without a separate review.
4. **"Improving" the PRNG or shuffle direction later** — both are contract-bearing (RFC-077 §6).
5. **Bumping the version** — this slice ships no release.
6. **Forgetting the example target-name prefix** — causes a workspace-wide build collision.
7. **Promoting `matten-mlprep`** — out of scope; the maturity decision is separate (RFC-077 §7).

## 11. Review stop

Acceptance makes this a commit point. It does not authorize a release, a version bump, or the
`matten-mlprep` maturity promotion.
