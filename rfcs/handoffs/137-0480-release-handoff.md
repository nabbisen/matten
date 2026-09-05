# RFC-137 Developer Handoff — Preparing `0.48.0`

**RFC:** `rfcs/accepted/137-0480-performance-and-metadata-release.md`
**Status:** Accepted 2026-09-05 by the owner.
**Target:** `Cargo.toml`, `crates/*/Cargo.toml`, live version pins, `CHANGELOG.md`,
`introduction.md`, `public-api-snapshot.md`
**Authority:** where this document and the RFC disagree, **the RFC wins.**

---

## 1. Scope in one line

**Prepare the release and stop.** One commit. No tag, no publish — each of those is a separate owner
authorization at the time, and neither has been given.

## 2. Do it in this order, and the order matters

```text
1  bump Cargo.toml to 0.48.0
2  run scripts/check-release-docs.sh   -> it now FAILS, and its failures ARE
                                          your worklist
3  fix what it names, re-run until green
4  Change C, the changelog, introduction.md, the snapshot header
5  full gate sweep, then ONE commit
```

**Step 2 is the point.** `check-release-docs.sh:377` derives the current minor from `Cargo.toml`
itself and rejects install pins, `X.Y.x family` labels, and `current [v]X.Y [release] family` prose
whose minor is not current. Bump first and the guard hands you the exact list.

**Do not work from the RFC's counts** (24 pins, 10 family references). They are this document's
arithmetic and are offered only so you can sanity-check the guard's output against roughly the right
magnitude. An RFC in this series has been wrong about a count before — RFC-127 §8 said "~31" where
the real number was 47, and following it literally would have left the defect alive. The guard's
count cannot be wrong in that way.

## 3. What must NOT be retargeted

```text
CHANGELOG.md    historical entries. [0.47.0] stays [0.47.0] forever.
ROADMAP.md      same -- the document-history rows are a record.
rfcs/**         same. Past RFCs said what they said.
```

These sit outside the guard's `USER_DOCS`, so it will not ask you to touch them. If you find
yourself editing a `0.47` string in any of the three, stop — that is rewriting history, not updating
a pin.

## 4. Change C — the manifest keys

```toml
# Cargo.toml, in [workspace.package]
homepage = "https://nabbisen.github.io/matten"
```

```toml
# each of the five crates/*/Cargo.toml, in [package]
homepage.workspace = true
```

**Do not add a `documentation` key.** Cargo already defaults it to docs.rs for published crates, and
adding one creates a hand-maintained string where a correct default exists. RFC-130 §6's own
parenthetical allows omitting it.

This is RFC-130's last unbuilt piece. It was directed into `0.47.0`'s slice, missed that window, and
its instruction has already been amended once — so it lands here or the amendment happens twice.

## 5. The CHANGELOG entry — floors, never multipliers

```text
## [0.48.0] - <date>

### Changed
- `matmul` is substantially faster on matrices that exceed cache, and axis
  reductions (`sum_axis`, `mean_axis`, `min_axis`, `max_axis`) are faster by an
  order of magnitude or more. **No numeric output changes**: both are exact
  restructurings, verified bit-identical rather than merely close.
- The published crates now carry a `homepage` link to the project book.
```

**You measured RFC-136 at 199.6× on axis 0 and reported the discrepancy honestly — that finding is
exactly why this entry must not name a number.** Review traced it: the old path's four million heap
allocations make the *baseline* swing ~2× with allocator warmth and ~2.6× with machine load, while
the hoisted path stays stable to within a few percent. 81×, 110× and 199× are all true of the same
change on the same machine. The floor (~10×, axis 2's worst case across four runs) is the only claim
that reproduces.

Say **"no numeric change"** explicitly. It is the most useful sentence in the entry for anyone
deciding whether to upgrade, and it is unusual enough to be worth stating outright.

## 6. `introduction.md` — rewrite lines 19-30, including the last one

The blockquote at `docs/src/introduction.md:19-30` describes `0.47`'s content. Replace it with
`0.48`'s. Two specific lines:

```text
line 19   "tracks the current 0.47 release family, carrying RFC-128 through RFC-132"
line 29   "see the `[0.47.0]` CHANGELOG entry for the complete list"   -> [0.48.0]
```

**Line 29 is inside the paragraph you are rewriting.** RFC-135's handoff told the last implementer
that the equivalent line was off-limits, and that was wrong for exactly this reason. It is the
paragraph's own closing sentence.

Write what a book reader needs: two operations got substantially faster, **no numbers changed**, and
the crates.io pages now link to the book. **Do not describe the loop transforms** — `i-j-k → i-k-j`
is implementation detail with no meaning to someone reading the introduction.

## 7. `public-api-snapshot.md` — header only

```text
public items added since 0.47.0:    0
public items removed:               0
```

Only the version in its header line changes. **A diff touching the body is a defect.**

Verify rather than assume, because one thing looks like a counterexample: RFC-136 changed
`nan_axis_reduce` from `-> Tensor` to `-> Result<Tensor, MattenError>`. That function is
module-private and both its callers already returned `Result`, so nothing public moved. Confirm it
yourself — `grep` the diff since the `0.47.0` tag for `pub fn` / `pub struct` / `pub enum` /
`pub const` additions and expect zero.

## 8. Verify before committing

```text
[ ] nine guards pass
[ ] cargo test --workspace, and --all-features
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo fmt --check (and the benchmarks manifest)
[ ] cargo publish --workspace --dry-run  -- verifies all five at 0.48.0 without
    uploading anything; catches a manifest mistake in Change C before it matters
[ ] cargo package --list -p matten | grep -c Cargo.toml   -- the manifest ships,
    so Change C really is packaged content
```

The dry run is the cheapest possible check on Change C. A malformed `homepage.workspace = true` in
one crate fails there rather than at publish time.

## 9. One commit

```text
Prepare 0.48.0: <n> pins, <m> family references, the manifest keys, the changelog entry (RFC-137)
```

Fill in the real numbers from what you actually changed, not from the RFC's estimate.

## 10. Then stop

```text
NO tag. NO publish. NO version bump beyond 0.48.0 itself.
```

The remaining sequence — push, confirm CI green **on the pushed commit**, tag the Prepare commit,
`cargo publish --workspace`, verify against the sparse index — belongs to the owner's authorization
and the reviewer's execution. Two details from `0.47.0` worth knowing even though they are not your
step: `gh run list --commit <sha>` silently returned an empty list, so the gate is confirmed by
reading each workflow's `head_sha` from the API instead; and commits touching zero `crates/` paths
may land on top of your Prepare commit before authorization arrives, which is fine but must be
*proved* fine rather than assumed.

## 11. Review request

Write `.git-exclude/review-request/RFC-137/`. Include the guard's before/after output (the failure
list it gave you, and the clean run), the retarget counts you actually applied versus the RFC's
estimate, the `cargo publish --dry-run` result, and your confirmation that the public-API snapshot's
body is untouched. If the guard's list disagreed with the RFC's counts, say so plainly — that is a
finding about this document, and the last four rounds have all produced one.
