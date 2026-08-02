# RFC-093: Browser Shape Playground

**Status:** **Accepted** 2026-08-01 — implementation authorized under
[the handoff](../handoffs/093-browser-shape-playground-handoff.md). First RFC to occupy
`accepted/`, the folder adopted by RFC-092 the same day. Phase 2 (§8) remains unauthorized
**Target:** Documentation-site feature; no published-crate change, no version, no release
**Theme:** Make the shape reasoning the `visual_*` examples print interactive, in the book that is already deployed
**Depends on:** RFC-022, RFC-030, RFC-049 §B1, RFC-070
**Related:** RFC-042 (scope-lock precedent), RFC-069, RFC-071, RFC-073, RFC-092

---

## 1. Summary

Add an interactive page to the `matten` book where a reader enters shapes and values and watches
broadcasting, reshape, axis reductions and `matmul` resolve — with the same plain-language glosses the
`visual_*` examples already print (*"b repeats across rows"*, *"row-major values stay in the same
order"*).

It is powered by a WebAssembly build of core `matten`, in a **workspace-excluded, `publish = false`**
binding crate. **No published crate changes. No public API is committed. Nothing new reaches
crates.io.**

Phase 2 — routing the report tool's HTML output through the same site — is **recorded in §8 and not
authorized here**, at the owner's direction to take the playground first and both eventually.

## 2. Why this is cheap, and why that is a fact rather than a hope

Two enabling conditions were verified before this RFC was written, not assumed:

```text
1. Core matten already runs in a browser.
   cargo build -p matten --target wasm32-unknown-unknown --no-default-features   -> builds
   ... --features dynamic                                                        -> builds
   cargo build -p matten-stats --target wasm32-unknown-unknown                   -> builds

   The only std::fs uses in core are behind the `json` and `csv` features
   (ops.rs load_json / load_csv). The lean core has nothing browser-hostile in it.

2. The project already deploys a web site.
   .github/workflows/docs.yaml builds the mdBook and publishes docs/book to
   GitHub Pages on every push. mdBook supports `additional-js` in book.toml
   (confirmed against the installed mdbook 0.5.4).
```

Neither is an accident. The first is what `#![forbid(unsafe_code)]` and a near-zero dependency graph
buy; the second has been in place since the book existed. **This RFC adds a page to a site that
already ships, running a library that already runs there.**

## 3. Why this is not the thing RFC-070 declined

RFC-070 audited public visualization and reporting and authorized none of: a public `matten-report`
crate, a `matten-viz` crate, a reusable renderer API, a public report model API, public JSON/SVG or
Vega-Lite output, notebook or browser integration, or core `Tensor` visualization methods.

Every one of those is a **commitment to public API surface**. This RFC commits to none:

| RFC-070 declined | This RFC |
|---|---|
| public `matten-report` / `matten-viz` crate | no new published crate; binding crate is `publish = false` |
| reusable renderer API, public report model | no library surface at all — only a compiled artifact is deployed |
| public JSON / SVG / Vega-Lite output | none; §5 forbids all three |
| core `Tensor` visualization methods | core is untouched |
| notebook / browser integration | **this is the one that overlaps** — see below |

**The overlap is real and must be argued, not stepped around.** RFC-070 declined "browser
integration". Two things distinguish this: it declined browser integration *as a product surface a
user embeds*, in a section otherwise about public crates and APIs; and what is proposed here is a
**page in this project's own documentation**, the same category as the `visual_*` examples, which are
already shipped teaching artifacts. If the reviewer disagrees, that is the finding to raise — this
RFC should be rejected on that basis rather than quietly reinterpreting RFC-070.

## 4. Structure

Follows the established local-tool precedent exactly. Three tools are already workspace-excluded and
`publish = false` — `tools/matten-report`, `tools/matten-migrate`, `benchmarks/` — with their
`Cargo.lock` files git-ignored so the repository keeps one workspace lock.

```text
tools/matten-playground/     new; workspace-excluded, publish = false, own lock git-ignored
  Cargo.toml                 depends on matten by path; wasm-bindgen
  src/lib.rs                 thin binding layer — parse input, call matten, format output

docs/src/playground.md       the page; a Playground section in SUMMARY.md
docs/theme/playground.js     the glue, wired via `additional-js` in docs/book.toml
docs/book.toml               [output.html] additional-js
.github/workflows/docs.yaml  build the wasm artifact before `mdbook build`
```

`check-published-dependency-isolation.sh` already asserts that no published crate carries a
forbidden dependency; `wasm-bindgen` enters only a workspace-excluded crate, so that guard keeps
holding without modification. **If it does not, the guard is right and this design is wrong.**

## 5. Scope

### In scope

```text
broadcasting, reshape, axis reductions (sum/mean/min/max), matmul
shape and value display, plus the plain-language "meaning" gloss
error display for shape mismatches — a rejected op teaches as much as an accepted one
```

### Out of scope — a diff touching these is a defect

```text
plotting, charts, SVG, Vega-Lite, any graphical rendering whatsoever
file upload, data persistence, local storage, cookies, network requests
any change to a published crate, its API, features, dependencies, or version
any public library surface from the playground crate
the report tool's HTML route (§8 — a later phase, not this one)
a tag, a publish, or a release
```

## 6. The scope lock

`matten-viz` is a crate this project has already declined once. The path from "simple interactive
page" to it is short and each step looks reasonable: shapes → a little bar for each value → a chart →
a chart library → a rendering API → the crate RFC-070 refused.

**So the lock is stated as a rule, in the RFC-042 style: the playground renders text, and only text.**
Numbers, shapes, and prose glosses. No pixels that represent data. A change that draws data — bars,
axes, lines, colour scales, an SVG element — is out of scope regardless of how small, and needs its
own RFC that argues against this section by name.

## 7. Risks, stated up front

```text
1. FIRST RUNTIME ARTIFACT SHIPPED TO USERS. Everything else here is text or a local
   tool. A broken page is user-visible in a way a broken guard is not.

2. THE GUARD ESTATE CANNOT CHECK IT. Every guard is grep- or compile-based. The Rust
   side fails to compile when the API moves, which is better than prose — but the JS
   glue is unguarded, and no existing harness can assert that the page behaves. This
   is a real gap, recorded rather than papered over. Mitigation: keep the JS glue
   thin and the logic in Rust, where the test suite already reaches.

3. THE WASM BUILD IS A NEW CI TARGET. docs.yaml gains a toolchain and a build step.
   If the wasm build breaks, the book deploy breaks with it — the failure is loud,
   which is the right direction, but it couples two things that were independent.

4. DRIFT. The page states API behaviour, so it can go stale like any other doc. Less
   than prose can: it calls the real API and fails to compile if that API moves.
```

## 8. Phase 2 — recorded, NOT authorized

The owner's direction was *"playground only is accepted with good reason; finally, both."* The second
part is therefore intent, not scope, and is recorded here so the sequencing is not lost:

```text
Route the report tool's HTML output through the same site, so `matten-report`'s five
demos are readable without a checkout. Requires its own RFC, which must answer:
  - does deploying generated HTML make the report tool a product surface, given
    RFC-070 declined exactly that as a public crate?
  - static pre-generated demos only, or the tool running in the browser too?
```

Bundling it here would make this RFC's scope lock unenforceable, which is the reason for the split.

## 9. Acceptance criteria

```text
[ ] tools/matten-playground exists: workspace-excluded, publish = false, lock git-ignored
[ ] docs/src/playground.md renders and is listed in SUMMARY.md
[ ] the page computes broadcasting, reshape, the four axis reductions, and matmul
[ ] a shape mismatch shows the real MattenError message, not a generic failure
[ ] output is text only — no SVG, canvas, or data-representing pixels (§6)
[ ] no published crate touched: git diff --name-only shows nothing under crates/
[ ] all seven guards pass, unmodified, INCLUDING check-published-dependency-isolation
[ ] mdbook build succeeds and check-doc-code.sh still passes
[ ] docs.yaml builds the wasm artifact before the book, and the deploy is green
[ ] no tag, no publish, no version change
```

## 10. Non-goals

```text
matten-viz, or any public visualization crate or API — RFC-070 stands
a desktop GUI — rejected: every toolkit pulls a dependency tree the core-boundary
  and published-isolation guards exist to keep out
replacing the visual_* examples; the page complements them and cites them
teaching content beyond shape reasoning — the tutorial theme is separately pended
```
