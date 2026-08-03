# RFC-096 Example Grid Rendering: Implementation Handoff

**Status:** Issued 2026-08-02. Implementation authorized under RFC-096, accepted the same day.
**Design authority:** `rfcs/accepted/096-example-grid-rendering.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Make `crates/matten/examples/57_visual_shape_axis_summary.rs` render rank-2 tensors as an aligned
grid, so its Reshape section stops printing two identical value lists.

**One file. One function.** `print_tensor_line` at line 11 is the only place the flat form is
produced, confirmed by grep across all 78 examples. If your diff touches a second example, stop and
report.

## 2. The requirement that makes this design legal

Read RFC-096 §4's subsection before writing the formatter. In short:

```text
The playground's formatter has NINE unit tests. This one can have none — verified:
a #[test] injected into an example and run with `cargo test -p matten --examples`
executes 0 tests, because examples build as binaries.

CI runs this example, so a panic is caught. A MIS-ALIGNED GRID IS NOT. It renders,
the example exits 0, CI goes green.

=> The example MUST assert its own rendered block, padding included, before printing.
```

This is not a nicety bolted onto the task; it is the condition under which keeping the formatter
local was accepted at all. An implementation that renders correctly but does not assert has not done
the job.

The convention already exists in this file — it asserts shapes and values at lines 29–30 and 38, and
49 of 56 core examples contain assertions. Build the string, `assert_eq!` it against the expected
block, then `println!` it.

```rust
let block = render_block("[2, 3] input", &a);
assert_eq!(block, "[2, 3] input     shape=[2, 3]\n  1  2  3\n  4  5  6");
println!("{block}");
```

At minimum one assertion must cover a rank-2 grid and one a rank-1 row.

## 3. What to render

```text
rank 2   aligned grid, right-aligned columns
rank 1   a single row
rank 0   the scalar
rank >2  the existing flat `shape=… values=…` line — keep the arm even though no
         tensor in this file reaches it
```

Keep the `meaning` lines. They explain; the grid now demonstrates. The two were never in competition.

## 4. Do NOT copy the playground's constants

RFC-096 §5 is explicit, and this is the instruction most likely to be "helpfully" ignored:

```text
MAX_DISPLAY_COLUMNS = 12         every tensor here has <= 3 columns. Dead code.
MAX_TENSOR_PREVIEW_VALUES = 12   same.
format_fixed_value -> {:.3}      turns 1.0 into 1.000 — three digits of noise per
                                 cell, on hand-chosen teaching values
```

Use **natural float rendering** and align on width. The playground needs `{:.3}` because it formats
arbitrary typed input; this file has hand-chosen values and a different medium. RFC-095 required the
two web surfaces to agree because RFC-093 §8 will put them on one site; a terminal example is not on
that site and is not required to match.

If that divergence looks like an inconsistency to fix, it is not — §5 records it as deliberate.

## 5. Scope

```text
IN    print_tensor_line and its call sites, inside this one example
OUT   any public API anywhere — no new `pub fn`, no Debug/Display change (RFC-020)
OUT   the other 77 examples
OUT   tools/matten-playground, tools/matten-report
OUT   docs/src/** — RFC-095 finished the page
OUT   version bump, tag, publish
```

## 6. Verification

```bash
cargo run -p matten --example 57_visual_shape_axis_summary     # the assertions run here
cargo check --workspace --examples --all-features
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh          # rule 002 §8
git diff --name-only -- crates/    # expect ONLY the one example file
```

`cargo run` is the real test here — it is what executes your assertions. Running it and seeing
`57_visual_shape_axis_summary: OK` is the check, not a formality.

## 7. Known pitfalls

```text
- rendering correctly but not asserting — fails §2, the condition the design rests on
- asserting shapes/values instead of the RENDERED BLOCK; the old assertions at
  lines 29-30 already cover shapes, and they are not what can silently break
- copying {:.3} or the truncation constants from the playground (§4)
- adding a `pub fn` anywhere to share the formatter — that is option (a), rejected
- touching a second example
- dropping the trailing "57_visual_shape_axis_summary: OK" line
```

## 8. What the review request must report

```text
- the example's full output, verbatim, before and after
- the assertions added, quoted — showing they compare rendered blocks with padding
- confirmation that no `pub fn` was added: git diff | grep '^+.*pub fn' is empty
- git diff --name-only -- crates/ showing exactly one file
- full gate output, seven guards, check-doc-code.sh under -D warnings
- confirmation that no tag was created and nothing was published
```

## 9. Review stop

Stop after committing. Report, and the high-capability model reviews.

Unlike RFC-093 and RFC-095, **this one does not deploy anything** — it changes a published crate's
example, which reaches users only at the next release, and RFC-094 leaves that a separate decision.
Pushing is safe.
