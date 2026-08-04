# RFC-099 Result-Form `try_matmul` / `try_dot`: Implementation Handoff

**Status:** Issued 2026-08-04. Implementation authorized under RFC-099, accepted the same day.
**Design authority:** `rfcs/accepted/099-result-form-matmul.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Add `try_dot` and `try_matmul` returning `Result<Tensor, MattenError>`; make `dot` and `matmul`
delegate to them.

**This is a robustness change, not a feature.** Nothing new becomes possible. The same inputs succeed,
the same inputs fail, and **every existing message must come out byte-identical**. If your diff makes
any input behave differently, that is a defect, not an improvement.

## 2. The pattern to follow

Already established by RFC-055/056 — copy its shape exactly:

```rust
pub fn try_sum(&self) -> Result<f64, MattenError> { reject_dynamic(self, "sum")?; … }
pub fn sum(&self) -> f64 { self.try_sum().unwrap_or_else(|e| panic!("{e}")) }
```

Validation and logic move into the `try_` form. The panicking form becomes one line.

## 3. The trap — read this before writing anything

`panic!("{e}")` prints the error's `Display`, so the refactor changes user-visible text unless the
error reproduces it exactly. **`dot`'s dynamic guard is bespoke and does NOT match the shared helper.**
Both strings captured from real runs, not read off the source:

```text
what dot() panics with TODAY — must be preserved
  matten unsupported error in dot/matmul: not supported on dynamic tensors;
  call try_numeric() on each operand first

what reject_dynamic(self, "dot") would produce — WRONG here
  matten unsupported error in dot: dot is not supported on dynamic tensors;
  call try_numeric() first
```

**Do not call `reject_dynamic` in `try_dot`.** It is the obvious move and it silently rewords a
message. Construct `MattenError::Unsupported` directly with operation `"dot/matmul"` and the existing
message text.

The existing wording is also the better one, so this is preservation rather than compatibility
paperwork: `dot/matmul` tells a caller both entry points share one guard, and "on each operand first"
is the right advice for a binary operation.

The four shape-failure panics in `matmul_dispatch` carry the same requirement — same text, now
delivered as an `Err`.

## 4. Where the net already is

You do not need to invent verification for the shape messages. It exists:

```text
tools/matten-playground   RFC-095 C1's catch_unwind tests compare the playground's
                          reproduced strings against matten's REAL panic payload, live,
                          every run — they fail the moment a message drifts
crates/matten/examples/57_visual_shape_axis_summary.rs   asserts its rendered output, run in CI
```

Run both. If they pass unmodified, the shape messages survived. **If you find yourself editing either
to make them pass, stop** — that is the net catching you, not a test needing an update.

**The dynamic-guard message is the gap:** nothing currently pins it, which is why §3 quotes it
verbatim. Add a test that asserts it, so the next refactor has the same net this one lacked.

## 5. Scope

```text
IN    try_dot, try_matmul; dot/matmul delegating; a test pinning the dynamic message
IN    docs/src/reference/math.md, docs/src/reference/public-api-snapshot.md
OUT   batched matmul or ANY change to supported rank combinations (RFC-098, archived)
OUT   new error variants — nothing new is needed
OUT   try_ forms for anything else; the other 41 exist
OUT   performance work, or "while I am here" tidying of adjacent code
OUT   a version bump, tag, or publish
```

## 6. Verification

```bash
cargo test --workspace --all-targets
cargo test -p matten --no-default-features                    # the non-dynamic build
cargo test -p matten --no-default-features --features dynamic # the guard's own path
cargo test --manifest-path tools/matten-playground/Cargo.toml # RFC-095 C1's live tests
cargo run -p matten --example 57_visual_shape_axis_summary
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh
mdbook build docs
```

Both feature profiles matter: the dynamic guard is `#[cfg(feature = "dynamic")]`, so a
no-default-features build does not exercise it at all.

## 7. Known pitfalls

```text
- using reject_dynamic in try_dot (§3) — the single most likely mistake
- "improving" a message while moving it; every string is frozen
- editing the playground's or example 57's assertions to make them pass (§4)
- adding a rank combination, or changing which ones are supported
- forgetting the non-dynamic build, where the guard compiles out
- deprecating dot/matmul; they stay, exactly as RFC-055's panicking forms did
```

## 8. What the review request must report

```text
- try_dot/try_matmul signatures, and the one-line bodies of dot/matmul
- the dynamic-guard message BEFORE and AFTER, quoted, showing byte-identity
- confirmation that reject_dynamic is NOT used for dot/matmul
- the four shape-failure messages before and after
- the new test pinning the dynamic message
- proof the playground's live tests and example 57 pass UNMODIFIED
- both feature profiles tested
- full gate output; git diff --name-only showing nothing unexpected
- confirmation that no tag was created and nothing was published
```

## 9. Review stop

Stop after committing. Report, and the high-capability model reviews.

This is a **published-crate** change, so unlike RFC-095/096/097 it deploys nothing on push — it
reaches users only at the next release, which RFC-094 leaves as a separate decision. It would,
however, be the first RFC-094 minor trigger since `0.42.0`.
