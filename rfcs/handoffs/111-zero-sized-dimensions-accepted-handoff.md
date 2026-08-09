# Developer Handoff — RFC-111: Zero-Sized Dimensions Accepted (Stage 3)

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/111-zero-sized-dimensions-accepted.md`
**Base:** `main` @ `12166dd`, clean tree, family at `0.45.0`.
**Sequencing:** RFC-105, RFC-108 and RFC-110 have all landed. That was the precondition; nothing else
blocks this.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Accept zero-sized dimensions: relax `checked_shape_len`, accept them in the ndarray bridge, render an
empty tensor's shape in `Display`, and report the caller audit.

## 2. This is a shape-model change, not a fix

Every previous task in this sequence removed a defect. **This one changes what the library permits.**
Two consequences:

```text
- The diff is SMALL and the blast radius is LARGE. One deleted early return changes
  ~18 call sites' behaviour. Do not judge the risk by the line count.
- Nothing here is urgent. If something looks wrong, stop and report. Six premises in
  this sequence have failed testing; a seventh would be information.
```

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `checked_shape_len` is at `shape.rs:40-60`; its zero check and overflow check are **independent** statements in one loop | `sed -n '44,60p' crates/matten/src/shape.rs` |
| E2 | It has exactly **two** callers | `grep -rn "checked_shape_len" crates/matten/src` → `shape.rs:35` (`validate_shape`), `limits.rs:132` (`check_shape`) |
| E3 | `validate_shape` fans out to 6 sites | `tensor.rs:107`, `reshape.rs:20`, `dynamic/tensor_ext.rs:33`, `parse/json.rs:115`, `dynamic/parse/json.rs:75,86` |
| E4 | `check_shape` fans out to 12 sites | `limits.rs:169,199,230`; `creation.rs:46,93`; `linalg.rs:181`; `composition.rs:150,247,335,430,545,635` |
| E5 | `from_arrayd` rejects at `convert.rs:60`; the variant is `error.rs:20` | direct read |
| E6 | `MattenNdarrayError` is `#[non_exhaustive]` (`error.rs:12`) | so **adding** variants is non-breaking; **removing** one is not |
| E7 | The rank > 2 `Display` fallback already shows structure | `tensor/display.rs:63` — `shape={shape:?} values={data:?}` |
| E8 | Doc claims to correct | `construction.md:85`, `shape-composition.md:123`, `stats.md:48`, and `migration/bridge-contracts.md:48`'s variant table |
| E9 | `matten-mlprep` returns an **empty tensor**, not NaN-filled, under a relaxed guard | probed at RFC-106's review by relaxing the guard and running it |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 4. Change 1 — `checked_shape_len`

```text
DELETE  the `if dim == 0 { return Err(Shape{..}) }` early return, and nothing else
KEEP    the checked_mul overflow guard and its exact message
```

**Do not rewrite the loop as `shape.iter().product()`.** It is the obvious simplification and it is
wrong twice: it drops the overflow guard (R2), and it changes nothing about rank-0 only by accident.
Delete the branch; leave the loop.

### 4.1 Verify the cascade rather than assume it

E3/E4's eighteen sites should need **no** change. Check each — if one needs special handling, that is
a discrepancy worth more than the retarget. Report it before working around it.

### 4.2 Rank-0 is the highest-severity risk

```text
shape []  ->  len 1   (the empty product is 1, not 0)
a scalar is NOT empty; is_empty() stays false for it (RFC-108)
```

Deleting a branch preserves this automatically. A rewrite may not. **Assert it explicitly**, because
breaking it breaks every scalar in the crate rather than one operation.

## 5. Change 2 — the ndarray bridge

```text
convert.rs:60   remove the rejection; accept a zero-length axis
error.rs:20     KEEP ZeroSizedAxis. Mark it #[deprecated] with a note. Do NOT remove
                it — the enum is non_exhaustive, but removing a variant still breaks
                anyone matching on it (E6).
error.rs        its doc comment says the shape is one "which core matten does not
                support". That is now false. Rewrite it.
```

Confirm `to_arrayd` → `from_arrayd` round-trips a `[0,3]` tensor. That round-trip is currently
one-way and is half the argument for this RFC.

## 6. Change 3 — `Display` on an empty tensor

An empty tensor currently renders as `""`. Make it show its shape, matching E7's existing fallback.

```text
Debug is UNCHANGED. Only Display.
RFC-093 §6 is not crossed — a shape is not a value encoded as magnitude.
```

Check whether any existing test or doc asserts the empty-string rendering. If one does, it is
asserting the behaviour this change replaces — report it, do not silently edit it.

## 7. Change 4 — the caller audit, which REPORTS and does not fix

```text
- every call site relying on a rejection it did not itself write
- matten-mlprep's standardize_columns / minmax_scale_columns: confirm the empty
  result (E9), and note that `std` is NaN there so the `if std == 0.0` ZeroVariance
  guard does not fire. Harmless because the output has no slots — say so explicitly
  rather than leaving it implied.
- matten-data's to_tensor: its doc already anticipates "a zero-length dimension when
  there are no rows". Confirm it now succeeds and that this is intended.
```

**Do not fix anything you find.** Report it; a follow-up RFC decides.

## 8. Required documentation

```text
construction.md:85          "Zero-sized dimensions are rejected (deferred to a
                            future RFC)" -- this RFC is that future RFC
shape-composition.md:123    "the shape model has no representation for a zero-sized
                            dimension" -- now false
stats.md:48                 "forbids zero-sized dimensions, so an empty tensor is
                            not constructible" -- now false
bridge-contracts.md:48      the MattenNdarrayError variant table
compatibility.md            any row resting on the old rejection
```

Each of these justifies something by an invariant that this RFC removes. **Rewrite them; do not edit
around them.**

```text
DO NOT TOUCH: CHANGELOG.md — the release RFC writes it.
DO NOT TOUCH: arange's "produces no elements" or from_json_dynamic's "empty arrays
              are not supported" — separate, deliberate checks about absent INPUT,
              not about shape.
```

## 9. Required tests

```text
T1  try_new(vec![], &[0,3]) -> Ok; shape [0,3]; len 0; is_empty() true
T2  RANK-0 UNCHANGED: scalar len 1, is_empty() false (§4.2)
T3  the overflow guard still fires on a shape overflowing usize (R2)
T4  reshape / concatenate / stack / repeat / repeat_axis / tile / meshgrid /
    outer / zeros / ones / full / linspace / eye each accept a zero-sized result
T5  serde: serialize a [0,3] tensor, deserialize it back, assert the round trip
T6  from_arrayd accepts a zero-length axis; to_arrayd -> from_arrayd round-trips
T7  Display on [0,3] shows the shape; Debug unchanged — assert both strings
T8  every pre-existing test passes UNMODIFIED except any that asserts the old
    rejection. For each such test: report it, then invert it deliberately —
    editing one quietly is the signal something went wrong.
```

T8 is where this task differs from the others. **Some existing tests assert the rejection this RFC
removes.** Inverting them is correct; doing so without listing them is not.

## 10. Acceptance criteria

```text
[ ] T1-T8 present and passing
[ ] only the zero-check branch deleted; the overflow guard byte-identical
[ ] the eighteen cascade sites verified as needing no change, or discrepancies reported
[ ] ZeroSizedAxis retained + deprecated + doc corrected; not removed
[ ] Display shows shape on empty; Debug unchanged
[ ] §7's audit reported, nothing fixed
[ ] every doc claim in §8 rewritten
[ ] every inverted pre-existing test listed explicitly
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] no version bump, tag, or publish
```

## 11. Risks

```text
R1  Rank-0 regression via a loop rewrite (§4.2). Breaks every scalar.
R2  The overflow guard removed alongside the zero check (§4). Silent until overflow.
R3  A caller relying on the rejection (§7). The one named instance was refuted;
    others may exist.
R4  A doc claim left standing (§8). Each becomes false the moment this ships.
R5  Removing ZeroSizedAxis instead of deprecating it (§5).
R6  Quietly editing an existing test that asserts the old rejection (T8).
```

## 12. Required review-request format

Write to:
`.git-exclude/review-request/RFC-111/matten-rfc111-zero-sized-dimensions-accepted-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, the §4.1 cascade
check, §7's audit findings, the T8 list of inverted tests, guard/clippy/test output, deviations with
reasoning, and anything you want answered at review.
