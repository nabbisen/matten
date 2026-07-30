# RFC-089: `0.41.0` — Core Shape and Slicing Release

**Status:** **Released** — prepared in commit *"Prepare 0.41.0: core shape and slicing release
(RFC-089)"*, reviewed and approved after one correction, then tagged `0.41.0` (signed, on the
*Prepare* commit) and published to crates.io on 2026-07-31. All five crates live at `0.41.0`,
verified against the registry; the release **matched the planned scope exactly**, needing no
post-release correction. §7's execution sequence is complete.

The correction (C1) was not in the implementation: `ROADMAP.md`'s Status paragraph carried a stale
clause attributing RFC-077's seeded split — `0.39.0`'s content — to `0.40.0`, because an earlier
partial edit at ROADMAP `3.39.0` had replaced only that sentence's opening. A mechanical version
retarget then swept the leftover up. Re-anchored to `0.40.0`'s actual content, with `0.39.0`'s own
story delegated to rows `3.25.0`/`3.26.0` rather than restated
**Target:** `0.41.0`, on the `0.x` line
**Theme:** Release RFC-087 and RFC-088 — the first two themes chosen against §1.1's planning baseline
**Depends on:** RFC-030, RFC-064, RFC-067, RFC-086, RFC-087, RFC-088
**Related:** RFC-008, RFC-015, RFC-039, RFC-076

---

## 1. Summary

Bump the lock-step family `0.40.0` → `0.41.0` and prepare the release of two completed RFCs, both
core public-surface work.

**No tag, no publish** — separate authorized steps (§7). **No blocking precondition this time:** the
orphaned-tag defect that gated `0.40.0` was repaired during that release, and all 100 tags now resolve
to ancestors of `main`.

## 2. Why now — the trigger fired as recorded

`0.40.0` shipped 2026-07-30. At RFC-087's disposition checkpoint the release-readiness question
(governance policy §6.4) was answered **"not yet"**, with the condition written down rather than left
to memory: *one further feature slice should prompt `0.41.0`*.

RFC-088 is that slice. The check has now returned both answers on consecutive uses, which is the
evidence it is a decision rather than a formality.

```text
RFC-087   repeat, repeat_axis, tile, meshgrid (+ try_ forms)   8 core functions
RFC-088   negative indices in slice_str                        grammar extension
```

Both are user-visible, both are education-facing under §1.1, and neither is currently reachable by
anyone reading docs.rs.

## 3. Release content

### 3.1 RFC-087 — shape composition

```text
Tensor::repeat / try_repeat              element repetition, flattens to rank-1
Tensor::repeat_axis / try_repeat_axis    element repetition along one axis, rank preserved
Tensor::tile / try_tile                  whole-tensor repetition
Tensor::meshgrid / try_meshgrid          two rank-1 inputs -> a coordinate-grid pair
```

Three notes the CHANGELOG must carry, because each is a decision a reader could otherwise get wrong:

- **`repeat` repeats elements; `tile` repeats the whole tensor.** The classic confusion; state both
  with an example.
- **`meshgrid` uses NumPy's `xy` indexing**, so for `x` of length `m` and `y` of length `n` both
  outputs are `[n, m]`.
- **`tile` rejects `reps` longer than the tensor's rank**, deliberately diverging from NumPy's silent
  rank promotion (RFC-087 §4).

### 3.2 RFC-088 — negative slice indices

```text
"-1"  last element      "0:-1"  all but the last      "-2:"  last two      "-1,:"  last row
```

Two notes required:

- **`slice_str` only.** The builder is unchanged and takes no negatives (RFC-088 §4).
- **Out of range errors; it does not clamp**, diverging from Python's clamping of range bounds.

## 4. Scope

### In scope

```text
version bump 0.40.0 -> 0.41.0 (Cargo.toml + Cargo.lock, all five crates — lock-step is unconditional)
37 current-family version-string retargets across 17 files (§5)
CHANGELOG.md [0.41.0] entry (§6)
ROADMAP.md release-table row, Status, history row, header bump
rfcs/README.md tracking
```

### Out of scope — a diff touching these is a defect

```text
any crates/*/src/*.rs change except crates/matten/src/lib.rs's install-pin doc comment
any public API change — the surface ships exactly as RFC-088 left it
public-api-snapshot.md's CONTENT — RFC-087 already updated its rows; only its
  version string retargets here (§5)
any dependency, feature, edition, or MSRV change
any maturity-label change — all five crates keep the labels 0.40.0 shipped
any pre-1.0 / 0.x wording change — this is a 0.x release
RFC-076, and compatibility.md's v1.0 requirements section
tag creation and crates.io publishing
```

## 5. Version-string retarget — 37 strings across 17 files

Measured at `735ddd5` **with `0\.40\b`, not `0\.40\.[0x]`** — applying the correction recorded at
ROADMAP `3.41.0` after RFC-086 §6 measured with the suffixed pattern and missed a site:

```text
README.md                                     11      docs/src/examples/data.md               4
crates/matten-ndarray/README.md                3      crates/matten-mlprep/README.md          3
crates/matten-stats/README.md                  3      crates/matten-data/README.md            2
Cargo.toml                                     1      crates/matten/README.md                 1
crates/matten/src/lib.rs                       1      docs/src/quick-start.md                 1
docs/src/contributing/architecture.md          1      docs/src/contributing/release-checklist.md  1
docs/src/reference/boundary.md                 1      docs/src/reference/compatibility.md     1
docs/src/reference/dynamic.md                  1      docs/src/reference/public-api-snapshot.md   1
docs/src/introduction.md                       1
```

**The lesson paid for itself immediately.** The old suffixed pattern finds only 35 of these. The two it
misses are exactly the bare-form sites:

```text
docs/src/introduction.md:17               "the current 0.40 release family"
docs/src/reference/public-api-snapshot.md:3  "at the current v0.40 release family"
```

Both also need a **content** update, not just a number — `introduction.md` still describes RFC-086's
release content, and `public-api-snapshot.md`'s opening says core's surface changed "in RFC-087",
which stays true but should now name RFC-088 as well.

**46 further occurrences are historical and must NOT change**: `CHANGELOG.md` (2), `ROADMAP.md` (8),
`rfcs/**` (36).

The implementer must re-measure — the count moves with every commit.

## 6. CHANGELOG

### 6.1 Required content

```text
Added    — RFC-087's eight functions, with the repeat-vs-tile contrast, meshgrid's xy
           convention, and tile's rank rejection (§3.1)
Changed  — RFC-088's slice_str grammar: negative indices accepted; slice_str only;
           out of range errors rather than clamping (§3.2)
Version  — lock-step family bump, all five crates
```

**No `Maturity` section this release.** All five labels are unchanged from `0.40.0`; inventing a
section to say "no change" would invite the silent-promotion reading RFC-067 forbids.

### 6.2 What not to claim

```text
do not describe negative indices as "NumPy-compatible" — the clamping behaviour differs
do not describe tile as NumPy-compatible — the rank rule differs deliberately
do not imply the builder gained negative indices — it did not
```

## 7. Release execution — separate and authorized

```text
1. tag 0.41.0 — bare SemVer, no v prefix, signed (all 100 existing tags are GPG-signed)
2. publish in dependency order: matten first, then matten-ndarray, matten-mlprep,
   matten-data, matten-stats
3. post-release status alignment commit — normal RFC flow
```

`matten` must be published first; companion dry-runs may fail before core is visible on crates.io,
which the release checklist records as a sequencing caveat rather than a dependency-policy failure.

**No tag precondition.** Verified after the `0.40.0` release: `origin/main` matches local `HEAD`, and
100 of 100 remote tags resolve to ancestors of `origin/main` with the signed invariant intact.

## 8. Acceptance criteria

```text
[ ] version 0.41.0 in Cargo.toml and Cargo.lock, all five crates, verified by cargo metadata
[ ] the live strings retargeted, count RE-MEASURED with `0\.40\b` — not the suffixed pattern
[ ] introduction.md and public-api-snapshot.md get CONTENT updates, not just numbers (§5)
[ ] zero change to CHANGELOG released entries, ROADMAP history rows, or rfcs/**
[ ] CHANGELOG [0.41.0] entry per §6.1, with none of §6.2's over-claims
[ ] no Maturity section — labels are unchanged (§6.1)
[ ] the only .rs change is crates/matten/src/lib.rs's install-pin doc comment
[ ] full gate set: fmt, clippy, workspace tests, doctests, MSRV, mdbook, all guards
[ ] no tag, no publish, no API change
```

## 9. Non-goals

```text
v1.0 preparation — RFC-076 stays deferred; v1.0 is not currently wanted
any new feature, API change, or maturity promotion
resolving RFC-088 §8's zero-sized-dimension inconsistency
```
