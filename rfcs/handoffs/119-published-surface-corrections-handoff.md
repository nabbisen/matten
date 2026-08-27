# Developer Handoff — RFC-119: Corrections to the Published Surface

**From:** High-capability model. **Date:** 2026-08-28.
**Design authority:** `rfcs/accepted/119-published-surface-corrections.md`
**Base:** `main` @ `245f57b`, clean tree, family at `0.46.0`, nothing unreleased.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Correct one panicking example and four false published statements, and add the CI execution that
would have caught the panic. **No version bump, no tag, no publish** — RFC-120 owns the release.

## 2. What makes this task different

Every previous fix RFC you have implemented changed *behaviour* or *tests*. **This one changes
almost nothing but statements** — and the statements are already on crates.io and docs.rs, where a
commit cannot reach them.

Two consequences worth carrying:

> **A correction that is not published does not correct anything.** `lib.rs`'s doc comment renders on
> docs.rs from the *published* crate; `matten-stats`'s README renders on crates.io the same way. Your
> diff fixes the repository. The release fixes the reader. That is why this RFC exists separately
> from RFC-120 and why its scope is so tightly drawn.

> **Four of these five were true when written.** None was a mistake at the time. They went false when
> something *else* changed and nothing pointed back at them. Treat neighbouring statements with
> suspicion while you are in these files — but **report what you find, do not fix it** (§9).

## 3. Evidence — re-derive before editing

The RFC's §3 table (E1–E20) is the authority. **Re-derive it.** Report any discrepancy first,
including one that shrinks the task.

Three rows are the ones most likely to differ from what you expect:

```text
E4   crates/matten/src/math/tests/matmul.rs:42-49 — an EXISTING PASSING TEST using
     the broken example's exact operands. This is what makes Change A provable
     rather than hand-computed. Read it before touching the example.
E15  78 example targets, 58 invoked by CI, 20 never executed. Derive from
     `cargo metadata`, NOT file basenames — 11 targets carry [[example]] name
     overrides and a basename diff gives the wrong answer. Mine did, first try.
E18  tools/matten-playground: 43 tests (34 lib.rs + 9 render.rs), zero CI
     invocation. docs.yaml BUILDS its wasm; nothing tests it.
```

## 4. Change A — the example panic

`crates/matten/examples/21_matrix_vector_product.rs`, lines 22-25.

```text
NOW      let w = Tensor::from_vec(vec![1.0, 2.0]);   // [2]
         let r2 = w.matmul(&m.transpose());          // m is [2,3] -> mᵀ is [3,2]
         println!("w·mᵀ = {r2:?}");
PANICS   matten shape error in dot: left length (2) must equal right rows (3)
```

**Preferred fix: delete the `.transpose()`.** E4 proves `w.matmul(&m)` → `[9,12,15]` because that
exact call is an already-passing core test. Then **relabel the println** — after the fix `w·mᵀ` would
itself be a false statement, which is the category of defect this whole RFC is about.

**Add assertions.** Lines 19-20 assert shape and values for the first half; the second half asserts
nothing, and that is the other half of why this survived 83 releases. Assert `[3]` and
`[9.0, 12.0, 15.0]`.

**The alternative is permitted** (RFC §4): keep `.transpose()` and make `w` length 3, preserving the
`w·mᵀ` label. If you take it, **compute the expected values and verify them by running.** Do not
trust any arithmetic in the RFC or this handoff that was not executed — figures quoted from reasoning
in this project have been wrong repeatedly, including mine this month.

## 5. Change B — `lib.rs:19`, the docs.rs landing page

```text
NOW   "Dynamic reshape/slicing/arithmetic are intentionally guarded until a
       future CoW-view milestone."
```

Both halves are stale. Slicing shipped in `0.44.0` (RFC-102) **using the CoW view this sentence
defers to**. Reshape and arithmetic are still guarded (`reshape.rs:39`, `:65`).

```text
DO     say slicing is available; say reshape and arithmetic are still guarded
DO NOT delete the sentence — the surviving guards are worth stating (R2)
DO NOT describe the storage mechanism; docs/src/reference/dynamic.md owns it
       and a fact in two places rots in one
```

## 6. Change C — the two zero-sized claims, together

```text
crates/matten-stats/README.md:146   PUBLISHED to crates.io for matten-stats
README.md:198                       GitHub front page only (not in any package)
```

`docs/src/migration/bridge-contracts.md:48` is already correct and already cites RFC-111 — **use it
as the wording model.**

For `matten-stats`: the `Empty`-on-fewer-than-two-elements rule is **correct and stays**. Only the
*"cannot represent zero elements at all"* premise is false. **Rewrite the justification, do not edit
around it** — editing around it is exactly what left it standing through RFC-111.

**Both, or neither.** Correcting one and leaving the other reproduces the split that produced this
finding (R3).

## 7. Change D — `stats.rs:7`

*"deferred to a possible future `matten-stats` companion"* → the shipped companion (published at
`0.46.0`, production-ready candidate, ships all four named functions). **Keep the RFC-040 §6/§8
citation** — it is still the reason core does not host them. Only the tense is wrong.

## 8. Change E — close the execution gap

```text
E.1  add the 20 unexecuted examples to test.yaml's smoke job, EXCEPT 10 and 11
E.2  10 and 11 read examples/data/… relative to cwd and fail from the repo root
     though their headers say `cargo run --example …`. EITHER make the path
     robust via CARGO_MANIFEST_DIR and add them to the smoke job, OR record the
     cwd requirement in the file header. Say which you chose and why.
E.3  add `cargo test` and `cargo clippy` for tools/matten-playground.
     It is the only tool with neither. LATENT, not a live break — the 43 tests
     pass today (E19). Do not report it as a break.
```

### The sequencing rule — read this twice

```text
1. WITHOUT Change A applied, run your new smoke step for
   21_matrix_vector_product and CAPTURE THE FAILURE.        (rule 002 §4)
2. THEN apply Change A.
3. Land A and E.1 in the SAME COMMIT.
```

Landing E.1 first would turn `main` red on purpose, against RFC-118, to demonstrate something a local
capture demonstrates just as well. The capture is the evidence; a red `main` is not required and is
not wanted.

## 9. If a newly-executed example fails

**Report it and stop. Do not fix it.**

Nineteen examples are about to run in CI for the first time. If one of them fails, that is a finding
— and scope is amended by the owner, not by the implementer. Change A is in scope only because it is
already known and already evidenced.

This is the single most likely way this task grows beyond its RFC (R5).

## 10. Out of scope — a diff touching these is a defect

```text
Cargo.toml version, any version pin, CHANGELOG.md   -> RFC-120 owns the release
any behaviour change in any src/                    -> statements and one example
ROADMAP.md, SECURITY.md, tools' unsafe policy       -> audit F5/F10/F11, Cycle 2
docs/design/v1-readiness-audit.md                   -> audit F6, owner decision
the other 19 examples' CONTENT                      -> §9
RFC-094's §4.1 amendment                            -> NOT yours; RFC-120's
mechanically blocking a tag on red CI               -> RFC-118 §9, still open
```

## 11. Risks

```text
R1  Fixing A by changing m or v rather than the erroneous transpose, silently
    altering what the example teaches. E4 pins the intended operation.
R2  Deleting lib.rs:19 outright instead of rewriting it (§5).
R3  Correcting one zero-sized site and not the other (§6).
R4  Landing E.1 before A, leaving main red, against RFC-118 (§8).
R5  Absorbing a newly-surfaced example failure instead of reporting it (§9).
R6  Presenting E.3 as a live break. It is latent.
R7  Treating this as authorizing a release. It does not.
R8  Deriving E15 from file basenames. 11 targets have [[example]] name
    overrides; the basename answer is wrong.
```

## 12. Acceptance criteria

```text
[ ] 21_matrix_vector_product RUNS to completion; both halves assert shape and
    values; the printed label matches what is computed
[ ] the BEFORE (panic) and AFTER (clean run) both captured
[ ] lib.rs:19 rewritten — slicing available, reshape/arithmetic still guarded,
    no "future CoW-view milestone"
[ ] both zero-sized sites corrected; matten-stats' fewer-than-2 rule preserved
[ ] stats.rs:7 corrected; RFC-040 citation retained
[ ] 20 examples added to CI except 10 and 11; 10/11 fixed or documented, choice
    justified
[ ] tools/matten-playground has cargo test + cargo clippy in test.yaml
[ ] E.1 proven able to fail, captured BEFORE the fix
[ ] Change A and E.1 in the SAME commit
[ ] git diff shows NO Cargo.toml version, NO pin, NO CHANGELOG, NO tag/publish
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
    — the CI form. Do NOT scope it to -p matten.
[ ] cargo test --workspace; both feature profiles build
```

## 13. Required evidence

```text
Change A   the panic before, the clean run after, and the assertion values.
           If you took the alternative repair, the values you COMPUTED and the
           run that CONFIRMED them.
Change E.1 the captured failure of the new smoke step BEFORE the fix.
E15        your derived count of unexecuted examples, and the method — if it
           differs from 20, that discrepancy is worth more than the edit.
Change C   quote both corrected passages, so the pair can be checked at review.
§9         any newly-executed example that failed, reported and NOT fixed.
```

## 14. Required review-request format

Write to:
`.git-exclude/review-request/RFC-119/matten-rfc119-published-surface-corrections-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 re-derivation with any discrepancy, §13's evidence,
your E.2 choice with reasoning, guard and test output, deviations with reasoning, and anything you
want answered at review.
