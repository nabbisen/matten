# RFC-102: Slicing on Dynamic Tensors

**Status:** **Accepted** 2026-08-08 by the owner. Implemented and reviewed; approved with one required
correction (§8.1's retention documentation), pending. Handoff:
`rfcs/handoffs/102-slicing-on-dynamic-tensors-handoff.md`.
**Target:** core `matten`; behaviour change to an existing API, so a minor release when it ships
**Theme:** Wire slicing to dynamic tensors — the machinery already exists and is unreachable
**Related:** RFC-008, RFC-011, RFC-012, RFC-020, RFC-094

---

## 1. Summary

Let `slice()` and `slice_str()` work on dynamic tensors, returning a dynamic tensor.

**This is smaller than every record of it has claimed, including mine.** The view machinery exists, is
tested, is copy-on-write correct, and has no caller outside its own tests. The index arithmetic
`execute_slice` already performs is exactly what that machinery needs. The work is wiring.

## 2. The premise this RFC was expected to answer does not exist

ROADMAP §3.1 framed this as *"the larger question of whether dynamic tensors should be sliceable at
all, and what a slice of heterogeneous `Element` data even means"* — a boundary RFC before any code.
I wrote that. It is wrong, and this is the third framing error on that one row.

**There is no semantic question.** Slicing selects *positions*; it does not interpret values. A slice
of a dynamic tensor is the dynamic tensor at the selected positions. Heterogeneity is irrelevant —
`Element`s ride along untouched, exactly as `f64`s do today. Nothing about `Text` or `None` makes
"which positions" ambiguous.

The genuine open questions elsewhere in the dynamic feature — coercion (RFC-011 §11), what
`try_numeric` accepts — are about *interpreting* values. Selecting them raises none of it.

## 3. What already exists

`DynamicTensor` carries the view kind a slice needs:

```rust
pub(crate) enum ViewKind {
    Contiguous { offset: usize },   // logical i -> storage[offset + i]
    Indexed(Vec<usize>),            // logical i -> storage[indices[i]]
}
```

And the constructor, already written and already documented as being for this:

```rust
/// Creates a slice sharing storage with this tensor. The slice covers
/// `indices` (logical flat indices into *this* tensor's logical layout).
pub(crate) fn slice_indices(&self, indices: Vec<usize>, new_shape: Vec<usize>) -> DynamicTensor
```

It `Arc::clone`s the storage rather than copying elements, and it composes correctly — slicing an
already-`Indexed` view maps through the existing indices rather than nesting views.

**Its only callers are three tests** in `dynamic/tests/lifecycle.rs`. It is reachable from nothing in
the public API. This was built during the RFC-012 copy-on-write work and never wired up.

## 4. The work

`execute_slice` rejects dynamic at the top, then does the real work:

```text
resolve each axis spec  ->  per-axis index lists        (shape-level, type-agnostic)
for each output position:
    compute src_flat    ->  a LOGICAL flat index         <- exactly what slice_indices wants
    out_data[dst_flat] = tensor.data[src_flat]           <- the only f64-specific line
```

So the dynamic path is: drop the rejection, collect `src_flat` in output order into a `Vec<usize>`,
and call `slice_indices(indices, out_shape)` instead of copying `f64`s. Every spec-resolution and
coordinate computation above that line is shared unchanged.

**The rank-0 collapse case works**: `out_shape` is empty for a fully-indexed slice, and
`slice_indices` takes `new_shape: Vec<usize>`, for which empty is valid — `reshape` beside it already
handles the empty case explicitly.

## 5. Semantics

```text
a slice of a dynamic tensor IS a dynamic tensor — is_dynamic() stays true
storage is SHARED, not copied — Arc::clone, per RFC-012's CoW model
every existing numeric result is unchanged, byte for byte
the slice grammar is unchanged — same specs, same rank rules, same errors
```

**Negative indices**: `slice_str` accepts them (RFC-088) and the builder does not. That asymmetry is
unchanged here; whatever the grammar accepts for numeric it accepts for dynamic, because spec
resolution is shared.

## 6. What this replaces

The current rejection tells the user:

> use `get_element(&[row, col])` for element access, or call `try_numeric()` first

Both are real, and both are worse than a slice. `get_element` is one call per element, so extracting
a column is a loop the caller writes. `try_numeric()` fails outright on the `Text` or `None` data
that is the reason to be using a dynamic tensor.

That is the actual user cost: **the workaround for "I have messy data" is "make it not messy first."**

## 7. Scope

### In scope

```text
execute_slice: dynamic path via slice_indices, replacing the rejection
tests: shared storage preserved, Indexed-of-Indexed composition, rank-0 collapse,
       Text/None/Bool elements surviving a slice unchanged
docs: compatibility.md's row, reference/slicing.md, reference/dynamic.md
```

### Out of scope — a diff touching these is a defect

```text
the slice grammar, or any numeric result
negative indices in the builder — a separate asymmetry (RFC-088)
mutation, or any public exposure of CoW (compatibility.md: numeric has no shared storage)
try_numeric, coercion, or anything that INTERPRETS an Element
```

## 8. Risks

```text
1. VIEW COMPOSITION. Slicing a slice must map through the existing Indexed view rather
   than nesting. slice_indices already does this; the test must prove it, because a
   nesting bug produces correct-looking output for the first slice only.
2. SHARED STORAGE IS INVISIBLE WHEN WRONG. A slice that copied instead of sharing would
   pass every value assertion. Assert Arc::strong_count, as lifecycle.rs already does.
3. THE REJECTION MESSAGE IS A PUBLIC STRING. Removing it changes observable behaviour
   for anyone matching on it — an intended consequence. It lands as Changed rather
   than Added; the release RFC writes the CHANGELOG entry, not this one.
```

### 8.1 The risk this RFC missed — retention (added 2026-08-08, at review)

**Shared storage has a cost this RFC never named, and the omission was mine.** Probed at review:

```text
100_000-element dynamic tensor -> 1-element slice -> drop the parent
storage still holds 100_000 elements
```

A slice retains its source's **entire** allocation for as long as the slice lives, even after the
source is dropped. That is inseparable from the cheap-chaining property §5 advertises — you cannot
have one without the other — and it is the standard hazard of any shared-view design.

It is sharper here because **the release valve is not public**: `DynamicTensor::materialize()` is
`pub(crate)`. A user's only route to a detached copy is
`Tensor::from_elements(slice.to_elements(), slice.shape())`.

**This does not change the design**, which is correct and intended. What it changes is the
documentation obligation: wherever this RFC's sharing is advertised, the retention consequence and
the released form must ship in the same breath. `slicing.md` and `compatibility.md` both carry it.

Recorded here rather than only in the review because the gap was the specification's. The RFC
reasoned about what sharing *buys* and never about what it *holds* — and it reached a reader only
because the implementation's docs made the promise specific enough to probe.

## 9. Acceptance criteria

```text
[ ] slice() and slice_str() work on dynamic tensors, returning is_dynamic() == true
[ ] storage is SHARED — asserted via Arc::strong_count, not inferred from values
[ ] slicing a slice composes correctly (Indexed of Indexed), asserted
[ ] rank-0 collapse works
[ ] Text, None and Bool elements survive a slice unchanged
[ ] every numeric slice result is byte-identical — asserted, not assumed
[ ] compatibility.md's "Slicing on dynamic tensors | Deferred" row is corrected
[ ] §8.1's retention consequence is documented wherever sharing is advertised,
    with the released form (from_elements(to_elements(), shape())) named
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish; the version bump is a separate decision under RFC-094
```

## 10. Non-goals

```text
a public mutation API, or exposing CoW (compatibility.md's row)
changing what try_numeric accepts
negative indices in the builder
```
