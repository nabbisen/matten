#!/usr/bin/env bash
# Release documentation truth check (RFC-015 §4, RFC-031).
# Run from the workspace root before any public release. Exits 1 on any issue.

set -euo pipefail
FAIL=0

CORE="crates/matten"
NDARRAY="crates/matten-ndarray"
MLPREP="crates/matten-mlprep"
DATA="crates/matten-data"

# ---------------------------------------------------------------------------
# Core checks
# ---------------------------------------------------------------------------

echo "=== Checking for stale runtime 'matten 0.x' version strings ==="
if grep -rn "matten 0\." "$CORE/src/" | grep -v "CHANGELOG\|#\[\|0\.1\.x\|0\.x" | grep -v "^Binary"; then
  echo "ERROR: versioned wording found in runtime code"
  FAIL=1
fi

echo "=== Checking for stale version-specific crate docs in lib.rs ==="
if grep -n "This is \*\*\`0\." "$CORE/src/lib.rs"; then
  echo "ERROR: version-stamped text found in crate-level docs"
  FAIL=1
fi

echo "=== Checking for stale RFC count phrases in README ==="
if grep -n "All [0-9]* design RFCs" "$CORE/README.md"; then
  echo "WARNING: stale RFC count — update to describe RFC range"
fi

echo "=== Checking that core root exports match the allowlist ==="
ACTUAL=$(grep "^pub use" "$CORE/src/lib.rs" | grep -v "#\[doc(hidden)\]" || true)
for required in "Tensor" "MattenError" "DataFormat" "SliceBuilder"; do
  if ! echo "$ACTUAL" | grep -q "$required"; then
    echo "ERROR: required core root export missing: $required"
    FAIL=1
  fi
done

echo "=== Checking core examples do not import hidden plumbing ==="
if grep -rn "IntoSliceRange\|SliceConvert\|SliceSpecRepr" "$CORE/examples/"; then
  echo "ERROR: examples import hidden plumbing"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# CHANGELOG versioning model (RFC-030, RFC-031)
# ---------------------------------------------------------------------------

echo "=== Checking CHANGELOG preamble does not claim independent per-crate SemVer ==="
# Only inspect the preamble — lines before the first release heading ("## [").
# Historical entries legitimately reference the old model by name.
PREAMBLE=$(sed '/^## \[/q' CHANGELOG.md | head -n -1)
if echo "$PREAMBLE" | grep -n "independent per-crate SemVer\|independent per-crate versioning"; then
  echo "ERROR: CHANGELOG preamble still claims independent per-crate SemVer (superseded by RFC-030)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Companion maturity-label checks (RFC-029, RFC-031)
# ---------------------------------------------------------------------------

echo "=== Checking matten-ndarray does not claim Experimental status ==="
if grep -in "experimental" "$NDARRAY/src/lib.rs" | grep -v "//\|#\["; then
  echo "ERROR: matten-ndarray lib.rs still claims Experimental status (should be production-ready candidate)"
  FAIL=1
fi
if grep -i "experimental" "$NDARRAY/Cargo.toml" | grep "description"; then
  echo "ERROR: matten-ndarray Cargo.toml description still says Experimental"
  FAIL=1
fi

echo "=== Checking matten-mlprep does not claim Experimental status ==="
if grep -in "Experimental (0\." "$MLPREP/src/lib.rs"; then
  echo "ERROR: matten-mlprep lib.rs still claims Experimental (0.x) status (should be beta)"
  FAIL=1
fi

echo "=== Checking matten-data declares Beta, not Experimental (RFC-036 / v0.22.0) ==="
# matten-data was promoted to Beta in v0.22.0 once the RFC-036 example suite and the
# malformed-CSV test cleared the RFC-023 §9 gate. Its own current docs (and the
# matten-data rows/sections of shared docs) must not call it Experimental. Historical
# references in rfcs/, CHANGELOG.md, and ROADMAP.md are allowed.
if grep -rIni "experimental" "$DATA/README.md" "$DATA/src/lib.rs" docs/src/examples/data.md "$DATA/examples" 2>/dev/null; then
  echo "ERROR: matten-data current docs still say Experimental (it is Beta as of v0.22.0)"
  FAIL=1
fi
if ! grep -qi "beta" "$DATA/README.md"; then
  echo "ERROR: matten-data README does not declare Beta status"
  FAIL=1
fi
if grep -niE "matten-data.*experimental|table-to-Tensor \(Experimental\)" README.md docs/src/examples/companions.md docs/src/reference/compatibility.md; then
  echo "ERROR: a shared doc still marks matten-data Experimental (should be Beta)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Companion dynamic-rejection guard soundness (RFC-031)
# ---------------------------------------------------------------------------

echo "=== Checking companion dynamic guards are NOT cfg-gated (RFC-031) ==="
if grep -n '#\[cfg(feature = "dynamic")\]' "$NDARRAY/src/convert.rs" "$MLPREP/src/util.rs" 2>/dev/null; then
  echo "ERROR: companion dynamic rejection guard is still behind #[cfg(feature = \"dynamic\")] (RFC-031 regression)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Companion dependency / import convention (RFC-032)
# ---------------------------------------------------------------------------

echo "=== Checking companions do not re-export core matten (RFC-032 §3.2/§3.3) ==="
# Matches `pub use matten;` and `pub use matten::<Item>;`. Whole-crate re-export
# (§3.3) is deferred; introducing it requires amending RFC-032 and relaxing this check.
if grep -rn "pub use matten\b" "$NDARRAY/src" "$MLPREP/src" "$DATA/src" 2>/dev/null; then
  echo "ERROR: companions must not re-export core matten types/crate (RFC-032)"
  FAIL=1
fi

echo "=== Checking Tensor is imported from matten, not a companion (RFC-032 §3.4) ==="
if grep -rn "use matten_ndarray::[^;]*Tensor\|use matten_mlprep::[^;]*Tensor" \
     "$NDARRAY/examples" "$MLPREP/examples" \
     "$NDARRAY/README.md" "$MLPREP/README.md" \
     docs/src 2>/dev/null; then
  echo "ERROR: import Tensor from matten, not a companion (RFC-032 §3.4)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Documentation release-truth checks (codebase deep review, v0.20.15)
# ---------------------------------------------------------------------------
# Scope: user-facing docs only. CHANGELOG.md, ROADMAP.md, and rfcs/ are
# intentionally excluded — they legitimately reference historical versions and
# superseded wording (this is the curated historical-content allowlist).

USER_DOCS=(
  README.md
  "$CORE/README.md" "$NDARRAY/README.md" "$MLPREP/README.md" "$DATA/README.md"
  "$CORE/src/lib.rs" "$NDARRAY/src/lib.rs" "$MLPREP/src/lib.rs" "$DATA/src/lib.rs"
  docs/src
)

echo "=== Checking for stale prior-family version references in user-facing docs ==="
# Current family minor, derived dynamically from the workspace version so it can
# never go stale on a bump (the previous hardcoded value was missed at the 0.23.0
# bump, which is exactly how stale 0.22 pins shipped). The checks below reject
# install pins, `X.Y.x` family labels, and "current vX.Y family" prose whose minor
# is not the current one. Full historical patch refs (e.g. "as of v0.20.1" shipped-in
# notes) are NOT matched, and rfcs/ + CHANGELOG.md + ROADMAP.md remain outside USER_DOCS.
CURRENT_MINOR="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"[0-9]+\.([0-9]+)\.[0-9]+".*/\1/')"
if [ -z "$CURRENT_MINOR" ] || ! echo "$CURRENT_MINOR" | grep -Eq '^[0-9]+$'; then
  echo "ERROR: failed to derive current minor from Cargo.toml"
  exit 1
fi
# (a) install-snippet version pins: `<crate> = "0.NN"` / `version = "0.NN"`
if grep -rInE '(version|matten[a-z-]*) = "0\.[0-9]+"' "${USER_DOCS[@]}" 2>/dev/null \
   | grep -vE "= \"0\.${CURRENT_MINOR}\""; then
  echo "ERROR: stale install-snippet version pin in user-facing docs (pin the current minor 0.${CURRENT_MINOR})"
  FAIL=1
fi
# (b) `X.Y.x family` labels (with or without surrounding backticks). Requires the
#     word "family" so generic patch-notation examples like "(0.13.x)" don't match.
if grep -rInE '0\.[0-9]+\.x.{0,2}family' "${USER_DOCS[@]}" 2>/dev/null \
   | grep -vE "0\.${CURRENT_MINOR}\.x"; then
  echo "ERROR: stale 'X.Y.x family' label in user-facing docs (current family is 0.${CURRENT_MINOR}.x)"
  FAIL=1
fi
# (c) "current [v]X.Y family" prose (e.g. the public-API snapshot header, the
#     introduction page). The `v` prefix is optional: both "current v0.24 family"
#     and "current 0.24 family" are matched, so a stale ref cannot hide behind a
#     spelling difference (v0.24.1 deep-review P2).
if grep -rInE 'current v?0\.[0-9]+ family' "${USER_DOCS[@]}" 2>/dev/null \
   | grep -vE "current v?0\.${CURRENT_MINOR} family"; then
  echo "ERROR: stale 'current [v]X.Y family' reference in user-facing docs"
  FAIL=1
fi

echo "=== Checking for skeleton-era / pre-API wording in user-facing docs ==="
if grep -rIn 'M0 skeleton\|when added\|When the public API lands\|coming in a later milestone' "${USER_DOCS[@]}" 2>/dev/null; then
  echo "ERROR: skeleton-era / pre-API wording in user-facing docs (docs must describe the shipped state)"
  FAIL=1
fi

echo "=== Checking public API snapshot lists the InvalidArgument variant ==="
if ! grep -q 'InvalidArgument' docs/src/reference/public-api-snapshot.md; then
  echo "ERROR: public-api-snapshot.md is missing InvalidArgument (snapshot drifted from the shipped MattenError enum)"
  FAIL=1
fi

echo "=== Checking for retired 'Phase 1 / Phase 2' wording in user-facing docs and examples ==="
# RFC-lifecycle ruling (pre-v0.19.0 audit, Q1) + v0.21.3 deep review: the Phase 1/
# Phase 2 vocabulary is retired from current user-facing docs AND examples in favor
# of numeric-Tensor / dynamic-ingestion terminology. Historical RFCs (rfcs/) and
# CHANGELOG.md may retain it. The RFC-049 benchmark docs (docs/src/benchmarks/) are
# excluded: their "Phase 1–4" is the benchmark program's staged-rollout vocabulary,
# a distinct concept from the retired dynamic-feature phases.
if grep -rIn --exclude-dir=benchmarks 'Phase[ -]1\|Phase[ -]2' "${USER_DOCS[@]}" "$CORE/examples" 2>/dev/null; then
  echo "ERROR: retired 'Phase 1 / Phase 2' wording in user-facing docs or examples (use 'numeric Tensor' / 'dynamic ingestion' terminology)"
  FAIL=1
fi

echo "=== Checking root README crate table uses family wording, not bare patch versions ==="
# Crate-table rows look like: | [`name`](path) | VERSION | STATUS | desc |
# A bare patch version (0.20.0) in the version cell drifts every release; require
# "N.M.x family" instead.
if grep -nE '^\| \[.*\]\(.*\) \| [0-9]+\.[0-9]+\.[0-9]+ ' README.md; then
  echo "ERROR: root README crate table has a bare patch version; use 'N.M.x family'"
  FAIL=1
fi

echo "=== Checking core matten example naming convention ==="
# Examples reorganization ruling (architect, 2026-06-24): core matten examples must
# follow one of the two accepted naming patterns — a two-digit-prefixed band name or
# the dynamic_ prefix.  Unnumbered stray files (fossils, ad-hoc snippets) are not
# permitted; they must be placed in an appropriate numbered band.
# Allowlist: no exceptions currently.
EXAMPLES_DIR="crates/matten/examples"
bad_examples=()
for f in "$EXAMPLES_DIR"/*.rs; do
  name=$(basename "$f" .rs)
  if [[ ! "$name" =~ ^[0-9]{2}_ ]] && [[ ! "$name" =~ ^dynamic_[0-9]{2}_ ]]; then
    bad_examples+=("$name")
  fi
done
if [ ${#bad_examples[@]} -gt 0 ]; then
  echo "ERROR: unnumbered example(s) in crates/matten/examples/ — place in an appropriate band:"
  printf '  %s\n' "${bad_examples[@]}"
  FAIL=1
fi

echo "=== Checking benchmark docs do not describe Phase 2 as unimplemented ==="
# RFC-049 Phase 2 (Rust peer comparison harness/template) shipped in v0.22.4. Guard
# against benchmark *status* docs drifting back to "only Phase 1 implemented" / "Phase 2
# deferred / not implemented". Scoped to current benchmark docs only — NOT RFC history
# (rfcs/) or CHANGELOG, where staged-rollout wording is legitimately preserved. Phase 3/4
# deferral wording is allowed; only Phase 2-as-unimplemented is flagged.
BENCH_DOCS_DIR="docs/src/benchmarks"
if [ -d "$BENCH_DOCS_DIR" ]; then
  if grep -RInE 'Only \*\*Phase 1.*implemented today' "$BENCH_DOCS_DIR" \
     || grep -RInE 'Phase 2[^.]*(not yet implemented|not implemented|is deferred|remains deferred|still deferred|not yet authorized)' "$BENCH_DOCS_DIR"; then
    echo "ERROR: benchmark docs still describe Phase 2 as unimplemented/deferred (it shipped in v0.22.4)"
    FAIL=1
  fi
fi

echo "=== Checking migration docs avoid overclaim phrases ==="
# RFC-050-052 migration docs (docs/src/migration/) must stay in the positioning register:
# no speed-superiority claims, no "drop-in replacement", no claim that matten auto-rewrites
# code. Phrase-anchored (multi-word) only, per architect ruling — no bare-word bans. Scoped
# to docs/src/migration/ only (NOT rfcs/ history or CHANGELOG). The one phrase that may
# legitimately appear in RFC-054 (matten-migrate) future/deferred context is allowed there.
MIG_DOCS_DIR="docs/src/migration"
if [ -d "$MIG_DOCS_DIR" ]; then
  if grep -RInE 'faster than|drop-in replacement|automatically convert|replace matten with|matten is better than|production-ready replacement' "$MIG_DOCS_DIR"; then
    echo "ERROR: migration docs contain an overclaim/ranking phrase (positioning, not ranking)"
    FAIL=1
  fi
  # "automatic conversion" is allowed only in matten-migrate future/deferred context.
  # "automatic conversion" is allowed in matten-migrate future/deferred context, and in the
  # negated advisory disclaimer ("does not perform automatic conversion") required by RFC-053.
  if grep -RInE 'automatic conversion' "$MIG_DOCS_DIR" | grep -viE 'matten-migrate|deferred|future|does not perform automatic'; then
    echo "ERROR: migration docs claim 'automatic conversion' outside RFC-054 future/deferred context"
    FAIL=1
  fi
fi

echo "=== Checking CHANGELOG release headings are well-formed ==="
# (1) The current workspace version must be the top-most release heading, so a release never
#     ships without its own heading. (2) No single release block may contain more than one
#     "### Threat model" section — that is the signature of a release block that lost its
#     "## [x.y.z]" heading and got nested under the previous release (the v0.23.4 regression).
CL_VERSION="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"([0-9]+\.[0-9]+\.[0-9]+)".*/\1/')"
CL_TOP="$(grep -m1 -oE '^## \[[0-9]+\.[0-9]+\.[0-9]+\]' CHANGELOG.md | tr -d '#[] ')"
if [ "$CL_TOP" != "$CL_VERSION" ]; then
  echo "ERROR: top CHANGELOG heading ($CL_TOP) does not match workspace version ($CL_VERSION)"
  FAIL=1
fi
if ! awk '
  /^## \[/            { if (tm > 1) { print hdr; bad=1 } hdr=$0; tm=0; next }
  /^### Threat model/ { tm++ }
  END                 { if (tm > 1) { print hdr; bad=1 } exit bad }
' CHANGELOG.md > /tmp/cl_nest 2>/dev/null; then
  echo "ERROR: a CHANGELOG release block has multiple '### Threat model' sections (missing heading?):"
  cat /tmp/cl_nest
  FAIL=1
fi

# ---------------------------------------------------------------------------
# matten-ndarray maturity-label freshness (RFC-057)
# ---------------------------------------------------------------------------
# matten-ndarray is production-ready as of v0.25.0. Its own current-status files
# (crate README, lib.rs, Cargo.toml description) must not still call it a
# "candidate". Historical contexts (CHANGELOG, rfcs/, migration narrative) are
# intentionally NOT scanned here, so prior-status references remain intact.
if grep -rInE 'production-ready candidate' \
     crates/matten-ndarray/README.md \
     crates/matten-ndarray/src/lib.rs \
     crates/matten-ndarray/Cargo.toml 2>/dev/null; then
  echo "ERROR: stale 'production-ready candidate' label in matten-ndarray status files (now production-ready)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# matten-mlprep maturity-label freshness (RFC-058)
# ---------------------------------------------------------------------------
# matten-mlprep is production-ready candidate as of v0.26.0. Its own current-status
# files must not still carry a "Beta" maturity label. Historical contexts
# (CHANGELOG, rfcs/, maturity-progression narrative) are intentionally NOT scanned.
if grep -rInE '\bBeta\b' \
     crates/matten-mlprep/README.md \
     crates/matten-mlprep/src/lib.rs \
     crates/matten-mlprep/Cargo.toml 2>/dev/null; then
  echo "ERROR: stale 'Beta' label in matten-mlprep status files (now production-ready candidate)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Result
# ---------------------------------------------------------------------------

if [ "$FAIL" -eq 0 ]; then
  echo ""
  echo "All release documentation checks passed."
else
  echo ""
  echo "One or more release documentation checks FAILED."
  exit 1
fi
