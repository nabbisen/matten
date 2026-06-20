#!/usr/bin/env bash
# Release documentation truth check (RFC-015 §4).
# Run before any public release. Exits 1 if any issue is found.

set -euo pipefail
FAIL=0

echo "=== Checking for stale runtime 'matten 0.x' version strings ==="
if grep -rn "matten 0\." src/ | grep -v "CHANGELOG\|#\[" | grep -v "^Binary"; then
  echo "ERROR: versioned wording found in runtime code"
  FAIL=1
fi

echo "=== Checking for stale version-specific crate docs in lib.rs ==="
if grep -n "This is \*\*\`0\." src/lib.rs; then
  echo "ERROR: version-stamped text found in crate-level docs"
  FAIL=1
fi

echo "=== Checking that root exports match the allowlist ==="
ACTUAL=$(grep "^pub use" src/lib.rs | grep -v "#\[doc(hidden)\]" || true)
for required in "Tensor" "MattenError" "DataFormat" "SliceBuilder"; do
  if ! echo "$ACTUAL" | grep -q "$required"; then
    echo "ERROR: required root export missing: $required"
    FAIL=1
  fi
done

echo "=== Checking examples do not import hidden plumbing ==="
if grep -rn "IntoSliceRange\|SliceConvert\|SliceSpecRepr" examples/; then
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
