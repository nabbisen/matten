# RFC-104: Numeric Mutation API

**Status:** Proposed
**Target:** core `matten`; new public API, so a minor release when it ships
**Theme:** In-place element mutation on numeric tensors — **numeric only**
**Related:** RFC-008, RFC-011, RFC-012, RFC-055, RFC-094, RFC-099, RFC-102

---

## 1. Summary

Add in-place element mutation to numeric `Tensor`. Dynamic tensors are **out of scope** and stay
unmutable, for a reason this RFC states rather than defers vaguely (§6).

## 2. The premise this was deferred on is wrong — and I proved it by building it

`compatibility.md` has said:

> **Numeric tensors have no shared storage to expose.** … for the numeric type it would mean
> **changing the representation first**, or designing mutation without CoW and reconciling the two
> halves.

**That reasoning inverts the problem.** Shared storage is what makes mutation *hard*; owning outright
is what makes it *easy*. `Tensor` owns its `Vec<f64>`, and `&mut self` is exclusive by the borrow
checker, so there is nothing to reconcile and nothing to copy.

Built as a scratch probe and measured, not argued:

```text
NUMERIC   before=[1.0, 2.0, 3.0, 4.0]   after=[1.0, 99.0, 3.0, 4.0]
          out-of-range -> Err("index 99 out of range")
```

Roughly eight lines, no representation change, no CoW.

**The aliasing case RFC-102 created the same day also holds**, using only the existing tested
`materialize()`:

```text
DYN   shared_before=true   shared_after=false
      slice  = [Int(0), Float(42.0), Int(2)]
      source = [Int(0), Int(1), Int(2), Int(3), Int(4), Int(5)]   SOURCE_INTACT=true
```

Mutating a slice detached its storage and left the source untouched — textbook copy-on-write,
working first try.

**This is the third consecutive §3.1 row whose difficulty I overstated**, after streaming and dynamic
slicing. All three were found the same way: by building the smallest real thing instead of reasoning
about it. Recorded as a method problem, not a third surprise.

## 3. What is *not* blocked

No global immutability stance exists to violate. The only nearby claim is `operators.md`'s
*"operands are never mutated"*, which is about **operators** and remains true — operators still
return new owned tensors and never take `&mut`.

## 4. API shape — and the one decision this RFC asks for

The library already splits numeric and dynamic accessors by name and type, so a numeric setter
constrains a future dynamic setter **not at all**:

```text
numeric                          dynamic
get(&[usize])   -> Option<f64>   get_element(&[usize]) -> Option<Element>
get_flat(usize) -> Option<f64>   to_elements()         -> Vec<Element>
```

`set`/`set_flat` mirror `get`/`get_flat`. A future `set_element` mirrors `get_element`. That is the
pattern the library already established, not a new precedent.

**The decision: what do the setters return?**

```text
A. Option<()>                mirrors the getters exactly, but a silently-ignored
                             failed WRITE is far worse than a failed read, and
                             `let _ = t.set(...)` is easy to write by accident.

B. Result<(), MattenError>   RECOMMENDED. Actionable error naming the index and
                             the bound. No panicking form at all.

C. panicking set + try_set   RFC-099's shape — but that pair exists to preserve
                             an ALREADY-PANICKING `dot`, whose messages had to
                             survive byte-identical. A new API has no such
                             constraint, and adding a panicking form creates a
                             panic path that never had to exist.
```

**Recommendation: B.** It is the reading of the owner's stated priority — *"clean and sophisticated
API design and careful error handling"* — and it keeps the surface at two methods rather than four.

`#[must_use]` is not needed on `Result`; the compiler already warns.

### 4.1 Dynamic input

`get`/`get_flat` call `panic_if_dynamic`. **The setters must not copy that**: they should return
`Err(MattenError::Unsupported)` naming `try_numeric()`, following RFC-099's `try_dot` rather than the
older panicking guards. Adding a new panic path immediately after shipping Result forms would move
the library backwards.

## 5. Scope

### In scope

```text
set(&mut self, coord: &[usize], value: f64)  -> Result<(), MattenError>
set_flat(&mut self, index: usize, value: f64) -> Result<(), MattenError>
tests, including the dynamic rejection and both feature profiles
docs: compatibility.md's `set_flat` and `Mutable element API` rows, and a
      mutation section wherever `get`/`get_flat` are documented
```

### Out of scope — a diff touching these is a defect

```text
dynamic mutation of any kind (§6)
IndexMut, get_mut, iter_mut, as_mut_slice — a separate and larger surface
any change to get/get_flat, or to any operator
CHANGELOG.md — the release RFC writes it
```

## 6. Why dynamic mutation is excluded — a real question, raised not deferred

The probe wrote `42.0` into an `Int` column and produced `[Int(0), Float(42.0), Int(2)]`, **silently
turning a homogeneous tensor heterogeneous**. That is not a bug in the probe; it is the undecided
question, and it is the first genuine semantic question found anywhere in this area:

```text
writing a Float into an Int column ->  allow?  reject?  coerce?
```

This is RFC-011 §11's coercion question in a new place, and nothing has ever forced it. **It is a
product decision and belongs to the owner, not to this RFC.** A dynamic setter cannot be specified
until it is answered.

**Trigger to reopen:** an owner decision on the rule above. That is a named condition, not "deferred
pending review."

## 7. Risks

```text
1. THE DYNAMIC PATH MUST NOT SILENTLY SUCCEED. If set() writes through to a
   dynamic tensor's numeric half, it corrupts a tensor whose real data lives in
   `dynamic`. Assert the Err, and assert the tensor is unchanged after it.
2. SLICES ARE NOW ALIASED (RFC-102). Numeric slices are owned copies, so numeric
   mutation cannot leak — but a test must PROVE it rather than assume it, because
   the same statement was true of dynamic slices until RFC-102 and is now false.
3. COORD VS FLAT. set() resolves through coord_to_flat and set_flat() does not.
   An off-by-one between them is invisible on a square tensor — test non-square.
```

## 8. Acceptance criteria

```text
[ ] set() and set_flat() mutate in place and return Ok(())
[ ] out-of-range returns Err naming the index and the bound; the tensor is UNCHANGED
[ ] a dynamic tensor returns Err(Unsupported) naming try_numeric(); it does NOT panic
    and is UNCHANGED afterwards — asserted, not assumed
[ ] mutating a numeric slice leaves its source unchanged — asserted (risk 2)
[ ] non-square tensor: set(&[r,c]) and set_flat() agree (risk 3)
[ ] no change to get/get_flat, to any operator, or to any existing result
[ ] both feature profiles build; no new panic path
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 9. Non-goals

```text
dynamic mutation, and the coercion rule it needs (§6)
IndexMut / get_mut / iter_mut / as_mut_slice
exposing CoW, or any change to the numeric representation
```
