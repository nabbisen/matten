# RFC-101 `0.43.0` Release Preparation: Implementation Handoff

**Status:** Issued 2026-08-04. Preparation authorized under RFC-101, accepted the same day.
**Design authority:** `rfcs/accepted/101-0430-core-surface-release.md`. Where this handoff and the
RFC disagree, the RFC wins — report the discrepancy rather than resolving it silently.

---

## 1. Purpose

Prepare `0.43.0`: bump the lock-step family, retarget the live version strings, write the CHANGELOG
entry. **Do not tag. Do not publish.** Those are separate owner authorizations (RFC-101 §7).

This release publishes two core API additions — RFC-099's `try_dot`/`try_matmul` and RFC-100's
`Display` — and carries RFC-096's example and the `crates/matten/README.md` playground link.

## 2. Two things invert from `0.42.0`. Copying that release produces a defect.

You have a recent, well-documented release to pattern-match against. **Do not**, on these two points.

### 2.1 This is Added-only — no `Changed` section

`0.42.0` made a `Changed` section mandatory, because RFC-090 moved two error message strings.
**Nothing changed behaviour this time.** RFC-099 preserved every message byte-identical; RFC-100
added a trait impl and left `Debug` untouched.

So the CHANGELOG gets `Added` and `Version` only. **Do not add an empty `Changed` heading to mirror
the previous entry.**

### 2.2 `public-api-snapshot.md` needs CONTENT — the opposite of last time

RFC-091 §5 required that page to get **the number only**, because `histogram` was a companion
addition and core's surface was untouched; citing RFC-090 there would have been a false claim.

**This release changes core's surface**, so the same page now needs the content update it was
refused last time. Its opening currently says:

> Core `matten`'s public API changed in RFC-087, which added `repeat`, `repeat_axis`, `tile`, and
> `meshgrid` … the first change to this page in a while.

Stale. RFC-099 added two functions; RFC-100 added a trait impl. Both belong.

**While editing that same paragraph, add RFC-090** to its list of companion-crate work that did not
touch core — currently *"The RFC-082 streaming feature and RFC-083 functions before it…"*. That is
RFC-093's O1 observation, deferred at the time to "the next release RFC". This is it.

`docs/src/introduction.md` also needs content: it names RFC-090 and must name RFC-099 and RFC-100.

## 3. Version-string retarget

RFC-101 §5 measured **39 live strings across 17 files** at `4fc37ab`. **Re-measure** — the RFC and
this handoff both add `0.42` occurrences under `rfcs/`.

```bash
grep -rn '0\.42\b' --include='*.md' --include='*.toml' --include='*.rs' . \
  | grep -v 'target/' | sed 's|^\./||' \
  | grep -vE '^(CHANGELOG\.md|ROADMAP\.md|rfcs/)'
```

Use `0\.42\b`, **not** `0\.42\.[0x]` — the suffixed form misses the two bare-form sites, which are
exactly the two needing content (§2.2). Your local `grep` may print paths without `./`, which
silently breaks a `^\./` exclusion; the pipeline above normalises first.

**44 occurrences are historical and must not move**: `CHANGELOG.md` (2), `ROADMAP.md` (11),
`rfcs/**` (31).

## 4. CHANGELOG

Per RFC-101 §6.1:

```text
Added    try_dot / try_matmul — Result forms for core's last two panic-only operations.
         The panicking dot/matmul are unchanged, including every message.

         Display for Tensor — aligned grid for rank <= 2, flat form above it, {:#} for
         untruncated, truncation at 12 columns, dynamic tensors rendered with Float
         distinguishable from Int. Debug is unchanged.

Version  lock-step family bump, all five crates.
```

No `Changed` (§2.1). No `Maturity` — all five labels unchanged, and inventing a section to say so
invites the silent-promotion reading RFC-067 forbids.

**Four things not to claim** (RFC-101 §6.2), each a publishable falsehood:

```text
- that Display matches ndarray. It deliberately renders 1.0 where ndarray renders 1,
  because matten's only element type is f64. The divergence is the point.
- that Debug changed, or that Display replaces it.
- that try_dot/try_matmul are new capability. The same inputs succeed and fail as
  before; what is new is being able to HANDLE the failure.
- that Display is a stable parsing target. It truncates.
```

## 5. Verification

```bash
cargo update --workspace
cargo metadata --format-version 1 --no-deps | grep -oE '"name":"matten[a-z-]*","version":"[^"]*"'
# expect all five at 0.43.0

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo test -p matten --no-default-features --features dynamic
for s in scripts/*.sh; do echo "== $s"; bash "$s" || echo "FAILED: $s"; done
RUSTFLAGS="-D warnings" bash scripts/check-doc-code.sh
mdbook build docs

git diff --name-only -- '*.rs'
# expect ONLY crates/matten/src/lib.rs
git diff CHANGELOG.md | grep -E '^-' | grep -v '^---'
# expect EMPTY — the new entry is a pure addition
```

**Eight guards now**, not seven — `check-report-demos.sh` arrived with RFC-097. It will fail if a
report demo's output drifts, which it should not here.

## 6. Known pitfalls

```text
- adding a Changed section because the last release had one (§2.1)
- giving public-api-snapshot.md the number only, as the last release required (§2.2)
- forgetting RFC-090 in that same paragraph — the O1 carry-forward (§2.2)
- measuring with 0\.42\.[0x] and missing both content-needing sites (§3)
- an exclusion pattern anchored on ./ that your grep does not emit (§3)
- sweeping ROADMAP/CHANGELOG/rfcs history into the retarget (§3)
- tagging or publishing — neither is authorized here (RFC-101 §7)
```

## 7. What the review request must report

```text
- the RE-MEASURED live count and file list, and the pattern used
- cargo metadata showing five crates at 0.43.0
- introduction.md and public-api-snapshot.md quoted before and after, showing BOTH got
  content, and that the snapshot's paragraph now names RFC-090 too
- the CHANGELOG entry in full, with no Changed and no Maturity section
- full gate output including all EIGHT guards
- confirmation git diff --name-only -- '*.rs' shows only crates/matten/src/lib.rs
- confirmation that no tag was created and nothing was published
```

## 8. Review stop

Stop after committing the preparation. Report, and the high-capability model reviews before the owner
is asked to authorize the tag and the publish, which are two separate asks.

**When those come, publishing is one command** — `cargo publish --workspace`, not five per-crate
invocations. RFC-091 §7's sequence was superseded during the `0.42.0` release: the workspace form
resolves the order itself and verifies every crate before uploading any, so a failure in the last
companion aborts before core is irreversibly live. That is not your step, but do not write the old
sequence into any report.
