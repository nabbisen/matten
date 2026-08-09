# RFC-111: Zero-Sized Dimensions Accepted — Stage 3

**Status:** Proposed
**Target:** core `matten` + `matten-ndarray`; a shape-model change — minor release when it ships
**Theme:** Close the model. Stop rejecting what the library already produces.
**Related:** RFC-003 §7.4, RFC-022 §10, RFC-030, RFC-087 §6, RFC-105, RFC-106, RFC-108, RFC-110

---

## 1. Summary

Accept zero-sized dimensions. Three changes, all approved by the owner:

```text
1. relax checked_shape_len  -> constructors, reshape, composition, serde stop rejecting
2. matten-ndarray::from_arrayd accepts a zero-length axis
3. Display on an empty tensor shows structure instead of an empty string
```

**Sequenced after RFC-110**, which landed. That ordering was load-bearing: this RFC makes empty
tensors easy to construct, and RFC-105/108/110 removed the sentinels and panics they would otherwise
have made common.

## 2. What is being closed

RFC-003 §7.4 rejected zero-sized dimensions in `0.1.0` because *"empty tensors complicate `mean`,
`min`, `max`, JSON shape inference, and scalar broadcasting."* The reasoning was sound. **The
mechanism never delivered it** — rejecting at the constructor does not stop an operation producing
one, and `slice`, `transpose`, and `matmul` all do.

The result was a model that disagreed with itself:

```text
transpose([0,3]) -> [3,0]     Ok        |  reshape([0,3], &[3,0])  ->  rejected
slice().range(0..0) -> [0,3]  Ok        |  Tensor::try_new(.., &[0,3]) -> rejected
to_arrayd([0,3])  -> Ok                 |  from_arrayd(that)       ->  rejected
serialize([0,3])  -> Ok                 |  deserialize(that)       ->  rejected
```

**Two of those are round-trips the library cannot complete on its own output.** That is the case
against the current state; the complications RFC-003 predicted are now handled by RFC-105, RFC-108
and RFC-110 rather than avoided.

## 3. Change 1 — `checked_shape_len`

One function, `shape.rs:40-60`. Its zero check and its overflow check are independent; **only the
zero check goes.**

```text
DELETE   the `if dim == 0 { return Err(Shape{..}) }` early return
KEEP     the checked_mul overflow guard, unchanged
RESULT   a shape containing 0 yields len 0, which is arithmetically what it means
```

### 3.1 The cascade — verified

`checked_shape_len` has exactly **two** callers, and they fan out to roughly eighteen sites:

```text
shape::validate_shape (shape.rs:35)
    tensor.rs:107 try_new · reshape.rs:20 · dynamic/tensor_ext.rs:33
    parse/json.rs:115 · dynamic/parse/json.rs:75,86

MattenLimits::check_shape (limits.rs:132)
    limits.rs:169,199,230 try_zeros/try_ones/try_full
    creation.rs:46 linspace · creation.rs:93 eye
    linalg.rs:181 outer
    composition.rs:150,247,335,430,545,635
      concatenate · stack · repeat · repeat_axis · tile · meshgrid
```

**None of these needs its own change.** That is the finding that made this tractable, and it must be
verified rather than assumed — if any site needs special handling, that is a discrepancy to report.

### 3.2 What must not change

```text
- rank-0: shape [] still has len 1. The empty product is 1, not 0. A scalar is
  NOT empty, and is_empty() must stay false for it (RFC-108).
- the overflow guard and its message
- MAX_NDIM and every other limit
- arange's own "produces no elements" check — a different, deliberate message
- from_json_dynamic's "empty arrays are not supported" — likewise
```

## 4. Change 2 — `matten-ndarray::from_arrayd`

`convert.rs:60` rejects a zero-length axis with `ZeroSizedAxis`. Once core accepts, that rejection is
an arbitrary barrier — and its message already asserts a falsehood:

> *"which core `matten` does not support"*

matten does produce them. Accept the conversion.

**`MattenNdarrayError` is `#[non_exhaustive]`** (`error.rs:12`), so adding variants is not breaking —
but **removing** `ZeroSizedAxis` is, for anyone matching on it. Keep the variant, mark it deprecated,
and make it unreachable. Its doc comment must stop claiming matten lacks support.

This is a companion API change under lock-step versioning (RFC-030) and ships in the same release.

## 5. Change 3 — `Display` on an empty tensor

Today an empty tensor renders as `""` — indistinguishable from any other empty tensor, and giving no
sign the value is a tensor at all. The rank > 2 fallback on the same page already shows structure
(`display.rs:63`, `shape={shape:?} values={data:?}`).

**Match it.** An empty tensor renders its shape, not nothing.

**RFC-093 §6 is not crossed**: this shows a shape, not a value encoded as magnitude. `Debug` is
unchanged.

## 6. Change 4 — the caller audit

RFC-106 warned that callers relying on the accidental rejection could break. **Its one named instance
was tested and refuted** — `matten-mlprep`'s `standardize_columns`/`minmax_scale_columns` return an
*empty tensor*, not NaN-filled data, because `out = vec![0.0; rows * cols]` has no slots when either
dimension is zero.

The audit is still owed, narrower than it looked:

```text
- every call site that today relies on a rejection it did not itself write
- matten-mlprep's two functions: confirm the empty result, and note that `std` is
  NaN there so the `if std == 0.0` ZeroVariance guard does not fire. Harmless
  today because the output has no slots — but say so, do not leave it implied.
- matten-data's to_tensor: its doc already anticipates "a zero-length dimension
  when there are no rows". Confirm it now succeeds, and that this is intended.
```

## 7. Scope

### In scope

```text
crates/matten/src/shape.rs         the zero check (§3)
crates/matten/src/tensor/display.rs the empty rendering (§5)
crates/matten-ndarray             from_arrayd + the deprecated variant (§4)
tests per §9
compatibility.md, construction.md, shape-composition.md, boundary.md,
  reference/stats.md — every doc claiming zero-sized dims are rejected
```

### Out of scope — a diff touching these is a defect

```text
the overflow guard, MAX_NDIM, or any other limit
arange's and from_json_dynamic's own empty-input checks
any reduction — RFC-105/108/110 settled them
is_empty() — RFC-108 settled it
CHANGELOG.md — the release RFC writes it
```

## 8. Risks

```text
R1  RANK-0 REGRESSION. shape [] must keep len 1. A naive "product of dims" that
    treats the empty product as 0 breaks every scalar in the crate. Highest
    severity here — it would be caught by tests, but it is the one that breaks
    everything rather than one thing.
R2  THE OVERFLOW GUARD RIDING ALONG. It is in the same loop. Removing both is
    silent until a huge shape overflows.
R3  A CALLER THAT RELIED ON THE REJECTION. §6. The one named instance was
    refuted; others may exist and the audit is how they are found.
R4  DOC CLAIMS LEFT STANDING. Several pages assert zero-sized dims are rejected
    "in every form". Each becomes false the moment this ships.
R5  REMOVING ZeroSizedAxis rather than deprecating it (§4).
```

## 9. Acceptance criteria

```text
[ ] Tensor::try_new(vec![], &[0,3]) -> Ok, shape [0,3], len 0, is_empty() true
[ ] rank-0 unchanged: scalar has len 1, is_empty() false — asserted (R1)
[ ] the overflow guard still fires on a shape that overflows usize (R2)
[ ] reshape, concatenate, stack, repeat, repeat_axis, tile, meshgrid, outer,
    zeros/ones/full, linspace, eye all accept a zero-sized result
[ ] serde round-trips a [0,3] tensor — serialize then deserialize, asserted
[ ] from_arrayd accepts a zero-length axis; to_arrayd -> from_arrayd round-trips
[ ] ZeroSizedAxis retained, deprecated, unreachable; its doc no longer claims
    matten lacks support
[ ] Display on an empty tensor shows its shape; Debug unchanged
[ ] the §6 audit reported, with each site's finding
[ ] every doc claim that zero-sized dims are rejected is corrected (R4)
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 10. Non-goals

```text
a v1.0 decision — RFC-076 remains deferred and unauthorized
any new API; this removes restrictions, it does not add surface
matten-mlprep or matten-data behaviour changes — the audit REPORTS, it does not fix
```
