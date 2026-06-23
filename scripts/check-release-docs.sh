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
