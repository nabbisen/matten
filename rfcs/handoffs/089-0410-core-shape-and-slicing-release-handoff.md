# RFC-089 `0.41.0` Release Preparation: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/089-0410-core-shape-and-slicing-release.md`
**Document kind:** Detailed implementation handoff
**Status:** Inherits RFC-089's state — **implemented and reviewed 2026-07-31** (approved after one correction). Preparation complete; tagging and publishing remain outstanding owner actions
**Date:** 2026-07-31

---

## 1. Purpose

Prepare `0.41.0`: bump the lock-step family, retarget the live version strings, and write the
`[0.41.0]` CHANGELOG entry for RFC-087 and RFC-088.

**You do not tag and you do not publish.** Both are owner actions. Unlike `0.40.0` there is no
blocking precondition — the orphaned-tag defect was repaired during that release.

## 2. Version bump

```text
Cargo.toml   [workspace.package] version = "0.41.0"
Cargo.lock   regenerate via `cargo build --workspace`; do not hand-edit
```

Verify with `cargo metadata`, not by reading the manifest:

```bash
cargo metadata --no-deps --format-version 1 \
  | python3 -c "import json,sys; [print(p['name'],p['version']) for p in json.load(sys.stdin)['packages']]"
# expect all five at 0.41.0
```

## 3. Version-string retarget — use the broad pattern

RFC-089 §5 measured **37 strings across 17 files** at `735ddd5`. **Re-measure before editing** — the
count moves with every commit.

```bash
for f in README.md Cargo.toml crates/matten/README.md crates/matten/src/lib.rs \
         crates/matten-ndarray/README.md crates/matten-mlprep/README.md \
         crates/matten-data/README.md crates/matten-stats/README.md \
         $(find docs/src -name '*.md'); do
  c=$(grep -c '0\.40\b' "$f" 2>/dev/null); [ "${c:-0}" -gt 0 ] && echo "$c $f"
done | sort -rn
```

**Use `0\.40\b`, not `0\.40\.[0x]`.** This is the correction recorded at ROADMAP `3.41.0`: RFC-086
measured with the suffixed form, which structurally cannot match a bare "0.40 release family", and the
guard caught the miss. Here the suffixed pattern finds only 35 of 37 — the two it misses are named in
§4 below.

### 3.1 What must NOT change — 46 occurrences

```text
CHANGELOG.md    2      released entries record what was true at their release
ROADMAP.md      8      history rows and past-release narrative
rfcs/**        36      RFC and handoff records
```

**No repo-wide `sed`.** RFC-079's review caught exactly that damage. Restrict edits to the measured
file list, then verify:

```bash
git diff --name-only | grep -E '^(CHANGELOG\.md|ROADMAP\.md|rfcs/)'
# expect only ROADMAP.md and rfcs/README.md (their §4 in-scope edits)
git diff -- CHANGELOG.md | grep -E '^[+-]' | grep -E '0\.[34][0-9]\.' | grep -v '0\.41\.0'
# expect EMPTY — no released entry touched
```

## 4. Two files need content, not just a number

Both are bare-form sites the suffixed pattern misses, and both say something that becomes false:

**`docs/src/introduction.md:17`** — currently *"tracks the current 0.40 release family, an RFC-086
release publishing …"*. Bumping `0.40` → `0.41` alone would leave it describing the **previous**
release's content. Rewrite the content clause for RFC-087 + RFC-088. This exact file needed the same
treatment last release; it is the one most likely to be number-swapped and left wrong.

**`docs/src/reference/public-api-snapshot.md:3`** — currently *"at the current v0.40 release family"*,
and its next sentence says core's surface changed "in RFC-087". That stays true but is now incomplete;
name RFC-088's grammar extension as well.

**Do not otherwise touch `public-api-snapshot.md`'s rows.** RFC-087 already added them, and RFC-088
changed no public item. Only the version string and that framing sentence move.

## 5. CHANGELOG

Follow the existing entry shape. RFC-089 §6.1 has the required content; five things are easy to get
wrong, and each would publish a false claim:

1. **`repeat` repeats elements, `tile` repeats the whole tensor.** Show both on the same input.
2. **`meshgrid` uses `xy`** — for `x` len `m`, `y` len `n`, both outputs are `[n, m]`.
3. **`tile` rejects `reps` longer than rank** — a deliberate divergence from NumPy, not a limitation.
4. **Negative indices are `slice_str` only** — the builder did not gain them.
5. **Out-of-range negatives error, they do not clamp** — so do **not** call any of this
   "NumPy-compatible". Both `tile`'s rank rule and the clamping behaviour differ on purpose.

**No `Maturity` section.** Every label is unchanged from `0.40.0`. Adding a section to say so invites
the silent-promotion reading RFC-067 forbids.

## 6. Verification

```bash
cargo build --workspace                 # regenerates Cargo.lock
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash scripts/check-matten-data-scope.sh
bash tools/matten-report/tests/process-boundary.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

`check-release-docs.sh` carries a current-family drift guard and the ROADMAP header/history parity
check — both are load-bearing for a version bump, so a failure there is real.

Scope confirmation:

```bash
git diff --name-only -- crates/ | grep -v 'README.md$'
# expect ONLY crates/matten/src/lib.rs
grep -rn '0\.40\b' README.md crates/*/README.md docs/src crates/matten/src/lib.rs Cargo.toml
# expect EMPTY
cargo metadata --no-deps --format-version 1 | grep -o '"version":"0\.41\.0"' | wc -l   # expect 5
```

## 7. Known pitfalls

1. **Tagging or publishing.** Not yours.
2. **Measuring with `0\.40\.[0x]`** — misses two real sites (§3).
3. **A repo-wide `sed`** — rewrites 46 historical occurrences (§3.1).
4. **Number-swapping `introduction.md`** and leaving RFC-086's content description (§4).
5. **Calling any of it "NumPy-compatible"** (§5).
6. **Adding a `Maturity` section** (§5).
7. **Hand-editing `Cargo.lock`.**
8. **Touching released CHANGELOG entries** or `public-api-snapshot.md`'s rows.

## 8. What the review request must report

```text
[ ] the re-measured count and file list, versus RFC-089 §5's 37/17
[ ] cargo metadata showing all five crates at 0.41.0
[ ] the [0.41.0] CHANGELOG entry in full
[ ] the introduction.md and public-api-snapshot.md CONTENT diffs (§4)
[ ] git diff proving no released CHANGELOG entry, ROADMAP history row, or rfcs/** record changed
[ ] confirmation the only .rs change is the install-pin doc comment
[ ] full gate set incl. MSRV and mdbook
[ ] explicit confirmation: no tag created, nothing published
```

## 9. Review stop

Acceptance makes this a commit point. Tagging and publishing remain separate owner actions.
