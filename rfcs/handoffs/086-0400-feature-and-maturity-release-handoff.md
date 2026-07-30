# RFC-086 `0.40.0` Release Preparation: Implementation Handoff

**Project:** `matten`
**Governing RFC:** `rfcs/proposed/086-0400-feature-and-maturity-release.md`
**Document kind:** Detailed implementation handoff
**Status:** Inherits RFC-086's state — **implemented and reviewed 2026-07-30** (approved, no
corrections). Preparation is complete; tagging and publishing remain outstanding owner actions, and
tagging is still blocked by RFC-086 §3
**Date:** 2026-07-30

---

## 1. Purpose

Prepare `0.40.0`: bump the lock-step family `0.39.0` → `0.40.0`, retarget the live version strings, and
write the CHANGELOG entry for RFC-082, RFC-083, RFC-084 and RFC-085.

**You do not tag and you do not publish.** Both are owner actions, and one of them is blocked (§2).

**You add no features.** Everything being released already landed. This slice bumps a number and tells
the truth about what shipped.

## 2. Precondition you must not work around

RFC-086 §3: the `0.38.0` and `0.39.0` tags are orphaned by the history rewrite and point at commits not
on `main`. That is the **owner's** repair and it blocks *tagging*, not preparation.

```text
you MAY   prepare the release, commit it, and submit for review
you MUST NOT  create the 0.40.0 tag, re-tag anything, or publish
```

If anyone asks you to tag or publish, escalate rather than comply — mid-model §13 prohibits it
outright.

## 3. Version bump

```text
Cargo.toml            [workspace.package] version = "0.40.0"
Cargo.lock            regenerate via a build; do not hand-edit
```

All five crates share the version (RFC-030, unconditional). Verify with `cargo metadata`, not by
reading the manifest:

```bash
cargo metadata --no-deps --format-version 1 \
  | python3 -c "import json,sys; [print(p['name'], p['version']) for p in json.load(sys.stdin)['packages']]"
# expect all five at 0.40.0
```

## 4. Version-string retarget — measure first

RFC-086 §6 lists **36 strings across 16 files**, measured at `2f00edf`. **Re-measure before editing** —
the count moves with every commit, and a stale figure in a completion report is a defect this project
has caught before.

```bash
for f in README.md Cargo.toml crates/matten/README.md crates/matten/src/lib.rs \
         crates/matten-ndarray/README.md crates/matten-mlprep/README.md \
         crates/matten-data/README.md crates/matten-stats/README.md \
         $(find docs/src -name '*.md'); do
  c=$(grep -c '0\.39\.[0x]' "$f" 2>/dev/null); [ "${c:-0}" -gt 0 ] && echo "$c $f"
done | sort -rn
```

### 4.1 What must NOT change — 132 occurrences

```text
CHANGELOG.md    4    released entries record what was true at their release
ROADMAP.md     13    history rows and past-release narrative
rfcs/**       115    RFC and handoff records
```

**RFC-079's review caught an over-broad retarget doing exactly this damage.** A `sed` across the repo
is the wrong tool here. Restrict edits to §4's measured file list and verify afterwards:

```bash
git diff --name-only | grep -E '^(CHANGELOG\.md|ROADMAP\.md|rfcs/)' 
# expect only ROADMAP.md and rfcs/README.md (their §5 in-scope edits), never CHANGELOG history
git diff -- CHANGELOG.md | grep -E '^[+-]' | grep -E '0\.3[0-9]\.'
# expect ONLY the new [0.40.0] entry's own lines and the §5 preamble fix
```

## 5. CHANGELOG

### 5.1 The `[0.40.0]` entry

Follow the existing entry shape. Content requirements are RFC-086 §7.1; three that are easy to get
wrong:

1. **`kurtosis` is EXCESS (Fisher)** — a normal distribution scores `0.0`, not `3.0`. Say so. A user
   comparing against pandas will otherwise think it is broken.
2. **The `streaming` feature is off by default**, and `CsvBatchReader` does **not** behave identically
   to `Table::from_csv_path` on malformed input — RFC-082 §4.3 records two accepted divergences
   (a blank-but-not-empty file, and invalid UTF-8). Do not write "equivalent" or "identical".
3. **All three maturity promotions named explicitly** (RFC-067 forbids wording that implies a silent
   promotion), with `matten-stats` stated as **production-ready candidate** — not production-ready.
   That distinction is the entire subject of RFC-084 and getting it wrong in the release notes would
   publish a claim three RFCs were careful not to make.

### 5.2 The preamble fix

`CHANGELOG.md`'s header still says the family is *"core `matten`, `matten-ndarray`, `matten-mlprep`,
and `matten-data`"* — omitting `matten-stats`, published since `0.39.0`. Add it.

This is the **only** pre-existing CHANGELOG text you may touch. It is current description, not a
released entry.

## 6. What you must not touch

```text
any crates/*/src/*.rs                except crates/matten/src/lib.rs's install-pin doc comment
any public API                       the surface ships exactly as RFC-085 left it
dependencies, features, edition, MSRV
maturity labels                      the three promotions already happened; this release
                                     publishes them, it does not perform them
pre-1.0 / 0.x wording                this is a 0.x release; that wording is still correct
rfcs/done/**, rfcs/handoffs/**, docs/design/**
compatibility.md's v1.0 requirements section
```

## 7. Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
bash scripts/check-release-docs.sh
bash scripts/check-matten-data-scope.sh
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-benchmark-dependency-sync.sh
bash scripts/check-streaming-scope.sh
bash tools/matten-report/tests/process-boundary.sh
mdbook build docs && rm -rf docs/book
git diff --check
```

`check-release-docs.sh` carries a `CURRENT_MINOR`-style drift guard and the ROADMAP header/history
parity check — both are load-bearing for a version bump, so a failure there is real, not incidental.

Scope confirmation:

```bash
git diff --name-only -- crates/ | grep -v 'README.md$'
# expect ONLY crates/matten/src/lib.rs
grep -rn '0\.39\.[0x]' README.md crates/*/README.md docs/src crates/matten/src/lib.rs
# expect EMPTY — every live string retargeted
cargo metadata --no-deps --format-version 1 | grep -o '"version":"0\.40\.0"' | wc -l
# expect 5
```

## 8. Known pitfalls

1. **Tagging or publishing.** Not yours, and one is blocked (§2).
2. **A repo-wide `sed` for the version string.** It will rewrite 132 historical occurrences. RFC-079's
   review caught precisely this.
3. **Claiming `CsvBatchReader` matches `from_csv_path`.** Two documented divergences (§5.1).
4. **Dropping the "excess" qualifier on `kurtosis`** (§5.1).
5. **Writing `matten-stats` as production-ready** instead of production-ready *candidate* (§5.1).
6. **Hand-editing `Cargo.lock`.** Regenerate it.
7. **Promoting anything.** The promotions already happened.
8. **Touching released CHANGELOG entries.** Only the preamble (§5.2).

## 9. What the review request must report

```text
[ ] the re-measured live-string count and file list, versus RFC-086 §6's 36/16
[ ] cargo metadata showing all five crates at 0.40.0
[ ] the [0.40.0] CHANGELOG entry in full
[ ] confirmation the three maturity labels are named, matten-stats as CANDIDATE
[ ] git diff proving no released CHANGELOG entry, ROADMAP history row, or rfcs/** record changed
[ ] confirmation the only .rs change is the install-pin doc comment
[ ] full gate set incl. MSRV and mdbook
[ ] explicit confirmation: no tag created, nothing published
```

## 10. Review stop

Acceptance makes this a commit point. Tagging and publishing remain separate owner actions, and
tagging is blocked until RFC-086 §3's precondition is resolved.
