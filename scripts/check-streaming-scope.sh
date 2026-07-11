#!/usr/bin/env bash
set -euo pipefail

# Streaming / large-CSV anti-scope guard (RFC-037).
#
# The project may document that streaming is deferred, but published crates must
# not accidentally grow streaming-shaped public APIs or examples before a future
# implementation RFC authorizes them. This guard scans definitions and example
# filenames only; it deliberately does not scan prose or comments.
#
# Run from the workspace root.

CRATES_GLOB="${MATTEN_CRATES_GLOB:-crates/*}"
FAIL=0

FN_ALT='stream_csv|large_csv_streaming'
TYPE_ALT='CsvStream|BatchReader|AsyncCsvReader'
EXAMPLE_ALT='stream_csv|csv_stream|batch_reader|async_csv|large_csv_streaming'

non_comment_matches() {
  local pattern=$1
  local dir=$2
  local matches

  matches=$(grep -rEn --include '*.rs' "$pattern" "$dir" 2>/dev/null || true)
  if [ -z "$matches" ]; then
    return 0
  fi

  printf '%s\n' "$matches" | awk '
    {
      content = $0
      sub(/^[^:]+:[0-9]+:/, "", content)
      if (content !~ /^[[:space:]]*(\/\/|\/\*|\*)/) {
        print $0
      }
    }
  '
}

echo "=== (1) published-crate streaming public-API guard ==="
for src_dir in $CRATES_GLOB/src; do
  [ -d "$src_dir" ] || continue

  matches=$(non_comment_matches "pub[[:space:]]+(async[[:space:]]+)?fn[[:space:]]+(${FN_ALT})[[:space:]]*[(<]" "$src_dir")
  if [ -n "$matches" ]; then
    printf '%s\n' "$matches"
    echo "ERROR: published crates must not expose streaming-shaped public functions before a future streaming RFC authorizes them."
    FAIL=1
  fi

  matches=$(non_comment_matches "pub[[:space:]]+(struct|enum|type)[[:space:]]+(${TYPE_ALT})\\b" "$src_dir")
  if [ -n "$matches" ]; then
    printf '%s\n' "$matches"
    echo "ERROR: published crates must not expose streaming-shaped public types before a future streaming RFC authorizes them."
    FAIL=1
  fi
done

echo "=== (2) published-crate streaming example-name guard ==="
for examples_dir in $CRATES_GLOB/examples; do
  [ -d "$examples_dir" ] || continue

  for f in "$examples_dir"/*.rs; do
    [ -e "$f" ] || continue
    base=$(basename "$f" .rs)
    if printf '%s' "$base" | grep -Eq "(^|_)(${EXAMPLE_ALT})(_|$)"; then
      echo "ERROR: example file name implies streaming or large-CSV support: ${f}"
      echo "       (streaming remains deferred until a future implementation RFC authorizes it)"
      FAIL=1
    fi
  done
done

if [ "$FAIL" -eq 0 ]; then
  echo ""
  echo "streaming scope guard passed."
else
  echo ""
  echo "streaming scope guard FAILED."
  exit 1
fi
