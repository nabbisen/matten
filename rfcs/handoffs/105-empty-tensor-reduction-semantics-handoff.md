# Developer Handoff — RFC-105: Empty-Tensor Reduction Semantics

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/105-empty-tensor-reduction-semantics.md`
**Base:** `main` @ `629783e`, clean tree, family at `0.44.0`.
**Sequencing:** **Do this before RFC-106.** That audit runs against post-fix code.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Make five reductions return an error on an empty tensor instead of panicking with a raw index error
or returning a sentinel value.

## 2. Why this is a defect and not a policy question

An empty tensor is **reachable today** without any constructor accepting one. Two reductions panic
with a Rust slice panic — which defeats the `try_` form entirely — and three return sentinels.

**This RFC decides nothing about whether zero-sized dimensions should be constructible.** That is
RFC-106. Every fix here is correct under all three of its possible answers. If you find yourself
needing to decide the shape-model question to finish this task, stop and report.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | An empty tensor is reachable | `Tensor::new(vec![1.,2.,3.,4.,5.,6.], &[2,3]).slice().range(0..0).all().build().unwrap()` → shape `[0,3]`, len 0 |
| E2 | `try_argmin`/`try_argmax` **panic** | `"index out of bounds: the len is 0 but the index is 0"` — a raw slice panic, not a `MattenError` |
| E3 | `try_mean` → `Ok(NaN)`; `try_min` → `Ok(inf)`; `try_max` → `Ok(-inf)` | probed against the built library |
| E4 | `try_var`/`try_std` are **already correct** | `stats.rs:118-128` — `Err(InvalidArgument{..})`, `"variance is undefined for an empty tensor"` |
| E5 | `try_sum` → `Ok(-0.0)` | probed; see §6 — **an observation, not a claim** |
| E6 | Panicking forms delegate via `unwrap_or_else(\|e\| panic!("{e}"))` | so they inherit whatever message `try_*` produces |
| E7 | Signatures: `sum`/`mean`/`min`/`max` in `math.rs:36,70,105,139`; `argmin`/`argmax` in `selection.rs:55,71`; `try_` forms at `math.rs:50,84,119,153`, `selection.rs:85,107` | grep |

Re-derive before editing. **Report any discrepancy first, including one that shrinks the task.**

## 4. Required implementation

Copy E4's shape exactly. No new error variant, no new pattern, no signature change.

```text
try_mean, try_min, try_max, try_argmin, try_argmax:
    if self.data.is_empty() {
        return Err(MattenError::InvalidArgument {
            operation: "<name>",
            argument: "self",
            message: "<...> is undefined for an empty tensor".to_string(),
        });
    }
```

Place the check **where `try_var` places it** — after the dynamic rejection, before any computation.
Word each message like `try_var`'s: name what is undefined, then "for an empty tensor".

**Do not touch `try_sum`.** Sum of an empty set is the additive identity; that is defined, and it is
what ndarray returns.

**Do not convert the panicking forms to `Result`.** The pair convention (RFC-055, RFC-099) stands.
Only the message they carry changes, and it changes for free via E6.

## 5. Required tests

```text
T1  each of the five try_ forms returns Err on an empty tensor
T2  each message names the operation and says "undefined for an empty tensor"
T3  argmin()/argmax() panic with THAT SENTENCE, not "index out of bounds" --
    assert the captured message, not merely that it panics
T4  try_var/try_std unchanged -- still Err, same messages
T5  try_sum still returns a zero on empty
T6  NON-empty behaviour is unchanged for all seven -- the existing suite must pass
    unmodified; if you need to edit an existing reduction test, that is a signal
    something is wrong, not a test to update
```

**Every fixture must be a SLICED-EMPTY tensor.** You cannot construct one directly — constructors
reject zero-sized dims — and that impossibility is precisely the point of the RFC. A helper such as:

```rust
fn empty_2x3() -> Tensor {
    Tensor::new(vec![1., 2., 3., 4., 5., 6.], &[2, 3])
        .slice().range(0..0).all().build().unwrap()
}
```

## 6. §5's `-0.0` — investigate, do not fix blind

`try_sum` on empty returns **`-0.0`** where Rust's `[].iter().sum::<f64>()` gives `0.0`. I observed
this and **cannot explain it**. It is harmless (`-0.0 == 0.0` compares true) and is not part of §4.

**Determine why.** If incidental, normalise to `0.0` and say so. If it turns out to be load-bearing,
leave it and say that instead. Report the finding either way — this is stated as an open question
because it is one, not as a task with a known answer.

## 7. Required documentation

Three code comments justify skipping an empty check **by citing an invariant that is false**. Correct
each; do not edit around them.

```text
math.rs (mean's doc)  "Behaviour on an empty tensor is unspecified (zero-sized dims
                       are rejected by constructors in Phase 1)" -> constructors do
                       reject, but slicing produces one anyway. Now specified: Err.
selection.rs:17       "Callers guarantee `data` is non-empty (core rejects zero-sized
                       dimensions)" -> this comment is WHY argmin panics.
stats.rs:112          "zero-sized dimensions, so an empty tensor is not constructible
                       in practice" -> false; the code it justifies returns inf.
```

```text
DO NOT TOUCH: compatibility.md's is_empty() row. It rests on the same false premise,
              but it is about the SHAPE MODEL and RFC-106 owns it.
DO NOT TOUCH: CHANGELOG.md — the release RFC writes it.
```

## 8. Acceptance criteria

```text
[ ] the five try_ forms return Err on empty, mirroring try_var's construction
[ ] argmin()/argmax() panic with the sentence, asserted against the captured message
[ ] every fixture is a sliced-empty tensor (§5)
[ ] try_var/try_std byte-identical; try_sum still returns a zero
[ ] §6 investigated and reported either way
[ ] no signature change; panicking/try_ pairs preserved
[ ] the existing reduction suite passes UNMODIFIED
[ ] §7's three comments corrected; compatibility.md and CHANGELOG.md untouched
[ ] both feature profiles; cargo test --workspace
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] clippy clean under RUSTFLAGS="-D warnings"; cargo fmt --check
[ ] no version bump, tag, or publish
```

## 9. Risks

```text
R1  Editing an existing non-empty reduction test to make it pass. That means the
    change reached non-empty behaviour, which it must not.
R2  Asserting "it panics" instead of asserting the message. The whole defect is
    WHICH panic — an index panic versus a diagnosis.
R3  Constructing the fixture some other way and silently testing nothing. If your
    fixture has len > 0, every new test passes vacuously.
R4  Fixing try_sum's -0.0 as though it were part of the task (§6).
```

## 10. Compatibility

This is a **behaviour change**, not purely additive: callers relying on `min() == inf` for empty input
now get an error. Intended, and it belongs under `Changed` when released — **recorded in the review
request, not written into `CHANGELOG.md`.**

## 11. Required review-request format

Write to:
`.git-exclude/review-request/RFC-105/matten-rfc105-empty-tensor-reduction-semantics-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §6's finding, the
T3 captured messages, guard and test output, deviations with reasoning, and anything you want
answered at review.
