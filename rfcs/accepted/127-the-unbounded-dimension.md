# RFC-127: The Unbounded Dimension

**Status:** **Accepted** 2026-09-01 by the owner. Not yet implemented. Handoff:
`rfcs/handoffs/127-the-unbounded-dimension-handoff.md`. Ships as `0.46.2`, prepared by a separate
release RFC. No version bump, tag, or publish in this RFC.
**Target:** `crates/matten/src/shape.rs`, `math.rs`, `slice.rs`, `stats.rs`, `tensor.rs`,
`parse/csv.rs`, `Cargo.toml`
**Theme:** Close every process abort in the external audit, and the silent wrong answer beside it
**Related:** RFC-111 (the regression's origin), RFC-018 (the limit policy it falsifies), RFC-094 §4.1
(patch contents), RFC-104, RFC-120

---

## 1. Summary

```text
F-1  bound EACH DIMENSION in checked_shape_len, not only the product   <- the root cause
F-2  add the missing guard at the unguarded shape-derived allocation sites
F-5  six `as isize` -> isize::try_from; delete the false comment beside them
D-1  restore Tensor::new's rustdoc; add missing_docs = "deny"
O1   a debug-only invariant assertion at the Tensor construction sites
```

**This is a correctness fix to already-published crate content and nothing else** — patch content
under RFC-094 §4.1 as amended by RFC-120. It ships as `0.46.2`, prepared by a separate RFC.

**`try_add`/`try_sub`/`try_mul`/`try_div` are NOT in this RFC.** They are new public API, which §4.1
excludes from a patch. That is deferred to `0.47.0` and is not a judgement about importance — see §9.

## 2. What is actually wrong, reproduced

An external architect audited `0.46.1`. I reproduced the Critical four ways before writing this.

```text
from_json(r#"{"shape":[400000000000,0],"data":[]}"#)   -> Ok, shape=[400000000000, 0], len=0
try_new(vec![], &[400000000000, 0])                     -> Ok        (not a parser bug)
  .sum_axis(1) on it  -> "memory allocation of 3200000000000 bytes failed" -> SIGABRT
                         catch_unwind did NOT recover
try_matmul([usize::MAX,0], [0,usize::MAX])              -> Ok
                         shape.product() = 3.4e38,  data.len() = 1
```

**36 bytes of JSON aborts the process, and the abort is not catchable.** Separately, a `Result`-zone
API returns a `Tensor` whose shape and data disagree — breaking the crate's one global invariant
silently, which every `expect("valid by construction")` downstream depends on.

### 2.1 The mechanism, and it traces to RFC-111

`checked_shape_len` guards only the *product*:

```rust
let mut len: usize = 1;
for &dim in shape {
    len = len.checked_mul(dim).ok_or_else(|| ...)?;   // a zero dim makes this unreachable
}
```

**A zero dimension makes the product `0`, so `checked_mul` can never overflow and no individual
dimension is ever bounded.** Before RFC-111, a zero dimension was rejected outright — which
*incidentally* bounded every dimension. RFC-111 removed the rejection deliberately and correctly, and
**nothing downstream was revisited.**

> RFC-111 was accepted on my review. The audit's framing is right: this is one root cause with a wide
> blast radius, not a scattered class of bugs. The zero-sized decision stays; the missing bound is the
> defect.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | `checked_shape_len` bounds the product only | `crates/matten/src/shape.rs:44-56`, direct read |
| E2 | A degenerate shape validates through `from_json` **and** `try_new` | probe, both returned `Ok` |
| E3 | An ordinary operation on it aborts, uncatchably | probe: allocation failure, `catch_unwind` did not recover |
| E4 | `try_matmul` returns a structurally corrupt `Tensor` | probe: `shape.product() = 3.4e38`, `data.len() = 1` |
| E5 | `t.slice().index(usize::MAX)` returns **the last row**; `index(9)` correctly errors | probe on a `[2,3]` tensor → `Ok([3], [4,5,6])` |
| E6 | No `try_add`/`try_sub`/`try_mul`/`try_div` exists | `grep` across `crates/matten/src` → nothing |
| E7 | `Tensor::new` has no rustdoc; a `pub(crate)` helper absorbed its doc block | `RUSTFLAGS="-W missing_docs"` → warning at `tensor.rs:95` |
| E8 | The crate-root promise that the Result zone *"never panics on ordinary invalid input"* is currently false | E3 |

**Re-derive E2–E5 before editing.** They are four lines of probe code and they are the whole
justification for this RFC.

## 4. Change A — bound each dimension (the root cause)

In `checked_shape_len`, reject any single dimension that exceeds the element budget, **before** the
product is computed — because the product cannot detect it once a zero is present.

```text
for each dim: if dim > <the element budget>, return MattenError::Allocation
then the existing checked_mul loop, unchanged
```

```text
DO NOT re-reject zero dimensions. RFC-111 accepted them deliberately after a
three-stage campaign (RFC-105/110/111) and they are load-bearing for slicing.
The defect is the ABSENT BOUND, not the accepted zero.

DO make the error the existing Allocation variant with an actionable message.
This is the same error class the guarded sites already produce.
```

**Both `validate_shape` and `MattenLimits::check_shape` reach this helper**, so one change covers the
~18 sites that fan out from them. Verify that claim rather than trusting it.

## 5. Change B — the unguarded allocation sites

The audit reports the limit policy applied at 8 of ~13 shape-derived allocation sites, with
`math.rs`, `slice.rs`, `stats.rs`, `parse/csv.rs` and `tensor/ops.rs` unguarded.

```text
DERIVE THE SITE LIST YOURSELF. I confirmed the pattern at the cited lines but did
NOT independently derive the denominator, and the audit's "~13" is approximate.
Report your list and its method; a discrepancy is worth more than the edit.

The correct three-line pattern already exists at linalg.rs:180-183. Copy it.
```

Change A alone may close most of these. **Apply Change A first, re-probe, and only then add guards
where a site is still reachable** — a guard that can no longer fire is the vacuous-check problem
RFC-117 warned about.

## 6. Change C — the sign flip

`slice.rs:301,302,311,320,335,336` cast `usize` to `isize`. Any value ≥ 2^63 becomes negative, and
negative means "from the end" (RFC-088).

```text
replace the six `as isize` with isize::try_from, erroring on failure
delete the comment at slice.rs:332-333, which tells the next reader this hazard
    is already handled — it is not, and a false reassurance is worse than silence
```

**This is the most damaging finding for the project's positioning**, whatever its severity rating: a
library whose thesis is legible failure returned a plausible-looking row instead of an error.

## 7. Change D — `Tensor::new`'s rustdoc, and a guard against recurrence

A `pub(crate)` helper sits between the doc block and the `pub fn new` it was written for, so Rust
bound the docs to the helper (E7). Move `panic_if_dynamic` out from between them.

Then add `missing_docs = "deny"` so it cannot recur.

```text
IF the deny surfaces gaps beyond Tensor::new — the audit says there are none in
194 items — REPORT THE LIST AND STOP. Do not fix them here. This is a correctness
patch, not a documentation sweep, and a widening diff is how a patch stops being
one.
```

## 8. Change E — the invariant, asserted

`shape.iter().product() == data.len()` is the crate's one global invariant. E4 shows it can be broken
through a public `Result` API, and ~31 `Tensor { .. }` construction sites assert it only by
convention.

Add a **debug-only** assertion and call it from those sites. Under `cfg(debug_assertions)` it costs
release builds nothing and turns all 772 tests plus 133 doctests into invariant checks.

**Do not make it a release-mode check.** That is a behaviour change, not a bug fix, and would exceed
patch scope.

## 9. What this RFC deliberately excludes, and why

```text
try_add / try_sub / try_mul / try_div
    NEW PUBLIC API. RFC-094 §4.1 excludes it from a patch, so it cannot ride
    0.46.2 without breaching the clause RFC-120 was written to keep honest.
    Deferred to 0.47.0.
    This is NOT a judgement that it matters less. The audit rates F-7 the
    highest-LIKELIHOOD finding in the report — an ordinary 2000x1000 tensor
    trips it. The ordering here is release mechanics, not risk.

enforcing max_parse_bytes    a behaviour change, and its rustdoc already
                             discloses that it is unenforced -> minor
performance work (P-1, P-2)  changes summation order -> `Changed` -> minor
property testing (M-2)       dev-dependency only, no release; its own RFC
the documentation batch      docs/src/** reaches no package
```

## 10. Scope

### Out of scope — a diff touching these is a defect

```text
re-rejecting zero-sized dimensions        RFC-111 stands (§4)
any new public API                        §9
CHANGELOG.md, Cargo.toml version, pins    the release RFC owns those
docs/src/**                               separate cycle
ROADMAP.md, rfcs/**                       records
```

## 11. Risks

```text
R1  Re-rejecting zero dimensions to make the abort go away. That reverts RFC-111
    and breaks slicing. The bound is per-dimension; the zero stays legal.
R2  Adding guards at sites Change A already closed — vacuous checks (§5).
R3  Making the invariant assertion release-mode. Behaviour change, exceeds patch
    scope (§8).
R4  Letting missing_docs = "deny" widen this into a doc sweep (§7).
R5  Adding try_* "while we are here". It is the one thing §9 forbids.
R6  Trusting this RFC's site list instead of deriving it (§5).
R7  Treating this as authorizing a release. RFC-128 prepares 0.46.2; the tag and
    the publish are separate owner authorizations.
```

## 12. Acceptance criteria

```text
[ ] E2-E5 re-derived and reported before editing
[ ] a degenerate shape is REJECTED at construction, from both from_json and try_new
[ ] zero-sized dimensions still work — RFC-111's own tests pass UNMODIFIED
[ ] no operation on any constructible tensor aborts the process; the E3 probe
    now returns an Err or a valid result
[ ] try_matmul cannot return a Tensor whose shape.product() != data.len()
[ ] slice().index(usize::MAX) returns Err, not the last row
[ ] slice.rs:332-333's false comment deleted
[ ] Tensor::new renders with docs; missing_docs = "deny" added; any further gaps
    REPORTED, not fixed
[ ] the debug-only invariant assertion present and called from the construction
    sites; cfg(debug_assertions) only
[ ] a regression test for each of E2, E3, E4 and E5
[ ] NO new public API — asserted against the public-api surface
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] no version bump, no tag, no publish
```

## 13. What this does not fix

```text
- F-7, the missing try_ arithmetic — the highest-likelihood finding (§9)
- max_parse_bytes, still documented and unenforced
- the 10-65x performance findings
- the absence of property testing, which is what would have caught THIS
- the documentation batch, SECURITY.md, CONTRIBUTING.md
```

**And it does not fix the reason this was missed.** A 772-test, example-based suite cannot see an
invariant nobody wrote down as a property. RFC-111 was reviewed, approved, and shipped by a process
that is careful by every measure this project has — and the bound it removed was implicit, so nothing
had a name to miss. Property testing is the change that addresses that, and it is deliberately a
separate RFC so this one can ship immediately.
