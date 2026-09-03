# RFC-135: `0.47.0` — The Result-Zone and Limit-Model Release

**Status:** Proposed
**Target:** `Cargo.toml`, live version pins, `CHANGELOG.md`, `introduction.md`,
`public-api-snapshot.md`
**Theme:** Release four closed RFCs, and invert almost every instruction the last two releases gave
**Related:** RFC-128, RFC-129, RFC-131, RFC-132 (what ships); RFC-094 §4.2 (minor triggers);
RFC-118 (the CI gate); RFC-121 / RFC-134 (the patch templates this contradicts)

---

## 1. Summary

```text
Prepare 0.47.0: lock-step bump, 24 exact pin retargets across 15 files,
8 FAMILY references moved 0.46.x -> 0.47.x, one CHANGELOG entry,
and TWO page rewrites the last two releases explicitly forbade.
NO tag, NO publish.
```

**This is a minor, and the two immediately preceding releases were patches.** Nearly every
"do not touch this" instruction from RFC-121 and RFC-134 inverts here. §3 is the whole of the risk.

## 2. What ships

```text
RFC-129   try_add / try_sub / try_mul / try_div      NEW PUBLIC API
RFC-132   the boundary-only limit model              BEHAVIOUR CHANGE
RFC-131   five rustdoc corrections                   packaged doc fixes
RFC-128   property testing                           test files (which DO ship)
```

RFC-129 is why this must be a minor: §4.1 excludes new public API from a patch. RFC-132 is a
behaviour change — `&big + &big` succeeds where it panicked. Either alone forces a minor.

**Trigger:** RFC-094 §4.2(c) — the owner authorized `0.47.0` early on 2026-09-01 to prioritize
RFC-129. Four themes have since landed, so (a) is satisfied independently.

## 3. Everything the last two releases told you, inverted

**Read this before touching a file.** `0.46.1` and `0.46.2` were patches inside the `0.46.x` family.
This is not.

| | `0.46.1` / `0.46.2` said | `0.47.0` requires |
|---|---|---|
| `0.46.x` family refs | **must NOT move** | **MUST move** → `0.47.x` |
| `introduction.md` | **no rewrite** | **rewrite required** (§6) |
| `public-api-snapshot.md` | **no new row** | version line updated (§7) |
| CHANGELOG shape | `Security` + `Fixed` / `Fixed` only | `Added` + `Changed` + `Fixed` + `Version` |

> **The 8 family references are the single most likely defect**, because the last two tasks both
> required leaving them alone and one of them named that as its top risk. **Here, leaving them is
> the defect.** A `0.46.x` string surviving this release is wrong.

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Family at `0.46.2`; **28** files changed under `crates/` since that tag | `git diff --name-only 0.46.2..HEAD -- crates/` |
| E2 | **24** exact `0.46.2` pins across **15** files | `git ls-files` → filter ext → exclude `Cargo.lock` by exact path → exclude `rfcs/`, `ROADMAP.md`, `CHANGELOG.md` → `grep -o` |
| E3 | **8** `0.46.x` family references, in five files, **all of which must move** | same method, pattern `0\.46\.x` |
| E4 | **3** `0.46.0` strings remain and must stay | `release-checklist.md:15` (tag-format example), `:276` (RFC-118's incident record), `introduction.md:34` (a `[0.46.0]` CHANGELOG heading pointer) |
| E5 | Zero `0.46.1` strings remain | same method |
| E6 | `introduction.md:19-20` says *"the current 0.46 release family, carrying RFC-110, RFC-111, and RFC-112"* — wrong family **and** wrong contents | direct read |
| E7 | `public-api-snapshot.md:3` says *"at the current v0.46 release"* | direct read |
| E8 | The four `try_` rows are **already** in the snapshot — RFC-129 added them | `grep -c 'try_add\|try_sub\|try_mul\|try_div'` → 4 |
| E9 | The pin count rose 21 → 24: `quick-start.md` gained one and `troubleshooting.md` is new with two | RFC-131's diff |

**Re-derive E2, E3 and E4 before editing.** E9 is the reason a remembered figure will be wrong.

### 4.1 The 24 pins

```text
Cargo.toml 1, README.md 6, crates/matten/README.md 1, crates/matten-data/README.md 1,
crates/matten-mlprep/README.md 1, crates/matten-ndarray/README.md 1,
crates/matten-stats/README.md 1, crates/matten/src/lib.rs 1,
docs/src/contributing/architecture.md 1, docs/src/examples/data.md 3,
docs/src/quick-start.md 2, docs/src/troubleshooting.md 2,
docs/src/reference/boundary.md 1, docs/src/reference/compatibility.md 1,
docs/src/reference/dynamic.md 1
```

### 4.2 The 8 family references — these MOVE

```text
crates/matten-data/README.md 1, crates/matten-mlprep/README.md 2,
crates/matten-ndarray/README.md 2, crates/matten-stats/README.md 2,
docs/src/examples/data.md 1
```

## 5. Required implementation

```text
1. Cargo.toml -> 0.47.0. Build so Cargo.lock regenerates. Commit the lock.
2. Retarget the 24 exact pins: "0.46.2" -> "0.47.0".
3. Retarget the 8 family refs:  "0.46.x" -> "0.47.x".
4. introduction.md's family paragraph — rewrite (§6).
5. public-api-snapshot.md's version line — update (§7).
6. CHANGELOG [0.47.0]: Added + Changed + Fixed + Version (§8).
```

## 6. `introduction.md` — a rewrite, and a within-file split

```text
LINES 19-20  "This documentation tracks the current 0.46 release family, carrying
              RFC-110, RFC-111, and RFC-112 — no new API, only behaviour changes
              and a restriction removed."
             -> WRONG FAMILY and WRONG CONTENTS. Rewrite for 0.47: try_ arithmetic
                (new API), the boundary-only limit model, property testing.

LINE 34      "see the `[0.46.0]` CHANGELOG entry"
             -> a pointer to a HEADING that keeps its name. DO NOT TOUCH.
```

**One file, two opposite instructions.** Getting this wrong in either direction is easy: a blanket
retarget breaks line 34, and a blanket "don't touch introduction.md" (RFC-121's rule) leaves lines
19-20 describing a family that is no longer current.

Accuracy points for the rewrite, each easy to get subtly wrong:

```text
- try_add and friends do NOT replace the operators; the operators still panic
  and now delegate
- the limit change makes `&big + &big` WORK. It is not new hardening — it is a
  bound REMOVED from arithmetic, and kept where an allocation can exceed its
  inputs combined
- property testing changes no behaviour and is not a user-facing feature
- do NOT describe 0.47.0 as a security release. 0.46.2 was.
```

## 7. `public-api-snapshot.md`

```text
line 3   "at the current v0.46 release"  ->  v0.47
```

**No new rows.** RFC-129 already added the four `try_` entries (E8). This is neither RFC-121's case
(no change at all) nor RFC-109's (add rows now) — the rows exist and only the version line is stale.
**Verify E8 yourself**; if the rows are absent, adding them is required and that is a finding.

## 8. CHANGELOG `[0.47.0]`

Four sections, all non-empty:

```text
Added    try_add / try_sub / try_mul / try_div — recoverable twins for the four
         operators, which continue to panic and now delegate to them (RFC-129)

Changed  the element budget no longer applies to arithmetic, reductions,
         slicing, or concatenation on data already in memory. It still applies
         at every boundary where a size comes from outside, and to any operation
         whose output can exceed its inputs combined — matmul, outer,
         broadcasting expansion, repeat/tile. `&big + &big` now succeeds where
         it panicked. (RFC-132)
         max_parse_bytes is now enforced at every file and string parser
         entry point, including matten-data's Table::from_csv_path. (RFC-132)

Fixed    five rustdoc statements corrected: a documented panic that cannot
         happen, a slicing guarantee that does not hold for dynamic tensors,
         two contradictory comments on one constant, a doc describing the
         opposite of its code, and undocumented non-reflexive PartialEq for
         NaN. (RFC-131)

Version  0.46.2 -> 0.47.0, lock-step
```

**Five claims that would be publishable falsehoods:**

```text
- calling the limit change "hardening" or "improved safety". It REMOVES a bound
  from arithmetic. The safety story is that boundaries got stricter while
  in-memory operations got out of the caller's way.
- implying try_ADD replaces the operator, or that the operators changed
  behaviour. They panic exactly as before, with byte-identical messages.
- describing property testing as a user-facing feature. It ships test files and
  changes nothing a consumer runs.
- calling this a security release. 0.46.2 was; this is not, and conflating them
  understates 0.46.2.
- claiming zero-sized dimensions or the RFC-127 bound changed. Neither did.
```

Assert **no removed line** in `CHANGELOG.md`, not a fixed occurrence count.

## 9. Out of scope

```text
E4's three 0.46.0 strings          preserved
introduction.md line 34            preserved (§6)
new public-api-snapshot rows       already present (E8)
any crates/**/*.rs                 the four RFCs shipped them; only lib.rs's
                                   install-pin doc comment may move
ROADMAP.md, rfcs/**                records
RFC-130, RFC-123, RFC-133          not in this release
```

## 10. Risks

```text
R1  Leaving the 8 "0.46.x" family refs alone. The last two tasks REQUIRED that;
    here it is the defect (§3).
R2  Not rewriting introduction.md, because the last two releases forbade it (§6).
R3  Retargeting introduction.md line 34's CHANGELOG heading pointer (§6).
R4  Retargeting E4's other two exclusions, especially release-checklist.md:276.
R5  Deriving 21 pins from memory instead of 24 (E9).
R6  Describing the limit change as hardening, or this as a security release (§8).
R7  Adding public-api-snapshot rows that already exist (E8).
R8  Tagging or publishing. Neither is authorized.
```

## 11. Acceptance criteria

```text
[ ] cargo metadata shows 0.47.0 for all five crates
[ ] 24 exact pins retargeted across 15 files
[ ] 8 family refs moved to 0.47.x — ZERO "0.46.x" strings remain, asserted
[ ] E4's three "0.46.0" strings untouched, including introduction.md:34
[ ] introduction.md's family paragraph rewritten; line 34 unchanged
[ ] public-api-snapshot.md's version line updated; row count unchanged
[ ] CHANGELOG [0.47.0]: Added + Changed + Fixed + Version, no §8 falsehood
[ ] CHANGELOG.md has no removed line
[ ] no .rs diff except crates/matten/src/lib.rs's install-pin doc comment
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"; mdbook clean
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] NO tag, NO publish
```

## 12. The release sequence

```text
1. push main
2. CONFIRM CI GREEN on the PUSHED COMMIT       RFC-118; a red run stops this
3. tag 0.47.0                                   separate owner authorization
4. publish                                      separate owner authorization
```

Tag on the **Prepare** commit, signed, bare SemVer, per the `0.37.0`–`0.46.2` convention.

## 13. What this does not fix

```text
- RFC-130's SECURITY.md / CONTRIBUTING.md, RFC-123's guard, RFC-133 — none ship here
- the audit's performance findings (P-1, P-2)
- whether test files should ship at all — a candidate, undecided
```
