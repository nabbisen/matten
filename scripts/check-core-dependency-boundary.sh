#!/usr/bin/env bash
set -euo pipefail

# Core dependency-boundary check (RFC-022 §10).
#
# Verifies that the core `matten` package does not depend on companion crates or
# heavy external numeric/data frameworks. Run from the workspace (or crate) root.
#
# IMPORTANT: the tree is inspected with `--all-features` so that an *optional*
# dependency behind a non-default feature (e.g. `ndarray = { optional = true }`
# gated by an `ndarray` feature) cannot slip past a default-feature `cargo tree`.
# `--edges normal,build` restricts the gate to the dependency graph that ships to
# downstream users; dev-only dependencies are intentionally out of scope here.

CORE_PACKAGE="${CORE_PACKAGE:-matten}"
FORBIDDEN=(
  ndarray
  nalgebra
  candle-core
  polars
  arrow
  datafusion
  matten-ndarray
  matten-mlprep
  matten-data
)

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

TREE="$(cargo tree -p "$CORE_PACKAGE" --all-features --edges normal,build --no-dedupe 2>/dev/null || true)"
if [ -z "$TREE" ]; then
  echo "Could not inspect cargo tree for package: $CORE_PACKAGE" >&2
  exit 1
fi

FAILED=0
for crate in "${FORBIDDEN[@]}"; do
  # Match "<crate> v" (cargo tree prints "name vX.Y.Z"); the trailing " v" guards
  # against prefix collisions such as `ndarray-stats` matching `ndarray`.
  if printf '%s\n' "$TREE" | grep -E "(^|[[:space:]])${crate} v" >/dev/null; then
    echo "Forbidden dependency found in core $CORE_PACKAGE dependency tree: $crate" >&2
    FAILED=1
  fi
done

if [ "$FAILED" -ne 0 ]; then
  echo "Core dependency boundary check failed." >&2
  exit 1
fi

echo "Core dependency boundary check passed for $CORE_PACKAGE."
