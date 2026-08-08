# Developer Handoff — RFC-106: Zero-Sized Dimensions Audit

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/106-zero-sized-shape-model-audit.md`
**Sequencing:** **Start only after RFC-105 is merged.** The audit must reflect post-fix code.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**.

---

## 1. Task title

Audit every public operation in core and the four companion crates for zero-sized-dimension
behaviour, and produce the §5 classification table.

## 2. This task produces a document, not a diff

**No code change. None.** If you find a defect, record it as a row — do not fix it. A fix mid-audit
makes the table describe code that no longer exists, and the table is the deliverable.

The one number that matters is **the count of `SEMANTIC` rows**. The owner has provisionally chosen
to accept zero-sized dimensions and needs to know whether that is one RFC or a multi-release theme.
A small count means one; a large count means the choice should be re-put. **You are sizing a decision
the owner has not finally made.**

## 3. Why this audit exists

The project's stated policy — zero-sized dimensions are rejected — **is not what the code does.**
Already measured, do not re-derive:

```text
PRODUCE   slice().range(0..0).all() -> [0,3]   transpose([0,3]) -> [3,0]
          matmul [0,3] x [3] -> [0]
REJECT    Tensor::new / try_new / from_elements -> Err
          reshape -> PANIC (same message)      matten-ndarray::from_arrayd -> Err(ZeroSizedAxis)
ACCEPT    transpose, add, sum_axis, slice, matmul -> Ok
```

`transpose` accepts what `reshape` rejects, on the same shape. Find the rest of those pairs.

## 4. Method

For each operation, **run it** — do not read the source and infer. Source reading tells you what a
branch says; it does not tell you what a fold's identity element leaks, which is exactly how
`min() == inf` survived.

Use a probe crate depending on `matten` by path, with `catch_unwind` so a panic becomes a data point
rather than an aborted run. The reachable empty fixture is:

```rust
Tensor::new(vec![1.,2.,3.,4.,5.,6.], &[2,3]).slice().range(0..0).all().build().unwrap()
```

For operations that cannot take an empty input, the question is only whether they can **produce** one.

## 5. The table

```text
| operation | on zero-sized INPUT | can it PRODUCE one | classification | open question |
```

Classification, exactly one per row:

```text
CORRECT       defensible under an accept-zero-sized model; no change needed
MECHANICAL    needs a bounds/empty check; NO semantic choice involved
SEMANTIC      needs a decision -- what SHOULD this return? -> write the question out
INCONSISTENT  disagrees with a sibling (transpose vs reshape). Name the sibling.
```

Every row cites the command or `file:line` that established it. **No row may read "not checked."**
If something is genuinely untestable, say why in the row.

### Coverage — all of it

```text
core   constructors; reshape/flatten/transpose/swap_axes; slice/slice_str;
       concatenate/stack/repeat/repeat_axis/tile/meshgrid; every operator and the
       broadcast paths; every reduction and axis reduction; argmin/argmax; dot/matmul;
       serde; Display; get/get_flat/get_mut/get_flat_mut; and the dynamic half of each
comps  matten-ndarray (BOTH directions), matten-mlprep, matten-data, matten-stats
```

**RFC-105's five reductions are DECIDED, not open.** Record them as `CORRECT` with a pointer to
RFC-105; do not re-litigate them.

## 6. Two findings to resolve into recommendations

```text
A. SERDE ROUND-TRIP. A [0,3] tensor serialises to {"shape":[0,3],"data":[]} and fails
   to deserialise -- the impl goes through the rejecting constructor. Verified
   end-to-end. Recommend a resolution; do not implement one.

B. is_empty(). compatibility.md marks it Not planned because "the shape model rejects
   zero-sized dimensions in every form". That premise is false. Revisit it explicitly
   -- it was declined on a claim that no longer holds, and letting that stand silently
   is the "records that lie" anti-pattern RFC-000 names.
```

## 7. Ecosystem reference — evidence, not a target

Already measured against `ndarray 0.17`; re-run if you want, do not assume:

```text
Array2::from_shape_vec((0,3), vec![]) -> Ok      slice(s![0..0,..]) -> [0,3], is_empty()
.mean() -> None (Option, not NaN)                .sum() -> 0.0
```

ndarray expresses undefined cases in the **type**. matten uses `f64` with `try_` pairs. **Do not
recommend adopting `Option` wholesale merely because ndarray does** — that is a larger API change
than it looks, and RFC-087 §6 licenses matching the ecosystem, not copying it.

## 8. Deliverable

```text
.git-exclude/review-request/RFC-106/matten-rfc106-zero-sized-shape-model-audit-v0.1.md

  1. the full table (§5), every operation classified
  2. the SEMANTIC count, stated plainly, each with its open question written out
  3. every INCONSISTENT pair named
  4. §6 A and B resolved into recommendations
  5. a recommendation among accept / prevent / document, with the table as its basis
  6. if the answer is multi-release, a proposed sequencing
```

## 9. Acceptance criteria

```text
[ ] git diff is EMPTY -- no code changed, no docs changed
[ ] every operation in §5's coverage list classified; no "not checked" rows
[ ] every row cites its command or file:line
[ ] the SEMANTIC count stated plainly
[ ] every INCONSISTENT pair named, with its sibling
[ ] §6 A and B both resolved into recommendations
[ ] a recommendation among the three options, justified by the table
[ ] no version bump, tag, or publish
```

## 10. Risks

```text
R1  Fixing something. The table is the deliverable; a fix invalidates it (§2).
R2  Inferring from source instead of running. min() == inf was invisible in source.
R3  Classifying a hard row as MECHANICAL to keep the SEMANTIC count low. The count
    is the OUTPUT -- an understated one produces a wrong decision by the owner. When
    unsure between MECHANICAL and SEMANTIC, choose SEMANTIC and say why.
R4  Skipping the companions because core is where the interest is. matten-ndarray
    already rejects zero-sized shapes; that boundary is a live part of the decision.
```

## 11. Escalation

The RFC's own framing may not survive contact. **If the audit shows the premise is wrong — for
instance that zero-sized shapes are far more or far less pervasive than §3 suggests — report that
rather than completing a table built on it.** Four premises in this area have failed testing already;
a fifth would be information, not an embarrassment.
