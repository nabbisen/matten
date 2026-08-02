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
# matrix across ALL FIVE published crates (four when this was first written, before
# matten-stats' addition, RFC-078), with one explicit allowance: the `matten-ndarray`
# bridge legitimately depends on `ndarray`.
#
# Like the core guard, the tree is inspected with `--all-features` (so an optional
# dependency behind a non-default feature cannot slip past) and `--edges normal,build`
# (dev-only dependencies — e.g. a future criterion dev-dependency — are out of scope;
# what matters is what ships to downstream users).
#
# It passes today (no published crate carries criterion/nalgebra/wasm-bindgen; only
# matten-ndarray carries ndarray). It is in place before RFC-049 Phase 2 introduces peer
# dependencies into the workspace-excluded harness, so any future leak into a published
# crate fails CI immediately.
#
# wasm-bindgen (RFC-093 §4) added 2026-08-02: RFC-093's central safety claim is that this
# guard already covers a wasm-bindgen leak from the shape playground's workspace-excluded
# binding crate. That was false when written — the blocklist below did not name
# wasm-bindgen at all, so it would have passed even with wasm-bindgen injected directly
# into a published crate (confirmed by injecting it into matten-ndarray and observing
# this script exit 0). Added here so the claim is actually true, not merely asserted.

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required" >&2
  exit 1
fi

PUBLISHED_CRATES=(matten matten-data matten-mlprep matten-ndarray matten-stats)

# Forbidden direct/transitive (normal,build) dependencies per published crate.
# `matten-ndarray` is allowed `ndarray` (its reason for existing); everything else
# forbids the peer/benchmark crates. Core additionally forbids the companions.
forbidden_for() {
  case "$1" in
    matten)         echo "criterion ndarray nalgebra wasm-bindgen matten-data matten-mlprep matten-ndarray matten-stats" ;;
    matten-data)    echo "criterion ndarray nalgebra wasm-bindgen" ;;
    matten-mlprep)  echo "criterion ndarray nalgebra wasm-bindgen" ;;
    matten-ndarray) echo "criterion nalgebra wasm-bindgen" ;;  # ndarray intentionally allowed (bridge)
    matten-stats)   echo "criterion ndarray nalgebra wasm-bindgen" ;;
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
