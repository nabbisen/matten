# Release Guard Checklist — Production Migration Docs

**Project:** `matten`  
**Scope:** RFC-050–054 migration documentation and local advisory tooling notes
**Purpose:** Prevent overclaiming and scope drift in current public docs  

---

## 1. Guard Goals

Protect these promises:

```text
matten remains small and dependency-light
migration support is guidance first
bridge support belongs outside core
matten does not claim to beat production libraries
matten does not promise automatic conversion
matten-data does not become a dataframe library
```

---

## 2. Suggested Current-Docs Scan Scope

```text
docs/src/migration
docs/src/companions.md
README.md, if migration text is added there
crates/matten-ndarray/README.md, if bridge contract text is added there
```

Do not treat historical RFCs as current docs unless they are copied into public guide pages.

---

## 3. Forbidden or Review-Required Phrases

Fail or manually review phrases like:

```text
faster than
drop-in replacement
automatic conversion
automatically convert
replace matten with
matten is better than
competitor ranking
production-ready replacement
```

Context matters. For example, this is acceptable:

```text
This guide does not provide automatic conversion.
```

So a first guard may be warning-only, followed by manual review.

**Phrase-anchored only (architect ruling 2026-06-24).** Do **not** add bare-word bans such
as `always`, `never`, `automatic`, or `replace`: legitimate docs say "always copies",
"never panics", "does not provide automatic conversion", and "`tools/matten-migrate` is a
local advisory helper", so single-word scans false-positive badly. Match multi-word phrases only, and
allow negated, educational, advisory-tool, and future-tense uses. This mirrors how the existing
`scripts/check-release-docs.sh` guards are deliberately scoped to precise patterns.

---

## 4. Dependency Guard

Before merge:

```bash
cargo tree -p matten --all-features
```

must not show new target-library dependencies such as:

```text
ndarray
nalgebra
polars
candle
numpy-related tooling
```

Exception:

```text
matten-ndarray may depend on ndarray.
```

Core `matten` must not.

---

## 5. Dataframe Scope Guard

Migration docs may say:

```text
If you need group-by, join, pivot, or query, migrate to Polars/Pandas.
```

Migration docs must not imply:

```text
matten-data will grow group-by/join/pivot/query.
```

---

## 6. Tooling Scope Guard

Current docs may say:

```text
tools/matten-migrate is a local, unpublished, advisory helper.
matten-migrate can draft a migration-readiness report.
matten-migrate is heuristic and must be reviewed manually.
```

Current docs must not imply:

```text
matten-migrate is a public/published crate.
matten-migrate automatically rewrites your project.
matten-migrate edits Cargo.toml or injects dependencies.
matten-migrate proves conversion correctness.
matten-migrate uploads source, uses the network, or sends telemetry.
```

RFC-054 is closed only for the reviewed local advisory `tools/matten-migrate`
scope. rewrite/apply, source mutation, public-crate packaging, and stronger
automation remain future-owned work requiring a separate RFC or explicit
release-policy decision.

---

## 7. Release Checklist

```text
[ ] migration docs reviewed for overclaims.
[ ] bridge docs reviewed for dependency boundary.
[ ] target playbooks reviewed for "always/never" wording.
[ ] README top-level text remains modest.
[ ] examples referenced exist.
[ ] mdBook builds.
[ ] RFC-054 future automation/public-crate scope remains extracted and unauthorized.
```
