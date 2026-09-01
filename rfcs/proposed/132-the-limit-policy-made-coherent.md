# RFC-132: The Limit Policy, Made Coherent

**Status:** Proposed — **this RFC asks for a decision before it asks for an implementation.**
**Target:** `crates/matten/src/limits.rs` and every allocating operation; `RFC-018`'s policy
**Theme:** Decide once what `MattenLimits` *is*, so no future operation has to re-derive it
**Related:** RFC-018 (the policy this reopens), RFC-001 (the threat model that motivates it),
RFC-127, RFC-129, external audit F-3/F-7/I-2/D-11

---

## 0. The decision, in plain terms

**The question is: when should `matten` refuse to allocate memory?**

Right now it has three different answers depending on which function you call, and the error message
it prints recommends something that does not work. This RFC asks you to pick one answer.

### What a user experiences under each option

Take a caller with a 2000×1000 tensor — 2 million numbers, about 16 MB. Ordinary data.

| | **A. Boundary-only** *(recommended)* | **B. Per-tensor** | **C. Per-call** | **D. Leave it** |
|---|---|---|---|---|
| `big + big` | **works** | works if `big` was made with a raised budget | works if you pass a budget: `big.try_add_with(&b, &limits)` | **panics, and you cannot fix it** |
| a hostile 36-byte JSON | **rejected** | rejected | rejected | rejected |
| `matten` reads a 10 GB file | **refused** | refused | refused | *currently not refused at all* |
| new operation added next year | **rule already decided** | rule already decided | rule already decided | maintainer guesses again |

### What each option *means*

```text
A  Limits guard the DOOR, not the room.
   matten checks sizes when data arrives from outside — a file, JSON, CSV, a
   shape you were handed. Once it is in memory and valid, it is your data and
   matten does not second-guess you.

B  Each tensor carries its own budget, set when it is created, and every
   operation on it respects that budget.

C  You pass a budget to each operation that needs one, the way you already pass
   a NumericPolicy to try_numeric_with.

D  Keep today's behaviour. Delete the sentence claiming MattenLimits is "the
   single source of truth", because it is not.
```

### Why I recommend A

Not because of this bug — because of the next operation someone adds. **A gives one test that answers
the question forever: does this read untrusted input?** B and C make every new operation decide
again, which is exactly how the current situation arose.

A also happens to fix three separate audit findings at once, and it is the only option under which
`max_parse_bytes` — public, documented, and enforced nowhere — stops being an anomaly and becomes
obviously required.

**The cost of A, stated honestly:** a caller who asks for something genuinely enormous with their own
in-memory data can exhaust their own memory, and `matten` will not stop them. NumPy and `ndarray`
both work this way. It is a real change from today's intent, and it is the thing to disagree with if
you are going to disagree.

---

## 1. Summary

```text
MattenLimits documents itself as "the single source of truth for all allocation
budgets in matten". Measured:

  3 public functions accept one     try_zeros_with_limits, try_ones_with_limits,
                                    try_full_with_limits — all fill constructors
  0 other operations accept one     not matmul, slicing, reductions, arithmetic,
                                    or any parser
  max_parse_bytes                   public, documented, enforced nowhere
```

**This RFC does not propose a fix. It proposes a decision** — global, per-`Tensor`, per-call, or
boundary-only — and then applies whichever the owner picks, once, everywhere.

## 2. The incoherence, reproduced

Not argued — run.

```text
let mut limits = MattenLimits::default();     // max_elements = 1_048_576
limits.max_elements = 8_000_000;
let big = Tensor::try_zeros_with_limits(&[2000, 1000], &limits)?;   // Ok, len = 2_000_000

&big + &big       -> PANIC
    "broadcast requested 2000000 elements, exceeding the limit of 1048576
     (MattenLimits::max_elements); use smaller shapes or increase the limit"

big.sum_axis(0)   -> Ok, len = 1000
```

**Three different behaviours, one tensor, in three consecutive lines:**

```text
construction   honours the caller's raised budget
arithmetic     ignores it, checks the DEFAULT, and panics
reductions     do not check at all
```

> **And the panic message advises an action the API does not support.** It says *"increase the
> limit"* — the caller already did; that is how the tensor exists. There is no way to raise the budget
> for arithmetic, so the remedy the error names is unreachable. That is a documentation-truth defect
> sitting inside an error message, which is the worst place for one: it is read exactly when someone
> is already stuck.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Exactly **3** public functions accept a `MattenLimits`, all fill constructors | `grep 'pub fn .*[Ll]imits'` → `try_zeros_with_limits`, `try_ones_with_limits`, `try_full_with_limits` |
| E2 | `limits.rs:36-38` calls itself *"the single source of truth for all allocation budgets"* | direct read |
| E3 | A raised budget is not carried into arithmetic | probe in §2 |
| E4 | Reductions do not check the budget at all | probe in §2 — `sum_axis` on 2 M elements succeeded |
| E5 | The panic message names an unreachable remedy | probe in §2 |
| E6 | `max_parse_bytes` is public, documented, and enforced nowhere; its own rustdoc says so | `limits.rs:69-70` |
| E7 | The audit measures the limit applied at 8 of ~13 shape-derived allocation sites | external audit F-3 — denominator not independently derived |
| E8 | The crate's one precedent for "operation plus explicit policy" is `try_numeric` / `try_numeric_with(NumericPolicy)` | `dynamic/tensor_ext.rs:117` — the **only** `_with` in the crate |

## 4. The question underneath

**What is a limit *for*?** The current code cannot answer, because it behaves as three different
things at once. RFC-001 §2 motivates the policy as protection against *"malformed or adversarial
inputs"*; RFC-018 then applies it to fill constructors, which take no input at all.

```text
if a limit protects against UNTRUSTED INPUT      -> it belongs at the boundary
if a limit protects the CALLER FROM THEMSELVES   -> it belongs on every operation
if a limit is a per-workload BUDGET              -> it belongs on the data or the call
```

**These are different products.** Choosing among them is the decision this RFC exists to make.

## 5. The options, with their costs

### Option A — boundary-only. *Recommended.*

```text
Limits bound what enters from OUTSIDE: JSON, CSV, file paths, caller-supplied
shapes, the slice mini-language. Once data is validated and in memory, the
caller owns it and operations do not second-guess them.
```

```text
+  matches RFC-001's stated motivation exactly — adversarial INPUT
+  one rule, statable in a sentence, with an obvious test: is this a boundary?
+  &big + &big just works. F-7's panic largely disappears as a budget problem
+  max_parse_bytes stops being an anomaly and becomes REQUIRED — it is the
   boundary control, so enforcing it (D-11) follows from the model rather than
   being a separate chore
+  no new API surface at all
-  a caller can still OOM on their own data. That is arguably correct — NumPy
   and ndarray both let you — but it IS a change from today's intent
-  `try_*_with_limits` (3 fns) lose their reason to exist; deprecate or keep as
   boundary-shape validators
```

### Option B — per-`Tensor`

```text
Each Tensor carries its budget; operations inherit it.
```

```text
+  &big + &big honours big's budget. The §2 incoherence disappears
+  no API explosion
-  adds a field to Tensor, the core type — memory, and it must not serialize
-  needs a rule when two tensors with different budgets meet (max? min? error?)
   and that rule will be surprising whichever way it goes
-  makes a safety control a property of DATA, which is conceptually odd
```

### Option C — per-call `_with`

```text
Every allocating operation gains a `_with(limits)` twin, following E8.
```

```text
+  explicit; follows the crate's one existing precedent
-  API EXPLOSION. Every allocating op doubles, in a crate whose stated value is
   a small approachable surface. This is the option most in tension with the
   product
-  the caller threads limits manually through every call site
```

### Option D — status quo, honestly documented

```text
Keep the behaviour; delete the "single source of truth" claim; document that
limits apply to fill constructors only.
```

```text
+  no code change
-  leaves E5's unreachable remedy in a live error message
-  leaves the 8-of-13 coverage gap, so which operations check remains arbitrary
-  this is the haphazard outcome the owner's direction argues against
```

## 6. Recommendation

**Option A**, for a reason that is about maintenance and extension rather than this bug:

> **A boundary-only rule answers the question for operations that do not exist yet.** Every future
> allocating operation has one test — *does this consume untrusted input?* — and the answer follows.
> Options B and C require each new operation to decide again, which is precisely the per-operation
> re-derivation this RFC exists to stop.

It also collapses three open findings into one model: F-3's coverage gap becomes "boundary sites,
completely"; D-11's unenforced `max_parse_bytes` becomes required; F-7's budget panic stops being a
surprise on the caller's own data.

**The decision is the owner's.** A, B and C are all defensible; D is defensible only if the owner
wants no change, and it should then be chosen deliberately rather than by default.

## 7. What this changes for RFC-129, and what it does not

**Nothing, and this was checked before RFC-129 was accepted.** `try_add(&self, other)` survives every
option:

```text
A  budget stops applying to arithmetic; try_add still returns Err for broadcast
   incompatibility, which is its other and more common job
B  the tensor carries the budget; the signature is unchanged
C  try_add_with(&self, other, &limits) is an ADDITIVE follow-up in E8's existing
   shape — not a breaking change
```

RFC-129 ships independently and is not blocked by this decision.

## 8. Scope

```text
IN     the model decision, then applying it once, everywhere
       max_parse_bytes: enforce it or delete it — the model decides which
       the "single source of truth" sentence, which must become true or go

OUT    RFC-127's fixes                 ship first; that is a bug, this is a design
       RFC-129                          independent (§7)
       per-operation performance work   unrelated
       a public MattenLimits builder    only if the chosen model needs one
```

## 9. Risks

```text
R1  Choosing a model and then applying it partially — ending with a fourth
    inconsistent behaviour. Whatever is chosen, the coverage must be COMPLETE
    or the claim must be narrowed to match. That is the whole point.
R2  Option A read as "remove the safety". It does not: the boundary is where
    untrusted input arrives, and it becomes MORE strictly enforced, not less.
R3  Implementing before the owner decides. This RFC is a decision request.
R4  Leaving E5's error message advising an unreachable remedy, whichever model
    wins. That sentence must change in every option including D.
R5  Treating the audit's "8 of ~13" as authoritative. E7 — the denominator was
    never independently derived.
```

## 10. Acceptance criteria

**Stage 1 — the decision.** No code.

```text
[ ] the owner chooses A, B, C or D, and the choice is recorded here with its reason
```

**Stage 2 — applying it.** Written once the model is known.

```text
[ ] the chosen model applied COMPLETELY, with the site list derived and reported
[ ] limits.rs's "single source of truth" sentence true, or replaced
[ ] the §2 probe produces one coherent behaviour, not three
[ ] E5's error message no longer names an unreachable remedy
[ ] max_parse_bytes enforced, or removed with its promise
[ ] nine guards; both feature profiles; no tag or publish
```

## 11. What this does not fix

```text
- the Critical. RFC-127 does that and ships first.
- the missing try_ arithmetic. RFC-129, independent.
- whether the DEFAULT budget is right. 1 048 576 elements is a 1024x1024 matrix,
  which the audit notes is small for ordinary data. That is a separate question
  and changing a default is a behaviour change deserving its own decision.
```

**And it does not make the limit policy correct — it makes it decidable.** Today a maintainer adding
an allocating operation has no rule to follow and must guess, which is how 8 of ~13 happened. The
value of this RFC is that afterwards there is an answer, and it is the same answer every time.
