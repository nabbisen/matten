# RFC-119: Corrections to the Published Surface

**Status:** **Accepted** 2026-08-28 by the owner. Not yet implemented. Handoff:
`rfcs/handoffs/119-published-surface-corrections-handoff.md`. The §9 RFC-094 amendment is **not**
part of this acceptance and remains open for RFC-120; this RFC does not depend on it.
**Target:** `crates/matten/examples/`, `crates/matten/src/lib.rs`, `crates/matten/src/stats.rs`,
`crates/matten-stats/README.md`, `README.md`, `.github/workflows/test.yaml`
**Theme:** Fix four false statements and one panic that are already on crates.io, and close the gap
that hid the panic for 83 releases
**Related:** RFC-094 §4.1/§4.3, RFC-102, RFC-111, RFC-117, RFC-118, rule 002 §4

---

## 1. Summary

```text
A  21_matrix_vector_product panics when run. Shipped since 0.17.0.
B  lib.rs's crate-root doc — docs.rs's landing page — is false since 0.44.0.
C  two READMEs deny zero-sized dimensions, accepted in 0.46.0 by RFC-111.
D  stats.rs defers to a "possible future matten-stats" that has shipped.
E  20 of 78 examples are compiled but never executed. That is what hid A.
```

**Every item in A–D is inside a published package**, so unlike RFC-117 and RFC-118 this RFC does
produce a `crates/` change and therefore a release. The release itself is **not** in this RFC; it is
RFC-120's, matching the RFC-108 → RFC-109 pattern.

**No public API changes and no behaviour changes.** Every correction is to a statement or an example.

## 2. Why these five together

They are one finding with one cause: **the project has no gate that reads a published claim.** All
nine guards are structural — dependency direction, scope boundaries, book-code compilation, demo
freshness. Not one can detect prose going false, and the two that open a README do not check what it
says. A–D were all found by a directed sweep, and each was true when written.

E is the same defect in the executable register: `cargo check --workspace --examples` proves an
example *compiles*, and nothing proves it *runs*. A has been shipping for 83 releases behind a green
CI for exactly that reason.

**A and E must ship together.** Fixing A without E leaves the next broken example equally invisible;
adding E without A turns `main` red.

## 3. Evidence

Re-derive every row before editing. **Report any discrepancy first, including one that shrinks the
task.** My measurement method has been wrong repeatedly in this project.

| # | Claim | Established by |
|---|---|---|
| E1 | `21_matrix_vector_product` prints three correct lines then panics: *"matten shape error in dot: left length (2) must equal right rows (3)"* | `cargo run -p matten --example 21_matrix_vector_product` |
| E2 | The cause is line 24, `w.matmul(&m.transpose())` — `w` is `[2]`, `m` is `[2,3]`, so `mᵀ` is `[3,2]` and `[2]×[3,2]` violates the `[n]×[n,p]` rule the file's own line 22 states | direct read |
| E3 | Line 24 has never been edited since `d7560c0` (RFC-025/027); **83 tags contain it, from `0.17.0`** | `git log -p --` on the file; `git tag --contains d7560c0` |
| E4 | The correct form is already a passing core test: `v=[1,2]`, `m=[2,3]`, `v.matmul(&m)` → `[9,12,15]` | `crates/matten/src/math/tests/matmul.rs:42-49`, `fn vector_matrix_mul` |
| E5 | `examples/` is packaged and published — the panicking file is inside the `matten 0.46.0` a user downloads | `cargo package --list -p matten` contains `examples/21_matrix_vector_product.rs` |
| E6 | `lib.rs:19` reads *"Dynamic reshape/slicing/arithmetic are intentionally guarded until a future CoW-view milestone"* | direct read |
| E7 | Dynamic **slicing shipped** in `0.44.0` (RFC-102), using the very CoW view the sentence defers to | `docs/src/reference/dynamic.md:209`; `:54` calls it *"CoW slices"* |
| E8 | Dynamic **reshape and arithmetic remain guarded** | `reshape.rs:39`, `:65`; `panic_if_dynamic` call sites |
| E9 | The stale line is in `0.44.0`, `0.45.0`, `0.46.0`, and absent-of-defect in `0.43.0` | `git show <tag>:crates/matten/src/lib.rs` |
| E10 | `crates/matten-stats/README.md:146` claims *"`matten::Tensor` cannot represent zero elements at all (every dimension must be non-zero)"* | direct read — false since RFC-111 |
| E11 | `README.md:198` claims *"Zero-sized axes are rejected … `from_arrayd` returns an error"* | direct read — refuted by the passing test `from_arrayd_zero_axis_is_accepted_not_rejected` (`crates/matten-ndarray/tests/conversion.rs:70`) |
| E12 | `docs/src/migration/bridge-contracts.md:48` already states the correct contract | direct read — use as the wording model |
| E13 | RFC-111's commit `2ac99bb` updated `crates/matten-ndarray/README.md` and eight `docs/src/` pages, but neither file in E10/E11 | `git show --stat 2ac99bb` |
| E14 | `stats.rs:7` defers quantile/covariance/correlation to *"a possible future `matten-stats` companion"*; `matten-stats` is published at `0.46.0`, production-ready candidate, and ships all of them | direct read; `crates/matten-stats/src/covariance.rs:163` |
| E15 | 78 example targets exist; CI invokes 58; **20 are compiled but never executed** | `cargo metadata` target names ∩ `test.yaml`. **Not file basenames** — 11 targets carry `[[example]] name` overrides |
| E16 | The 20 include the whole beginner path (`01`–`05`, `20`–`24`) and all four `dynamic_01`–`04` | the set difference |
| E17 | `10_json_roundtrip` and `11_csv_numeric_loading` read `examples/data/…` relative to cwd and fail from the repo root, though their headers say `cargo run --example …` | run from root, then from `crates/matten/` |
| E18 | `tools/matten-playground` has **43 tests** (34 in `lib.rs`, 9 in `render.rs`) and **no CI invocation of any kind**; `docs.yaml` only *builds* its wasm | `grep -rc '#\[test\]'`; `grep -rn playground .github/workflows/` |
| E19 | The 43 pass today | `cargo test --manifest-path tools/matten-playground/Cargo.toml` → `43 passed; 0 failed` |
| E20 | Nothing is currently unreleased: `git diff --name-only 0.46.0..HEAD -- crates/` is empty across 20 commits | direct |

## 4. Change A — the example panic

**Delete the `.transpose()`.** E4 is decisive: `v.matmul(&m)` with these exact operands is already a
passing core test returning `[9,12,15]`. The example's second half is a corrupted copy of a
correct, tested operation.

```text
line 23-24   let w = Tensor::from_vec(vec![1.0, 2.0]);
             let r2 = w.matmul(&m);          // was: &m.transpose()
line 25      relabel the println — it currently says "w·mᵀ", which after this
             fix would itself be a false statement
```

**Add the assertions the first half already has.** Lines 19–20 assert shape and values for `m·v`;
the second half asserts nothing, which is the other half of why this survived. Assert
`shape() == [3]` and `as_slice() == [9.0, 12.0, 15.0]`.

**The alternative is defensible and you may take it instead:** keep `.transpose()` and make `w`
length 3, giving `[3]×[3,2] → [2]` and preserving the `w·mᵀ` label. I prefer dropping the transpose
because E4 makes it provably correct against an existing test rather than newly hand-computed. **If
you take the alternative, compute the expected values and verify them by running — do not trust an
arithmetic claim in this RFC that I have not executed.**

## 5. Change B — the crate-root doc

`lib.rs:19` is the **docs.rs landing page** for `matten`. Rewrite it so that:

```text
- dynamic SLICING is described as available (RFC-102, shipped 0.44.0)
- dynamic RESHAPE and ARITHMETIC are still described as guarded (E8) — do not
  delete the sentence wholesale; the surviving guards are worth stating
- the phrase "until a future CoW-view milestone" GOES. The CoW view is what
  RFC-102 shipped; deferring to it reads as though it never arrived.
```

**Do not describe the storage mechanism here.** `docs/src/reference/dynamic.md` owns that, and a
fact in two places rots in one (RFC-107 §8 risk 1).

## 6. Change C — the zero-sized claims

Two files, **corrected together**:

```text
crates/matten-stats/README.md:146   PUBLISHED to crates.io for matten-stats
README.md:198                       GitHub front page only
```

`bridge-contracts.md:48` (E12) is the wording model — it is already correct and already cites
RFC-111.

For the `matten-stats` row, note what is *not* wrong: the `Empty`-on-fewer-than-two-elements rule is
correct and stays. Only the *"cannot represent zero elements at all"* premise is false. **Rewrite the
justification, do not edit around it** — that is what left it standing through RFC-111.

E13 is the finding under this change and belongs in the RFC record: RFC-111 correctly updated the
README of the crate whose *code* it changed, and missed these two because **a claim about crate A's
behaviour was living in crate B's README.**

## 7. Change D — the stats module doc

`stats.rs:7`: *"deferred to a possible future `matten-stats` companion"* → the shipped companion.
The direction of the sentence is right and should survive; only its tense is wrong. Keep the
RFC-040 §6/§8 citation, which is still the reason core does not host these.

## 8. Change E — close the execution gap

```text
E.1  add the 20 unexecuted examples to test.yaml's smoke job, EXCEPT 10 and 11
E.2  10 and 11 (E17): either make the fixture path robust via CARGO_MANIFEST_DIR
     and add them too, or record IN THE FILE HEADER why they need
     cwd = crates/matten/. Silence is what makes this read as an oversight
     rather than a decision. Your call which; say which and why.
E.3  add `cargo test` and `cargo clippy` for tools/matten-playground (E18).
     It is the only tool with neither, and RFC-113 and RFC-115 both changed it.
```

**Prove E.1 can fail (rule 002 §4).** Before applying Change A, run the new smoke step and capture
the failure. **Then apply A, and land both in the same commit** — RFC-118 forbids tagging on red, and
landing E.1 first would put `main` red deliberately to make a point that a local capture makes just
as well.

E.3 is latent, not live (E19). Do not present it as a break.

## 9. A policy question this RFC cannot answer itself

RFC-094 §4.1 defines a patch as *"correctness fixes to already-published crate **code**, and nothing
else."* §4.3 excludes *"documentation … that does not reach crates.io."*

**Changes B, C and D are published documentation — neither category.** Not "code", so §4.1 does not
plainly admit them; they *do* reach crates.io, so §4.3 does not exclude them.

Both of the policy's own mechanisms point toward releasing: §4.3's mechanical test
(`cargo package --list`, the same test that declined `0.42.1`) puts all four files inside published
packages, and §4.1's rationale — *"a user hitting a wrong answer should not wait"* — describes a
reader of a false capability claim precisely.

**Proposed amendment, for the owner:** §4.1's *"crate code"* → *"crate content (code, rustdoc, or
packaged README)"*. One line, no change to the triggers.

**This RFC does not depend on the amendment.** Change A is a correctness fix to published code under
any reading and carries the release alone. If the owner declines the amendment, B–D still ship in the
same patch as A; only the *stated justification* narrows.

## 10. Scope

```text
IN    the five corrections, the CI additions, and the RFC-094 §9 question
```

### Out of scope — a diff touching these is a defect

```text
Cargo.toml version, any pin, CHANGELOG.md   -> RFC-120 owns the release
any src/ change that alters BEHAVIOUR       -> this RFC changes statements and
                                               one example, nothing else
ROADMAP.md, SECURITY.md, tools' unsafe policy -> audit F5/F10/F11, Cycle 2
docs/design/v1-readiness-audit.md            -> audit F6, an owner decision
the 19 other examples' CONTENT               -> add them to CI; if one fails,
                                               REPORT it, do not fix it here
mechanically blocking a tag on red CI        -> RFC-118 §9, still open
```

**If a newly-executed example fails, that is a finding, not this RFC's work.** Report it and stop;
scope is amended by the owner, not by the implementer. A is in scope because it is already known.

## 11. Risks

```text
R1  Fixing A by changing m or v rather than the erroneous transpose, silently
    altering what the example teaches. E4 pins the intended operation.
R2  Deleting lib.rs:19 entirely instead of rewriting it (§5). Reshape and
    arithmetic ARE still guarded and a reader needs to know.
R3  Correcting one zero-sized site and not the other (§6) — precisely the
    failure that produced this finding.
R4  Landing E.1 before A and leaving main red, against RFC-118.
R5  Scope creep into the other 19 examples once they start running (§10).
R6  Asserting E.3 is a live break. It is latent; the 43 tests pass.
R7  Treating this RFC as authorizing a release. It does not — RFC-120 does,
    and the owner authorizes the tag and the publish separately (RFC-094 §5).
```

## 12. Acceptance criteria

```text
[ ] 21_matrix_vector_product RUNS to completion; shape and values asserted for
    BOTH halves; the printed label matches what is computed
[ ] the before/after of A captured — the panic, then the clean run
[ ] lib.rs:19 rewritten: slicing available, reshape/arithmetic still guarded,
    no "future CoW-view milestone"
[ ] both zero-sized sites corrected together; matten-stats' fewer-than-2 rule
    preserved
[ ] stats.rs:7 corrected; RFC-040 citation retained
[ ] the 20 examples added to CI except 10 and 11; 10/11 either fixed or their
    cwd requirement recorded in-file, with the choice justified
[ ] tools/matten-playground has cargo test + cargo clippy in test.yaml
[ ] E.1 proven able to fail, captured BEFORE the fix (rule 002 §4)
[ ] A and E.1 in the SAME commit
[ ] no version bump, no pin change, no CHANGELOG edit, no tag, no publish
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
```

## 13. What this does not fix

```text
- the ROADMAP Status block, five releases stale                    (audit F5)
- the v1.0 readiness audit, eight releases stale                   (audit F6)
- SECURITY.md, which needs the owner's disclosure contact          (audit F10)
- the three tools' unsafe policy                                   (audit F11)
- mechanically blocking a tag on red CI                            (RFC-118 §9)
```

**And it does not fix the cause.** Four of these five statements were true when written and went
false when something else changed. Nothing added here detects the next one — E only closes the
*executable* half of the gap. A guard that reads published claims is a real design problem, not a
line item, and it is deliberately not attempted in this RFC.
