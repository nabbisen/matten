# RFC-093 Browser Shape Playground: Implementation Handoff

**Status:** Issued 2026-08-01. Implementation authorized under RFC-093, accepted the same day.
**Design authority:** `rfcs/accepted/093-browser-shape-playground.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Add an interactive page to the book where a reader enters shapes and values and watches broadcasting,
reshape, the four axis reductions and `matmul` resolve, with plain-language glosses.

**No published crate is touched.** If `git diff --name-only` shows anything under `crates/`, stop and
report — that is a defect, not a judgement call.

Read RFC-093 §6 before writing any rendering code. The lock is **text only, no pixels that represent
data**. It is the single most likely thing to be got wrong here, because each step past it looks
reasonable.

## 2. Structure

Follow the existing local-tool precedent exactly — `tools/matten-report`, `tools/matten-migrate` and
`benchmarks/` are all workspace-excluded with `publish = false` and git-ignored locks.

```text
tools/matten-playground/Cargo.toml    publish = false; matten by path; wasm-bindgen
tools/matten-playground/src/lib.rs    thin binding: parse -> call matten -> format
docs/src/playground.md                the page + a SUMMARY.md entry
docs/src/playground/                  generated wasm + js glue land HERE (see §4)
docs/book.toml                        [output.html] additional-js
.github/workflows/docs.yaml           build wasm before `mdbook build`
```

Three registration steps are easy to miss and each has an existing precedent to copy:

```text
1. root Cargo.toml  -> add "tools/matten-playground" to workspace.exclude
                       (alongside benchmarks, tools/matten-report, tools/matten-migrate)
2. .gitignore       -> add /tools/matten-playground/Cargo.lock, with the same comment
                       shape the other three carry
3. docs/src/SUMMARY.md -> add the page, or mdBook will not render it at all
```

Miss (1) and the wasm crate joins the workspace, dragging `wasm-bindgen` into the shared lockfile.
Miss (2) and a second lockfile gets tracked, which the other three exclusions exist to prevent.

## 3. Keep the logic in Rust

RFC-093 §7 records that the guard estate **cannot check an interactive page** — every guard is grep-
or compile-based. The only available mitigation is where you put the logic.

```text
Rust (tested, fails to compile when the API moves):
  parsing the shape/values input, calling matten, formatting the output string,
  turning a MattenError into its display text

JavaScript (unguarded — keep it as close to nothing as you can):
  read the input box, call the wasm export, write the result into the DOM
```

A reviewer will look at the JS line count. If it contains arithmetic, shape logic, or error-message
construction, that logic is in the wrong language.

## 4. Where the build artifacts go

mdBook copies non-Markdown files under `src/` into the output verbatim. Put the generated `.wasm` and
its JS shim in `docs/src/playground/` so they ship with the book.

**Decide and report:** whether those generated artifacts are git-ignored or committed. Recommendation
is **git-ignored**, built in CI — but that means a local `mdbook build` produces a page whose script
is missing, so say so in the page itself or in `docs/src/contributing/`. Do not leave a reader to
discover a dead page and assume it is broken.

`docs.yaml` runs with `defaults.run.working-directory: docs`. A wasm build step therefore needs an
explicit `working-directory` or a `--manifest-path`; a bare `cargo build` in that job will not find
the tool.

## 5. What the page must do

```text
broadcasting        two shapes -> result shape + the "b repeats across rows" style gloss
reshape             shape + target -> result, or the error
axis reductions     sum / mean / min / max over a chosen axis
matmul              [m,n] x [n,p] -> [m,p], and the rejection when it does not
errors              show the REAL MattenError message, not a generic "invalid input"
```

That last one is load-bearing. A rejected operation teaches as much as an accepted one, and the error
messages are a deliberate product of RFC-005 and RFC-020 — surfacing them is the point, not a
fallback.

Mirror the vocabulary of `crates/matten/examples/57_visual_shape_axis_summary.rs`. It is the existing
answer to "how does this project explain a shape", and the page should not invent a second one.

## 6. Verification

```bash
cargo build -p matten --target wasm32-unknown-unknown --no-default-features   # must still build
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh    # rule 002 §8 — the CI environment
mdbook build docs
cargo clippy --manifest-path tools/matten-playground/Cargo.toml -- -D warnings
```

Two guards deserve specific attention:

- **`check-published-dependency-isolation.sh`** is the one that proves the central claim of this RFC.
  If `wasm-bindgen` reaches a published crate it will fail — and it will be right, and the design
  will be wrong. Do not modify it to pass.
- **`check-doc-code.sh`** compiles *and runs* every non-ignored ```rust fence in `docs/src/`. If
  `playground.md` contains Rust fences they must compile against the real API or be marked
  ```rust,ignore. Run it with `RUSTFLAGS="-D warnings"`, per rule 002 §8 — that exact omission
  broke CI on 2026-08-01.

```bash
git diff --name-only -- crates/
# expect EMPTY
git diff --name-only | grep -E '^(CHANGELOG|Cargo.lock)'
# expect EMPTY — no version change, no workspace lock change
```

## 7. Known pitfalls

```text
- drawing anything (bars, axes, colour scales, SVG, canvas) — RFC-093 §6 forbids it
- forgetting workspace.exclude, pulling wasm-bindgen into the shared lockfile
- forgetting the SUMMARY.md entry, so the page silently never renders
- a bare `cargo build` in docs.yaml, which runs with working-directory: docs
- shape or error logic drifting into the JS, where nothing can test it
- running check-doc-code.sh without RUSTFLAGS and shipping a CI failure
- touching a published crate for convenience
```

## 8. What the review request must report

```text
- git diff --name-only -- crates/  showing EMPTY
- the JS line count, and confirmation that no shape or error logic lives there
- the wasm target build output
- full gate output, all seven guards, with check-doc-code.sh run under -D warnings
- the artifact decision from §4 (ignored vs committed) and where it is documented
- a description of what the page renders, sufficient to confirm §6's text-only lock
- confirmation that no tag was created and nothing was published
```

## 9. Review stop

Stop after committing. Report, and the high-capability model reviews before anything is deployed. The
book deploy is triggered by a push to `main`, so this is one of the few changes where landing the
commit *is* the release — flag anything you are unsure about before committing rather than after.
