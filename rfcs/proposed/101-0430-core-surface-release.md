# RFC-101: `0.43.0` — Core Surface Release

**Status:** Proposed
**Target:** `0.43.0`, on the `0.x` line
**Theme:** Release RFC-099 and RFC-100 — two additions to core's public surface
**Depends on:** RFC-030, RFC-064, RFC-067, RFC-094, RFC-096, RFC-099, RFC-100
**Related:** RFC-015, RFC-020, RFC-076, RFC-091, RFC-093 §8

---

## 1. Summary

Bump the lock-step family `0.42.0` → `0.43.0` and prepare the release of two completed RFCs, both
core public-surface work, plus two changes that have been waiting for a release to carry them.

**No tag, no publish** — separate authorized steps (§7).

## 2. Why now — the trigger fired, and this time it fired properly

RFC-094 §4.2(a): *two or more themes have landed unreleased*. Two have:

```text
RFC-099   try_dot / try_matmul        new public API
RFC-100   Display for Tensor          new public API
```

This is worth contrasting with `0.42.0`, which RFC-091 §2 had to record as proceeding on an owner
override because its trigger had **not** fired — one function, and four of five crates republished
unchanged. The policy written after that release has now done the job it was written for: it held
through a README-only period, and released when there was something to release.

Two further changes ride along, neither a trigger on its own:

```text
RFC-096   the grid rendering in 57_visual_shape_axis_summary
          the crates/matten/README.md playground link — reaches crates.io for the first time
```

## 3. Release content

### 3.1 RFC-099 — `try_dot` and `try_matmul`

```rust
pub fn try_dot(&self, rhs: &Tensor) -> Result<Tensor, MattenError>;
pub fn try_matmul(&self, rhs: &Tensor) -> Result<Tensor, MattenError>;
```

Core's last two panic-only operations gain Result forms, matching the 41 `try_*` functions that
already existed. **No message changed**, verified against captures taken before the work.
`dot` and `matmul` still panic exactly as before.

### 3.2 RFC-100 — `Display for Tensor`

Rank ≤ 2 renders as an aligned grid; rank > 2 keeps the flat form. `{:#}` means untruncated.
Truncation at 12 columns. Dynamic tensors render, with `Float` distinguishable from `Int`.

**`Debug` is unchanged**, byte-for-byte. RFC-020 owns it, and the two forms are deliberately
different: `Debug` for developers, `Display` for humans.

### 3.3 This is an Added-only release — unlike `0.42.0`

Nothing user-visible changed behaviour. RFC-099 preserved every message; RFC-100 added a trait impl
without touching `Debug`. So the CHANGELOG needs **no `Changed` section**, which is the inverse of
`0.42.0`, where two error strings moved and RFC-091 §6.1 had to make `Changed` mandatory.

Do not add an empty `Changed` heading to mirror the last release.

## 4. Scope

### In scope

```text
version bump 0.42.0 -> 0.43.0, lock-step, all five crates (Cargo.toml + Cargo.lock)
the 39 live version strings across 17 files (§5)
CHANGELOG [0.43.0] entry (§6)
the O1 carry-forward (§5.2)
```

### Out of scope — a diff touching these is a defect

```text
any .rs change other than crates/matten/src/lib.rs's install-pin doc comment
any API, feature, dependency, edition, or MSRV change
any maturity-label movement — all five labels stay as they are
CHANGELOG released entries, ROADMAP history rows, rfcs/** version references
the tag and the publish (§7)
```

## 5. Version-string retarget — 39 strings across 17 files

Measured at `4fc37ab` with **`0\.42\b`, not `0\.42\.[0x]`** — the correction recorded at ROADMAP
`3.41.0`:

```text
README.md                                    11    docs/src/examples/data.md                  4
crates/matten-stats/README.md                 3    crates/matten-ndarray/README.md            3
crates/matten-mlprep/README.md                3    docs/src/introduction.md                   2
docs/src/contributing/release-checklist.md    2    crates/matten-data/README.md               2
crates/matten/README.md                       1    crates/matten/src/lib.rs                   1
Cargo.toml                                    1    docs/src/quick-start.md                    1
docs/src/contributing/architecture.md         1    docs/src/reference/boundary.md             1
docs/src/reference/compatibility.md           1    docs/src/reference/dynamic.md              1
docs/src/reference/public-api-snapshot.md     1
```

**44 further occurrences are historical and must NOT change**: `CHANGELOG.md` (2), `ROADMAP.md`
(11), `rfcs/**` (31).

Re-measure before starting — this RFC's own text adds occurrences under `rfcs/`.

### 5.1 Both bare-form sites need CONTENT this time — the inverse of `0.42.0`

The suffixed pattern misses two sites, the same two as last release. **But their treatment inverts.**

```text
docs/src/introduction.md:19          "the current 0.42 release family, an RFC-090 release"
docs/src/reference/public-api-snapshot.md:3   "at the current v0.42 release"
```

For `0.42.0`, RFC-091 §5 required `public-api-snapshot.md` to get **the number only**, because
`histogram` was a companion addition and core's surface was untouched. Citing the RFC there would
have been a false claim about core.

**This release changes core's public surface**, so that page needs a content update it did not need
last time. Its opening currently reads:

> Core `matten`'s public API changed in RFC-087, which added `repeat`, `repeat_axis`, `tile`, and
> `meshgrid` … the first change to this page in a while.

That is now stale: RFC-099 added two functions and RFC-100 added a trait impl. Both belong there.

`introduction.md` names RFC-090 and must name RFC-099 and RFC-100 instead.

### 5.2 The O1 carry-forward, now due

RFC-093's review recorded an observation deferred to "the next release RFC". This is it.

`public-api-snapshot.md`'s opening enumerates recent companion-crate work that did *not* touch core
— *"The RFC-082 streaming feature and RFC-083 functions before it…"* — and RFC-090's `histogram` is
missing from that list. Add it while editing the same paragraph.

## 6. CHANGELOG

### 6.1 Required content

```text
Added    — try_dot / try_matmul (§3.1): Result forms for core's last two panic-only
           operations; the panicking forms are unchanged
         — Display for Tensor (§3.2): aligned grid for rank <= 2, flat form above,
           {:#} untruncated, dynamic tensors rendered; Debug unchanged
Version  — lock-step family bump, all five crates
```

**No `Changed` section** (§3.3). **No `Maturity` section** — all five labels are unchanged, and
inventing one to say "no change" invites the silent-promotion reading RFC-067 forbids.

### 6.2 What not to claim

```text
do not describe Display as matching ndarray — it deliberately renders 1.0 where ndarray
  renders 1, and the divergence is the point (RFC-100 §5.2)
do not imply Debug changed, or that Display replaces it
do not describe try_dot/try_matmul as new capability — the same inputs succeed and fail
  as before; what is new is being able to handle the failure
do not claim Display is a stable parsing target — it truncates (RFC-100 §8)
```

## 7. Release execution — separate and authorized

```text
1. tag 0.43.0 — bare SemVer, no v prefix, signed (all 102 existing tags are GPG-signed)
2. publish with `cargo publish --workspace` — ONE command, not five
3. post-release status alignment commit — normal RFC flow
```

**Note the change from RFC-091 §7**, which prescribed publishing `matten` first and then each
companion. That was superseded during the `0.42.0` release: `cargo publish --workspace` resolves the
order itself and verifies every crate before uploading any, so a failure in the last companion aborts
before core is irreversibly live. The release checklist carries the reasoning.

Verify afterwards against the **sparse index**, not the JSON API, which returns HTTP 403 under the
crates.io data-access policy.

## 8. Acceptance criteria

```text
[ ] version 0.43.0 in Cargo.toml and Cargo.lock, all five crates, verified by cargo metadata
[ ] the live strings retargeted, count RE-MEASURED with `0\.42\b`
[ ] BOTH introduction.md and public-api-snapshot.md get CONTENT updates (§5.1) — the
    inverse of last release, where the snapshot took the number only
[ ] public-api-snapshot.md's companion list gains RFC-090 (§5.2)
[ ] zero change to CHANGELOG released entries, ROADMAP history rows, or rfcs/**
[ ] CHANGELOG [0.43.0] per §6.1: Added and Version only, no Changed, no Maturity
[ ] none of §6.2's four over-claims appears
[ ] the only .rs change is crates/matten/src/lib.rs's install-pin doc comment
[ ] full gate set: fmt, clippy, tests, doctests, MSRV, mdbook, all EIGHT guards
[ ] no tag, no publish, no API change
```

## 9. Non-goals

```text
v1.0 preparation — RFC-076 stays deferred
any new feature, API change, or maturity promotion
batched matmul (RFC-098, archived) or the mutation API
```
