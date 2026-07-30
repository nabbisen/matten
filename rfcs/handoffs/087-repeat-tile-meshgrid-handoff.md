# RFC-087 `repeat` / `tile` / `meshgrid`: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/087-repeat-tile-meshgrid.md`
**Document kind:** Detailed implementation handoff
**Status:** Proposed; inherits RFC-087's state. Not authorized for implementation until accepted
**Date:** 2026-07-30

---

## 1. Purpose

Add eight functions to core `matten` — `repeat`, `repeat_axis`, `tile`, `meshgrid` and their `try_`
forms — closing RFC-039 §8's three deferred APIs.

**This is a real core public-surface change**, the first in a while. `docs/src/reference/public-api-snapshot.md`
must be updated, unlike the last several slices where it correctly stayed still.

No release, no version bump. Version stays `0.40.0`.

## 2. Follow `composition.rs`'s existing pattern exactly

Everything goes in `crates/matten/src/composition.rs`, beside `concatenate`/`stack`. Copy their shape,
do not invent one:

```rust
/// # Errors
/// - [`MattenError::Shape`] if ...
/// - [`MattenError::Allocation`] if the result exceeds the allocation limit.
pub fn try_tile(&self, reps: &[usize]) -> Result<Tensor, MattenError> { ... }

/// # Panics
/// Panics if [`Self::try_tile`] would return an error.
pub fn tile(&self, reps: &[usize]) -> Tensor {
    self.try_tile(reps).unwrap_or_else(|e| panic!("{e}"))
}
```

Three helpers already exist in that file and must be reused rather than reimplemented:

```text
reject_dynamic(...)                        RFC-031 dynamic rejection
MattenLimits::default().check_shape(&out_shape, "<op>")?    the allocation guard
MattenError::Shape { operation, message }  the error shape, with a message naming
                                           the actual numbers involved
```

`check_shape` does the checked product for you and returns `MattenError::Allocation`. **Do not compute
the output length with a bare `*`** — `concatenate` uses `checked_*` plus `check_shape` for a reason,
and `repeat`/`tile`/`meshgrid` overflow far more easily than it does.

## 3. Semantics — the exact behaviours to implement

```text
repeat(n)              flatten, repeat each ELEMENT n times, return rank-1
                       [1,2,3].repeat(2) -> [1,1,2,2,3,3]        NOT [1,2,3,1,2,3]

repeat_axis(n, axis)   repeat each element along `axis`, rank preserved
                       [[1,2],[3,4]].repeat_axis(2, 0) -> [[1,2],[1,2],[3,4],[3,4]]

tile(&reps)            repeat the WHOLE tensor
                       [1,2,3].tile(&[2]) -> [1,2,3,1,2,3]       NOT [1,1,2,2,3,3]

meshgrid(x, y)         x rank-1 len m, y rank-1 len n  ->  both outputs shape [n, m]
                       numpy 'xy' indexing: out_x[i][j] = x[j],  out_y[i][j] = y[i]
```

`repeat` vs `tile` is the classic confusion. Get them the right way round, and make the doc comments
show both on the same input so a reader sees the contrast.

### 3.1 Error cases — all of these must error, none may silently succeed

```text
repeat / repeat_axis / tile with n == 0, or any rep == 0   -> Shape
tile with empty reps                                        -> Shape
tile with reps.len() > rank                                 -> Shape, naming BOTH lengths
repeat_axis on a rank-0 tensor                              -> Axis
repeat_axis with axis >= rank                               -> Axis
meshgrid with a non-rank-1 input                            -> Shape (do NOT flatten)
any dynamic tensor input                                    -> via reject_dynamic
output exceeding limits                                     -> Allocation, via check_shape
```

`tile` with `reps.len() < rank` is **not** an error — prepend `1`s (RFC-087 §4).

## 4. Required tests

```text
[ ] repeat: [1,2,3] n=2 -> [1,1,2,2,3,3]   (element repetition, exact)
[ ] tile:   [1,2,3] reps=[2] -> [1,2,3,1,2,3]   (whole-tensor, exact)
    -- these two together are what prevent the classic swap
[ ] repeat on a rank-2 input flattens to rank-1; repeat_axis preserves rank
[ ] repeat_axis on axis 0 and axis 1 of the same matrix, both checked exactly
[ ] tile with reps SHORTER than rank: prepends 1s, correct shape and values
[ ] tile with reps LONGER than rank: errors, message names both lengths
[ ] MESHGRID WITH UNEQUAL INPUT LENGTHS -- see §5, this one is mandatory
[ ] meshgrid output values: out_x[i][j] == x[j] and out_y[i][j] == y[i], checked
    element-by-element, not just by shape
[ ] every §3.1 error case, asserting the variant
[ ] allocation: an input/reps combination whose product exceeds max_elements
    returns MattenError::Allocation rather than attempting the allocation
[ ] dynamic rejection for all three (feature-gated like the existing dynamic tests)
```

## 5. The one test that must not be got wrong

**`meshgrid` must be tested with inputs of *different* lengths.**

With equal lengths, `xy` and `ij` indexing differ only by a transpose, and both produce the same
*shape* — so a transposed implementation passes a shape assertion and passes any value assertion that
is itself symmetric. The bug ships silently, which is precisely the failure mode RFC-087 §5 chose `xy`
to avoid.

```text
x = [1, 2, 3]        (len 3)
y = [10, 20]         (len 2)
both outputs MUST be shape [2, 3]        -- not [3, 2]
out_x == [[1,2,3],[1,2,3]]
out_y == [[10,10,10],[20,20,20]]
```

If your implementation produces `[3, 2]`, you have implemented `ij`. Fix the implementation, not the
test.

## 6. Documentation

```text
crates/matten/src/composition.rs        doc comments; the repeat-vs-tile contrast shown
                                        on the same input; meshgrid's 'xy' convention
                                        stated, with a note that 'ij' is a transpose away
crates/matten/README.md                 public API list
docs/src/reference/shape-composition.md the three APIs, with the broadcasting contrast
docs/src/reference/public-api-snapshot.md  MANDATORY -- core surface changed
crates/matten/examples/                 one example, numbered per the existing convention;
                                        add its [[example]] entry and a smoke-run line in
                                        .github/workflows/test.yaml alongside the others
```

The example is not an afterthought here — RFC-087 §2 justifies this whole slice on teaching value, so
an example that demonstrates `repeat` vs `tile` and evaluates a function over a `meshgrid` grid is
part of the deliverable, not decoration.

## 7. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
cargo run -p matten --example <the new example>
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-matten-data-scope.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

Scope confirmation:

```bash
git diff --name-only -- crates/matten-data crates/matten-mlprep crates/matten-ndarray crates/matten-stats
# expect EMPTY -- core only
grep -m1 '^version' Cargo.toml     # still 0.40.0
```

## 8. Known pitfalls

1. **Swapping `repeat` and `tile`.** The two exact-value tests in §4 exist for this.
2. **Implementing `ij` and testing with equal lengths** (§5).
3. **Computing output size with a bare `*`** instead of `check_shape` (§2).
4. **Flattening a rank-2 `meshgrid` input** instead of erroring.
5. **Returning an empty tensor for `n = 0`.** The shape model rejects zero-sized dimensions; error.
6. **Treating `reps.len() < rank` as an error.** Only *longer* than rank is rejected.
7. **Forgetting `public-api-snapshot.md`** — this slice genuinely changes the core surface.
8. **Adding an `indexing` parameter or an N-D meshgrid.** Both explicitly out of scope.

## 9. What the review request must report

```text
[ ] the repeat-vs-tile exact-value tests, side by side
[ ] the unequal-length meshgrid test with its asserted shape and values (§5)
[ ] the allocation test, showing MattenError::Allocation rather than a large allocation
[ ] every §3.1 error case with its asserted variant
[ ] the public-api-snapshot.md diff
[ ] the example, its Cargo.toml entry, and its CI smoke-run line
[ ] confirmation no companion crate changed; version still 0.40.0
[ ] full gate set incl. MSRV and mdbook
```

## 10. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, or publish.
