#!/usr/bin/env bash
set -euo pipefail

fail() {
    printf 'process-boundary: FAIL: %s\n' "$*" >&2
    exit 1
}

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
REPO_ROOT="$(realpath -- "$REPO_ROOT")"
MATTEN_REPORT_PROCESS_TARGET="$REPO_ROOT/target/matten-report-process"
MATTEN_REPORT_PROCESS_TMP="$MATTEN_REPORT_PROCESS_TARGET/tmp"

mkdir -p -- "$MATTEN_REPORT_PROCESS_TARGET" "$MATTEN_REPORT_PROCESS_TMP"
MATTEN_REPORT_PROCESS_TARGET="$(realpath -- "$MATTEN_REPORT_PROCESS_TARGET")"
MATTEN_REPORT_PROCESS_TMP="$(realpath -- "$MATTEN_REPORT_PROCESS_TMP")"

case "$MATTEN_REPORT_PROCESS_TARGET" in
    "$REPO_ROOT"/target/*) ;;
    *) fail "target directory escaped the repository target tree" ;;
esac

case "$MATTEN_REPORT_PROCESS_TMP" in
    "$MATTEN_REPORT_PROCESS_TARGET"/*) ;;
    *) fail "linker temporary directory escaped the process target directory" ;;
esac

MUTATE_MARKDOWN_DIGEST=false
case "${1-}" in
    "") ;;
    --mutate-markdown-digest) MUTATE_MARKDOWN_DIGEST=true ;;
    *) fail "unknown argument: $1" ;;
esac
[[ $# -le 1 ]] || fail "expected at most one argument"

cd -- "$REPO_ROOT"
env TMPDIR="$MATTEN_REPORT_PROCESS_TMP" cargo build \
    --manifest-path tools/matten-report/Cargo.toml \
    --target-dir target/matten-report-process

MATTEN_REPORT_BIN="$MATTEN_REPORT_PROCESS_TARGET/debug/matten-report"
[[ -x "$MATTEN_REPORT_BIN" ]] || fail "built binary is missing or not executable"

CASE_DIR="$(mktemp -d "$MATTEN_REPORT_PROCESS_TARGET/cases.XXXXXX")"
CASE_DIR="$(realpath -- "$CASE_DIR")"
case "$CASE_DIR" in
    "$MATTEN_REPORT_PROCESS_TARGET"/cases.*) ;;
    *) fail "case directory escaped the process target directory" ;;
esac

cleanup() {
    case "$CASE_DIR" in
        "$MATTEN_REPORT_PROCESS_TARGET"/cases.*)
            rm -rf -- "$CASE_DIR"
            ;;
        *)
            printf 'process-boundary: refusing unsafe cleanup path: %s\n' "$CASE_DIR" >&2
            ;;
    esac
}
trap cleanup EXIT

run_case() {
    local name="$1"
    shift
    local stdout_file="$CASE_DIR/$name.stdout"
    local stderr_file="$CASE_DIR/$name.stderr"

    set +e
    "$MATTEN_REPORT_BIN" "$@" >"$stdout_file" 2>"$stderr_file"
    CASE_STATUS=$?
    set -e
    CASE_STDOUT="$stdout_file"
    CASE_STDERR="$stderr_file"
}

assert_status() {
    local name="$1"
    local expected="$2"
    [[ "$CASE_STATUS" -eq "$expected" ]] ||
        fail "$name: expected exit $expected, got $CASE_STATUS"
}

assert_nonzero_status() {
    local name="$1"
    [[ "$CASE_STATUS" -ne 0 ]] || fail "$name: expected a nonzero exit"
}

assert_empty() {
    local name="$1"
    local stream="$2"
    local file="$3"
    [[ ! -s "$file" ]] || fail "$name: expected empty $stream"
}

assert_fingerprint() {
    local name="$1"
    local file="$2"
    local expected_bytes="$3"
    local expected_sha256="$4"
    local actual_bytes
    local actual_sha256

    actual_bytes="$(wc -c <"$file" | tr -d '[:space:]')"
    [[ "$actual_bytes" == "$expected_bytes" ]] ||
        fail "$name: expected $expected_bytes bytes, got $actual_bytes"

    actual_sha256="$(sha256sum "$file" | awk '{print $1}')"
    [[ "$actual_sha256" == "$expected_sha256" ]] ||
        fail "$name: expected SHA-256 $expected_sha256, got $actual_sha256"
}

assert_process_error() {
    local name="$1"
    assert_nonzero_status "$name"
    assert_empty "$name" stdout "$CASE_STDOUT"
    [[ "$(wc -l <"$CASE_STDERR" | tr -d '[:space:]')" == 1 ]] ||
        fail "$name: expected exactly one stderr line"
    grep -q '^matten-report error: ' "$CASE_STDERR" ||
        fail "$name: stderr did not begin with the process error prefix"
    [[ "$(tail -c 1 "$CASE_STDERR" | od -An -t x1 | tr -d '[:space:]')" == 0a ]] ||
        fail "$name: stderr was not newline-terminated"
}

HELP_SHA256="0daaf8e57e0cc4471baa30d6b05bdef76efb265b665aa0ad3fd51e0415286930"
MARKDOWN_SHA256="bdb6014f637455ed235af7eedcda0872b9161f76e362661bbbbe3fe8247e4c22"
JSON_SHA256="6491d3856293572e80f0388be6002703178336447f24afb330087c82ad680fac"
INPUT_JSON_SHA256="84ec3f794c5ccf225bcf5fe88aa1f3d2043179492d776940fe5206c14cae7767"
INPUT_ERROR_JSON_SHA256="f7c7125819e88635e21ab6c4a4769aee0f3a4ba3dc0e16dbfc20c1c82f267751"
if [[ "$MUTATE_MARKDOWN_DIGEST" == true ]]; then
    MARKDOWN_SHA256="0000000000000000000000000000000000000000000000000000000000000000"
fi

run_case help --help
assert_status help 0
assert_fingerprint help "$CASE_STDOUT" 1613 "$HELP_SHA256"
assert_empty help stderr "$CASE_STDERR"

run_case html_requires_output --demo data-readiness --format html
assert_status html_requires_output 1
assert_empty html_requires_output stdout "$CASE_STDOUT"
printf '%s\n' \
    'matten-report error: --format html requires --output <report.html>' \
    >"$CASE_DIR/html_requires_output.expected-stderr"
cmp -s "$CASE_STDERR" "$CASE_DIR/html_requires_output.expected-stderr" ||
    fail "html_requires_output: stderr did not match the policy error"

run_case markdown_stdout --demo data-readiness
assert_status markdown_stdout 0
assert_fingerprint markdown_stdout "$CASE_STDOUT" 404 "$MARKDOWN_SHA256"
assert_empty markdown_stdout stderr "$CASE_STDERR"

JSON_OUTPUT="$CASE_DIR/report.json"
run_case json_file --demo data-readiness --format json --output "$JSON_OUTPUT"
assert_status json_file 0
assert_empty json_file stdout "$CASE_STDOUT"
assert_empty json_file stderr "$CASE_STDERR"
[[ -f "$JSON_OUTPUT" ]] || fail "json_file: expected output file was not created"
assert_fingerprint json_file "$JSON_OUTPUT" 952 "$JSON_SHA256"

INPUT_JSON_OUTPUT="$CASE_DIR/input.json"
run_case input_json_file \
    --input tools/matten-report/fixtures/small.csv \
    --kind data-readiness --select sales,cost \
    --format json --output "$INPUT_JSON_OUTPUT"
assert_status input_json_file 0
assert_empty input_json_file stdout "$CASE_STDOUT"
assert_empty input_json_file stderr "$CASE_STDERR"
[[ -f "$INPUT_JSON_OUTPUT" ]] || fail "input_json_file: expected output file was not created"
assert_fingerprint input_json_file "$INPUT_JSON_OUTPUT" 3176 "$INPUT_JSON_SHA256"

INPUT_ERROR_JSON_OUTPUT="$CASE_DIR/input-error.json"
run_case input_error_json_file \
    --input tools/matten-report/fixtures/non_numeric.csv \
    --kind data-readiness --select sales,cost \
    --format json --output "$INPUT_ERROR_JSON_OUTPUT"
assert_status input_error_json_file 0
assert_empty input_error_json_file stdout "$CASE_STDOUT"
assert_empty input_error_json_file stderr "$CASE_STDERR"
[[ -f "$INPUT_ERROR_JSON_OUTPUT" ]] ||
    fail "input_error_json_file: expected output file was not created"
assert_fingerprint \
    input_error_json_file "$INPUT_ERROR_JSON_OUTPUT" 3077 "$INPUT_ERROR_JSON_SHA256"

HEADER_ONLY_OUTPUT="$CASE_DIR/header-only.json"
run_case header_only_absent \
    --input tools/matten-report/fixtures/header_only.csv \
    --kind data-readiness --select a,b \
    --format json --output "$HEADER_ONLY_OUTPUT"
assert_process_error header_only_absent
[[ ! -e "$HEADER_ONLY_OUTPUT" ]] ||
    fail "header_only_absent: pre-write failure created an output artifact"

HEADER_ONLY_EXISTING_OUTPUT="$CASE_DIR/header-only-existing.json"
HEADER_ONLY_SENTINEL="$CASE_DIR/header-only-sentinel.expected"
printf 'preserve-existing-output\n' >"$HEADER_ONLY_SENTINEL"
cp -- "$HEADER_ONLY_SENTINEL" "$HEADER_ONLY_EXISTING_OUTPUT"
run_case header_only_existing \
    --input tools/matten-report/fixtures/header_only.csv \
    --kind data-readiness --select a,b \
    --format json --output "$HEADER_ONLY_EXISTING_OUTPUT"
assert_process_error header_only_existing
cmp -s "$HEADER_ONLY_EXISTING_OUTPUT" "$HEADER_ONLY_SENTINEL" ||
    fail "header_only_existing: pre-write failure changed existing output"

NON_FINITE_OUTPUT="$CASE_DIR/non-finite.json"
run_case non_finite_absent \
    --input tools/matten-report/fixtures/non_finite.csv \
    --kind data-readiness --select sales,cost \
    --format json --output "$NON_FINITE_OUTPUT"
assert_process_error non_finite_absent
grep -q 'non-finite numeric value' "$CASE_STDERR" ||
    fail "non_finite_absent: stderr did not identify finite-number policy rejection"
[[ ! -e "$NON_FINITE_OUTPUT" ]] ||
    fail "non_finite_absent: representation failure created an output artifact"

MISSING_OUTPUT="$CASE_DIR/missing/report.md"
run_case missing_parent --demo data-readiness --output "$MISSING_OUTPUT"
assert_process_error missing_parent

printf 'process-boundary: PASS\n'
