# RFC-100: `Display` for `Tensor`

**Status:** **Accepted** 2026-08-04 — implementation authorized under
[the handoff](../handoffs/100-display-for-tensor-handoff.md). §5.4's open question — whether
`{:#}` means untruncated — was **not** resolved at acceptance and is carried into the handoff as
an implementer decision to report, not to assume
**Target:** core `matten`; a public API addition, so a minor release when it ships
**Theme:** Decide the formatting contract that has been deferred since `0.1.0`, and stop the third-party duplication it caused
**Related:** RFC-002, RFC-020, RFC-078 §4.1, RFC-087 §6, RFC-095, RFC-096, RFC-097, RFC-099

---

## 1. Summary

Implement `Display` for `Tensor`, rendering rank ≤ 2 as an aligned grid.

**What has been deferred is the contract, not the code.** `compatibility.md` says so in as many
words — *"Formatting contract deferred to a future design decision"*. Writing a formatter is an
afternoon; committing permanently to what `{}` prints is the decision. §5 is therefore the substance
of this RFC and §7 is nearly an afterthought.

## 2. The cost of the deferral is now visible in three places

Grid rendering exists three times, written independently over four days because core offers nothing:

```text
tools/matten-playground/src/render.rs                   render_matrix / render_row
tools/matten-report/src/render/markdown/grid.rs         render_matrix_block(format_cell)
crates/matten/examples/57_visual_shape_axis_summary.rs  render_matrix / render_row
```

**All three can reach core** — `matten-report` and `matten-playground` both depend on `matten` by
path, and example 57 lives inside the crate. So the duplication is a real consequence of the missing
contract, not an accident of packaging.

**How much this actually fixes, stated honestly.** Two of the three collapse to `Display`. The report
tool does not fully: its `render_matrix_block` takes a `format_cell` closure because two of its sites
render `{:.3}` with the `-0.000` clamp while the rest use `{:?}`. A fixed `Display` cannot serve
both. So this removes the duplication for the default case and leaves the report tool a narrower
reason to keep its own path — unless core also exposes a cell-format parameter, which §8 rejects as
public surface nobody has asked for.

## 3. What exists today

```text
Debug   Tensor(shape=[2, 3], data=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])     single line
        governed by RFC-020, which lists "Debug output for tensor" as a diagnostic event
Display none
```

`Debug` is **not** changed by this RFC (§8). RFC-020 owns it, and a developer-facing single-line form
is worth keeping distinct from a human-facing one.

## 4. Ecosystem precedent, checked rather than assumed

RFC-078 §4.1 says to match the ecosystem default for a function of this name. So I ran it:

```text
ndarray 0.17, a [2,3] of f64
  Display   [[1, 2, 3],
             [4, 5, 6]]
  Debug     [[1.0, 2.0, 3.0],
             [4.0, 5.0, 6.0]], shape=[2, 3], strides=[3, 1], layout=Cc, const ndim=2
```

Two things follow. **Multi-line `Display` is normal** for array types — worth stating because
multi-line `Display` is unusual in Rust generally, and a reviewer might object on that ground alone.
NumPy's `str()` behaves the same way.

**And the ecosystem drops the `.0`.** That is the one place matching it would be wrong — see §5.2.

## 5. The contract

### 5.1 Shape of the output

Recommendation: **the three existing formatters' style**, not ndarray's brackets.

```text
rank 0   3.5
rank 1   1.0 2.0 3.0
rank 2   1.0 2.0 3.0
         4.0 5.0 6.0
rank >2  see §5.3
```

Space-separated, per-column right-aligned, no brackets and no commas. Reasons: it is what this
project already renders in three places and on two deployed pages, so adopting brackets now would
make the book disagree with the library; and RFC-095's amended §6 test — *does the rendering encode a
value as something other than that value* — is satisfied either way, so the tie breaks on consistency
with what readers already see.

### 5.2 Float format — diverging from the ecosystem, deliberately

**`{:?}` per cell, so a whole number renders `1.0` and not `1`.**

ndarray's `Display` prints `1`. Matching it would reintroduce **exactly** the defect RFC-096's C1
corrected two days ago: `matten`'s only element type is `f64`, and a grid reading `1 2 3` tells a
reader they are looking at integers. That review found the contradiction visible within one screen of
output, where a grid said `1 2 3` and an axis line below said `[2.5, 3.5, 4.5]`.

RFC-078 §4.1's ecosystem rule is real, and this diverges from it knowingly, on the ground RFC-087 §6
gives: match the ecosystem where a divergence would be silent, diverge where the ecosystem's choice
would mislead. `1` for an `f64` misleads.

### 5.3 Rank > 2

Recommendation: **fall back to the flat form**, as RFC-095 §6 and RFC-096 decided for the same
question.

```text
rank >2   shape=[2, 2, 2] values=[1.0, 2.0, …]
```

A 3-D tensor has no honest 2-D arrangement, and inventing a reading order is worse than a list. This
is a boundary, not a deferral — the same wording both prior RFCs used, and the same answer, so the
three surfaces stay consistent.

### 5.4 Truncation

Recommendation: **truncate, marked**, at the constants `tools/matten-report` already fixed —
`MAX_DISPLAY_COLUMNS = 12`, `MAX_TENSOR_PREVIEW_VALUES = 12`.

`Display` on a `[1000, 1000]` tensor must not emit a million numbers into someone's terminal. NumPy
and ndarray both truncate; this is the ecosystem default and §4's rule applies with no conflict.

**Open question for review:** whether `{:#}` (alternate) should mean *untruncated*. It is a natural
escape hatch and costs one branch. I lean yes but have no precedent to point at.

### 5.5 Dynamic tensors

Recommendation: **render them**, using `Element`'s own formatting, same grid.

A dynamic tensor is precisely the case where a human wants to look at the data — that is what the
feature is for. Refusing to display it, or printing a placeholder, would make `Display` useless in
the one situation it is most wanted. This is the only part of the contract with no existing
implementation to copy, since all three formatters take `&[f64]`.

## 6. Why this is safety-oriented

By the test that split RFC-098: **does this fix something wrong today, or add something absent?**

It fixes something wrong. Three formatters exist that can drift apart, and drift between them would
be invisible — the playground's tests assert its own output, the example asserts its own, the report
tool asserts its own, and **nothing compares them to each other**. That is the same class as the four
governance amendments overwritten on 2026-07-30: three copies of one truth, no check that they agree.

## 7. Scope

### In scope

```text
impl Display for Tensor, per §5
tests asserting the exact rendered string for each rank, including padding
playground, report tool and example 57 migrated to it where the contract fits (§2)
docs: reference/math.md or a formatting section, public-api-snapshot.md, compatibility.md's
  "Display for Tensor | Not implemented" row
```

### Out of scope — a diff touching these is a defect

```text
Debug — RFC-020 owns it and it stays exactly as it is
a cell-format parameter, a Formatted wrapper, or any configurable formatter API
the report tool's two {:.3} sites; they keep their own path (§2)
serde, JSON, CSV, or any other serialization
```

## 8. Non-goals

```text
matching ndarray's bracket syntax (§5.1) or its float rendering (§5.2)
a public formatting API beyond the Display impl itself — RFC-002 minimalism
making Display a stable parsing target; it is for humans, and §5.4 truncates
```

## 9. Acceptance criteria

```text
[ ] Display renders rank 0/1/2 per §5.1 and rank >2 per §5.3
[ ] whole numbers render "1.0", never "1" (§5.2) — asserted
[ ] truncation at 12, marked, with a test at the boundary (§5.4)
[ ] dynamic tensors render (§5.5) — asserted under --features dynamic
[ ] Debug output is UNCHANGED, byte-for-byte — asserted, not assumed
[ ] the playground and example 57 use Display and lose their local formatters
[ ] compatibility.md's "Display for Tensor | Not implemented" row is corrected
[ ] all eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish; the version bump is a separate decision under RFC-094
```
