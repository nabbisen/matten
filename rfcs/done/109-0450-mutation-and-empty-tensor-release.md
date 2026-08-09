# RFC-109: `0.45.0` — Mutation and Empty-Tensor Release

**Status:** **Prepared** 2026-08-09 in commit *"Prepare 0.45.0: mutation and empty-tensor release
(RFC-109)"*, reviewed and approved with **no corrections**. **Not tagged, not published** — both remain
separate owner authorizations. Accepted by the owner, who directed the cut ("Cut it"), though unlike
`0.42.0` and `0.44.0` this release is **not an override**: RFC-094's two-theme trigger had already
fired, with three themes. Handoff: `rfcs/handoffs/109-0450-mutation-and-empty-tensor-release-handoff.md`.
**Target:** `0.45.0`, on the `0.x` line
**Related:** RFC-030 (lock-step), RFC-094 (cadence), RFC-104, RFC-105, RFC-108

---

## 1. Summary

Bump the lock-step family `0.44.0` → `0.45.0` and prepare the release of RFC-104, RFC-105, and
RFC-108. Preparation only: **no tag, no publish** — both remain separate owner authorizations.

## 2. Why now — the cadence trigger fired, and there is a live defect

RFC-094's triggers:

```text
two themes accumulated  -> YES. Three: RFC-104, RFC-105, RFC-108.
28 days since the last  -> no. 0.44.0 shipped 2026-08-09, the same day.
the owner asks          -> YES, and the owner directed the cut.
```

Unlike `0.42.0` and `0.44.0`, **this release is not an owner override** — the two-theme trigger
fired on its own terms first. Recorded so the release table reads accurately.

**The stronger reason is a live defect.** `0.44.0` is published and contains a raw panic that escapes
`try_dot`'s `Result` (RFC-108 §2). Anyone slicing a matrix to zero columns and multiplying hits it
today, and the fix is unshipped. This is a fix users cannot get, not a feature they are waiting on.

## 3. Release content — three themes, both an `Added` and a `Changed`

```text
RFC-104  get_mut / get_flat_mut / get_element_mut          ADDED
RFC-105  mean/min/max/argmin/argmax error on empty
         instead of panicking or leaking NaN/inf           CHANGED
RFC-108  mm_mul zero-column panic fixed                    CHANGED
         Tensor::is_empty()                                ADDED
```

**This is the first release in this sequence carrying both sections.** `0.43.0` was `Added`-only and
`0.44.0` was `Changed`-only, so **both prior releases' shapes are wrong to pattern-match against**.

### 3.1 Two `Changed` entries read as improvements but are behaviour changes

Both are fixes. Both are also observable:

```text
RFC-105  code relying on min() == inf, or on catching argmin's index panic, now
         gets an Err (or a panic carrying a sentence). Intended.
RFC-108  code catching the "chunk size must be non-zero" panic now gets Ok([m,0]).
         Intended.
```

State them as changes, not only as fixes. A user matching on the old behaviour needs to see it.

## 4. Scope

```text
Cargo.toml       0.44.0 -> 0.45.0 (workspace, lock-step, all five crates)
Cargo.lock       regenerated, committed
38 live version pins across 17 files (§5)
CHANGELOG.md     a [0.45.0] entry: Added + Changed + Version (§6)
introduction.md  CONTENT rewrite — the current text describes 0.44.0 (§5.1)
public-api-snapshot.md  CONTENT update — core's surface genuinely changed (§5.2)
```

### Out of scope — a diff touching these is a defect

```text
any .rs file except the install-pin doc comment at crates/matten/src/lib.rs
any behaviour, test, or guard
the 41 record occurrences in rfcs/**, ROADMAP.md, CHANGELOG.md's existing entries
RFC-106 Stage 2 or Stage 3 — neither is in this release
a tag, a publish, or a maturity-label change
```

## 5. Version retarget — 38 live pins across 17 files, 41 records that must not move

`0\.44\b` matches **79 lines** in tracked `md`/`toml`/`rs`/`yml` files, measured by enumerating
`git ls-files`, excluding `Cargo.lock` by **exact path**, and counting per file — not a tree-wide
`grep -rn | wc -l`, which has been wrong twice before (RFC-103 §5.0).

```text
41 records   rfcs/** (32), ROADMAP.md (3), CHANGELOG.md (2) and the RFC-103/104/105/
             107/108 documents that narrate their own releases. MUST NOT MOVE.
38 live      the same 17 files as the last release: README.md (11),
             docs/src/examples/data.md (4), the three companion READMEs (3 each),
             introduction.md (2), matten-data/README.md (2), and one each in
             public-api-snapshot.md, dynamic.md, compatibility.md, boundary.md,
             quick-start.md, release-checklist.md, contributing/architecture.md,
             crates/matten/src/lib.rs, crates/matten/README.md, Cargo.toml.
```

The two new RFC-107 pages (`docs/src/architecture.md`, `reference/data-model.md`) carry **no** version
pin — verified, they are absent from the match set. Nothing new to retarget there.

### 5.1 `introduction.md` — rewrite, not renumber

The current paragraph describes `0.44.0` as *"an RFC-102 release"* and details dynamic slicing. Under
`0.45.0` that is a description of the previous release sitting on the documentation's front page.
Rewrite it for RFC-104/105/108.

### 5.2 `public-api-snapshot.md` — a real content update this time

Its head says core's API *"most recently changed in RFC-099 … and RFC-100."* **That is now false.**
RFC-104 added three public methods and RFC-108 added a fourth:

```text
get_mut, get_flat_mut, get_element_mut   (RFC-104)
is_empty                                 (RFC-108)
```

Move the "most recently changed" claim to RFC-104/RFC-108 and add the four items to the page's
inventory. **This is the opposite of RFC-103's instruction**, where RFC-102 changed no public item
and a new row would have been a defect. Here, omitting them would be.

## 6. CHANGELOG

A `[0.45.0]` entry with **`Added`, `Changed`, and `Version`**.

### 6.1 Claims that would be publishable falsehoods

```text
- "no existing behavior changed". Two entries are behaviour changes (§3.1).
- describing RFC-108's matmul fix as a new capability. It removes a panic from an
  operation that already existed.
- mentioning get_element_mut's storage sharing without the materialize-on-write
  consequence, or vice versa. compatibility.md and the reference pages ship both
  halves; the CHANGELOG must not ship only one.
- claiming is_empty() is new *behaviour*. The state it reports was always reachable;
  what was missing was the method.
- any suggestion that zero-sized dimensions are now constructible. They are not —
  that is RFC-106 Stage 3 and it is not in this release.
```

## 7. Release execution — separate and authorized separately

```text
1. tag 0.45.0 — bare SemVer, no v prefix, signed, on the Prepare commit
2. publish — one `cargo publish --workspace`
```

Neither is authorized by this RFC. Both come to the owner as **two separate asks** after review, per
the `0.43.0`/`0.44.0` precedent. `main` must be pushed **before** the tag, or the tag points at a
commit absent from the remote — the orphaned-tag defect this project repaired once already.

## 8. Acceptance criteria

```text
[x] cargo metadata shows 0.45.0 for all five crates
[x] 38 live pins retargeted; the 41 record occurrences UNCHANGED — assert both
[x] no .rs diff except crates/matten/src/lib.rs's doc comment
[x] CHANGELOG [0.45.0]: Added + Changed + Version, none of them empty
[x] introduction.md rewritten; no text describing 0.44.0 as current survives
[x] public-api-snapshot.md: the four new items added, the "most recently changed"
    claim moved to RFC-104/RFC-108
[x] ROADMAP.md and rfcs/** untouched
[x] eight guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[x] cargo clippy --workspace --all-targets --all-features -- -D warnings (the CI form)
[x] cargo test --workspace; both feature profiles build
[x] no tag, no publish, no maturity-label change
```

## 9. Non-goals

```text
RFC-106 Stage 2 (axis reductions) and Stage 3 (the shape-model decision)
a v1.0 decision — RFC-076 remains deferred and unauthorized
any behaviour, API, dependency, feature, edition, or MSRV change
```
