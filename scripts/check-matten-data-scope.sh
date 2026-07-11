#!/usr/bin/env bash
set -euo pipefail

# matten-data anti-scope guard (RFC-042).
#
# `matten-data` may borrow the idea of named columns and table preparation, but it
# must not become a dataframe library. This guard enforces that boundary with three
# PRECISE checks (RFC-042 §8 / architect ruling Q13). It deliberately does NOT
# body-scan for broad/common terms (`index`, `join`, `loc`, `query`), because those
# legitimately appear as loop variables, `Path::join` / `str.join`, and substrings
# of `local`/`location`/`block`.
#
# Run from the workspace root.

DATA_DIR="${MATTEN_DATA_DIR:-crates/matten-data}"
SRC_DIR="${DATA_DIR}/src"
EXAMPLES_DIR="${DATA_DIR}/examples"
README="${DATA_DIR}/README.md"
FAIL=0

# ---------------------------------------------------------------------------
# (1) Example file-name guard.
#     Reject dataframe-story terms in example *file names* (not source bodies).
#     Terms are matched as `_`-delimited tokens, so `join_customers_orders.rs`
#     fails while a file that merely calls `path.join(...)` does not.
# ---------------------------------------------------------------------------
echo "=== (1) matten-data example file-name scope guard ==="
NAME_ALT='groupby|group_by|join|merge|pivot|query|rolling|dataframe|data_frame|series|loc|iloc'
if [ -d "$EXAMPLES_DIR" ]; then
  for f in "$EXAMPLES_DIR"/*.rs; do
    [ -e "$f" ] || continue
    base=$(basename "$f" .rs)
    if printf '%s' "$base" | grep -Eq "(^|_)(${NAME_ALT})(_|$)"; then
      echo "ERROR: example file name implies a dataframe operation: ${f}"
      echo "       (dataframe-story examples are out of matten-data scope, RFC-042 §7)"
      FAIL=1
    fi
  done
fi

# ---------------------------------------------------------------------------
# (2) Public API identifier guard.
#     Reject dataframe-shaped public DEFINITIONS in companion source — matched as
#     definitions (`pub struct/enum/type DataFrame|Series`, `pub fn <verb>(`), not
#     arbitrary text. `Path::join` (a call) and `pub fn joined`/`join_tables` (other
#     names) are not matched.
# ---------------------------------------------------------------------------
echo "=== (2) matten-data public-API scope guard ==="
if grep -rEn 'pub[[:space:]]+(struct|enum|type)[[:space:]]+(DataFrame|Series)\b' "$SRC_DIR"; then
  echo "ERROR: matten-data must not define a public DataFrame/Series type (RFC-042 §4/§5)"
  FAIL=1
fi
if grep -rEn 'pub[[:space:]]+(async[[:space:]]+)?fn[[:space:]]+(groupby|group_by|join|merge|pivot|query|loc|iloc)[[:space:]]*[(<]' "$SRC_DIR"; then
  echo "ERROR: matten-data must not expose a dataframe-style public API (groupby/join/merge/pivot/query/loc/iloc) (RFC-042 §4/§5)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# (3) Documentation scope statement (positive presence check).
#     The README must state the non-goal explicitly. Forbidden words ARE allowed in
#     non-goal sections, so no body scan for them is performed here.
# ---------------------------------------------------------------------------
echo "=== (3) matten-data README scope statement ==="
if [ ! -f "$README" ]; then
  echo "ERROR: ${README} is missing"
  FAIL=1
elif ! grep -qi 'not a dataframe library' "$README"; then
  echo "ERROR: ${README} must state that matten-data is 'not a dataframe library' (RFC-042 §6)"
  FAIL=1
fi

# ---------------------------------------------------------------------------
# Result
# ---------------------------------------------------------------------------
if [ "$FAIL" -eq 0 ]; then
  echo ""
  echo "matten-data scope guard passed."
else
  echo ""
  echo "matten-data scope guard FAILED."
  exit 1
fi
