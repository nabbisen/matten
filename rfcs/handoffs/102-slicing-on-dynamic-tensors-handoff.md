# Developer Handoff — RFC-102: Slicing on Dynamic Tensors

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/102-slicing-on-dynamic-tensors.md`
**Base:** `main` @ `fee8a24`, clean except `docs/src/reference/compatibility.md`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding** rather than picking one — per your operating instructions,
> an RFC/handoff disagreement is an escalation trigger, not a judgement call. If the disagreement is
> discovered mid-work and the handoff's extra requirement is a pure addition you can simply omit,
> omitting it and reporting prominently is acceptable; anything else, ask first.
>
> **Corrected 2026-08-08 after implementation:** this rule was missing from the handoff as issued,
> and §5/§9/§10/§11 below wrongly required a `CHANGELOG.md` entry that RFC-102 §7 does not list. The
> implementer caught it, checked project history, and omitted it. **They were right and the handoff
> was wrong** — `CHANGELOG.md` is written at release preparation, by the release RFC, which is the
> only point at which the version is known. The requirement is struck below.

---

## 1. Task title

Route dynamic tensors through `execute_slice` into the existing `slice_indices` view constructor,
replacing the current rejection.

## 2. Purpose

`slice()` and `slice_str()` currently reject dynamic tensors. The view machinery that would serve
them already exists, is tested, and has no caller outside its own tests. This wires it up.

## 3. Background — and the claim I got wrong

ROADMAP §3.1 has framed this as an open design question: *what does a slice of heterogeneous
`Element` data even mean?* **That framing is wrong and it is mine.** There is no semantic question —
slicing selects positions, it does not interpret values. RFC-102 §2 carries the correction.

Treat this as wiring, not design. **If you find a semantic decision you have to invent, stop and
report it** — its existence would mean I am wrong a fourth time on this row, which is worth more to
me than the implementation.

## 4. Evidence for every factual claim below

Each claim carries the command or `file:line` that established it. Re-run them; they are falsifiable.

| # | Claim | Established by |
|---|---|---|
| E1 | `execute_slice` rejects dynamic before doing anything else | `crates/matten/src/slice.rs:144-153` |
| E2 | `slice_indices` exists, `Arc::clone`s storage, returns an `Indexed` view | `crates/matten/src/dynamic/storage.rs:103-124` |
| E3 | It composes: an `Indexed` source maps through `idxs[i]`, it does not nest | `storage.rs:112-115` |
| E4 | Its only callers are three tests | `grep -rn "slice_indices" crates/ --include="*.rs"` → `storage.rs:103` + `dynamic/tests/lifecycle.rs:34,45,60` |
| E5 | The slice loop computes `src_flat`, a **logical** flat index | `slice.rs:201` |
| E6 | Exactly one line in that loop is `f64`-specific | `slice.rs:208` — `out_data[dst_flat] = tensor.data[src_flat]` |
| E7 | Empty `new_shape` is a supported shape, with `len` 1 | `storage.rs:128-133` (`reshape` handles it explicitly) |
| E8 | A dynamic `Tensor` is `data: Vec::new()` + `shape` mirroring `dyn.shape` | `crates/matten/src/dynamic/tensor_ext.rs:168-175` |
| E9 | `get_element` resolves through the **outer** `shape`, then the dynamic view | `tensor_ext.rs:54-57` |
| E10 | `is_unique()` is `Arc::strong_count == 1` | `storage.rs:82-84` |

**E9 is the load-bearing one.** Because `get_element` goes `coord_to_flat(coord, &self.shape)` and
only then into the view, the outer `Tensor.shape` **must** equal the inner `dyn.shape` on the result.
Get that wrong and `get_element` silently reads the wrong position on a slice — no panic, no error,
just wrong data. Follow E8's construction exactly.

## 5. Change scope

```text
crates/matten/src/slice.rs          the dynamic branch in execute_slice
crates/matten/src/dynamic/          only if a pub(crate) accessor is genuinely needed
crates/matten/src/dynamic/tests/    new tests
docs/src/reference/compatibility.md the "Slicing on dynamic tensors" row
docs/src/reference/slicing.md       the dynamic limitation, wherever stated
docs/src/reference/dynamic.md       likewise
CHANGELOG.md                        STRUCK -- release-prep RFCs only; see the priority rule
```

## 6. Explicit non-change scope — a diff touching these is a defect

```text
the slice grammar, resolve_spec, or ANY numeric result
negative-index asymmetry between slice_str and the builder (RFC-088) — leave exactly as is
try_numeric, coercion, or anything that INTERPRETS an Element
mutation, or any public exposure of CoW
Cargo.toml versions — RFC-094 makes the bump a separate decision
```

## 7. Required implementation

Do **not** write a parallel dynamic slice path. The spec resolution and coordinate arithmetic in
`execute_slice` are type-agnostic (E5) and must be shared unchanged. Only the innermost write differs
(E6).

```text
1. Remove the early rejection at slice.rs:144-153.
2. Keep every existing computation up to and including src_flat (slice.rs:201) shared.
3. At the write point, branch:
     numeric  -> out_data[dst_flat] = tensor.data[src_flat]     (unchanged)
     dynamic  -> record src_flat at position dst_flat
4. For dynamic, call slice_indices(indices, out_shape) and wrap per E8:
     Tensor { data: Vec::new(), shape: <the dyn shape>, dynamic: Some(Box::new(..)) }
5. `operation` stays threaded through for error messages.
```

**The index vector must be in output order.** The loop iterates `out_flat`, but writes at `dst_flat`,
and for a rank-collapsing slice those differ. Build a `vec![0usize; out_len]` and assign at
`dst_flat` — do not `push`. A `push`-built vector is correct for the common case and wrong for
collapsing slices, which is the worst failure shape available here.

**Rank-0 collapse:** `out_shape` is empty, `out_len` is 1 (`slice.rs:175-179`), and `slice_indices`
sets `len = indices.len()` = 1 with `shape = []`. That is consistent with E7. Verify rather than
assume.

## 8. Required tests

Assertions, not inspections. Three of these fail *invisibly* if the implementation is wrong — a slice
that copies instead of sharing passes every value check.

```text
T1  slice() on a dynamic tensor -> is_dynamic() is true; shape is correct
T2  slice_str() likewise
T3  STORAGE IS SHARED — assert Arc::ptr_eq or !is_unique(), per lifecycle.rs:35 / E10.
    A value-only assertion does NOT test this.
T4  COMPOSITION — slice a slice; assert the second result reads correct values AND
    still shares the ORIGINAL storage. A nesting bug is correct for the first slice
    only, so a single-level test cannot catch it.
T5  rank-0 collapse: a fully-indexed slice yields shape [] and reads its element
T6  Text, None and Bool elements survive a slice unchanged (round-trip via get_element)
T7  get_element on a slice returns the SLICE's element, not the parent's — the E9 trap.
    Use a slice whose offset makes the two differ (e.g. column 1 of a 2x3).
T8  every existing numeric slice test still passes, unmodified
```

Do not weaken T3/T4 to value equality. If either is awkward to assert from the public API, add a
`#[cfg(test)]` accessor rather than dropping the assertion, and say so in the review request.

## 9. Required documentation updates

```text
compatibility.md   the "Slicing on dynamic tensors" row -> Supported.
                   NOTE: I corrected this row earlier in the session to say both
                   slice() and slice_str() are numeric-only. Read the CURRENT text
                   before editing — do not edit from this description of it.
slicing.md         remove/replace the dynamic limitation wherever it appears
dynamic.md         likewise; the get_element workaround is no longer the only route
CHANGELOG.md       STRUCK -- not this RFC's; the release RFC writes it (priority rule)
```

Any ```rust fence you add or touch must compile *and run* under `scripts/check-doc-code.sh`.

## 10. Acceptance criteria

```text
[ ] slice() and slice_str() work on dynamic tensors, returning is_dynamic() == true
[ ] storage is SHARED — asserted via Arc identity or is_unique(), not inferred from values
[ ] slicing a slice composes AND still shares the original storage — asserted
[ ] rank-0 collapse works
[ ] Text, None and Bool survive a slice unchanged
[ ] get_element on a slice reads the slice's own positions (T7)
[ ] every numeric slice result byte-identical; existing tests unmodified
[ ] compatibility.md, slicing.md, dynamic.md corrected
[ ] cargo test --workspace, and with --features dynamic
[ ] all eight scripts/ guards
[ ] scripts/check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] clippy clean under RUSTFLAGS="-D warnings"
[ ] no tag, no publish, no version bump
```

## 11. Compatibility constraints

The rejection message (E1) is observable. Removing it changes behaviour for anyone matching on
`MattenError::Unsupported` from a slice call. That is intended, and it is why this lands as
**Changed** rather than Added, and a minor rather than a patch under RFC-094. **Record that framing
in the review request; do not write it into `CHANGELOG.md`** — the release RFC does that, at the
point the version is actually chosen. Do not decide the version.

Under `--no-default-features` (no `dynamic`), the `#[cfg(feature = "dynamic")]` branch vanishes and
behaviour must be byte-identical to today. Build both ways.

## 12. Security constraints

No new dependency, feature, edition, or MSRV change. `#![forbid(unsafe_code)]` stands — index
arithmetic here must stay in safe Rust, including the `storage_indices` mapping.

`slice_indices` indexes `idxs[i]` directly (E3), which panics on a malformed index vector rather than
corrupting memory. Do not "fix" that into a silent fallback; a panic on an internally-constructed
index is a bug signal worth keeping.

## 13. Known risks

```text
R1  Wrong outer shape -> get_element silently misreads (E9). Highest-severity failure
    here because nothing errors. T7 exists for this.
R2  push-built index vector -> wrong under rank collapse only (§7 step 4).
R3  Copy-instead-of-share -> invisible to value assertions (T3).
R4  Nested rather than composed views -> correct for one level only (T4).
All four produce plausible-looking output. Assert; do not eyeball.
```

## 14. Required evidence

For each of T3, T4, T7: state the assertion line and what it would print if the corresponding risk
were live. For the numeric-unchanged claim, state the command, not the word "verified".

Verify §4's claims against the code **before** starting. Per your operating instructions §2, report
any discrepancy first — **including one that makes this task smaller or removes it** (E4 is exactly
such a finding, and it is why this RFC exists).

## 15. Prohibited shortcuts

```text
- a separate dynamic slice path duplicating spec resolution
- materialize() to sidestep view composition — it defeats CoW, which is the point
- relaxing T3/T4 to value equality
- touching numeric behaviour "while in there"
- deciding the version number
```

## 16. Required review-request format

Write to: `.git-exclude/review-request/RFC-102/matten-rfc102-slicing-on-dynamic-tensors-implementation-review-request-v0.1.md`

Include: files changed with line counts; the §4 verification result including any discrepancy; the
§14 evidence; guard and test output; anything you deviated from and why; and any question you want
answered at review.
