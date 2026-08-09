# RFC-114: `0.46.0` — The Empty-Tensor Release

**Status:** **Accepted** 2026-08-09 by the owner. Not yet implemented. Handoff:
`rfcs/handoffs/114-0460-empty-tensor-release-handoff.md`.
**Target:** `0.46.0`, on the `0.x` line
**Related:** RFC-030 (lock-step), RFC-094 (cadence), RFC-110, RFC-111, RFC-112

---

## 1. Summary

Bump the lock-step family `0.45.0` → `0.46.0` and release RFC-110, RFC-111 and RFC-112. Preparation
only: **no tag, no publish** — both remain separate owner authorizations.

## 2. Why now, and the one constraint

RFC-094's triggers: **two themes accumulated — yes, three.** The 28-day clock has not run.

**The constraint that matters more than the cadence:**

```text
RFC-110, RFC-111 and RFC-112 MUST ship together.
RFC-110 alone is a regression; RFC-112 is its fix.
```

RFC-110 turned a sentinel leak into a panic, which was correct for direct callers and wrong for
`matten-mlprep`, which called the panicking axis forms *because* the sentinel made them safe. Shipping
RFC-110 without RFC-112 would put a panic in a published crate — the second time after `0.44.0`.

They are all in `main` and none is released, so this is a sequencing note, not a rescue.

## 3. Release content — `Changed`-only, and that is verified

```text
RFC-110  mean_axis/min_axis/max_axis/var_axis/std_axis error on a zero-length
         REDUCED axis instead of leaking NaN/inf/-inf. sum_axis unchanged.
RFC-111  zero-sized dimensions are accepted throughout: constructors, reshape,
         the composition family, linspace/eye, serde, and the ndarray bridge.
         Display shows an empty tensor's shape. ZeroSizedAxis deprecated.
RFC-112  matten-mlprep's scaling functions use the try_ axis forms.
```

**No public item was added.** Verified:

```text
git diff 0.45.0..HEAD -- 'crates/*/src/**.rs' | grep '^+\s*pub (fn|struct|enum|const|type)'
-> nothing
```

So the entry is **`Changed` and `Version` only** — no `Added`. That inverts `0.45.0`, which had both.

**The project has never used a `Deprecated` section** (0 occurrences in `CHANGELOG.md`). Record
`ZeroSizedAxis`'s deprecation inside `Changed` rather than inventing a section for one item.

## 4. The claim this release must NOT make

**RFC-112 fixes a panic no released version ever had.** RFC-110 introduced it and RFC-112 removes it,
both inside this release. Measured, `0.45.0` against the current tree:

```text
0.45.0    standardize_columns(empty)  Err("matten rejected the result: matten shape error…")
0.46.0    standardize_columns(empty)  Err("matten rejected the result: matten invalid argument…")
```

**A user upgrading sees a different error message, never a panic.** An entry claiming this release
"fixes a panic" would describe a defect no user could have experienced — a publishable falsehood of
exactly the class this project guards against.

State it as what it is: the error `matten-mlprep` returns for a zero-row input now comes from the axis
reduction rather than from tensor construction.

## 5. Scope

```text
Cargo.toml       0.45.0 -> 0.46.0 (workspace, lock-step, all five crates)
Cargo.lock       regenerated, committed
38 live version pins across 17 files (§6)
CHANGELOG.md     a [0.46.0] entry: Changed + Version (§7)
introduction.md  CONTENT rewrite — the current text describes 0.45.0
public-api-snapshot.md  see §6.2 — a sentence, and NO new row
```

### Out of scope — a diff touching these is a defect

```text
any .rs file except the install-pin doc comment at crates/matten/src/lib.rs
any behaviour, test, or guard
the 32 record occurrences in rfcs/**, ROADMAP.md, CHANGELOG.md's existing entries
RFC-113 (the playground) — publish = false, ships in no release
a tag, a publish, or a maturity-label change
```

## 6. Version retarget — 38 live pins across 17 files, 32 records

`0\.45\b` matches **70 lines** in tracked `md`/`toml`/`rs`/`yml` files, measured by enumerating
`git ls-files`, excluding `Cargo.lock` by **exact path**, and counting per file.

```text
32 records   rfcs/** , ROADMAP.md, CHANGELOG.md's existing [0.45.0] entry. MUST NOT MOVE.
38 live      the same 17 files as the last two releases.
```

**Expect `CHANGELOG.md` to gain a `0.45` occurrence** in the new entry's bump line. Assert **no
removed line**, not a fixed count.

### 6.1 `introduction.md` — rewrite

Its paragraph describes `0.45.0`'s mutable access and empty-tensor fixes. Rewrite for this release.
Accuracy points, each easy to get wrong:

```text
- zero-sized dimensions are now CONSTRUCTIBLE, not merely reachable by slicing
- Display shows an empty tensor's SHAPE; Debug is unchanged
- axis reductions ERROR on a zero-length reduced axis; sum_axis still returns its identity
- no new API — this release removes restrictions and changes behaviour
- do NOT say a panic was fixed (§4)
```

### 6.2 `public-api-snapshot.md` — a sentence, no new row

Its head says core's API *"most recently changed in RFC-104 … and RFC-108."* **That stays true** — no
public item was added (§3). Add a sentence in the RFC-088/RFC-102/RFC-105 style recording RFC-110 and
RFC-111 as behaviour changes with no new row, and note `ZeroSizedAxis`'s deprecation.

**This is the RFC-103 shape, not the RFC-109 shape.** Adding a row would be a defect.

## 7. CHANGELOG

`Changed` and `Version` only. No `Added`, and no empty `Added` heading.

### 7.1 Claims that would be publishable falsehoods

```text
- "fixes a panic" (§4). No released version had one.
- calling zero-sized acceptance a new feature. It removes a restriction; no API
  was added.
- mentioning Display's new empty rendering without saying Debug is unchanged.
- claiming axis reductions changed for a zero-length SURVIVING axis. Only the
  REDUCED axis's length matters; the surviving case was and stays Ok.
- claiming sum/sum_axis changed. They did not — the additive identity is correct.
- "no existing behavior changed". The entire release is behaviour change.
```

## 8. Release execution — separate and authorized separately

```text
1. push main BEFORE tagging — the tag must point at a commit on the remote
2. tag 0.46.0 — bare SemVer, no v prefix, signed, on the Prepare commit
3. publish — one `cargo publish --workspace`, verified against the sparse index
```

None is authorized by this RFC. Each comes to the owner as its own ask.

## 9. Acceptance criteria

```text
[ ] cargo metadata shows 0.46.0 for all five crates
[ ] 38 live pins retargeted; the 32 record occurrences UNCHANGED — assert both
[ ] no .rs diff except crates/matten/src/lib.rs's doc comment
[ ] CHANGELOG [0.46.0]: Changed + Version, no Added, no Deprecated section
[ ] no falsehood from §7.1 — especially §4's
[ ] introduction.md rewritten; no text describing 0.45.0 as current survives
[ ] public-api-snapshot.md: a sentence, NO new row, claim not moved
[ ] ROADMAP.md and rfcs/** untouched
[ ] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] no tag, no publish, no maturity-label change
```

## 10. Non-goals

```text
RFC-113 — the playground is publish = false
a v1.0 decision — RFC-076 remains deferred and unauthorized
any behaviour, API, dependency, feature, edition, or MSRV change
```
