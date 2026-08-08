# Developer Handoff — RFC-104: Mutable Element Access

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/104-mutable-element-access.md`
**Base:** `main` @ `a4db83f`, clean tree, family at `0.44.0`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. If the disagreement is a pure addition in the handoff you
> can simply omit, omitting it and reporting prominently is acceptable; anything else, ask first.

---

## 1. Task title

Add `get_mut`, `get_flat_mut` (numeric) and `get_element_mut` (dynamic), mirroring the existing
`get`, `get_flat`, `get_element`.

## 2. Read this before the evidence table

**This RFC was wrong twice before it was right, and both corrections are recorded inside it.** §4 was
specified as `set`/`set_flat` returning `Result`; §6 claimed a coercion decision blocked dynamic
mutation. Neither survived. You are implementing the third version.

That matters to you in one concrete way: **§2's probe output is quoted from scratch code I wrote and
reverted, not from shipped behaviour.** Treat it as a claim to re-verify, not a result. It is the
strongest evidence in the RFC and it is still only a probe.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `get(&self, coord) -> Option<f64>` | `crates/matten/src/tensor/ops.rs:222` |
| E2 | `get_flat(&self, index) -> Option<f64>` | `tensor/ops.rs:240` |
| E3 | Both call `panic_if_dynamic` as their first statement | `ops.rs:223`, `ops.rs:241` |
| E4 | The panic message is fixed and shared | `tensor.rs:90-96` — `"matten unsupported error in {operation}: this numeric API is not supported on dynamic tensors; use to_elements() or try_numeric() first"` |
| E5 | `get_element(&self, coord) -> Option<Element>`, resolving via the **outer** shape | `dynamic/tensor_ext.rs:54-57` |
| E6 | `materialize(&mut self)` exists, is `pub(crate)`, no-ops when contiguous **and** unique | `dynamic/storage.rs:89-101` |
| E7 | `is_unique()` is `Arc::strong_count == 1` | `storage.rs:82-84` |
| E8 | `storage: Arc<Vec<Element>>` is `pub(crate)` | `storage.rs:48` |
| E9 | `Element` is `pub`, re-exported | `dynamic/element.rs:35`; `lib.rs:106` |
| E10 | `Tensor.data` is an owned `Vec<f64>`; no `Arc` on the numeric side | `tensor.rs` struct definition |

Re-derive these before writing code. **Report any discrepancy first, including one that shrinks or
removes the task** — that instruction has now paid out four times in this area.

## 4. Required implementation

### 4.1 Numeric

```text
get_mut(&mut self, coord: &[usize]) -> Option<&mut f64>
    panic_if_dynamic("get_mut")            <- FIRST, exactly as get does (E3)
    coord_to_flat(coord, &self.shape)?     <- same resolution as get (E1)
    self.data.get_mut(flat)

get_flat_mut(&mut self, index: usize) -> Option<&mut f64>
    panic_if_dynamic("get_flat_mut")
    self.data.get_mut(index)
```

E10 is why this needs no CoW: `&mut self` is exclusive by the borrow checker and `data` is owned.

### 4.2 Dynamic

```text
get_element_mut(&mut self, coord: &[usize]) -> Option<&mut Element>
    resolve flat through the OUTER shape, exactly as get_element does (E5)
    bounds-check against the LOGICAL length before materializing
    materialize()                          <- E6; no-op when already unique+contiguous
    Arc::get_mut(&mut storage)             <- sound ONLY because materialize guarantees
                                              uniqueness; do not unwrap it blindly
    index into the materialized vec
```

**Order matters.** Bounds-check *before* `materialize()`, or an out-of-range call silently pays for a
full copy and then returns `None`. That is a performance bug with no visible symptom.

**Do not** call `materialize()` on the numeric path. There is nothing to materialize (E10).

## 5. Required tests

```text
T1  get_mut / get_flat_mut return Some and the write lands
T2  read-modify-write in ONE expression: *t.get_mut(&[r,c])? += 1.0
T3  out-of-range -> None, and the tensor is UNCHANGED afterwards
T4  dynamic input PANICS, asserted against E4's message via #[should_panic(expected=...)]
    -- assert the MESSAGE, not merely that it panics
T5  non-square tensor (e.g. 2x3): get_mut(&[r,c]) and get_flat_mut agree on the same
    element. Square tensors hide coord/flat transposition.
T6  numeric slice: mutate the slice, assert the SOURCE is unchanged
T7  get_element_mut: write lands; Text/None/Bool can be written as well as Int/Float
T8  DYNAMIC SLICE ALIASING -- the one that matters. Slice a dynamic tensor, write
    through get_element_mut, then assert BOTH:
      (a) the source's elements are unchanged
      (b) Arc::ptr_eq(source.storage, slice.storage) is now FALSE
    (a) alone passes if materialize() copies; (b) alone passes if it never shared.
T9  get_element_mut on an already-unique contiguous tensor does NOT reallocate --
    assert the storage pointer is stable across two writes (E6's no-op path)
```

T8 and T9 are the pair that proves `materialize()` is being used correctly rather than
unconditionally. Neither is inferable from values.

## 6. Required documentation

```text
compatibility.md   `Mutable element API` row: Deferred -> Supported. Its current text
                   argues mutation needs a representation change — that claim is WRONG
                   and RFC-104 §2 disproves it. Rewrite the row; do not edit around it.
                   The `set_flat` row points at that row and needs the same treatment;
                   note that the shipped spelling is get_flat_mut, not set_flat.
reference docs     a mutation section beside where get/get_flat are documented
dynamic.md         get_element_mut, and the materialize-on-write behaviour
CHANGELOG.md       do NOT touch — the release RFC writes it
```

**Worth documenting because it is genuinely useful and non-obvious:** mutating a dynamic slice
materializes it, which *releases the parent's allocation*. That is RFC-102 §8.1's retention escape
hatch arriving as a side effect. Users hitting the retention caveat will want to know.

## 7. Acceptance criteria

```text
[ ] get_mut / get_flat_mut / get_element_mut per §4
[ ] T1-T9 all present and passing
[ ] the dynamic panic message asserted, not just the panic (T4)
[ ] bounds check precedes materialize() (§4.2) -- state how you verified the ORDER
[ ] no change to get / get_flat / get_element, to any operator, or to any result
[ ] no IndexMut, iter_mut, as_mut_slice, or set/set_flat (RFC §5, deferred with reasons)
[ ] compatibility.md's two rows rewritten, not edited around
[ ] both feature profiles build; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] clippy clean under RUSTFLAGS="-D warnings"; cargo fmt --check
[ ] no version bump, tag, or publish
```

## 8. Compatibility, security, risks

Purely additive: three new methods, no signature changed, no behaviour altered. No dependency,
feature, edition, MSRV, or maturity change. `#![forbid(unsafe_code)]` stands — `Arc::get_mut` is safe
and returns `Option`; do not reach for `unsafe` to avoid the check.

```text
R1  Arc::get_mut unwrapped without materialize() -> panics under aliasing, or worse,
    a write reaching a shared parent. T8 exists for this.
R2  materialize() called unconditionally -> correct but silently quadratic on repeated
    writes. T9 exists for this.
R3  None meaning both "dynamic" and "out of range" -> the reason get_mut panics rather
    than returning None. Do not "improve" this into a None.
R4  Square-tensor tests hiding a coord/flat transposition. T5 exists for this.
```

## 9. Required evidence

For T4, quote the asserted message. For §4.2's ordering, state how you proved the bounds check runs
first. For T8 and T9, give the assertion lines and what each would print if its risk were live.

## 10. Required review-request format

Write to:
`.git-exclude/review-request/RFC-104/matten-rfc104-mutable-element-access-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §9's evidence,
guard and test output, deviations with reasoning, and anything you want answered at review.
