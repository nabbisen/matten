#!/usr/bin/env bash
# Release documentation truth check (RFC-015 §4, RFC-031).
# Run from the workspace root before any public release. Exits 1 on any issue.

set -euo pipefail
FAIL=0

CORE="crates/matten"
NDARRAY="crates/matten-ndarray"
MLPREP="crates/matten-mlprep"
DATA="crates/matten-data"
STATS="crates/matten-stats"

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

# Shared present-tense maturity-CLAIM pattern, used by the matten-data docs check and by
# the companion example check at the end of this section. Split into a verb lead and a
# per-crate label set, because the correct label differs by crate and a check must not
# reject a crate for stating its own true one.
#
# A label is matched only as this project writes labels: bolded (**Experimental**), or
# bare but capitalised as a proper noun (Experimental), which is the banner form. Bare
# lowercase "experimental"/"beta" is deliberately NOT matched -- that is ordinary prose,
# as in "an experimental approach to schema inference". "production-ready candidate" gets
# no such carve-out: it is multi-word with no innocent adjectival reading, so it is
# rejected in either case.
CLAIM_LEAD='\b(is|remains|stays)\b( +(a|an|the|still|currently|now|at|only))* +'
# Each set carries its own trailing boundary, so call sites must NOT append one: a `\b`
# after the closing `**` can never match, since `*` and the following `.` or space are both
# non-word characters. Appending it silently killed the bolded branch -- the most common way
# a label is actually written -- while the bare branch kept working, so the check looked
# alive. Caught by probing each shape separately rather than trusting one passing probe.
LABELS_FOR_PRODUCTION_READY='(\*\*([Ee]xperimental|[Bb]eta|[Pp]roduction-ready candidate)\*\*|(Experimental|Beta|[Pp]roduction-ready candidate)\b)'
LABELS_FOR_CANDIDATE='(\*\*([Ee]xperimental|[Bb]eta)\*\*|(Experimental|Beta)\b)'

# Every companion that has reached production-ready is checked by one function, so the
# four assertions below cannot drift apart per crate and a future promotion inherits them
# by adding a call. This replaces four hand-rolled variants that had drifted badly:
# matten-ndarray's lib.rs check was DEAD (it piped through `grep -v "//"`, which strips
# every `//!` doc-comment line in the file, so no doc comment could ever trip it), neither
# matten-ndarray nor matten-mlprep checked its README banner for Experimental at all, and
# neither had any positive assertion -- deleting the declaration outright passed. All three
# proven by deliberate injection before this function was written. Provenance of the checks
# folded in here: RFC-029/RFC-031 (original), RFC-057 (ndarray production-ready),
# RFC-058/RFC-080 (mlprep), RFC-059/RFC-085 (data).
#
# Historical contexts (CHANGELOG, rfcs/, ROADMAP, compatibility.md) are deliberately NOT
# scanned, and within these files a promotion narrative is allowed: the negative checks are
# anchored to the banner/Status-line START, and the body is checked only for a PRESENT-TENSE
# claim. A whole-file word ban was tried for these crates and is what this replaces -- it
# works only while the file happens to contain no maturity history, and booby-traps the
# first person to add some (RFC-084 review C1: rejecting legitimate history is the tail
# wagging the dog).
check_production_ready_crate() {
  local crate="$1" dir="crates/$1"
  # 1. Banner/Status line must not carry a superseded label.
  if grep -niE '^> \*\*(Beta|Experimental|Production-ready candidate)\b' "$dir/README.md" 2>/dev/null \
     || grep -niE '^//! \*\*(Beta|Experimental|Production-ready candidate)\b' "$dir/src/lib.rs" 2>/dev/null \
     || grep -niE 'experimental|beta|candidate' "$dir/Cargo.toml" 2>/dev/null; then
    echo "ERROR: stale Beta/Experimental/candidate maturity LABEL in $crate status files (now production-ready)"
    FAIL=1
  fi
  # 2. Body prose must not claim one either. Present-tense only, so "promoted from Beta in
  #    v0.22.0" survives. This is the coverage a banner anchor alone loses.
  if grep -nE "${CLAIM_LEAD}${LABELS_FOR_PRODUCTION_READY}" "$dir/README.md" "$dir/src/lib.rs" 2>/dev/null; then
    echo "ERROR: $crate body prose still claims Experimental/Beta/candidate (now production-ready)"
    FAIL=1
  fi
  # 3./4. The declaration must be PRESENT, not merely not-wrong -- an absent label is a
  #    different failure from a stale one. Anchored to the real banner shapes: `\bProduction-ready\b`
  #    alone also matches "Production-ready candidate" at the space (RFC-085 review C2), so the
  #    README form requires the " (" that only the bare label is followed by, and the lib.rs form
  #    requires "." or "*" immediately after -- matten-data writes "**Production-ready** (RFC-085)"
  #    while ndarray and mlprep write "**Production-ready.**", and both must pass.
  if ! grep -qE '^> \*\*Production-ready \(' "$dir/README.md" 2>/dev/null; then
    echo "ERROR: $crate README does not declare production-ready status at its banner"
    FAIL=1
  fi
  if ! grep -qE '^//! \*\*Production-ready[.*]' "$dir/src/lib.rs" 2>/dev/null; then
    echo "ERROR: $crate lib.rs does not declare production-ready status at its Status line"
    FAIL=1
  fi
}

echo "=== Checking production-ready companions declare that label, and no superseded one ==="
check_production_ready_crate matten-ndarray
check_production_ready_crate matten-mlprep
check_production_ready_crate matten-data

echo "=== Checking matten-data declares production-ready, not candidate/Experimental/Beta (RFC-059, RFC-085) ==="
# matten-data: Experimental -> Beta (v0.22.0, RFC-036) -> production-ready candidate (v0.27.0,
# RFC-059) -> production-ready (RFC-085), closing RFC-059 SS6's deferred full-production review.
# Its own current-status LABEL and the matten-data rows/sections of current-status shared docs
# must reflect the current rung. Context-aware: the historical "promoted to Beta in v0.22.0, then
# to production-ready candidate in v0.27.0" narrative in the README body is allowed, as are
# per-family entries in compatibility.md and references in rfcs/, CHANGELOG.md, ROADMAP.md.
#
# An earlier blanket `grep -rIni "experimental"` check across four whole paths (including general
# reference pages) was tried and rejected (RFC-084 review C1 for matten-stats): it also rejects
# legitimate maturity-history prose. Replacing it with nothing was ALSO tried and found wrong by a
# later review round (RFC-085 review C1): docs/src/examples/data.md and crates/matten-data/examples/
# are not otherwise covered by the anchored checks above (those only look at README.md/lib.rs), so
# removing the blanket check silently dropped real coverage -- confirmed live, since
# crates/matten-data/examples/csv_to_tensor.rs still said "matten-data is **Beta**" at the time.
# The correct middle ground: a PRESENT-TENSE claim only ("is"/"remains"/"stays" ... LABEL), which
# rejects "is Experimental" / "is currently ... candidate" while accepting "was promoted from ..."
# and an unrelated "an experimental approach to ...". Only articles/adverbs may sit between the
# verb and the label (a/an/the/still/currently/now/at/only) -- an earlier "any 50 characters"
# version let an unrelated "experimental" mentioned anywhere downstream of any "is" false-positive
# on ordinary prose (RFC-085 review round-2 R1).
#
# The claim pattern is shared, defined just above, and case-SENSITIVE for the one-word
# labels: the case-insensitive version this check shipped with rejected "is an experimental
# approach to schema inference", the very sentence the paragraph above claims it accepts.
# Scope note: this check now owns only the shared docs page. Every companion's
# examples/*.rs is covered uniformly by the example-label check further below, which
# reuses the same pattern -- keeping matten-data's example directory here as well would
# double-report the same line.
if grep -nE "${CLAIM_LEAD}${LABELS_FOR_PRODUCTION_READY}" \
     docs/src/examples/data.md 2>/dev/null; then
  echo "ERROR: matten-data current docs still say Experimental/Beta/candidate (now production-ready)"
  FAIL=1
fi
#
# "production-ready candidate" CONTAINS "production-ready" as a substring, and matten-data's own
# README/lib.rs legitimately narrate "then to production-ready candidate in v0.27.0 (RFC-059)" as
# HISTORY mid-sentence (the same shape as the pre-existing "promoted to Beta" allowance below) --
# so the candidate check, like Beta/Experimental, must be anchored to the banner/Status-line START,
# not a blanket whole-file substring search that would also reject that legitimate sentence.
# matten-data's own status files are covered by check_production_ready_crate above.
# Current-status shared docs (NOT compatibility.md — it carries allowed per-family history).
if grep -niE 'matten-data.*\((Experimental|Beta|production-ready candidate)\)' docs/src/examples/companions.md docs/src/examples/index.md 2>/dev/null \
   || grep -niE 'matten-data.*\| (experimental|beta|production-ready candidate) \|' README.md 2>/dev/null \
   || grep -niE 'matten-data` is (a )?\*\*(Experimental|Beta|production-ready candidate)\*\*' docs/src/examples/companions.md docs/src/examples/data.md 2>/dev/null; then
  echo "ERROR: a current-status shared doc still marks matten-data Experimental/Beta/candidate (should be production-ready)"
  FAIL=1
fi

echo "=== Checking matten-stats declares production-ready candidate, not Experimental (RFC-084) ==="
# matten-stats: Experimental (RFC-078) -> production-ready candidate (RFC-084), once its
# six-function surface settled (RFC-083). Its own current-status LABEL and the matten-stats
# rows/sections of current-status shared docs must reflect the current rung. Context-aware:
# the historical "Experimental" narrative in rfcs/, CHANGELOG.md, and ROADMAP.md is allowed, as
# is per-family history in compatibility.md.
#
# This targets a current-tense label claim only ("... is a separate, `Experimental` companion
# crate ..."), not a promotion-history sentence ("promoted from Experimental in RFC-084") and not
# an unrelated use of the word "experimental" elsewhere on this general reference page. A blanket
# ban on the word anywhere in the file was tried and rejected: it also rejects legitimate
# maturity-history prose, which is the tail wagging the dog (RFC-084 review C1).
if grep -nE '`Experimental`[[:space:]]*companion' docs/src/reference/stats.md 2>/dev/null; then
  echo "ERROR: docs/src/reference/stats.md still describes matten-stats as an Experimental companion crate (now production-ready candidate)"
  FAIL=1
fi
# Current LABEL must not be Experimental: the lead README banner, the lib.rs Status line, or
# the Cargo.toml description.
if grep -nE '^> \*\*Experimental \(' "$STATS/README.md" 2>/dev/null \
   || grep -nE '^//! \*\*Experimental' "$STATS/src/lib.rs" 2>/dev/null \
   || grep -niE 'experimental' "$STATS/Cargo.toml" 2>/dev/null; then
  echo "ERROR: stale Experimental maturity LABEL in matten-stats status files (now production-ready candidate)"
  FAIL=1
fi
if ! grep -qi "production-ready candidate" "$STATS/README.md" 2>/dev/null; then
  echo "ERROR: matten-stats README does not declare production-ready candidate status"
  FAIL=1
fi
# matten-data has had a POSITIVE lib.rs Status-line check since RFC-085; matten-stats did
# not, so deleting its declaration outright passed silently (proven by deliberate removal
# before this check was written). The negative check above only fires on a WRONG label --
# an absent one is a different failure and needs its own assertion. Not anchored to the
# file head: the declaration sits below the boundary and estimator sections, unlike
# matten-data's, which leads its module doc.
if ! grep -qE '^//! \*\*Production-ready candidate\*\* \(' "$STATS/src/lib.rs" 2>/dev/null; then
  echo "ERROR: matten-stats lib.rs does not declare production-ready candidate status"
  FAIL=1
fi
# Current-status shared docs (NOT compatibility.md — it carries allowed per-family history).
if grep -niE 'matten-stats.*\(Experimental\)' docs/src/examples/companions.md docs/src/examples/index.md 2>/dev/null \
   || grep -niE 'matten-stats.*\| experimental \|' README.md 2>/dev/null \
   || grep -niE 'matten-stats` is (a )?\*\*Experimental\*\*' docs/src/examples/companions.md docs/src/reference/stats.md 2>/dev/null; then
  echo "ERROR: a current-status shared doc still marks matten-stats Experimental (should be production-ready candidate)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Companion example-file maturity claims (RFC-084 review C1, RFC-085 review C1/R1)
# ---------------------------------------------------------------------------
# The per-crate blocks above check each companion's own README banner, lib.rs Status line
# and Cargo.toml description, plus named current-status shared docs. Until now only ONE of
# the four also checked crates/*/examples/*.rs: matten-data, and it got that coverage only
# because RFC-085 review C1 caught csv_to_tensor.rs still asserting "matten-data is
# **Beta**" long after the promotion. The hole was left open for the other three. It was
# reachable, not theoretical: an example line reading "`matten-stats` is **Experimental**"
# was injected into crates/matten-stats/examples/histogram.rs and this script still exited
# 0. Enumerating the sites one promotion at a time is what rule 002 warns against, so all
# four are covered in one loop and a new companion inherits the check by existing.
#
# Same present-tense pattern as the matten-data docs check above -- only a CLAIM
# ("is"/"remains"/"stays", optional article or adverb, LABEL) is rejected, so
# promotion-history prose ("promoted from Experimental in RFC-084") and unrelated wording
# ("an experimental approach to ...") still pass. The forbidden set is per-crate because
# the correct answer differs: matten-stats sits at production-ready candidate, so that
# label is banned for the other three only -- banning it everywhere would reject
# matten-stats for being right.
check_example_labels() {
  local crate="$1" labels="$2"
  if grep -nE "${CLAIM_LEAD}${labels}" "crates/$crate"/examples/*.rs 2>/dev/null; then
    echo "ERROR: a $crate example claims a stale maturity label (see that crate's Status line for the current one)"
    FAIL=1
  fi
}
echo "=== Checking companion examples claim no stale maturity label (RFC-084, RFC-085) ==="
check_example_labels matten-ndarray "$LABELS_FOR_PRODUCTION_READY"
check_example_labels matten-mlprep  "$LABELS_FOR_PRODUCTION_READY"
check_example_labels matten-data    "$LABELS_FOR_PRODUCTION_READY"
check_example_labels matten-stats   "$LABELS_FOR_CANDIDATE"

# ---------------------------------------------------------------------------
# Companion dynamic-rejection guard soundness (RFC-031)
# ---------------------------------------------------------------------------

echo "=== Checking companion dynamic guards are NOT cfg-gated (RFC-031) ==="
if grep -n '#\[cfg(feature = "dynamic")\]' \
     "$NDARRAY/src/convert.rs" "$MLPREP/src/util.rs" \
     "$STATS/src/covariance.rs" "$STATS/src/quantile.rs" 2>/dev/null; then
  echo "ERROR: companion dynamic rejection guard is still behind #[cfg(feature = \"dynamic\")] (RFC-031 regression)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Companion dependency / import convention (RFC-032)
# ---------------------------------------------------------------------------

echo "=== Checking companions do not re-export core matten (RFC-032 §3.2/§3.3) ==="
# Matches `pub use matten;` and `pub use matten::<Item>;`. Whole-crate re-export
# (§3.3) is deferred; introducing it requires amending RFC-032 and relaxing this check.
if grep -rn "pub use matten\b" "$NDARRAY/src" "$MLPREP/src" "$DATA/src" "$STATS/src" 2>/dev/null; then
  echo "ERROR: companions must not re-export core matten types/crate (RFC-032)"
  FAIL=1
fi

echo "=== Checking Tensor is imported from matten, not a companion (RFC-032 §3.4) ==="
if grep -rn "use matten_ndarray::[^;]*Tensor\|use matten_mlprep::[^;]*Tensor\|use matten_stats::[^;]*Tensor" \
     "$NDARRAY/examples" "$MLPREP/examples" "$STATS/examples" \
     "$NDARRAY/README.md" "$MLPREP/README.md" "$STATS/README.md" \
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
  "$CORE/README.md" "$NDARRAY/README.md" "$MLPREP/README.md" "$DATA/README.md" "$STATS/README.md"
  "$CORE/src/lib.rs" "$NDARRAY/src/lib.rs" "$MLPREP/src/lib.rs" "$DATA/src/lib.rs" "$STATS/src/lib.rs"
  docs/src
)

echo "=== Checking for stale prior-family version references in user-facing docs ==="
# Current family minor, derived dynamically from the workspace version so it can
# never go stale on a bump (the previous hardcoded value was missed at the 0.23.0
# bump, which is exactly how stale 0.22 pins shipped). The checks below reject
# install pins, `X.Y.x` family labels, and "current vX.Y family" prose whose minor
# is not the current one. Full historical patch refs (e.g. "as of v0.20.1" shipped-in
# notes) are NOT matched, and rfcs/ + CHANGELOG.md + ROADMAP.md remain outside USER_DOCS.
CURRENT_MINOR="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"[0-9]+\.([0-9]+)\.[0-9]+(-[A-Za-z0-9.+-]+)?".*/\1/')"
if [ -z "$CURRENT_MINOR" ] || ! echo "$CURRENT_MINOR" | grep -Eq '^[0-9]+$'; then
  echo "ERROR: failed to derive current minor from Cargo.toml"
  exit 1
fi
# (a) install-snippet version pins: `<crate> = "0.NN"` / `version = "0.NN"`
#     / exact prerelease pins such as `0.NN.0-pre.1`.
if grep -rInE '(version|matten[a-z-]*) = "0\.[0-9]+([^"]*)?"' "${USER_DOCS[@]}" 2>/dev/null \
   | grep -vE "= \"0\.${CURRENT_MINOR}([.\"-]|$)"; then
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
# (c) "current [v]X.Y [release] family" prose (e.g. the public-API snapshot header, the
#     introduction page). The `v` prefix is optional: both "current v0.24 family"
#     and "current 0.24 release family" are matched, so a stale ref cannot hide
#     behind a spelling difference (v0.24.1 deep-review P2; v0.37.0 release-review B1).
if grep -rInE 'current v?0\.[0-9]+( release)? family' "${USER_DOCS[@]}" 2>/dev/null \
   | grep -vE "current v?0\.${CURRENT_MINOR}( release)? family"; then
  echo "ERROR: stale 'current [v]X.Y [release] family' reference in user-facing docs"
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

echo "=== Checking educational positioning consistency and overclaim guard (RFC-065) ==="
# RFC-065 keeps matten's public positioning broad enough for learning/teaching
# while still bounded to small workflows and early prototypes. Scope this check
# to current high-visibility positioning surfaces; RFCs and historical notes are
# intentionally excluded.
POSITIONING_DOCS=(
  README.md
  "$CORE/README.md"
  "$CORE/src/lib.rs"
  docs/src/introduction.md
  docs/src/philosophy.md
  docs/src/tutorial/start-here.md
  docs/src/examples/visual-understanding.md
)
for doc in "${POSITIONING_DOCS[@]}"; do
  if ! grep -qiE 'learn|learning' "$doc"; then
    echo "ERROR: RFC-065 positioning doc lacks learning-oriented wording: $doc"
    FAIL=1
  fi
done
for doc in README.md "$CORE/README.md" "$CORE/src/lib.rs" docs/src/introduction.md; do
  if ! grep -qi 'teaching' "$doc"; then
    echo "ERROR: RFC-065 high-visibility positioning doc lacks 'teaching': $doc"
    FAIL=1
  fi
done
if grep -RInE 'business-critical|business workflows|production performance|production-scale|scales to|faster than' "${POSITIONING_DOCS[@]}" 2>/dev/null; then
  echo "ERROR: RFC-065 positioning docs contain an overclaim phrase"
  FAIL=1
fi

echo "=== Checking CHANGELOG release headings are well-formed ==="
# (1) The current workspace version must be the top-most release heading, so a release never
#     ships without its own heading. (2) No single release block may contain more than one
#     "### Threat model" section — that is the signature of a release block that lost its
#     "## [x.y.z]" heading and got nested under the previous release (the v0.23.4 regression).
CL_VERSION="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"([0-9]+\.[0-9]+\.[0-9]+(-[A-Za-z0-9.+-]+)?)".*/\1/')"
CL_TOP="$(grep -m1 -oE '^## \[[0-9]+\.[0-9]+\.[0-9]+(-[A-Za-z0-9.+-]+)?\]' CHANGELOG.md | tr -d '#[] ')"
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
# ROADMAP header / document-history parity (RFC-073 0.38.0 release-prep review)
# ---------------------------------------------------------------------------
# The header's Document Version/Date must equal the LAST row of the
# document-history table, mirroring the CHANGELOG top-heading check above. A
# 0.38.0 release-prep pass bumped the header without appending its history
# row and no existing guard caught it; this check closes that gap.

echo "=== Checking ROADMAP.md header matches the last document-history row ==="
RM_HEADER_VERSION="$(grep -m1 -oE '\*\*Document Version:\*\* `[0-9]+\.[0-9]+\.[0-9]+`' ROADMAP.md | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')"
RM_HEADER_DATE="$(grep -m1 -oE '\*\*Date:\*\* [0-9]{4}-[0-9]{2}-[0-9]{2}' ROADMAP.md | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}')"
RM_LAST_ROW="$(grep -E '^\| [0-9]+\.[0-9]+\.[0-9]+ \| [0-9]{4}-[0-9]{2}-[0-9]{2} \|' ROADMAP.md | tail -1)"
RM_LAST_VERSION="$(echo "$RM_LAST_ROW" | awk -F'|' '{gsub(/ /,"",$2); print $2}')"
RM_LAST_DATE="$(echo "$RM_LAST_ROW" | awk -F'|' '{gsub(/ /,"",$3); print $3}')"
if [ -z "$RM_HEADER_VERSION" ] || [ -z "$RM_HEADER_DATE" ] || [ -z "$RM_LAST_VERSION" ] || [ -z "$RM_LAST_DATE" ]; then
  echo "ERROR: failed to parse ROADMAP.md Document Version/Date header or last document-history row"
  FAIL=1
elif [ "$RM_HEADER_VERSION" != "$RM_LAST_VERSION" ] || [ "$RM_HEADER_DATE" != "$RM_LAST_DATE" ]; then
  echo "ERROR: ROADMAP.md header (Document Version $RM_HEADER_VERSION / Date $RM_HEADER_DATE) does not match the last document-history row ($RM_LAST_VERSION / $RM_LAST_DATE)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Benchmark results page freshness (RFC-060)
# ---------------------------------------------------------------------------
# The curated results page (docs/src/benchmarks/results.md) cites the accepted
# Baseline/Report IDs. Those IDs must still exist in the corresponding report
# files, so the book summary cannot outlive the reports it cites. (Checks ID
# presence, not the numeric values — humans curate those.)
RESULTS_PAGE="docs/src/benchmarks/results.md"
if [ -f "$RESULTS_PAGE" ]; then
  for pair in \
    "matten-rfc049-internal-baseline-v0.1:benchmarks/reports/internal-baseline-v0.1.md" \
    "matten-rfc049-rust-peer-comparison-v0.1:benchmarks/reports/peer-comparison-v0.1.md" \
    "matten-rfc049-internal-baseline-v0.2:benchmarks/reports/internal-baseline-v0.2.md" \
    "matten-rfc049-rust-peer-comparison-v0.2:benchmarks/reports/peer-comparison-v0.2.md"; do
    id="${pair%%:*}"; report="${pair##*:}"
    if grep -q "$id" "$RESULTS_PAGE" 2>/dev/null && ! grep -q "$id" "$report" 2>/dev/null; then
      echo "ERROR: results page cites '$id' but it is not in $report (stale benchmark citation)"
      FAIL=1
    fi
  done
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
