# Developer Handoff — RFC-114: `0.46.0` The Empty-Tensor Release

**From:** High-capability model. **Date:** 2026-08-09.
**Design authority:** `rfcs/accepted/114-0460-empty-tensor-release.md`
**Base:** `main` @ `03ad353`, clean tree, family at `0.45.0`.

> **Priority rule.** The RFC is the design authority; this handoff never overrides it. Where the two
> disagree, **raise it before proceeding**. A pure addition in the handoff you can simply omit may be
> omitted and reported prominently; anything else, ask first.

---

## 1. Task title

Prepare `0.46.0`: lock-step bump, 38 live pin retargets, one CHANGELOG entry, two content updates.
**No tag, no publish.**

## 2. The two traps, both inversions of the last release

```text
0.45.0 (RFC-109)  Added + Changed. Snapshot needed FOUR NEW ROWS.
0.46.0 (this)     CHANGED ONLY. Snapshot needs a SENTENCE and NO NEW ROW.
```

**No public item was added.** Verified:

```text
git diff 0.45.0..HEAD -- 'crates/*/src/**.rs' | grep '^+\s*pub (fn|struct|enum|const|type)'
-> nothing
```

So: no `Added` section, no empty `Added` heading, and **adding a snapshot row would be a defect** —
the exact opposite of what RFC-109 required two releases ago. This is the RFC-103 shape.

## 3. The claim this release must NOT make

**Do not write that this release fixes a panic.**

RFC-110 introduced a panic in `matten-mlprep` and RFC-112 removed it, **both inside this release**.
No released version ever had it. Measured, `0.45.0` against the current tree:

```text
0.45.0   standardize_columns(empty)  Err("matten rejected the result: matten shape error…")
0.46.0   standardize_columns(empty)  Err("matten rejected the result: matten invalid argument…")
```

**A user upgrading sees a different error message, never a panic.** An entry claiming a panic was
fixed would describe a defect nobody could have experienced.

State it as what it is: the error `matten-mlprep` returns for a zero-row input now comes from the
axis reduction rather than from tensor construction.

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Family at `0.45.0`, lock-step | `Cargo.toml:42`; `crates/*/Cargo.toml` `version.workspace = true` |
| E2 | `0\.45\b` matches **70 lines** in tracked `md/toml/rs/yml` | `git ls-files` → filter ext → exclude `Cargo.lock` by **exact path** → `grep -c` per file |
| E3 | **32** are records — `rfcs/**`, `ROADMAP.md`, `CHANGELOG.md` | E2's per-file breakdown |
| E4 | **38** live pins across **17** files | E2 − E3; same file set as the last two releases |
| E5 | No public item added since `0.45.0` | §2's grep |
| E6 | The project has **never** used a `Deprecated` CHANGELOG section | `grep -c "^### Deprecated" CHANGELOG.md` → 0 |
| E7 | `matten-mlprep`'s error text changed, but was an `Err` both before and after | §3's measurement, `0.45.0` tag vs current |
| E8 | CI's clippy gate is the workspace all-features form | `.github/workflows/test.yaml:40` |

**Re-derive E2–E4 before editing.** If your numbers differ from 70/32/38, one of our methods is
broken and that is worth more than the retarget. Measuring at a specific commit with
`git grep <rev>` — as you did for `0.45.0` — is better than my method and avoids the
moving-target problem; keep doing that.

## 5. Required implementation

```text
1. Cargo.toml:42 -> 0.46.0. Build so Cargo.lock regenerates. Commit the lock.
2. Retarget the 38 live pins. Both "0.45.0" and "0.45.x" forms occur.
3. crates/matten/src/lib.rs — the ONLY .rs edit permitted, an install-pin doc comment.
4. CHANGELOG [0.46.0]: Changed + Version. No Added. No Deprecated section (E6) —
   record ZeroSizedAxis's deprecation inside Changed.
5. introduction.md content rewrite (§7).
6. public-api-snapshot.md: a sentence, NO new row (§8).
```

## 6. The 32 that must not move

`rfcs/**`, `ROADMAP.md`, and `CHANGELOG.md`'s existing `[0.45.0]` entry. Retargeting any rewrites
history.

**Expect `CHANGELOG.md` to gain a `0.45` occurrence** in the new entry's bump line. Assert **no
removed line**, not a fixed count.

## 7. `introduction.md` — rewrite

Its paragraph describes `0.45.0`. Rewrite for RFC-110/111/112. Accuracy points, each easy to get
wrong:

```text
- zero-sized dimensions are now CONSTRUCTIBLE, not merely reachable by slicing
- Display shows an empty tensor's SHAPE; Debug is unchanged
- axis reductions error on a zero-length REDUCED axis. The SURVIVING-axis case
  was and stays Ok — do not claim otherwise
- sum and sum_axis are UNCHANGED; the additive identity is correct
- no new API — this release removes restrictions and changes behaviour
- do NOT say a panic was fixed (§3)
```

## 8. `public-api-snapshot.md` — a sentence, no row

Its *"most recently changed in RFC-104 … and RFC-108"* claim **stays true** (E5). Add a sentence in
the RFC-088/RFC-102/RFC-105 style recording RFC-110 and RFC-111 as behaviour changes with no new row,
and note `ZeroSizedAxis`'s deprecation.

**Do not carry RFC-109's instruction forward.** There, four items were added and omitting them was
the defect. Here, adding a row is.

## 9. Acceptance criteria

```text
[ ] cargo metadata shows 0.46.0 for all five crates
[ ] 38 live pins retargeted; the 32 records UNCHANGED — assert both
[ ] no .rs diff except crates/matten/src/lib.rs's doc comment
[ ] CHANGELOG [0.46.0]: Changed + Version only; no Added, no Deprecated heading
[ ] no falsehood from RFC §7.1 — especially §3's
[ ] introduction.md rewritten; no text describing 0.45.0 as current survives
[ ] public-api-snapshot.md: sentence added, NO new row, claim not moved
[ ] ROADMAP.md and rfcs/** untouched
[ ] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features (E8)
[ ] cargo test --workspace; both feature profiles
[ ] NO tag, NO publish
```

## 10. Risks

```text
R1  Pattern-matching 0.45.0. It had Added + Changed and four new snapshot rows;
    this has neither (§2).
R2  Claiming a panic was fixed (§3). The most likely falsehood in this entry,
    because "we fixed a panic" is how the work felt.
R3  Over-claiming the axis-reduction change to cover the surviving-axis case,
    or claiming sum/sum_axis changed (§7).
R4  Retargeting a record file (§6).
R5  Asserting a fixed CHANGELOG occurrence count instead of "no removed line".
R6  Narrowing the clippy gate (E8).
```

## 11. Required evidence

For E2–E4, give the command and your numbers. For §6, paste `git diff --stat` showing `rfcs/` and
`ROADMAP.md` absent. For §3, §7 and §8, quote what you wrote so the claims can be checked against the
code rather than against this handoff.

## 12. Required review-request format

Write to:
`.git-exclude/review-request/RFC-114/matten-rfc114-0460-empty-tensor-release-implementation-review-request-v0.1.md`

Include files changed with line counts, the §4 verification with any discrepancy, §11's evidence,
guard/clippy/test output, deviations with reasoning, and anything you want answered at review.
