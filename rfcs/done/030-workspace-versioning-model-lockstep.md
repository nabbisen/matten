# RFC-030: Workspace Versioning Model — Lock-step Family Versioning

**Status:** Implemented (v0.19.0)
**Target:** v0.19.0
**Theme:** Versioning model for the matten workspace
**Supersedes:** RFC-022 §7 (independent per-crate SemVer)
**Depends on:** RFC-022, RFC-029

---

## 1. Summary

The matten workspace adopts **lock-step family versioning**: every crate in the
workspace shares one version, set once in `[workspace.package].version` and
inherited by each crate via `version.workspace = true`. This release aligns the
family at **0.19.0**.

This **supersedes** RFC-022 §7, which specified independent per-crate SemVer. The
reasoning is recorded below; RFC-022 §7 is annotated as superseded but retained
for history.

## 2. The two roles a version plays, separated

A version number was being asked to signal two different things at once —
*compatibility* and *maturity*. We split them:

- **Version = compatibility.** All crates released together carry the same
  number, so a user knows on sight that `matten = "0.19"`,
  `matten-ndarray = "0.19"`, and `matten-mlprep = "0.19"` are a matched set. No
  compatibility matrix to consult.
- **Maturity = the Status label.** Each crate's maturity (experimental / beta /
  production-ready candidate / production-ready) is declared in its README/docs
  Status, per RFC-022 §9 and RFC-029. A crate at `0.19.0` may still be labeled
  *beta*; the version says "part of the 0.19 family," not "battle-tested."

## 3. Rationale (why lock-step suits matten)

1. **The crates only ship together.** They are released as coordinated milestone
   artifacts, not on independent cadences. Independent SemVer's main benefit —
   independent release timelines — is therefore unused, while its cost
   (per-crate version bookkeeping, a compatibility matrix for users) is paid in
   full.
2. **User ergonomics.** "Use the same number" is the simplest possible
   compatibility contract for the people consuming the family.
3. **Maturity is already labeled.** Because RFC-029 puts maturity in the Status
   label, freeing the version to mean compatibility loses no information.
4. **Cargo supports it directly** via `workspace.package.version` inheritance.

## 4. One-time alignment

Moving to a single family version requires a one-time jump:

```text
matten          0.16.0 -> 0.19.0
matten-ndarray  0.1.1  -> 0.19.0
matten-mlprep   0.1.1  -> 0.19.0
```

No crate's public API or behavior changes in this alignment; only the version
number moves. The CHANGELOG records the jump explicitly so the history is not
misread as 18 minor releases of churn.

## 5. Consequences (accepted)

- A change to a single crate bumps and republishes the **whole family**. This is
  the intended model: the family moves together. It is acceptable because the
  crates already co-release.
- If a future need arises to release one crate independently of the others, this
  decision is revisited — that is the trigger to return to independent SemVer
  (and to the per-crate `CHANGELOG`/`LICENSE` split deferred in RFC-022 §12).

## 6. Mechanics

- `[workspace.package] version = "X.Y.Z"`; each member uses `version.workspace = true`.
- Companion crates depend on core as `matten = { version = "X.Y", ... }`, matching
  the family minor.
- Per-crate `keywords`/`categories`/`description` stay **per crate** (they differ
  and aid crates.io discoverability); only truly shared metadata
  (version, edition, rust-version, license, authors, repository) is inherited.

## 7. Documentation impact

- README must not claim independent SemVer; it states the family-version model
  and that maturity is the Status column.
- RFC-022 §7 annotated as superseded by this RFC.
- ROADMAP §10 updated to the lock-step model.
