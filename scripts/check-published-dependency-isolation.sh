#!/usr/bin/env bash
set -euo pipefail

# Published-crate dependency isolation check.
#
# Architect ruling (RFC-049 Phase 2 design, 2026-06-24, §B1): peer/benchmark
# dependencies must be *positively proven* not to leak into the published crates,
# rather than relying only on the benchmark harness being workspace-excluded.
#
# This guard is complementary to check-core-dependency-boundary.sh (RFC-022, which
# guards core `matten` specifically). It asserts a per-crate forbidden-dependency
# matrix across ALL four published crates, with one explicit allowance: the
# `matten-ndarray` bridge legitimately depends on `ndarray`.
#
# Like the core guard, the tree is inspected with `--all-features` (so an optional
# dependency behind a non-default feature cannot slip past) and `--edges normal,build`
# (dev-only dependencies — e.g. a future criterion dev-dependency — are out of scope;
# what matters is what ships to downstream users).
#
# It passes today (no published crate carries criterion/nalgebra; only matten-ndarray
# carries ndarray). It is in place before RFC-049 Phase 2 introduces peer dependencies
# into the workspace-excluded harness, so any future leak into a published crate fails
# CI immediately.

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

PUBLISHED_CRATES=(matten matten-data matten-mlprep matten-ndarray)

# Forbidden direct/transitive (normal,build) dependencies per published crate.
# `matten-ndarray` is allowed `ndarray` (its reason for existing); everything else
# forbids the peer/benchmark crates. Core additionally forbids the companions.
forbidden_for() {
  case "$1" in
    matten)         echo "criterion ndarray nalgebra matten-data matten-mlprep matten-ndarray" ;;
    matten-data)    echo "criterion ndarray nalgebra" ;;
    matten-mlprep)  echo "criterion ndarray nalgebra" ;;
    matten-ndarray) echo "criterion nalgebra" ;;  # ndarray intentionally allowed (bridge)
    *)              echo "" ;;
  esac
}

FAILED=0
for crate in "${PUBLISHED_CRATES[@]}"; do
  TREE="$(cargo tree -p "$crate" --all-features --edges normal,build --no-dedupe 2>/dev/null || true)"
  if [ -z "$TREE" ]; then
    echo "Could not inspect cargo tree for published crate: $crate" >&2
    exit 1
  fi
  for dep in $(forbidden_for "$crate"); do
    # Match "<dep> v" (cargo tree prints "name vX.Y.Z"); the trailing " v" guards
    # against prefix collisions (e.g. `ndarray-stats` must not match `ndarray`).
    if printf '%s\n' "$TREE" | grep -E "(^|[[:space:]])${dep} v" >/dev/null; then
      echo "Isolation violation: published crate '$crate' depends on forbidden crate '$dep'" >&2
      FAILED=1
    fi
  done
done

if [ "$FAILED" -ne 0 ]; then
  echo "Published dependency isolation check failed." >&2
  exit 1
fi

echo "Published dependency isolation check passed (matten-ndarray -> ndarray allowed)."
