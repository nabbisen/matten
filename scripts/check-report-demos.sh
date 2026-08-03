#!/usr/bin/env bash
set -euo pipefail

# Report-demo freshness guard (RFC-097 §6).
#
# docs/src/reports/<demo>.md are GENERATED from `matten-report`'s five fixed demos,
# but — unlike the shape playground's wasm artifact — they are COMMITTED, not
# git-ignored. RFC-097 §6 tested rather than assumed the reason: with a SUMMARY.md
# entry pointing at a missing file, mdBook creates it EMPTY and exits 0. Git-ignoring
# these pages would mean a failed or skipped generation step deploys five blank pages
# with nothing reporting a problem — a missing wasm asset degrades one page visibly;
# a missing generated page is invisible.
#
# Committing generated content is only safe with a guard that catches drift, so this
# script regenerates each demo into a temp directory and diffs it against the
# committed copy. It does NOT regenerate in place — a guard that silently fixes drift
# hides the drift instead of reporting it (the same principle
# check-benchmark-dependency-sync.sh's header records for a hand-maintained value).

DEMOS=(shape-flow educational-path mlprep-standardization data-readiness dynamic-readiness)
REPORT_MANIFEST="tools/matten-report/Cargo.toml"
COMMITTED_DIR="docs/src/reports"

if [ ! -f "$REPORT_MANIFEST" ]; then
  echo "ERROR: $REPORT_MANIFEST not found (run from the repository root)"
  exit 1
fi

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "=== Regenerating five report demos and diffing against docs/src/reports/ ==="

FAILED=0
for demo in "${DEMOS[@]}"; do
  committed="$COMMITTED_DIR/$demo.md"
  regenerated="$TMP_DIR/$demo.md"

  if [ ! -f "$committed" ]; then
    echo "ERROR: $committed does not exist — run:" >&2
    echo "  cargo run --manifest-path $REPORT_MANIFEST -- --demo $demo --output $committed" >&2
    FAILED=1
    continue
  fi

  cargo run --manifest-path "$REPORT_MANIFEST" --quiet -- --demo "$demo" --output "$regenerated"

  if ! diff -u "$committed" "$regenerated" >"$TMP_DIR/$demo.diff"; then
    echo "Demo drift: $demo" >&2
    echo "  $committed does not match what tools/matten-report currently generates." >&2
    echo "  Regenerate and commit it:" >&2
    echo "    cargo run --manifest-path $REPORT_MANIFEST -- --demo $demo --output $committed" >&2
    echo "  --- diff (committed vs regenerated) ---" >&2
    cat "$TMP_DIR/$demo.diff" >&2
    FAILED=1
  fi
done

if [ "$FAILED" -ne 0 ]; then
  echo "Report demo freshness check failed." >&2
  exit 1
fi

echo "All five report demos match tools/matten-report's current output."
