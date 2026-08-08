# RFC-106: Zero-Sized Dimensions — Audit Before Decision

**Status:** Proposed — **this RFC scopes an audit; it does not decide the policy**
**Target:** core `matten` and the companion crates; no code change in this RFC
**Theme:** Establish what the shape model actually does before changing what it claims
**Related:** RFC-003 §7.4, RFC-030, RFC-087 §6, RFC-105

---

## 1. Summary

The project's stated policy is that zero-sized dimensions are rejected. **That is not what the code
does.** Some operations produce them, some accept them, some panic on them, and the companion bridge
rejects them. Before choosing a policy, measure the actual behaviour.

**This RFC deliberately proposes no fix.** Its deliverable is the audit table in §5. The owner has
provisionally chosen to accept zero-sized dimensions (option A of three), and this audit exists to
size that choice honestly rather than commit to it on a partial picture.

## 2. The stated policy, and its stated reason

RFC-003 §7.4 (`0.1.0`):

> zero-sized dimensions should be rejected by constructors … empty tensors complicate `mean`, `min`,
> `max`, JSON shape inference, and scalar broadcasting

The reasoning was sound. **The mechanism does not deliver it** — rejecting at the constructor does not
prevent an operation from producing one, and RFC-105 documents the exact complications §7.4 predicted,
occurring anyway.

## 3. What is already known — measured, not assumed

```text
PRODUCE a zero-sized shape
  slice().range(0..0).all()   -> [0, 3]
  transpose() on [0,3]        -> [3, 0]
  matmul [0,3] x [3]          -> [0]

REJECT a zero-sized shape
  Tensor::new / try_new       -> Err "zero-sized dimensions are not supported"
  from_elements               -> Err, same
  reshape                     -> PANIC, same message
  matten-ndarray::from_arrayd -> Err(ZeroSizedAxis)

ACCEPT a zero-sized shape
  transpose, add, sum_axis, slice, matmul  -> all return Ok
```

**`transpose` accepts what `reshape` rejects, on the same shape.** That is not a policy boundary; it
is an inconsistency, and it is the clearest evidence that the model was never closed.

## 4. Why this is not a one-line change

Three findings that make "relax the constructor check" the wrong size:

```text
1. LOAD-BEARING COMMENTS. selection.rs:17, stats.rs:19, stats.rs:112 and math.rs:~74
   each justify skipping an empty check BY CITING the constructor rejection. Those are
   invariant claims, not prose, and they are already false.

2. PARTIAL ADOPTION IS WORSE THAN NONE. Empty tensors are rare today because only
   slicing (and transpose/matmul on an already-empty input) produce them. Accepting
   them in constructors makes them easy to build — turning RFC-105's rare argmin
   panic into a common one, unless the audit lands first.

3. IT CROSSES A CRATE BOUNDARY. matten-ndarray rejects zero-sized shapes with a
   dedicated error variant. If core accepts them, that rejection becomes incoherent,
   and changing it is a companion API change under lock-step versioning (RFC-030).
```

## 5. Deliverable — the audit table

For **every** public operation in core and the four companions, one row:

```text
| operation | on zero-sized INPUT | can it PRODUCE one | correct? | decision needed |
```

Classify each as one of:

```text
CORRECT        defensible under an accept-zero-sized model, no change
MECHANICAL     needs a bounds/empty check; no semantic choice involved
SEMANTIC       needs a decision (what SHOULD mean of empty be?) -> escalate
INCONSISTENT   disagrees with a sibling operation (transpose vs reshape)
```

**The count of `SEMANTIC` rows is the actual output.** If it is small, accepting zero-sized dimensions
is one RFC. If it is large, it is a multi-release theme and the owner should know that before
committing, not after.

### 5.1 Must be covered

```text
core: every constructor, reshape/flatten/transpose/swap_axes, slice/slice_str,
      concatenate/stack/repeat/tile/meshgrid, every operator and broadcast path,
      every reduction and axis reduction, argmin/argmax, dot/matmul, serde,
      Display, the dynamic half of each of the above
companions: matten-ndarray (both directions), matten-mlprep, matten-data, matten-stats
```

### 5.2 Known already, do not re-derive

RFC-105 fixes `mean`/`min`/`max`/`argmin`/`argmax`. Record them as **decided**, not as open rows.

## 6. Two findings the audit must resolve, already in hand

```text
A. SERDE ROUND-TRIP. A [0,3] tensor serialises to {"shape":[0,3],"data":[]} and then
   FAILS to deserialise, because the serde impl goes through the rejecting constructor.
   matten can serialise a tensor it cannot deserialise. Verified end-to-end.

B. is_empty(). compatibility.md marks it Not planned, reasoning that "the shape model
   rejects zero-sized dimensions in every form". That premise is false. Whether
   is_empty() should exist is downstream of this audit and must be revisited here —
   NOT quietly, since it was declined on a claim that no longer holds.
```

## 7. Ecosystem reference

Measured against `ndarray 0.17`, not recalled:

```text
Array2::from_shape_vec((0,3), vec![])  -> Ok
slice(s![0..0, ..])                    -> shape [0,3], is_empty() == true
.mean()                                -> None          (Option, not NaN)
.sum()                                 -> 0.0
```

ndarray accepts zero-sized shapes throughout and expresses the undefined cases in the **type** rather
than in a sentinel. Under RFC-087 §6 this is a match-the-ecosystem case: matten's divergence here is
silent, producing wrong numbers and failed round-trips rather than an error that teaches.

**This is a reference point, not a decision.** matten's reductions return `f64` with `try_` pairs, not
`Option`; adopting ndarray's shape wholesale would be a larger API change than RFC-105's, and whether
to is exactly what the audit informs.

## 8. Scope

### In scope

```text
the §5 audit table, covering §5.1
a recommendation on the three options, WITH the SEMANTIC count as its evidence
a proposed sequencing if the answer is multi-release
```

### Out of scope — this RFC changes no code

```text
any implementation, including the constructor relaxation
RFC-105's five reductions
a v1.0 decision (RFC-076 remains deferred)
```

## 9. Acceptance criteria

```text
[ ] every operation in §5.1 classified; NO operation listed as "not checked"
[ ] each row cites the command or file:line that established it
[ ] every INCONSISTENT pair named explicitly (transpose vs reshape is one; find the rest)
[ ] the SEMANTIC count stated plainly, with each such row's open question written out
[ ] §6's A and B both resolved into recommendations
[ ] a recommendation among accept / prevent / document, with the audit as its basis
[ ] no code change; no version bump, tag, or publish
```

## 10. Non-goals

```text
implementing any option
deciding is_empty() ahead of the audit that informs it
matching ndarray for its own sake — §7 is evidence, not a target
```
