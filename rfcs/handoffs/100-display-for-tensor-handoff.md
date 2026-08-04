# RFC-100 `Display` for `Tensor`: Implementation Handoff

**Status:** Issued 2026-08-04. Implementation authorized under RFC-100, accepted the same day.
**Design authority:** `rfcs/accepted/100-display-for-tensor.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Implement `Display` for `Tensor` per RFC-100 §5, then migrate the duplicate formatters to it.

**The contract is the work.** The formatter itself is short. What matters is that `{}` will print the
same thing in five years, so every decision in §5 is a permanent commitment and none of them should
be re-litigated in code.

## 2. The one decision NOT made for you

RFC-100 §5.4 leaves open whether `{:#}` (alternate) should mean **untruncated**. It was deliberately
not resolved at acceptance.

**Decide it, implement your decision, and report which you chose and why.** Do not treat the RFC's
"I lean yes" as an instruction — it was a lean, not a decision. If you implement it, `{:#}` must be
tested; if you do not, say so, and the escape hatch stays unavailable until someone asks for it.

Everything else in §5 is settled and is not open.

## 3. The float format is the trap

**`{:?}` per cell. A whole number must render `1.0`, never `1`.**

This is the one place RFC-100 knowingly diverges from the ecosystem, and it will look wrong to anyone
who checks ndarray:

```text
ndarray 0.17 Display   [[1, 2, 3],       <- drops the .0
                        [4, 5, 6]]
matten Display         1.0 2.0 3.0       <- required
                       4.0 5.0 6.0
```

`matten`'s only element type is `f64`. A grid reading `1 2 3` tells a reader they are looking at
integers, which is the defect RFC-096 C1 corrected two days ago. If you find yourself reaching for
`Display`/`to_string()` on the cell because it looks cleaner, that is the mistake this paragraph
exists to stop.

## 4. What to render — settled, from §5

```text
rank 0   the scalar                       3.5
rank 1   one row                          1.0 2.0 3.0
rank 2   aligned grid, per-column         1.0 2.0 3.0
         right-aligned                    4.0 5.0 6.0
rank >2  the flat form, UNCHANGED         shape=[2, 2, 2] values=[1.0, …]
```

Space-separated, right-aligned per column. **No brackets, no commas** — do not adopt ndarray's
syntax; the book and two deployed pages already show this style, and changing it would make the
library disagree with its own documentation.

Rank > 2 is a boundary, not a deferral. RFC-095 §6 and RFC-096 §6 drew the same line for the same
reason: a 3-D tensor has no honest 2-D arrangement.

**Truncate at 12**, marked, using the constants `tools/matten-report/src/render/common.rs` fixed
(`MAX_DISPLAY_COLUMNS`, `MAX_TENSOR_PREVIEW_VALUES`). `Display` on a `[1000, 1000]` must not emit a
million numbers.

**Dynamic tensors render** (§5.5), using `Element`'s own formatting. This is the only part with no
existing implementation to copy — all three current formatters take `&[f64]`.

## 5. `Debug` must not move

RFC-020 owns `Debug` output for tensor and this RFC does not touch it.

```text
Tensor(shape=[2, 3], data=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
```

**Assert it byte-identical**, do not eyeball it. It is the kind of thing that changes by accident
when you are editing the impl block next to it.

## 6. The migration, and what it will not reach

After `Display` exists, migrate:

```text
crates/matten/examples/57_visual_shape_axis_summary.rs   drop its local formatter
tools/matten-playground/src/render.rs                    drop its local formatter
```

**The report tool is different and mostly stays.** Its `render_matrix_block` takes a `format_cell`
closure because two `mlprep-standardization` sites render `{:.3}` with the `-0.000` clamp while the
rest use `{:?}`. A fixed `Display` cannot serve both, and RFC-100 §8 explicitly rejects adding a
configurable formatter to core to close that gap. Leave it alone unless a site uses the plain default,
in which case that site may move.

**Both migrated files have tests asserting exact output.** If `Display` is right, they pass with only
the formatter call changed. **If you find yourself editing an expected string, stop** — either
`Display` disagrees with the contract, or you have changed rendering that RFC-095/096 fixed
deliberately.

`scripts/check-report-demos.sh` will fail if any report demo's output changes. That is the guard
working, not a test to update.

## 7. Verification

```bash
cargo test -p matten --no-default-features
cargo test -p matten --no-default-features --features dynamic   # §4's dynamic rendering
cargo test --workspace --all-targets
cargo test --manifest-path tools/matten-playground/Cargo.toml
cargo run -p matten --example 57_visual_shape_axis_summary
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh
mdbook build docs
```

## 8. Known pitfalls

```text
- Display/to_string() per cell instead of {:?}, so 1.0 prints as 1 (§3)
- adopting ndarray's [[1, 2], [3, 4]] syntax (§4)
- rendering rank 3 as stacked blocks (§4 — a boundary, not a gap)
- no truncation, so a large tensor floods the terminal (§4)
- changing Debug while editing the impl block beside it (§5)
- editing a migrated file's expected strings to make it pass (§6)
- adding a configurable formatter API to serve the report tool (§6, RFC-100 §8)
- treating §2's open question as decided
```

## 9. What the review request must report

```text
- the rendered output for rank 0, 1, 2, >2, dynamic, and a truncated case, verbatim
- YOUR DECISION on {:#} and the reasoning (§2)
- proof Debug is byte-identical
- which formatters were removed, and confirmation the report tool's two {:.3} sites
  still use their own path
- proof the migrated files' expected strings were NOT edited
- full gate output, eight guards, check-doc-code.sh under -D warnings
- confirmation that no tag was created and nothing was published
```

## 10. Review stop

Stop after committing. Report, and the high-capability model reviews.

This is a **published-crate** change: pushing deploys nothing, and it reaches users at the next
release. It is also new public API, so it joins `try_dot`/`try_matmul` as RFC-094 minor-trigger
content.
