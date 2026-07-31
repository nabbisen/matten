# RFC-091 `0.42.0` Release Preparation: Implementation Handoff

**Status:** Issued 2026-08-01. Implementation authorized under RFC-091, accepted the same day.
**Design authority:** `rfcs/proposed/091-0420-statistics-release.md`. Where this handoff and the RFC
disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Prepare `0.42.0`: bump the lock-step family, retarget the live version strings, and write the
CHANGELOG entry. **Do not tag. Do not publish.** Those are separate steps the owner authorizes
individually (RFC-091 §7).

This release publishes exactly one thing — RFC-090's `histogram`. Everything else committed since
`0.41.0` is documentation, guards, or `ROADMAP.md`, and none of it belongs in the CHANGELOG
(RFC-091 §3.3).

## 2. Version bump

```bash
# Cargo.toml — the single workspace version
# then refresh the lock so Cargo.lock's five entries follow
cargo update --workspace
cargo metadata --format-version 1 --no-deps | grep -oE '"name":"matten[a-z-]*","version":"[^"]*"'
# expect all five at 0.42.0
```

## 3. Version-string retarget — RE-MEASURE first, with the broad pattern

RFC-091 §5 measured 37 live strings across 17 files at `48e857b`. **That count is already stale:**
RFC-091 and this handoff both add `0.41` occurrences under `rfcs/`. Re-measure before you start.

```bash
grep -rn '0\.41\b' --include='*.md' --include='*.toml' --include='*.rs' . \
  | grep -v 'target/' | sed 's|^\./||' \
  | grep -vE '^(CHANGELOG\.md|ROADMAP\.md|rfcs/)'
```

Use `0\.41\b`, **not** `0\.41\.[0x]`. The suffixed form misses the two bare-form sites (`"the
current 0.41 release family"`, `"at the current v0.41 release"`) — the exact miss RFC-086 §6 made,
recorded as a correction at ROADMAP `3.41.0`.

Note your local `grep` may print paths **without** a leading `./`, which silently breaks a
`^\./(...)` exclusion and inflates the live count by the historical files. The pipeline above
normalises with `sed` first. Verify your live count excludes `CHANGELOG.md`, `ROADMAP.md` and
`rfcs/`.

### 3.1 What must NOT change

`CHANGELOG.md` released entries, `ROADMAP.md` history rows, and everything under `rfcs/` carry
historical version references. Roughly 43 occurrences at measurement time.

```bash
git diff --name-only
# expect no rfcs/** file other than this release's own disposition edits
git diff CHANGELOG.md | grep -E '^-' | grep -v '^---'
# expect EMPTY — no released entry touched, only the new [0.42.0] section added
```

## 4. One file needs content; one must NOT get it

This differs from `0.41.0`. **Do not copy that release's handling of these two files.**

- **`docs/src/introduction.md:17`** — currently *"the current 0.41 release family, an RFC-089
  release"*. Bumping the number alone leaves it naming the wrong RFC. Rewrite to name **RFC-090**
  and describe this release: `matten-stats` gains `histogram`.

- **`docs/src/reference/public-api-snapshot.md:3`** — currently *"at the current v0.41 release"*.
  This page covers **core `matten`**, whose public surface this release does **not** touch;
  `histogram` lives in `matten-stats`. Change the **number only**. Adding RFC-090 here would be a
  false claim about core's surface.

## 5. CHANGELOG

Per RFC-091 §6.1. **A `Changed` section is mandatory** — this is not an Added-only release.

```text
Added    matten-stats: histogram + Histogram; bins is REQUIRED and there is no automatic
         rule (Sturges/FD/Scott/auto are all absent, deliberately); closed last bin;
         constant input errors rather than inventing a range. New variants
         InvalidBinCount and AllocationLimit — additive, the enum is #[non_exhaustive]

Changed  matten-stats: ZeroVariance message "correlation is undefined when either input
         has zero variance" -> "this operation is undefined when an input has zero
         variance". It already applied to skewness and kurtosis, for which the old text
         was wrong. NonFiniteValue message broadened with "or produced by a computation
         over it", so it stays true when every input is finite but the derived range
         overflows

Version  lock-step family bump, all five crates
```

**No `Maturity` section.** All five labels unchanged. `matten-stats` stays **production-ready
candidate** — RFC-084 §8 tied full production to usage history the project does not measure.

Do not claim any of RFC-091 §6.2's five over-claims. In particular do not call `histogram`
NumPy-compatible (constant input diverges) and do not present the message changes as cosmetic.

## 6. Verification

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --doc --all-features
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
mdbook build docs
```

**Seven guards now, not six.** `scripts/check-doc-code.sh` is new since `0.41.0`: it compiles every
non-ignored ```rust block in `docs/src/` and **runs** the runnable ones. If your edit to
`introduction.md` breaks a code block, this is what will catch it.

```bash
git diff --name-only -- '*.rs'
# expect ONLY crates/matten/src/lib.rs (its install-pin doc comment)
grep -rn '0\.41\b' --include='*.md' --include='*.toml' --include='*.rs' . \
  | grep -v 'target/' | sed 's|^\./||' | grep -vE '^(CHANGELOG\.md|ROADMAP\.md|rfcs/)'
# expect EMPTY
```

## 7. Known pitfalls

```text
- measuring with 0\.41\.[0x] and missing the two bare-form sites (§3)
- an exclusion pattern anchored on ./ that your grep does not emit (§3)
- giving public-api-snapshot.md a content update it must not have (§4)
- omitting the Changed section because "we only added a function" (§5)
- sweeping ROADMAP/CHANGELOG/rfcs history into the retarget (§3.1)
- tagging or publishing — neither is authorized by this handoff (RFC-091 §7)
```

## 8. What the review request must report

```text
- the RE-MEASURED live count and file list, and the pattern used
- cargo metadata output showing five crates at 0.42.0
- the two §4 files, quoted before and after, showing one got content and one did not
- the CHANGELOG entry in full
- full gate output including all SEVEN guards
- confirmation that git diff --name-only -- '*.rs' shows only crates/matten/src/lib.rs
- confirmation that no tag was created and nothing was published
```

## 9. Review stop

Stop after committing the preparation. Report, and the high-capability model reviews before the
owner is asked to authorize the tag.
