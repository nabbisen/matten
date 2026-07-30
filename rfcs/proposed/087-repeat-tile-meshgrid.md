# RFC-087: `repeat`, `tile`, and `meshgrid`

**Status:** Proposed; additive core API. No release, no version bump
**Target:** Post-`0.40.0`, on the `0.x` line
**Theme:** Close RFC-039 §8's three deferred shape-composition APIs, the first work selected against
the §1.1 planning baseline
**Depends on:** RFC-002, RFC-005, RFC-018, RFC-031, RFC-039, RFC-040
**Related:** RFC-006, RFC-007, RFC-078, RFC-083

---

## 1. Summary

Add three shape-composition APIs to core `matten`, each in the established paired form:

```rust
impl Tensor {
    pub fn try_repeat(&self, n: usize)                      -> Result<Tensor, MattenError>;
    pub fn repeat(&self, n: usize)                          -> Tensor;
    pub fn try_repeat_axis(&self, n: usize, axis: usize)    -> Result<Tensor, MattenError>;
    pub fn repeat_axis(&self, n: usize, axis: usize)        -> Tensor;

    pub fn try_tile(&self, reps: &[usize])                  -> Result<Tensor, MattenError>;
    pub fn tile(&self, reps: &[usize])                      -> Tensor;

    pub fn try_meshgrid(x: &Tensor, y: &Tensor) -> Result<(Tensor, Tensor), MattenError>;
    pub fn meshgrid(x: &Tensor, y: &Tensor)     -> (Tensor, Tensor);
}
```

RFC-039 §8 deferred all three with *"needs decisions"* lists rather than designs. This RFC makes those
decisions. No dependency, feature, or MSRV change.

## 2. Why these, and why now

First theme selected against the §1.1 planning baseline — `matten` as a *family car* for education,
learning, PoC and prototyping. These three score well on the baseline's first criterion, **what a
learner meets early and often**:

```text
meshgrid   the primitive for evaluating f(x, y) over a grid — where nearly every
           tutorial goes as soon as it leaves 1-D
tile       building a repeating pattern or padding a small matrix out to a shape
repeat     the classic demonstration of materialising what broadcasting does implicitly
```

They are also the cheapest real work available: RFC-039 already classified them, and only the open
decisions remained.

**Not claimed:** these are not needed for correctness, and nothing is blocked on them. This is
additive convenience chosen for teaching value.

## 3. `repeat` — element repetition, with an `_axis` variant

**Decision (RFC-039 §8.1, "whole tensor or along axis?"): both, as separate functions**, following
the project's own `var`/`var_axis`, `std`/`std_axis` precedent (RFC-040) rather than NumPy's
`axis: Option<usize>` parameter. An `Option` argument whose `None` case silently changes both the
operation *and* the output rank is exactly the implicit behaviour this project avoids.

```text
repeat(n)              flattens, repeats each ELEMENT n times, returns rank-1
                       [1, 2, 3].repeat(2)  ->  [1, 1, 2, 2, 3, 3]

repeat_axis(n, axis)   repeats each element along `axis`, preserving rank
                       [[1, 2], [3, 4]].repeat_axis(2, 0)  ->  [[1, 2], [1, 2], [3, 4], [3, 4]]
```

This matches `numpy.repeat`'s element semantics. **`repeat` repeats elements; `tile` repeats the
whole tensor** — the single most confused pair in this area, and the doc comments must state the
contrast explicitly with the example above.

**Scalar semantics (§8.1):** a rank-0 tensor (shape `[]`, one element) repeats to rank-1 of length
`n`. `repeat_axis` on a rank-0 tensor is an `Axis` error — there is no axis to repeat along.

**`n = 0`:** an error, not an empty tensor. The shape model rejects zero-sized dimensions
(`Tensor::zeros(&[0])` already fails), so returning one is impossible; erroring is the honest answer
and the message should say so.

**Relation to broadcasting (§8.1):** they are deliberately different, and the docs should teach that.
Broadcasting is implicit and materialises nothing; `repeat` is explicit and allocates. A learner who
understands why `[1,2,3] * 2` differs from `[1,2,3].repeat(2)` has learned something real.

## 4. `tile` — whole-tensor repetition

```text
tile(&[2])        [1, 2, 3]        ->  [1, 2, 3, 1, 2, 3]
tile(&[2, 1])     [[1, 2]]         ->  [[1, 2], [1, 2]]
```

**Decision (RFC-039 §8.2, "rank padding semantics"): accept `reps.len() <= rank`, reject
`reps.len() > rank`.**

- Shorter than rank → prepend `1`s. NumPy-compatible, and the convenient direction learners actually
  use (`tile(&[2])` on a matrix).
- Longer than rank → **error**. NumPy silently *promotes the tensor's rank*, which is the genuinely
  surprising direction: the result has more dimensions than the input, and a learner debugging an
  unexpected rank has no obvious place to look. An explicit `Shape` error naming both lengths teaches
  more than a silent promotion.

This is a deliberate, documented divergence from NumPy in one direction only. §6 addresses why the
project's ecosystem-matching principle does not override it here.

**`reps` containing `0`, or an empty `reps`:** errors, same reasoning as `repeat`'s `n = 0`.

## 5. `meshgrid` — the one genuinely contestable decision

**Decision (RFC-039 §8.3): two inputs only, returning a tuple, with NumPy's `xy` indexing and no
option.**

```rust
pub fn meshgrid(x: &Tensor, y: &Tensor) -> (Tensor, Tensor);
// x: rank-1 len m, y: rank-1 len n  ->  both outputs shape [n, m]
```

**Output type (§8.3):** a 2-tuple, not `Vec<Tensor>`. The N-dimensional form is rare outside
scientific computing, and `Vec<Tensor>` forces every caller to index and unwrap. Two rank-1 inputs
covers the teaching case. N-D is a future RFC if ever wanted.

**Inputs must be rank-1.** A rank-2 input is a `Shape` error, not a flatten — flattening silently
would hide a real mistake.

**Indexing style (§8.3) — `xy`, and I changed my mind on this.** My first instinct was `ij`, on the
grounds that `out[i][j] ↔ (x[i], y[j])` is the intuitive matrix reading, and that `xy`'s justification
is a plotting convention this project has no plotting for (RFC-070 closed public visualisation).

That reasoning does not survive one consideration: **when `x` and `y` have equal length, the two
conventions differ only by a transpose, and the mistake is silent.** A learner porting NumPy code
would get numerically wrong results with no shape error to catch it. Diverging on a *visible* axis
(§4's rank rejection) is teaching; diverging on an *invisible* one is a trap.

So `xy`, matching NumPy's default, with the `ij` alternative explained in the doc comment so a reader
who needs it knows to transpose. This also keeps faith with RFC-078 §4.1's principle — match the
ecosystem default a function's name implies — which RFC-083 §4.1 reaffirmed.

## 6. The principle tension, stated rather than glossed

RFC-078 §4.1 established *"match the ecosystem default for a function of this name"*. §4 diverges from
NumPy on `tile`'s rank promotion. Both cannot be unconditionally true, so the boundary needs stating:

```text
MATCH the ecosystem when a divergence would be SILENT — wrong numbers, or a wrong
      shape the caller cannot see.        (meshgrid's indexing: matched)

DIVERGE where the ecosystem's behaviour is itself implicit, the divergence surfaces
      as an explicit error, and the error teaches.   (tile's rank promotion: rejected)
```

That is consistent with both RFC-078 and this project's standing preference for explicit over silent
(RFC-035's numeric conversion, RFC-005's boundary rules). It is not a licence to diverge generally.

## 7. Allocation safety, dynamic tensors, and error model

**Allocation (RFC-039 §6, RFC-018).** All three multiply sizes and can overflow trivially — `repeat`
by a large `n`, `tile` by a product of reps, `meshgrid` by `m × n` *twice*. Every output size must be
computed with a **checked product** and validated against `MattenLimits::max_elements` *before*
allocating. The `try_*` form returns `MattenError::Allocation`; the convenience form panics with the
same message, per the existing `unwrap_or_else(|e| panic!("{e}"))` pattern in `composition.rs`.

**Dynamic tensors (RFC-039 §7, RFC-031):** rejected, unconditionally, as `concatenate`/`stack` already
do. Convert with `try_numeric()` first.

**Error model (RFC-005):** `try_*` never panics; the convenience form panics only on programmer error,
with an actionable message. No new `MattenError` variant — `Shape`, `Axis` and `Allocation` cover
every case above.

## 8. Scope

```text
IN    crates/matten/src/composition.rs   the three APIs and their try_ forms
      crates/matten/src/composition/tests.rs
      crates/matten/README.md, crate docs, docs/src/reference/shape-composition.md
      one example, teaching repeat-vs-tile and a meshgrid grid evaluation
      docs/src/reference/public-api-snapshot.md  — core surface DOES change here

OUT   N-dimensional meshgrid; an `indexing` parameter; `repeat` with a per-element
      repeat vector (numpy's `repeats` array form); dynamic-tensor support;
      any companion-crate change; version bump, CHANGELOG, tag, publish
```

## 9. Acceptance criteria

```text
[ ] all eight functions implemented, paired try_/panicking per composition.rs's pattern
[ ] repeat vs repeat_axis vs tile distinction demonstrated in tests AND in doc comments
[ ] meshgrid xy convention pinned by a test with UNEQUAL input lengths, so a
    transposed implementation cannot pass
[ ] checked-product allocation guard on all three, proven by a test that would
    overflow and instead returns MattenError::Allocation
[ ] n = 0, empty reps, reps longer than rank, rank-2 meshgrid input: all errors
[ ] dynamic tensors rejected for all three
[ ] public-api-snapshot.md updated — this is a real core surface change
[ ] full gate set; no version bump, CHANGELOG, tag, or publish
```

## 10. Non-goals

```text
performance — the family car does not need to be fast (§1.1)
N-dimensional meshgrid, or an indexing option
replacing broadcasting; repeat exists to CONTRAST with it
any companion-crate work
```
