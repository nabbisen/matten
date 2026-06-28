#!/usr/bin/env bash
set -euo pipefail

# Benchmark-harness dependency sync check.
#
# The RFC-049 benchmark harness (benchmarks/) is intentionally OUTSIDE the Cargo
# workspace (workspace.exclude in the root Cargo.toml), so its benchmark-only and
# peer dependencies never enter the published crates' dependency graph or the shared
# lockfile. A deliberate consequence of that exclusion is that the harness CANNOT
# inherit workspace dependencies via `{ workspace = true }` — an excluded crate has
# no workspace root to inherit from (`cargo` errors "failed to find a workspace root").
#
# Therefore the harness's peer `ndarray` pin must be kept in sync with the workspace's
# `ndarray` requirement BY HAND. This guard makes "forgot to sync" impossible to miss:
# it fails if the two diverge. Precedent: v0.28.3 had to manually bump the harness peer
# pin 0.16 -> 0.17 after the matten-ndarray bridge moved to ndarray 0.17 (RFC-062).
#
# It compares string-for-string, which is the simplest honest contract: if the two
# requirements are meant to differ deliberately, update this guard alongside them.

ROOT_MANIFEST="Cargo.toml"
BENCH_MANIFEST="benchmarks/Cargo.toml"

for f in "$ROOT_MANIFEST" "$BENCH_MANIFEST"; do
  if [ ! -f "$f" ]; then
    echo "ERROR: $f not found (run from the repository root)"
    exit 1
  fi
done

# Workspace requirement: the `ndarray = "X"` line in [workspace.dependencies].
ws_line="$(grep -E '^ndarray[[:space:]]*=' "$ROOT_MANIFEST" || true)"
ws_ver="$(printf '%s' "$ws_line" | sed -E 's/.*"([^"]+)".*/\1/')"

# Harness peer pin: `ndarray = { version = "X", optional = true }` in benchmarks/Cargo.toml.
bench_line="$(grep -E '^ndarray[[:space:]]*=' "$BENCH_MANIFEST" || true)"
bench_ver="$(printf '%s' "$bench_line" | sed -E 's/.*version[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/')"

if [ -z "$ws_ver" ] || [ "$ws_ver" = "$ws_line" ]; then
  echo "ERROR: could not parse the workspace 'ndarray' requirement from $ROOT_MANIFEST"
  echo "       (expected a line like 'ndarray = \"0.17\"' under [workspace.dependencies])"
  exit 1
fi
if [ -z "$bench_ver" ] || [ "$bench_ver" = "$bench_line" ]; then
  echo "ERROR: could not parse the harness 'ndarray' peer pin from $BENCH_MANIFEST"
  echo "       (expected a line like 'ndarray = { version = \"0.17\", optional = true }')"
  exit 1
fi

if [ "$ws_ver" != "$bench_ver" ]; then
  echo "ERROR: benchmark harness 'ndarray' pin is out of sync with the workspace requirement."
  echo "       workspace   ($ROOT_MANIFEST):  ndarray = \"$ws_ver\""
  echo "       harness peer ($BENCH_MANIFEST): ndarray = \"$bench_ver\""
  echo ""
  echo "  The benchmark harness is workspace-excluded and cannot inherit { workspace = true },"
  echo "  so its peer pin must be updated by hand to match. Edit $BENCH_MANIFEST to use"
  echo "  ndarray = \"$ws_ver\" (and re-run the peers bench to refresh the comparison numbers)."
  exit 1
fi

echo "Benchmark harness 'ndarray' peer pin ($bench_ver) matches the workspace requirement ($ws_ver)."
