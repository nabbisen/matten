# Developer Handoff — RFC-109: `0.45.0` Mutation and Empty-Tensor Release

**From:** High-capability model. **Date:** 2026-08-08.
**Design authority:** `rfcs/accepted/109-0450-mutation-and-empty-tensor-release.md`
**Base:** `main` @ `1c6c208`, clean tree, family at `0.44.0`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Prepare `0.45.0`: lock-step bump, 38 live pin retargets, one CHANGELOG entry, two content rewrites.
**No tag, no publish.**

## 2. Both recent releases are the wrong template

You have two release preparations to pattern-match against, and **each is wrong in a different
direction**:

```text
0.43.0 (RFC-101)  Added-only.    Forbade an empty `Changed` heading.
0.44.0 (RFC-103)  Changed-only.  Forbade an empty `Added` heading.
0.45.0 (this)     BOTH.          Three non-empty sections: Added, Changed, Version.
```

And the API-snapshot instruction **inverts from last time**:

```text
RFC-103 said: RFC-102 changed no public item -> a new row would be a DEFECT.
RFC-109 says: RFC-104 and RFC-108 added FOUR public items -> omitting them IS the defect.
```

If you carry either release's instincts forward unexamined, you will get one of these wrong.

## 3. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Family at `0.44.0`, lock-step, four crates inherit | `Cargo.toml:42`; `crates/*/Cargo.toml` `version.workspace = true` |
| E2 | `0\.44\b` matches **79 lines** in tracked `md/toml/rs/yml` | `git ls-files` → filter ext → exclude `Cargo.lock` by **exact path** → `grep -c` per file |
| E3 | **41** are records — `rfcs/**` (32), `ROADMAP.md` (3), `CHANGELOG.md` (2), plus RFC docs narrating their own releases | E2's per-file breakdown |
| E4 | **38** are live pins across **17** files | E2 − E3 |
| E5 | The 17 live files are the **same set** as `0.43.0`→`0.44.0` | compared against RFC-103 §5 |
| E6 | RFC-107's two new pages carry **no** version pin | absent from E2's match set |
| E7 | `introduction.md:19-26` describes `0.44.0` as *"an RFC-102 release"* | direct read |
| E8 | `public-api-snapshot.md:5-7` claims core *"most recently changed in RFC-099 … and RFC-100"* | direct read — now false |
| E9 | Four public items were added since `0.44.0` | `get_mut`, `get_flat_mut`, `get_element_mut` (RFC-104); `is_empty` (RFC-108) |
| E10 | CI's clippy gate is the **workspace, all-features** form | `.github/workflows/test.yaml:40` |

**Re-derive E2–E4 before editing.** My measurement method has been wrong twice in this project
(RFC-103 §5.0). If your numbers differ from 79/41/38, one of our methods is broken and that is worth
more than the retarget.

## 4. The 41 that must not move

```text
rfcs/**        32   including RFC-103's own document and handoff, which narrate the
                    0.43->0.44 transition and must keep citing 0.44 forever
ROADMAP.md      3   history rows
CHANGELOG.md    2   the existing [0.44.0] entry
```

Retargeting any of these rewrites history. **Assert they are unchanged** — `git diff --stat` naming
`rfcs/`, `ROADMAP.md`, or a *removed* line in `CHANGELOG.md` is a defect.

**Expect `CHANGELOG.md` to gain a `0.44` occurrence**, in the new entry's `Release bump 0.44.0 ->
0.45.0` line. That is correct and mirrors every prior entry. Assert *no removed line*, not a fixed
count — a fixed count is the wrong invariant and RFC-103's review corrected me on exactly this.

## 5. Required implementation

```text
1. Cargo.toml:42 -> 0.45.0. Build so Cargo.lock regenerates. Commit the lock.
2. Retarget the 38 live pins. BOTH forms occur:
     "0.44.0"  exact pins
     "0.44.x"  family references (README table, companion README banners)
3. crates/matten/src/lib.rs — the ONLY .rs edit permitted, an install-pin doc comment.
4. CHANGELOG [0.45.0]: Added + Changed + Version, all three non-empty (§6).
5. introduction.md content rewrite (§7).
6. public-api-snapshot.md content update — four new items (§8).
```

## 6. CHANGELOG `[0.45.0]`

Three sections, none empty.

```text
Added    get_mut / get_flat_mut / get_element_mut (RFC-104)
         is_empty (RFC-108)
Changed  mean/min/max/argmin/argmax now error on an empty tensor instead of
         panicking with a raw index error or returning NaN/inf/-inf (RFC-105)
         dot/matmul no longer panic on a zero-column product; they return the
         empty [m,0] result (RFC-108)
Version  0.44.0 -> 0.45.0, lock-step
```

**Five claims would be publishable falsehoods** (RFC §6.1):

```text
- "no existing behavior changed" — 0.44.0's boilerplate; two entries here ARE
  behaviour changes, and a user matching on the old shapes needs to see that.
- calling RFC-108's matmul fix a new capability. It removes a panic from an
  operation that already existed.
- mentioning get_element_mut's storage SHARING without the materialize-on-write
  consequence, or vice versa. Both halves ship together everywhere else.
- claiming is_empty() is new BEHAVIOUR. The state was always reachable; the
  method was missing.
- any suggestion zero-sized dimensions are now constructible. They are NOT —
  that is RFC-106 Stage 3 and it is not in this release.
```

## 7. `introduction.md` — rewrite, do not renumber

E7's paragraph describes `0.44.0`'s dynamic slicing. Under `0.45.0` it is the previous release
sitting on the front page. Rewrite for RFC-104/105/108.

Accuracy points, each verified at review and each easy to get subtly wrong:

```text
- get_element_mut SHARES storage and materializes on first write, releasing the
  parent's allocation as a side effect
- the empty-tensor reductions now ERROR; sum still returns a zero (its identity)
- matmul's fix removes a panic; it does not add batching or any new shape support
- is_empty() reports a state that was always reachable via slicing
```

## 8. `public-api-snapshot.md` — add four items, move the claim

E8's *"most recently changed in RFC-099 … and RFC-100"* is now **false**. Move it to RFC-104 and
RFC-108, and add E9's four items to the page's inventory.

**This is the inverse of RFC-103's instruction.** There, RFC-102 changed no public item and adding a
row would have been a defect. Here, omitting them is. Do not carry that instruction forward.

## 9. Acceptance criteria

```text
[ ] cargo metadata shows 0.45.0 for all five crates
[ ] 38 live pins retargeted; zero 0.44 outside the 41 records (plus the new
    CHANGELOG bump line, §4)
[ ] rfcs/**, ROADMAP.md unchanged; CHANGELOG.md has no REMOVED line — asserted
[ ] no .rs diff except crates/matten/src/lib.rs's doc comment
[ ] CHANGELOG [0.45.0]: Added + Changed + Version, none empty, no falsehood from §6
[ ] introduction.md rewritten; no text describing 0.44.0 as current survives
[ ] public-api-snapshot.md: four items added, claim moved to RFC-104/RFC-108
[ ] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
    — the CI form (E10). Do NOT scope it to -p matten.
[ ] cargo test --workspace; both feature profiles build
[ ] NO tag, NO publish
```

## 10. Risks

```text
R1  Pattern-matching either recent release. Both are wrong here (§2).
R2  Carrying RFC-103's "no new snapshot row" instruction forward (§8).
R3  Retargeting a record file — silently rewrites history (§4).
R4  Asserting a fixed CHANGELOG occurrence count instead of "no removed line" (§4).
R5  A stray .rs edit. Only lib.rs's doc comment.
R6  Narrowing the clippy gate. RFC-108's review found a CI-red branch that a
    narrowed gate hid. Run the workspace all-features form (E10).
```

## 11. Required evidence

For E2–E4, give the command and your numbers. For §4, paste `git diff --stat` and let it show
`rfcs/` and `ROADMAP.md` absent. For §6, §7, §8, quote what you wrote so the claims can be checked
against the code rather than against this handoff.

## 12. Required review-request format

Write to:
`.git-exclude/review-request/RFC-109/matten-rfc109-0450-mutation-and-empty-tensor-release-implementation-review-request-v0.1.md`

Include files changed with line counts, the §3 verification with any discrepancy, §11's evidence,
guard/clippy/test output, deviations with reasoning, and anything you want answered at review.
