# RFC-121: `0.46.1` — The Published-Surface Patch

**Status:** **Accepted** 2026-08-28 by the owner. Not yet implemented. Handoff:
`rfcs/handoffs/121-0461-published-surface-patch-handoff.md`. **The tag and the publish are NOT
authorized by this acceptance** — each is a separate owner authorization at the time (RFC-094 §5).
**Target:** `Cargo.toml`, live version pins, `CHANGELOG.md`
**Theme:** Publish RFC-119's corrections — the only way they reach a reader
**Related:** RFC-094 §4.1 (as amended by RFC-120), RFC-030, RFC-118, RFC-119, RFC-103

---

## 1. Summary

```text
Prepare 0.46.1: lock-step bump, 21 live pin retargets across 14 files, one
CHANGELOG entry. NO tag, NO publish — those are separate owner authorizations.
```

**This is the project's first patch release since `0.28.5` (2026-06-28) and the first ever under
RFC-094.** Fifty-three patch tags exist historically; RFC-094 §3 recorded that the option had
*"quietly stopped existing"* — fourteen minors and zero patches since `0.28`. This exercises §4.1 for
the first time, including the clause RFC-120 amended three commits ago.

## 2. Why a patch, and why now

RFC-119 corrected a panicking example and three false statements. **Every one of them is inside a
published package, and none of them reaches a reader until a publish happens** — docs.rs renders the
published rustdoc and crates.io renders the published README. The repository is already fixed. The
users are not.

RFC-094 §4.1, as amended by RFC-120: *"correctness fixes to already-published crate content — code,
rustdoc, or a packaged README — and nothing else."* That is exactly and only what RFC-119 changed.
No new public API. No behaviour change.

**§4.1's trigger is "as soon as the fix is reviewed. No batching, no waiting for company."** The fix
is reviewed and closed. There is no candidate `0.47.0` theme to wait for.

## 3. Three release RFCs are the wrong template — each differently

You have three release preparations to pattern-match against and **all three will mislead you**,
because all three were minors:

```text
0.44.0 (RFC-103)  Changed-only. Forbade an empty `Added` heading.
0.45.0 (RFC-109)  Added + Changed + Version. Required four new snapshot rows.
0.46.0 (RFC-114)  Changed + Version.
0.46.1 (this)     Fixed + Version. NO Added. NO Changed. NO new snapshot row.
                  And — the one that matters most — the FAMILY references
                  do NOT move.
```

**The trap is the pin retarget.** Every minor retargeted *both* forms:

```text
minor 0.45.0 -> 0.46.0 :  "0.45.0" -> "0.46.0"   AND   "0.45.x" -> "0.46.x"
patch 0.46.0 -> 0.46.1 :  "0.46.0" -> "0.46.1"   BUT   "0.46.x" STAYS "0.46.x"
```

`0.46.1` is in the `0.46.x` family. **All 13 `0.46.x` references are already correct**, and
retargeting them to `0.46.1.x` or `0.47.x` would be a defect introduced by muscle memory. This is
the single most likely way to get this release wrong.

## 4. Evidence

**Re-derive §4.1–§4.3 before editing.** My measurement method has been wrong repeatedly in this
project — most recently in RFC-119's own E13, which the implementer corrected. If your figures
differ from 24/21/13, one of our methods is broken and that is worth more than the retarget.

| # | Claim | Established by |
|---|---|---|
| E1 | Family at `0.46.0`, lock-step, four crates inherit | `Cargo.toml:42`; `crates/*/Cargo.toml` `version.workspace = true` |
| E2 | **24** exact `0.46.0` occurrences in live files | `git ls-files` → filter `md/toml/rs/yml/yaml` → exclude `Cargo.lock` by **exact path** → exclude `rfcs/`, `ROADMAP.md`, `CHANGELOG.md` → `grep -o` per file |
| E3 | **3** of those 24 must **not** move (§4.2) | direct read of each |
| E4 | **21** live pins to retarget, across **14** files | E2 − E3 |
| E5 | **13** `0.46.x` family references in live files, **all already correct** | same method, pattern `0\.46\.x` |
| E6 | RFC-119 changed **6** files under `crates/`, all in published packages | `git diff --name-only 0.46.0..HEAD -- crates/`; `cargo package --list` |
| E7 | No public item was added, removed, or changed by RFC-119 | its diff is `//!` doc lines, one example, and `.github/` |
| E8 | The last patch was `0.28.5`, 2026-06-28, and it was itself a **documentation/examples** release — *"No change to any published crate's code, public API, runtime, or dependencies"* | `CHANGELOG.md`, `[0.28.5]` |
| E9 | Recent entries carry only the sections they need; none of the last four used `### Fixed` | `CHANGELOG.md` headings for `0.43.0`–`0.46.0` |
| E10 | CI's clippy gate is the workspace, all-features form | `.github/workflows/test.yaml:40` |

### 4.1 The 21 to move

```text
Cargo.toml                                  1   the workspace version — the bump itself
README.md                                   6
crates/matten/README.md                     1
crates/matten-data/README.md                1
crates/matten-mlprep/README.md              1
crates/matten-ndarray/README.md             1
crates/matten-stats/README.md               1
crates/matten/src/lib.rs                    1   the install-pin doc comment
docs/src/contributing/architecture.md       1
docs/src/examples/data.md                   3
docs/src/quick-start.md                     1
docs/src/reference/boundary.md              1
docs/src/reference/compatibility.md         1
docs/src/reference/dynamic.md               1
                                           --
                                           21   across 14 files
```

### 4.2 The 3 that must NOT move, inside otherwise-live files

**These are the reason a blind `sed` over "live files" is wrong.**

```text
docs/src/contributing/release-checklist.md:276
    "`0.46.0` was tagged and published across four consecutive red CI runs"
    -> RFC-118's INCIDENT RECORD. Retargeting it rewrites history and would
       claim 0.46.1 shipped on red CI. It did not.

docs/src/introduction.md:34
    "see the `[0.46.0]` CHANGELOG entry"
    -> a reference to a CHANGELOG HEADING that exists and keeps its name.

docs/src/contributing/release-checklist.md:15
    "Release tags use bare SemVer with no `v` prefix, for example `0.46.0`."
    -> a tag-FORMAT example. `0.46.0` remains a real tag. Leave it.
       If you disagree, say so — this one is a judgement, not a rule.
```

### 4.3 The 13 that are already correct

All `0.46.x` family references — the root README crate table (5), the four companion README banners
(7 across them), and `docs/src/examples/data.md` (1). **Assert they are unchanged.** A diff touching
any `0.46.x` string is a defect.

## 5. Required implementation

```text
1. Cargo.toml:42 -> 0.46.1. Build so Cargo.lock regenerates. Commit the lock.
2. Retarget the 21 live pins (§4.1). Exact form "0.46.0" -> "0.46.1" only.
3. CHANGELOG [0.46.1]: Fixed + Version, both non-empty (§6).
4. NOTHING ELSE. See §7 for what specifically must not change.
```

## 6. CHANGELOG `[0.46.1]`

Two sections. `### Fixed` is the right heading and none of the last four releases used it (E9).

```text
Fixed    21_matrix_vector_product panicked when run — it multiplied a [2]
         vector by a [3,2] transpose, violating the rule its own comment
         stated. Shipped in every release since 0.17.0. (RFC-119)

         The crate-root documentation described dynamic slicing as guarded;
         it has been available since 0.44.0. (RFC-102, corrected by RFC-119)

         Two READMEs stated that zero-sized dimensions are rejected; they
         have been accepted since 0.46.0. (RFC-111, corrected by RFC-119)

         The core statistics module described matten-stats as a possible
         future companion; it has shipped since 0.38.0. (RFC-119)

Version  0.46.0 -> 0.46.1, lock-step
```

**Four claims would be publishable falsehoods here:**

```text
- calling any of this a BEHAVIOUR change. Nothing in the library behaves
  differently. The example's behaviour changed; no API's did.
- calling the example fix a new capability, or implying matmul gained
  anything. It lost a bug in a demonstration.
- saying dynamic reshape or arithmetic became available. They did NOT —
  only slicing was mis-described, and reshape/arithmetic remain guarded.
- listing "z-score" as newly documented in matten-stats. It lives in
  matten-mlprep. Getting this wrong at the last step would be the third
  time in this sequence.
```

## 7. What must NOT change — and two of these invert a previous release RFC

```text
docs/src/reference/public-api-snapshot.md
    NO new row, NO changed claim. RFC-119 added, removed and changed ZERO
    public items (E7). **This inverts RFC-109**, which required four new
    rows and called omitting them the defect. Here, ADDING one is the defect.
    This is RFC-103's instruction, not RFC-109's.

docs/src/introduction.md
    NO content rewrite. Every minor rewrote it because the release had new
    content to describe. This one does not: no API, no behaviour. Line 19's
    "the current 0.46 release family" stays true. **This inverts RFC-109 §7.**

rfcs/**, ROADMAP.md
    Records. Unchanged. Assert it.

CHANGELOG.md's existing entries
    Assert NO REMOVED LINE. Expect [0.46.1] to ADD occurrences of "0.46.0"
    in its own Version line — that is correct and mirrors every prior entry.
    Assert "no removed line", NOT a fixed occurrence count; a fixed count is
    the wrong invariant and RFC-103's review corrected me on exactly this.

any crates/*/src/*.rs, any example, .github/**
    RFC-119 shipped those. This RFC only bumps and records.
```

## 8. The release sequence

RFC-118's step applies, and this is its first use on a release it was written for:

```text
1. push main            CI cannot report on an unpushed commit
2. CONFIRM CI GREEN     on THE PUSHED COMMIT — not "recently", not the
                        previous run. `gh run list --limit 5`, matched
                        against the SHA just pushed. A red run STOPS this.
3. tag 0.46.1           separate owner authorization
4. publish              separate owner authorization
```

**Steps 3 and 4 are not authorized by this RFC** and each requires the owner's word at the time
(RFC-094 §5). `0.46.0` was tagged and published across four red runs; step 2 exists because of that.

## 9. Acceptance criteria

```text
[ ] cargo metadata shows 0.46.1 for all five crates
[ ] 21 live pins retargeted across 14 files; the 3 exclusions untouched (§4.2)
[ ] all 13 "0.46.x" family references UNCHANGED — asserted explicitly
[ ] Cargo.lock regenerated and committed
[ ] CHANGELOG [0.46.1]: Fixed + Version, neither empty, no falsehood from §6
[ ] public-api-snapshot.md UNCHANGED — no new row
[ ] introduction.md UNCHANGED
[ ] rfcs/**, ROADMAP.md unchanged; CHANGELOG.md has NO removed line — asserted
[ ] no .rs diff at all — RFC-119 already shipped those
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
    — the CI form (E10). Do NOT scope it to -p matten.
[ ] cargo test --workspace; both feature profiles build
[ ] NO tag, NO publish
```

## 10. Risks

```text
R1  Retargeting the "0.46.x" family references (§3, §4.3). THE defect this
    release is most likely to produce, because all three recent templates
    required exactly that and this one forbids it.
R2  Retargeting release-checklist.md:276 — rewrites RFC-118's incident record
    into a claim that 0.46.1 shipped on red CI (§4.2).
R3  Adding a public-api-snapshot row, carrying RFC-109's instruction forward
    when RFC-103's applies (§7).
R4  Describing any of this as a behaviour change in the CHANGELOG (§6).
R5  Asserting a fixed CHANGELOG occurrence count instead of "no removed line".
R6  Touching a .rs file. RFC-119 shipped all of them; this RFC bumps only.
R7  Treating §8's steps 3-4 as authorized. They are not.
```

## 11. What this does not fix

```text
- the ROADMAP Status block, still describing 0.41.0        (audit F5)
- the v1.0 readiness audit, now nine releases stale        (audit F6)
- SECURITY.md, awaiting the owner's disclosure contact     (audit F10)
- the three tools' unsafe policy                           (audit F11)
- mechanically blocking a tag on red CI                    (RFC-118 §9)
- a guard that can read a published claim — the cause behind RFC-119's
  four findings, and still unaddressed
```

`0.46.1` publishes corrections. It does not make the next stale statement any easier to find.
