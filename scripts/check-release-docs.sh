#!/usr/bin/env bash
# Release documentation truth check (RFC-015 §4).
# Run from the workspace root before any public release. Exits 1 on any issue.

set -euo pipefail
FAIL=0

CORE="crates/matten"

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

if [ "$FAIL" -eq 0 ]; then
  echo ""
  echo "All release documentation checks passed."
else
  echo ""
  echo "One or more release documentation checks FAILED."
  exit 1
fi
