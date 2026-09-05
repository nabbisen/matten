# RFC-137: `0.48.0` — The Performance and Metadata Release

**Status:** **Accepted** 2026-09-05 by the owner. Handoff:
`rfcs/handoffs/137-0480-release-handoff.md`. **The tag and the publish are NOT authorized by this
acceptance** — each is a separate owner authorization at the time (RFC-094 §5).
**Target:** `Cargo.toml`, `crates/*/Cargo.toml`, live version pins, `CHANGELOG.md`,
`introduction.md`, `public-api-snapshot.md`
**Theme:** The first release that changes nothing a user's code can observe — except speed
**Related:** RFC-133, RFC-136 (what ships), RFC-130 Change C (what finally ships), RFC-094 §4.1/§4.2
(why a minor), RFC-118 (the CI gate), RFC-135 (the template, and its two mistakes)

---

## 1. Summary

```text
RFC-133   matmul loop interchange i-j-k -> i-k-j     no numeric change
RFC-136   the axis-reduction hoist                    no numeric change
RFC-130 C the manifest homepage keys                  metadata only
```

**This release adds no public API, removes none, and changes no observable behaviour.** Every
existing program produces bit-identical output. That is unusual enough to state plainly, because it
makes the version decision look wrong at first glance (§2) and it changes what the changelog can
honestly promise (§7).

## 2. Why this is a minor, when nothing observable changed

The instinct says *"no API change, no behaviour change — that's a patch."* It is not, and the
reasoning is now settled precedent rather than argument:

```text
RFC-094 §4.1 as amended by RFC-120:
    a patch carries "correctness fixes to already-published crate content —
     code, rustdoc, or a packaged README — and nothing else"
```

**Neither thing here is a correctness fix.** Nothing was wrong: `matmul` and the axis reductions
produced correct results, just slowly, and the manifests were merely missing a link. A patch cannot
carry either, so both need a minor.

> This is the fourth time the RFC-120 amendment has produced a clean answer where the original
> wording would have left a genuine argument. Recorded, as the previous three were, because that is
> the amendment's entire value.

**RFC-094 §4.2's trigger is satisfied independently:** two themes (RFC-133, RFC-136) have landed
since `0.47.0`, so this release is due on the cadence rule and not only because there is content to
ship.

## 3. What ships

| RFC | Content | User-visible effect |
|---|---|---|
| RFC-133 | `mm_mul` loop interchange | `matmul` faster on shapes exceeding cache; **bit-identical output** |
| RFC-136 | `axis_reduce` + `nan_axis_reduce` hoist | `sum_axis`/`mean_axis`/`min_axis`/`max_axis` faster; **bit-identical output** |
| RFC-130 C | `homepage` in the manifests | crates.io pages link to the book |

**Mechanical confirmation** (RFC-094 §4.3's test, with `cargo package --list` as the operative
oracle):

```text
$ git diff --name-only 0.47.0..HEAD -- crates/
crates/matten/src/math.rs
```

One packaged file, plus the manifests Change C adds below. Nothing else in the release reaches a
published package.

## 4. Change C — RFC-130 §6, satisfied at last

RFC-130 §6 directed the manifest keys to be *"folded into that release's slice."* It named `0.47.0`,
that release shipped without them, and the instruction has now been amended once already. **This
release executes it.**

```toml
# Cargo.toml [workspace.package]
homepage = "https://nabbisen.github.io/matten"
```

```toml
# each of crates/*/Cargo.toml [package]
homepage.workspace = true
```

Six lines total: one workspace key, five inheritance lines. The book URL returns **200**.

**`documentation` is deliberately omitted**, as RFC-130 §6's own parenthetical allows — cargo already
defaults it to docs.rs for published crates, and this project's consistent habit is to delete
hand-maintained strings rather than add them (RFC-123 Change C removed the crate table's version
cells for exactly this reason).

> **The wording lesson from RFC-130's amendment applies here too.** *"C waits for `0.47.0`"* named a
> specific release and went stale in three days. Nothing in this RFC should name a future release as
> a condition; where sequencing matters, §10 states it as an ordering, not a version.

## 5. The mechanical retarget — 34 references, all guard-enforced

```text
24  exact `0.47.0` pins        install snippets, READMEs, lib.rs, docs/src/**
10  family references          `0.47.x family`, `current 0.47 release family`,
                                `v0.47 release` (the public-API snapshot header)
```

**This is the one part of the release nobody has to get right by hand.**
`scripts/check-release-docs.sh:370-404` derives `CURRENT_MINOR` from `Cargo.toml` itself and rejects
three classes — install pins, `X.Y.x family` labels, and `current [v]X.Y [release] family` prose —
whose minor is not the current one. The comment records why it is dynamic: a hardcoded value was
missed at the `0.23.0` bump, *"which is exactly how stale 0.22 pins shipped."*

So the moment `Cargo.toml` reads `0.48.0`, the guard fails until all 34 are retargeted. **Bump
first, then let the guard drive the list** rather than working from the counts above — they are this
RFC's arithmetic, and this RFC has been wrong about counts before (RFC-127 §8's "~31" against an
actual 47).

`CHANGELOG.md`, `ROADMAP.md` and `rfcs/` are outside `USER_DOCS` and must **not** be swept: they are
historical records, and retargeting them would rewrite what past releases said.

## 6. `introduction.md` — rewrite the paragraph, and mind its last sentence

`docs/src/introduction.md:19-30` is a blockquote describing `0.47`'s content in detail. It is
replaced wholesale with `0.48`'s, and its closing sentence carries the CHANGELOG pointer:

```text
line 19   "tracks the current 0.47 release family, carrying RFC-128 through RFC-132"
line 29   "see the `[0.47.0]` CHANGELOG entry for the complete list"
```

> **RFC-135's §6 said "line 34 — DO NOT TOUCH" of the equivalent pointer and was wrong**, because the
> pointer is the closing sentence *of the paragraph being rewritten*. It is `[0.48.0]` here. The
> mistake is recorded rather than quietly avoided, since the same shape of instruction will be
> written again for `0.49.0`.

The replacement paragraph should say what this release is: two operations got substantially faster,
**no numbers changed**, and the crates.io pages now link to the book. It should not enumerate the
loop transforms — that is implementation detail a book reader has no use for.

## 7. The CHANGELOG entry — floors, not multipliers

```text
## [0.48.0] - <date>

### Changed
- `matmul` is substantially faster on matrices that exceed cache, and axis
  reductions (`sum_axis`, `mean_axis`, `min_axis`, `max_axis`) are faster by an
  order of magnitude or more. **No numeric output changes**: both are exact
  restructurings, verified bit-identical rather than merely close.
- The published crates now carry a `homepage` link to the project book.
```

**Do not quote a headline multiplier.** RFC-136's review established why:

```text
the same change, same machine, measured 81x / 110x / 199x on axis 0
    -- because the OLD path's four million heap allocations make the BASELINE
       swing ~2x with allocator warmth and ~2.6x with machine load, while the
       hoisted path stays stable to within a few percent
```

The numerator is reproducible; the denominator is not. A floor is the correct shape for a
performance claim anyway — it is what a user is entitled to rely on — and *"an order of magnitude or
more"* holds under every methodology tried (axis 2, the worst case, measured 10.1× / 14.4× / 14.6× /
16.6× across four runs).

**The same conservatism applies to RFC-133's entry.** Its ~10× was measured on a much more stable
operation and reproduced twice, but stating one as a floor and the other as a precise multiple would
misrepresent which is which.

## 8. `public-api-snapshot.md` — header only, and this is the interesting part

```text
public items added since 0.47.0:  0
public items removed:             0
```

**Only the version in its header line changes.** `0.47.0` was the opposite case — RFC-129 added four
`try_*` entries and RFC-135 §5 had to reason carefully about whether rows were already present.
Here there is nothing to reason about, and a diff touching the snapshot's *body* is a defect.

This is worth checking rather than assuming: `nan_axis_reduce`'s signature changed from `Tensor` to
`Result<Tensor, MattenError>` in RFC-136, which *looks* like an API change and is not — the function
is module-private and both callers already returned `Result`. Confirmed at review; re-confirm during
prep.

## 9. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | 24 exact `0.47.0` pins outside history files | `grep`, excluding CHANGELOG/rfcs/ROADMAP |
| E2 | 10 family references (`0.47.x` / `v0.47` / `0.47 release family`) | same |
| E3 | **0** public items added or removed since `0.47.0` | `git diff 0.47.0..HEAD -- crates/` |
| E4 | One packaged file changed: `crates/matten/src/math.rs` | `git diff --name-only 0.47.0..HEAD -- crates/` |
| E5 | The retarget is guard-enforced and minor-derived | `check-release-docs.sh:377` |
| E6 | The book URL returns 200 | `curl` |
| E7 | Both perf changes are bit-identical | RFC-133 and RFC-136 reviews, independently reproduced |
| E8 | RFC-094 §4.2's two-theme trigger is satisfied | RFC-133 + RFC-136 landed since `0.47.0` |

## 10. Release sequence

Unchanged from RFC-118, and **each step after the push is a separate owner authorization**:

```text
1  prepare      version bump, 34 retargets, Change C, CHANGELOG, introduction,
                snapshot header. One commit: "Prepare 0.48.0: ..."
2  push
3  CONFIRM CI GREEN ON THE PUSHED COMMIT  -- read the workflows' head_sha from
                                             the API; `gh run list --commit`
                                             has silently returned nothing
4  tag          signed, bare SemVer, on the Prepare commit
5  publish      cargo publish --workspace, ONE invocation
6  verify       the sparse index (index.crates.io), not the JSON API
7  record       ROADMAP row, close RFC-133/136/137, RFC-130 at last
```

**Step 3's method is not optional detail.** At `0.47.0`, `gh run list --commit <sha>` returned an
empty list with no error; taking that at face value, or falling back to the newest green-looking
run, would have confirmed the gate against the wrong commit — precisely the failure RFC-118 exists
to prevent.

**On step 4:** if commits touching zero `crates/` paths land on top of the Prepare commit before
authorization arrives (as happened at `0.47.0`), tagging the Prepare commit is still correct — but
prove it with `git diff --name-only <prepare>..HEAD -- crates/` returning empty, rather than
assuming.

## 11. Risks

```text
R1  Quoting a headline speedup multiplier (§7). It will not reproduce.
R2  Sweeping CHANGELOG/ROADMAP/rfcs during the retarget (§5) -- that rewrites
    history rather than updating pins.
R3  Treating §5's counts as authoritative instead of letting the guard drive
    the list. This RFC's arithmetic has been wrong before.
R4  Leaving introduction.md's `[0.47.0]` pointer, or conversely treating it as
    off-limits (§6) -- it is the closing sentence of the rewritten paragraph.
R5  Editing the public-API snapshot's body (§8). Nothing public changed.
R6  Deferring Change C a second time. RFC-130 §6 has already been amended once
    for exactly this.
R7  Confirming CI against the wrong commit (§10 step 3).
R8  Claiming a behaviour change. There is none -- both perf changes are
    bit-identical, and saying otherwise would be the mirror of RFC-133 §5.1's
    original error.
```

## 12. Acceptance criteria

```text
[ ] version 0.48.0 in Cargo.toml; all 5 crates inherit it
[ ] Change C: homepage in [workspace.package] + 5 inheritance lines; NO
    documentation key
[ ] every stale 0.47 reference retargeted, with the guard driving the list
[ ] CHANGELOG/ROADMAP/rfcs untouched by the retarget
[ ] CHANGELOG [0.48.0] entry, floors not multipliers, "no numeric change" stated
[ ] introduction.md paragraph rewritten; its pointer reads [0.48.0]
[ ] public-api-snapshot.md: header version only, body unchanged
[ ] nine guards; cargo test --workspace; both feature profiles
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo publish --workspace --dry-run verifies all five at 0.48.0
[ ] NO tag, NO publish -- each is a separate owner authorization (RFC-094 §5)
```

## 13. What this release does not do

```text
- change any observable behaviour. Programs produce bit-identical output.
- add, remove, or alter any public API.
- make matten a performance crate. Speed is still not the proposition; this
  release only declines to be an order of magnitude slower than necessary.
- fix reshape.rs's and composition.rs's surviving per-element coordinate
  round trips (RFC-136 review §8). A candidate, unmeasured, and transpose's
  permutation does not decompose the same way.
- close RFC-130 by itself. Change C is its last piece, so the RFC closes when
  this release records it -- see step 7.
```
