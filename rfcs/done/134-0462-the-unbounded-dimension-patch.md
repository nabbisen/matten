# RFC-134: `0.46.2` — The Unbounded-Dimension Patch

**Status:** **Implemented** 2026-09-01 in commit *"Prepare 0.46.2: retarget 21 live pins, add the
Security/Fixed changelog entry (RFC-134)"* (`5f41660`), reviewed and approved after **one required
correction** — the `Security` bullet claimed the defect was reachable from CSV; it is not, since every
CSV path derives its shape from counts of parsed data. Corrected, and improved on the request by
stating **"CSV is not affected"** explicitly with the reason. **Prepared, not yet tagged** at the time
of closing. Handoff: `rfcs/handoffs/134-0462-the-unbounded-dimension-patch-handoff.md`.
**Target:** `Cargo.toml`, live version pins, `CHANGELOG.md`
**Theme:** Ship the Critical. It is fixed in git and still live on crates.io.
**Related:** RFC-127 (what ships), RFC-094 §4.1 (patch contents), RFC-118 (the CI gate), RFC-121 (the
`0.46.1` template, which mostly applies)

---

## 1. Summary

```text
Prepare 0.46.2: lock-step bump, 21 live pin retargets across 14 files, one
CHANGELOG entry. NO tag, NO publish.
```

**This is the most urgent release this project has prepared.** RFC-127 closed an uncatchable process
abort reachable from a 36-byte JSON document, and until this ships **every published crate still has
it**. The repository has been correct since `0646fd3`; users have not.

## 2. Why this RFC exists at all — a numbering error worth recording

The dev team's RFC-127 report closed with *"RFC-128 owns 0.46.2."* **It does not.** RFC-128 is
Property Testing, and no release RFC existed.

```text
schedule v0.1   numbered the 0.46.2 release RFC as "RFC-128"
schedule v0.2   renumbered when the audit RFCs were drafted; 128 became proptest
the handoff     carried v0.1's wording
```

**The reviewer's error, not the implementer's**, and it left a fixed Critical with nothing to ship
it. Recorded so the gap between "fixed" and "released" is visible in the record rather than assumed
away.

## 3. What RFC-121 established that still applies

`0.46.1` was the first patch since `0.28.5`. Its template holds, with the same traps:

```text
STILL TRUE   the 8 remaining `0.46.x` family references must NOT move — this is a
             patch inside the same minor, and every pre-0.46.1 release was a
             MINOR that retargeted both forms
STILL TRUE   no new public-api-snapshot row: RFC-127 added ZERO public items,
             verified by grep over its whole diff. RFC-103's rule, not RFC-109's.
STILL TRUE   no introduction.md rewrite
CHANGED      the family-reference count is 8, not 13 — the owner's README change
             removed five when the crate table's Version column became badges
```

## 4. Evidence

| # | Claim | Established by |
|---|---|---|
| E1 | Family at `0.46.1`; **18** files changed under `crates/` since that tag | `git diff --name-only 0.46.1..HEAD -- crates/` |
| E2 | **21** exact `0.46.1` pins in live files, across **14** files | `git ls-files` → filter ext → exclude `Cargo.lock` by exact path → exclude `rfcs/`, `ROADMAP.md`, `CHANGELOG.md` → `grep -o` |
| E3 | **8** `0.46.x` family references, all already correct | same method, pattern `0\.46\.x` |
| E4 | **3** `0.46.0` strings remain and must stay — `release-checklist.md:15` (tag-format example), `:276` (RFC-118's incident record), `introduction.md:34` (a `[0.46.0]` CHANGELOG heading pointer) | direct read; preserved deliberately by RFC-121 |
| E5 | RFC-127 added no public item | `grep` for `pub fn/struct/enum/trait/const/static` not `pub(crate)` over its diff → zero |
| E6 | `CHANGELOG.md` has used a `### Security` heading **4** times | `grep -c '^### Security'` |
| E7 | The defect was reachable from untrusted JSON and aborted uncatchably | RFC-127 §2, reproduced at review in both profiles |

**Re-derive E2, E3 and E4 before editing.** The counts moved once already this month.

### 4.1 The 21 pins

```text
Cargo.toml 1, README.md 6, crates/matten/README.md 1, crates/matten-data/README.md 1,
crates/matten-mlprep/README.md 1, crates/matten-ndarray/README.md 1,
crates/matten-stats/README.md 1, crates/matten/src/lib.rs 1,
docs/src/contributing/architecture.md 1, docs/src/examples/data.md 3,
docs/src/quick-start.md 1, docs/src/reference/boundary.md 1,
docs/src/reference/compatibility.md 1, docs/src/reference/dynamic.md 1
```

**Same 14 files as `0.46.1`.** `release-checklist.md` and `introduction.md` are absent because their
only version strings are E4's exclusions.

## 5. Required implementation

```text
1. Cargo.toml -> 0.46.2. Build so Cargo.lock regenerates. Commit the lock.
2. Retarget the 21 live pins. Exact form "0.46.1" -> "0.46.2" ONLY.
3. CHANGELOG [0.46.2]: Security + Fixed + Version (§6).
4. Nothing else.
```

## 6. CHANGELOG `[0.46.2]` — and this one gets a `Security` heading

**`### Security`, not just `### Fixed`.** E6 shows the project has used it before, and people scan
changelogs for exactly that word.

```text
Security  A malformed shape — reachable from untrusted JSON or CSV — could make
          an ordinary operation abort the process. The abort was an allocator
          failure, so it could NOT be caught with catch_unwind. Any application
          accepting user-supplied JSON or CSV inherited an uncatchable denial of
          service. Fixed by bounding each individual dimension, not only the
          shape product. (RFC-127)

Fixed     slice().index(n) with n >= 2^63 silently returned a row counted from
          the end instead of erroring — a wrong answer, not a failure. (RFC-127)
          try_matmul could return a Tensor whose shape and data disagreed.
          (RFC-127)
          Tensor::new had no rustdoc: a private helper had absorbed its doc
          block. (RFC-127)

Version   0.46.1 -> 0.46.2, lock-step
```

**Falsehoods to avoid, each specific:**

```text
- do NOT say zero-sized dimensions are restricted. They are NOT — RFC-111 stands
  and its tests pass unmodified. The bound is PER-DIMENSION.
- do NOT call the slice fix a behaviour change for ordinary callers. Valid
  indices behave identically; only >= 2^63 changes, and only from wrong to Err.
- do NOT claim the release adds hardening or limits generally. It closes
  specific defects. RFC-132 is where the limit MODEL is decided, and it has not
  shipped.
- do NOT imply CHANGELOG entries before this one were wrong about anything.
```

## 7. A decision this RFC cannot make — does the DoS warrant an advisory?

```text
the defect   an uncatchable process abort, reachable from untrusted input,
             present in every published version up to and including 0.46.1
```

A CHANGELOG entry reaches people who read changelogs. A **GitHub Security Advisory** reaches
`cargo audit` and the RustSec database, and therefore people who do not.

```text
FOR   it is a genuine remote DoS in five published crates
AGAINST  matten is explicitly scoped to PoC, learning and small workflows; an
      advisory carries weight that may overstate the realistic exposure
NOTE  private vulnerability reporting is currently DISABLED on the repository,
      measured 2026-09-01. Publishing an advisory and having no channel to
      RECEIVE the next report is an odd pairing (RFC-130).
```

**This is the owner's decision and it does not block the release.** Ship `0.46.2` either way; an
advisory can follow.

### 7.1 DECIDED 2026-09-03 — no advisory, with a stated trigger

The owner raised the right objection: *"matten has never been in production use. Therefore, I did not
know if it was worth doing."* Measured rather than assumed:

```text
reverse dependencies on matten   4  -> matten-data, matten-mlprep, matten-ndarray, matten-stats
EXTERNAL consumers               0  -> all four are its own companions
age / versions                   74 days, 108 published versions
downloads                        2,983 total  =  ~27 per version
```

**~27 downloads per version is the signature of docs.rs builds, CI and mirrors, not people.** And an
advisory's entire mechanism is `cargo audit` warning a *downstream consumer*. There are none. Filing
would consume RustSec reviewer attention and add a line to every downstream audit report, to reach
nobody.

**Decision: no advisory.** The `### Security` CHANGELOG entry already does everything an advisory
would, for the population that exists.

**The trigger for revisiting**, so this is not a permanent "no" reached once and forgotten:

```text
curl .../crates/matten/reverse_dependencies
   -> if any dependent name does NOT start with "matten", re-ask.
```

An external dependent cannot be reached any other way, and at that point the calculus inverts.

**Separately decided and unrelated:** private vulnerability reporting was **enabled** on 2026-09-03.
That question is about how the *next* report arrives, not about this defect's exposure, and it does
not depend on adoption at all — this project attracts external audits, which is how this Critical was
found, by someone who had nowhere private to report it. RFC-130 is unblocked.

## 8. Out of scope — a diff touching these is a defect

```text
the 8 `0.46.x` family references     already correct (§3)
E4's three `0.46.0` strings           deliberately preserved
public-api-snapshot.md                zero public items changed (E5)
introduction.md                       no new content to describe
any crates/**/*.rs                    RFC-127 shipped them; only lib.rs's
                                      install-pin doc comment may move
ROADMAP.md, rfcs/**                   records
RFC-128 through RFC-133               none of them ship here
```

## 9. Risks

```text
R1  Retargeting the 8 `0.46.x` references. Same trap as 0.46.1: every release
    before it was a MINOR that moved both forms.
R2  Retargeting E4's three exclusions — `release-checklist.md:276` is RFC-118's
    record that 0.46.0 shipped on red CI, and rewriting it would claim that of
    this release.
R3  Adding a public-api-snapshot row (E5).
R4  Describing the fix as restricting zero-sized dimensions (§6).
R5  Omitting the `Security` heading, so `cargo audit` users and changelog
    scanners see only "Fixed".
R6  Asserting a fixed CHANGELOG occurrence count instead of "no removed line".
R7  Treating this as authorizing the tag or the publish. Each is separate.
```

## 10. Acceptance criteria

```text
[ ] cargo metadata shows 0.46.2 for all five crates
[ ] 21 pins retargeted across 14 files; E4's three exclusions untouched
[ ] all 8 `0.46.x` references UNCHANGED — asserted by diff, not by count alone
[ ] Cargo.lock regenerated and committed
[ ] CHANGELOG [0.46.2]: Security + Fixed + Version, none empty, no §6 falsehood
[ ] public-api-snapshot.md and introduction.md UNCHANGED
[ ] rfcs/**, ROADMAP.md unchanged; CHANGELOG.md has NO removed line
[ ] no .rs diff except crates/matten/src/lib.rs's install-pin doc comment
[ ] nine guards; check-doc-code.sh under RUSTFLAGS="-D warnings"
[ ] RUSTFLAGS="-D warnings" cargo clippy --workspace --all-targets --all-features
[ ] cargo test --workspace; both feature profiles
[ ] NO tag, NO publish
```

## 11. The release sequence

```text
1. push main
2. CONFIRM CI GREEN on the PUSHED COMMIT        RFC-118; a red run stops this
3. tag 0.46.2                                    separate owner authorization
4. publish                                       separate owner authorization
```

The tag sits on the **Prepare** commit per the `0.37.0`–`0.46.1` convention, signed, bare SemVer.

## 12. What this does not fix

```text
- F-7's missing try_ arithmetic          RFC-129, rides 0.47.0
- the limit model's incoherence           RFC-132, rides 0.47.0
- the absence of property testing         RFC-128 — the technique that would
                                          have caught RFC-127 before release
- SECURITY.md, the docs batch, performance
```

**And it does not close the window it opened.** The defect was live in every published version from
`0.17.0`; `0.46.2` ends that for anyone who upgrades. Users who do not upgrade stay exposed, which is
the argument in §7.
