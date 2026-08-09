#!/usr/bin/env bash
set -euo pipefail

# Workspace-excluded-tool test guard (RFC-117).
#
# tools/*/tests/*.sh are shell test suites for tools that Cargo.toml's
# `exclude` list keeps out of the workspace, so `cargo test --workspace`
# never reaches them. Before this guard, nothing local ran them either —
# .github/workflows/test.yaml was the ONLY thing that did. That gap is how
# RFC-111's process-boundary regression shipped and stayed unnoticed through
# eight guards, clippy, and `cargo test --workspace`, right up to a tagged
# and published release: every one of them looks somewhere this guard now
# also looks.
#
# The command list is DERIVED from test.yaml rather than hand-copied — a
# hand-copied list is the next thing to drift, and this guard exists because
# of exactly that kind of drift between what CI runs and what anything local
# runs.

WORKFLOW=".github/workflows/test.yaml"

if [ ! -f "$WORKFLOW" ]; then
  echo "ERROR: $WORKFLOW not found (run from the repository root)"
  exit 1
fi

COMMANDS_FILE="$(mktemp)"
trap 'rm -f "$COMMANDS_FILE"' EXIT

grep -E '^[[:space:]]*- run: bash tools/' "$WORKFLOW" |
  sed -E 's/^[[:space:]]*- run: //' >"$COMMANDS_FILE"

if [ ! -s "$COMMANDS_FILE" ]; then
  echo "ERROR: derived zero commands from $WORKFLOW — the pattern this guard" >&2
  echo "  greps for ('- run: bash tools/...') may have drifted from the" >&2
  echo "  workflow's actual wording. Fix the pattern, do not hand-copy a list." >&2
  exit 1
fi

echo "=== Running $(wc -l <"$COMMANDS_FILE" | tr -d '[:space:]') tool-test command(s) derived from $WORKFLOW ==="

FAILED=0
while IFS= read -r cmd; do
  echo "--- $cmd ---"
  if ! bash -c "$cmd"; then
    echo "FAILED: $cmd" >&2
    FAILED=1
  fi
done <"$COMMANDS_FILE"

if [ "$FAILED" -ne 0 ]; then
  echo "Tool-test guard failed." >&2
  exit 1
fi

echo "All derived tool-test commands passed."
