# RFC-104: Mutable Element Access

**Status:** **Accepted** 2026-08-08 by the owner, with the §6.1 scope decision resolved to **WIDEN**:
numeric and dynamic mutable access ship together. Not yet implemented.
**Target:** core `matten`; new public API, so a minor release when it ships
**Theme:** Mutable element access via `get_mut` — numeric **and** dynamic
**Related:** RFC-008, RFC-011, RFC-012, RFC-055, RFC-094, RFC-099, RFC-102

---

## 1. Summary

Expose mutable element access on `Tensor` as `get_mut` / `get_flat_mut`, mirroring the existing
`get` / `get_flat`. **Dynamic tensors are included** — the owner resolved §6.1 to widen. An earlier version excluded
them on a blocker that does not exist, and §6 records that correction rather than hiding it.

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

## 4. API shape — CORRECTED to `get_mut`

**An earlier version of this RFC specified `set(coord, value)` / `set_flat(index, value)` returning
`Result<(), MattenError>`, and the owner approved that return type. Both the method and the return
type were wrong, and the owner found it by asking whether the design was clean.** The correction is
recorded here rather than silently applied, because the approved decision no longer exists to honour.

### 4.1 Why `set` was the weaker primitive

```rust
let v = t.get(&[0, 1]).unwrap();
t.set(&[0, 1], v + 1.0)?;        // set:      two lookups, two bounds checks
*t.get_mut(&[0, 1])? += 1.0;     // get_mut:  one
```

`set` is a one-liner on top of `get_mut`; `get_mut` cannot be built from `set`. Read-modify-write —
the common case for mutation — is exactly what `set` cannot express.

### 4.2 Why it also failed RFC-087 §6

That rule permits divergence only *"where the ecosystem's behaviour is itself implicit, the
divergence surfaces as an explicit error, and the error teaches."* Verified against `ndarray 0.17`,
run rather than recalled:

```rust
a[[0, 1]] = 9.0;                       // IndexMut
*a.get_mut((1, 0)).unwrap() += 100.0;  // get_mut -> Option<&mut f64>
for v in a.iter_mut() { *v *= 2.0; }   // iter_mut
```

A Rust user reaching for `get_mut` would have hit a compile error teaching only that matten spells it
differently. That is gratuitous divergence, and the rule forbids it.

### 4.3 The surface

```text
get_mut(&mut self, coord: &[usize])  -> Option<&mut f64>
get_flat_mut(&mut self, index: usize) -> Option<&mut f64>
```

Same name plus `_mut`, same `Option` return, same argument shape as the getters they mirror. Nothing
new is invented.

**`Option` is correct here where `Option<()>` was not.** The hazard that drove the earlier `Result`
decision was `let _ = t.set(...)` silently losing a write. `get_mut` has no such hazard: the value
must be dereferenced to write at all, so a discarded handle writes nothing. The safety argument was
sound and applied to an API that no longer exists.

### 4.4 Dynamic input — matching `get`, and why that reverses the earlier draft's ruling

`get` / `get_flat` call `panic_if_dynamic`. **`get_mut` must do the same.** The earlier draft argued
against new panic paths, citing RFC-099. That argument does not transfer:

```text
RFC-099's try_ forms replaced panics for operations that FAIL ON DATA — shape
mismatch, non-numeric elements. A caller can act on that.

get_mut on a dynamic tensor is a PROGRAMMING ERROR, the same one `get` already
panics on. The signature returns Option, so the alternative is None — which would
be indistinguishable from out-of-bounds and therefore silent.
```

Consistency with the sibling getter is both the safer and the more honest choice.

## 5. Scope

### In scope

```text
get_mut / get_flat_mut on numeric Tensor, per §4.3
get_element_mut(&mut self, coord) -> Option<&mut Element>   (§6.1, widened)
tests per §8, both feature profiles
docs: compatibility.md's `set_flat` and `Mutable element API` rows, plus a
      mutation section beside where get/get_flat are documented
```

### Deliberately not in this cut — with reasons, not bare exclusions

```text
IndexMut (t[[i,j]] = v)   panics on out-of-range. It is the ecosystem idiom, but it
                          adds a panic path, and get_mut covers the same need with an
                          Option. Reconsider once get_mut has shipped and its ergonomics
                          are known in practice.
iter_mut / as_mut_slice   bulk mutation, a strictly larger commitment: as_mut_slice
                          exposes contiguity as a promise. Both are cheap to add later
                          ON TOP of get_mut; neither is cheap to withdraw.
set / set_flat            expressible in one line over get_mut. Add only if real use
                          shows the sugar earns its surface.
```

### Out of scope — a diff touching these is a defect

```text
any change to get / get_flat / get_element, or to any operator
exposing CoW, or any change to the numeric representation
CHANGELOG.md — the release RFC writes it
```

## 6. CORRECTED — dynamic mutation is not blocked, and I invented the blocker

**This section previously claimed a coercion decision blocked dynamic mutation. That was wrong**, and
the owner caught it by asking whether the question was a dynamic-API question at all.

The probe wrote `42.0` into an `Int` column and produced `[Int(0), Float(42.0), Int(2)]`. I reported
that as a semantic question — *allow, reject, or coerce?* It is neither:

```text
DynamicTensor = Arc<Vec<Element>> + shape + len + view
  -> no per-column type, no schema, no declared homogeneity
  -> schema_summary() is DERIVED on demand, not a stored invariant
  -> Element is pub, re-exported at lib.rs:106, variants public
```

There is no "Int column" to violate. A dynamic tensor holding all `Int`s was never homogeneous *by
contract*. And with the right signature the library never chooses a variant at all:

```text
get_element_mut(&mut self, coord: &[usize]) -> Option<&mut Element>
    -> the caller reads the variant, changes it, or replaces it. The library
       never chooses one, so there is nothing to coerce.
```

**My probe took `f64` and picked `Float` for it.** That was the probe's shortcut; I then reported its
consequence as a design question. RFC-011 §11's coercion question is real — but it governs
`try_numeric`, which *reads* and must interpret `Element -> f64`. Writing interprets nothing.

**This is the fourth consecutive premise of mine to fail in this area**, after streaming, dynamic
slicing, and this RFC's own §2. The first three were found by building the smallest real thing; this
one was found by a question I should have asked myself. The pattern is now specific enough to name:
**I reason forward from a probe's incidental output instead of asking what the type actually
requires.**

### 6.1 RESOLVED — widened (owner, 2026-08-08)

The split originally recommended to the owner rested on dynamic being blocked. It is not, so the
choice was re-put and resolved to **widen**: `get_mut` / `get_flat_mut` (numeric) and
`get_element_mut` (dynamic) ship in one RFC. Same theme, same test shape, same `compatibility.md`
row, and shipping numeric alone would leave a gap users hit immediately after RFC-102 handed them
dynamic slices.

**One consequence worth documenting when it ships:** mutating a dynamic slice calls `materialize()`,
which detaches its storage — so it *incidentally releases the parent allocation*. That is RFC-102
§8.1's retention escape hatch arriving as a side effect of an unrelated operation.

## 7. Risks

```text
1. NONE MEANS TWO THINGS. If get_mut returns None for a dynamic tensor instead of
   panicking, the caller cannot distinguish "not dynamic-safe" from "out of range".
   Assert the panic and its message (§4.4).
2. SLICES ARE NOW ALIASED (RFC-102). Numeric slices are owned copies, so numeric
   mutation cannot leak — but a test must PROVE it, because the same statement was
   true of dynamic slices until RFC-102 made it false the same week.
3. COORD VS FLAT. get_mut resolves through coord_to_flat; get_flat_mut does not.
   An off-by-one between them is invisible on a square tensor — test non-square.
4. get_element_mut MUST materialize() before handing out &mut Element,
   or a write reaches a shared parent. materialize() is a no-op when contiguous and
   unique, so the cost is paid only when it must be. Assert a slice's write does NOT
   reach its source, and assert storage identity BREAKS on first write.
```

## 8. Acceptance criteria

```text
[ ] get_mut / get_flat_mut return Some(&mut f64) and writes land in place
[ ] out-of-range returns None; the tensor is UNCHANGED
[ ] read-modify-write works in one expression: *t.get_mut(&[r,c])? += 1.0
[ ] a dynamic tensor PANICS with the same message shape as get/get_flat — asserted
    against the captured message, not merely "it panics"
[ ] mutating a numeric slice leaves its source unchanged — asserted (risk 2)
[ ] non-square tensor: get_mut(&[r,c]) and get_flat_mut agree (risk 3)
[ ] get_element_mut materializes; a slice's write does not reach its
    source; Arc identity breaks on first write — all asserted (risk 4)
[ ] no change to get / get_flat / get_element, to any operator, or to any result
[ ] both feature profiles build
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no version bump, tag, or publish
```

## 9. Non-goals

```text
IndexMut, iter_mut, as_mut_slice, set/set_flat — deferred WITH reasons in §5
exposing CoW, or any change to the numeric representation
the RFC-011 §11 coercion rule, which governs try_numeric (reading) and is
  untouched by writing — see §6
```
