# RFC-091: `0.42.0` — Statistics Release

**Status:** **Released** — prepared in commit *"Prepare 0.42.0: statistics release (RFC-091)"*,
reviewed and approved after one correction, then tagged `0.42.0` (signed, on the *Prepare* commit)
and published to crates.io on 2026-08-01. All five crates live at `0.42.0`, verified against the
sparse index, none yanked; 102 of 102 tags resolve to ancestors of `main` and all are annotated and
signed.

Two corrections, neither in the implementation. C1: `introduction.md` carried an inherited clause,
*"without any other public API, dependency, or runtime behavior change"*, accurate for `0.41.0` and
false here — it denied the two `matten-stats` error message changes §6.1 made a mandatory `Changed`
section for. **§7's publish sequence was also wrong, and the owner caught it before the first
upload:** publishing crate-by-crate risks a permanently half-published family, since crates.io has
no unpublish. `cargo publish --workspace` verifies all five before uploading any. §7 is superseded
by the release checklist's rewritten publishing section
**Target:** `0.42.0`, on the `0.x` line
**Theme:** Release RFC-090 — `matten-stats` gains `histogram`, and two existing error messages change
**Depends on:** RFC-030, RFC-064, RFC-067, RFC-086, RFC-089, RFC-090
**Related:** RFC-015, RFC-040, RFC-076, RFC-078, RFC-083, RFC-084

---

## 1. Summary

Bump the lock-step family `0.41.0` → `0.42.0` and prepare the release of one completed RFC.

This is the smallest release the project has cut: a single companion-crate function. It is worth
naming that plainly rather than dressing it up, because the honest case for it is not size.

**No tag, no publish** — separate authorized steps (§7). No blocking precondition: `0.41.0`'s
release verified all 101 tags resolve to ancestors of `main` with the signed invariant intact.

## 2. Why now — the recorded trigger did NOT fire

The `§6.4` checkpoint at RFC-090's closure recorded the next release trigger as *"a second theme
lands, or the owner wants `histogram` out sooner"*. **No second theme has landed.** Every commit
since `0.41.0` other than RFC-090's is documentation, guards, or `ROADMAP.md` — none of it reaches
crates.io.

So this release happens under the second clause: the owner elected to cut it when asked. The
high-capability model's recommendation was **to wait**, on the grounds that `0.40.0` shipped on
2026-07-30, `0.41.0` on 2026-07-31, and a third release inside forty-eight hours to publish one
function is churn against a project whose §1.1 baseline explicitly makes adoption not a success
measure.

That recommendation was heard and overridden, which is the owner's call under §6.7. It is recorded
here rather than quietly omitted, because a release RFC that implies its trigger fired when it did
not is precisely the "status fields that lie" anti-pattern RFC-000 names — and because the
counter-argument is genuinely strong: `histogram` resolves RFC-040 §8, the oldest open question in
the project, and holding a finished, reviewed feature indefinitely is the *"releasing nothing of
what was built"* failure mode §6.4 exists to catch.

## 3. Release content

### 3.1 RFC-090 — `histogram`

`matten-stats` goes from six functions to seven:

```rust
pub struct Histogram { pub counts: Vec<usize>, pub edges: Vec<f64> }
pub fn histogram(x: &Tensor, bins: usize) -> Result<Histogram, MattenStatsError>;
```

The policy is that **there is no automatic bin rule**. `bins` is required; no Sturges,
Freedman–Diaconis, Scott, Doane or `"auto"` mode exists, because each is a statistical assumption
wearing a default's clothing. Matches NumPy on the closed last bin; diverges deliberately on
constant input, erroring rather than inventing a `±0.5` range.

Two new error variants, both on a `#[non_exhaustive]` enum and therefore additive:
`InvalidBinCount` (for `bins == 0`) and `AllocationLimit { requested_bins, limit }`.

### 3.2 Two existing error messages changed — this is not an Added-only release

Easy to miss, and the reason this release is not purely additive in user-visible behaviour. Both
affect functions that **already shipped in `0.41.0`**:

```text
ZeroVariance    "correlation is undefined when either input has zero variance"
             -> "this operation is undefined when an input has zero variance"
                affects correlation, skewness, kurtosis — the old text was already wrong
                for the latter two, which involve neither a correlation nor a second input

NonFiniteValue  "... was found in the input"
             -> "... found in the input, or produced by a computation over it"
                broadened so it stays TRUE for RFC-090's C1 case: every input finite,
                but the derived hi - lo overflowing to infinity
```

Neither is a type-level break. Both change strings a user may be reading, so both belong under
`Changed`.

### 3.3 Not in this release, despite landing since `0.41.0`

Everything else since the tag is invisible to crates.io users and must not appear in the CHANGELOG
as though it shipped: the six corrected `Tensor` signatures in `docs/src/reference/`, the book's
converted self-verifying result blocks, `scripts/check-doc-code.sh`, the guard-audit repairs, and
the `ROADMAP.md` history rows.

## 4. Scope

### In scope

```text
version bump 0.41.0 -> 0.42.0, lock-step, all five crates (Cargo.toml + Cargo.lock)
the 37 live version strings across 17 files (§5)
CHANGELOG [0.42.0] entry (§6)
```

### Out of scope — a diff touching these is a defect

```text
any .rs change other than crates/matten/src/lib.rs's install-pin doc comment
any API, feature, dependency, edition, or MSRV change
any maturity-label movement — all five labels stay exactly as they are
CHANGELOG released entries, ROADMAP history rows, rfcs/** version references
the tag and the publish (§7)
```

## 5. Version-string retarget — 37 strings across 17 files

Measured at `48e857b` with **`0\.41\b`, not `0\.41\.[0x]`** — the correction recorded at ROADMAP
`3.41.0`, after RFC-086 §6 measured with the suffixed pattern and missed a site:

```text
README.md                                    11    docs/src/examples/data.md                  4
crates/matten-stats/README.md                 3    crates/matten-ndarray/README.md            3
crates/matten-mlprep/README.md                3    crates/matten-data/README.md               2
Cargo.toml                                    1    crates/matten/README.md                    1
crates/matten/src/lib.rs                      1    docs/src/quick-start.md                    1
docs/src/introduction.md                      1    docs/src/contributing/architecture.md      1
docs/src/contributing/release-checklist.md    1    docs/src/reference/boundary.md             1
docs/src/reference/compatibility.md           1    docs/src/reference/dynamic.md              1
docs/src/reference/public-api-snapshot.md     1
```

**The bare-form pattern earns its keep again.** The suffixed pattern finds only 35 of these; the two
it misses are:

```text
docs/src/introduction.md:17                  "the current 0.41 release family"
docs/src/reference/public-api-snapshot.md:3  "at the current v0.41 release"
```

**One of those two needs a CONTENT change, and one does not** — a distinction `0.41.0` did not have
to make, so do not copy that release's handling:

- `introduction.md:17` says *"an RFC-089 release"*. Retargeting the number alone leaves it naming
  the wrong RFC. It must name RFC-090 and describe this release's content.
- `public-api-snapshot.md:3` covers **core `matten`**, whose public surface is unchanged by this
  release — `histogram` is in `matten-stats`. It needs the number and nothing else. Adding
  RFC-090 to it would be an outright false claim about core's surface.

**43 further occurrences are historical and must NOT change**: `CHANGELOG.md` (2), `ROADMAP.md`
(13), `rfcs/**` (28).

The implementer must re-measure — the count moves with every commit, and this RFC's own text adds
occurrences to `rfcs/`.

## 6. CHANGELOG

### 6.1 Required content

```text
Added    — matten-stats: histogram + Histogram (§3.1), the no-automatic-bin-rule policy,
           the closed last bin, the constant-input error; InvalidBinCount and
           AllocationLimit variants, additive on a #[non_exhaustive] enum
Changed  — matten-stats: ZeroVariance and NonFiniteValue message text (§3.2)
Version  — lock-step family bump, all five crates
```

**No `Maturity` section.** All five labels are unchanged from `0.41.0`; inventing a section to say
"no change" invites the silent-promotion reading RFC-067 forbids. `matten-stats` in particular
stays **production-ready candidate** — RFC-084 §8 tied full production to usage history the project
has decided not to measure, and a seventh function does not change that.

### 6.2 What not to claim

```text
do not describe histogram as NumPy-compatible — the constant-input case diverges deliberately
do not imply any automatic/auto bin rule exists — the absence of one IS the policy
do not imply matrix-wide or axis-wise covariance/correlation are now unblocked — RFC-090 §5's
  boundary amendment was written specifically so that they are not
do not present the two message changes as cosmetic — ZeroVariance's old text was WRONG for
  skewness and kurtosis, which is why it changed
do not describe this as a core release — core matten is untouched
```

## 7. Release execution — separate and authorized

```text
1. tag 0.42.0 — bare SemVer, no v prefix, signed (all 101 existing tags are GPG-signed)
2. publish in dependency order: matten first, then matten-ndarray, matten-mlprep,
   matten-data, matten-stats
3. post-release status alignment commit — normal RFC flow
```

`matten` must be published first; companion dry-runs may fail before core is visible on crates.io,
which the release checklist records as a sequencing caveat, not a dependency-policy failure.

Note that four of the five crates will be republished at `0.42.0` with **no change other than the
version**. That is inherent to lock-step versioning (RFC-030) and is not a defect.

## 8. Acceptance criteria

```text
[ ] version 0.42.0 in Cargo.toml and Cargo.lock, all five crates, verified by cargo metadata
[ ] the live strings retargeted, count RE-MEASURED with `0\.41\b` — not the suffixed pattern
[ ] introduction.md gets a CONTENT update naming RFC-090; public-api-snapshot.md gets the
    NUMBER ONLY, because core's surface is unchanged (§5)
[ ] zero change to CHANGELOG released entries, ROADMAP history rows, or rfcs/**
[ ] CHANGELOG [0.42.0] entry per §6.1, WITH a Changed section, and none of §6.2's over-claims
[ ] no Maturity section — labels unchanged, matten-stats still production-ready candidate
[ ] the only .rs change is crates/matten/src/lib.rs's install-pin doc comment
[ ] full gate set: fmt, clippy, workspace tests, doctests, MSRV, mdbook, all seven guards
    (check-doc-code.sh is new since the last release and is now part of the set)
[ ] no tag, no publish, no API change
```

## 9. Non-goals

```text
v1.0 preparation — RFC-076 stays deferred; v1.0 is not currently wanted
any new feature, API change, or maturity promotion
the matrix-wide / axis-wise covariance and correlation forms RFC-083 §6 deferred
a matten-linalg-lite decision — RFC-041 §6.2's recommendation stands unratified
```
