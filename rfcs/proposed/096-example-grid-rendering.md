# RFC-096: Grid Rendering in the Shape and Axis Example

**Status:** Proposed
**Target:** `crates/matten/examples/57_visual_shape_axis_summary.rs`; no API change, no version, no release
**Theme:** Apply RFC-095's fix where more people see it — a shipped example, not only the web page
**Related:** RFC-002, RFC-020, RFC-021, RFC-043, RFC-093 §6, RFC-095, RFC-094

---

## 1. Summary

Render rank-2 tensors as an aligned grid in `57_visual_shape_axis_summary`, the same defect and the
same fix as RFC-095, applied to a shipped example instead of the browser page.

**The formatter stays local to the example.** §4 argues that, rather than adding a public helper to
core or importing the playground's.

## 2. The same defect, in published code

RFC-095 was prompted by the owner using the deployed playground and finding that Reshape printed two
identical value lists. The shipped example still does exactly that:

```text
== Reshape ==
[2, 3] input     shape=[2, 3] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
[3, 2] view      shape=[3, 2] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
meaning         row-major values stay in the same order
```

The `meaning` line asserts what the output should have shown. A grid shows it:

```text
[2, 3] input     shape=[2, 3]      [3, 2] view      shape=[3, 2]
  1  2  3                            1  2
  4  5  6                            3  4
                                     5  6
```

This reaches further than the page did. The example runs from a `cargo run` in an installed crate;
the playground needs a reader to find a URL. RFC-021 makes the example suite a teaching path with a
quality gate, and RFC-095 §8 recorded this as a separate decision precisely because shipped examples
meet a different bar than a documentation page.

## 3. Scope is one function

```text
crates/matten/examples/57_visual_shape_axis_summary.rs   74 lines
  print_tensor_line()  line 11 — the ONLY place the flat form is produced
  every tensor in the file is at most 3 columns
```

A repository-wide grep for the `shape=… values=…` form across all 78 example programs returns this
one file. There is no sweep to do, and §7 records the guard that keeps it that way.

## 4. Where the formatter lives — the actual decision

Three options were considered. **The recommendation is (c).**

**(a) A public helper in core `matten`.** Rejected. It permanently adds public surface for a
presentation concern, which RFC-002's minimalism rule exists to resist, and it would be core's first
formatting API — a category shift, not an increment. RFC-020 already governs tensor `Debug` output;
adding a parallel public formatter alongside it invites the question of which is canonical, for a
benefit one example needs.

**(b) Import the playground's formatter.** Impossible, and would be wrong anyway.
`tools/matten-playground` is workspace-excluded and `publish = false`; a published crate cannot depend
on it, and making it publishable to serve an example would invert the dependency the whole playground
design rests on.

**(c) A local formatter in the example.** Recommended. It is roughly fifteen lines replacing a
six-line helper, in a file whose purpose is to be read.

A self-contained example is also *better teaching*, not merely cheaper: a reader following
`57_visual_shape_axis_summary` to learn how shapes work can see how the alignment is produced instead
of calling into a helper whose behaviour is elsewhere. RFC-043's example-structure policy already
favours readable, self-contained programs.

## 5. It does not inherit the playground's constants, deliberately

RFC-095 §4 required the playground to adopt `tools/matten-report`'s three display constants, because
RFC-093 §8 would put both surfaces on one site where disagreeing would be visible to a reader. **That
reasoning does not extend here**, and copying them anyway would be cargo-culting:

```text
MAX_DISPLAY_COLUMNS = 12        every tensor in this file has ≤ 3 columns. Dead code.
MAX_TENSOR_PREVIEW_VALUES = 12  same. Dead code.
format_fixed_value -> {:.3}     see below — actively worse here
```

**The float format should diverge.** The playground renders `{:.3}`, because it formats arbitrary
user input and needs columns that line up whatever is typed. This example uses hand-chosen values
like `1.0` and `20.0`, where `1.000` adds three digits of noise to every cell for no gain. The example
keeps natural float rendering and aligns on width.

This is a divergence with a reason, stated so a later reader does not "fix" it into consistency: the
two surfaces have different media and different inputs. A terminal example and a web page are not
required to agree byte-for-byte the way two pages on one site are.

## 6. Scope

### In scope

```text
print_tensor_line -> an aligned rank-2 grid; rank 1 a row; rank 0 the scalar
rank > 2 unchanged (no such tensor exists in this file; the arm exists for safety)
the `meaning` lines stay — they explain, and the grid now demonstrates
```

### Out of scope — a diff touching these is a defect

```text
any public API in any crate — no new pub fn, no Debug/Display change (RFC-020)
the other 77 examples — none uses this form (§3); a future one is covered by §7
tools/matten-playground or tools/matten-report
docs/src/** — the page was RFC-095's job and is done
a version bump, tag, or publish
```

## 7. The guard question, answered honestly

There is no guard preventing a future example from printing the flat form again, and this RFC does
**not** add one. Rule 002 prefers a guard to a sweep, so the omission needs a reason rather than
silence.

The reason: a guard here would have to ban a *formatting shape* in example source, and the honest
version of that check — "does this example print a rank-2 tensor without aligning it" — is not
expressible as a grep. A pattern banning `values={:?}` would be trivially evaded by any other
spelling and would fire on legitimate rank-1 and rank-3 output, which correctly uses that form.

What exists instead is cheaper and real: this file is the only occurrence, and RFC-021's example
quality gate already puts new examples through review. Recorded as a known gap rather than papered
over with a guard that would not hold.

## 8. Release consequence, noted not decided

This is the first change to published crate code since `0.42.0`. Under RFC-094 it is neither a
correctness fix nor new public API, so it does not itself trigger a release; it accumulates. It does
mean the pending `crates/matten/README.md` playground link stops being the sole unreleased item.

Whether to cut a release remains RFC-094's decision at the next disposition point, not this RFC's.

## 9. Acceptance criteria

```text
[ ] Reshape's two blocks visibly differ in arrangement — the defect in §2 is gone
[ ] rank 1 renders as a row; rank 0 as the scalar; rank >2 arm present and unchanged
[ ] natural float rendering, NOT {:.3} (§5)
[ ] no new public API: git diff shows no `pub fn` added in any crate
[ ] the example still ends with its "57_visual_shape_axis_summary: OK" line
[ ] cargo run -p matten --example 57_visual_shape_axis_summary succeeds
[ ] all seven guards pass; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] no tag, no publish, no version change
```

## 10. Non-goals

```text
a public formatting API in core — §4(a), rejected
consistency with the playground's float format — §5, diverged deliberately
RFC-093 §8's phase 2 — the owner's stated next preference, and separate
retrofitting the other 77 examples
```
