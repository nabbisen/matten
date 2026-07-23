#!/usr/bin/env bash
set -euo pipefail

fail() {
    printf 'module-boundaries: FAIL: %s\n' "$*" >&2
    exit 1
}

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/../../.." && pwd)"
REPO_ROOT="$(realpath -- "$REPO_ROOT")"
REPORT_ROOT="$REPO_ROOT/tools/matten-report"
MAX_RUST_LINES=500

FAILED=0

finding() {
    local file="$1"
    local line="$2"
    local message="$3"
    local source="$4"
    printf 'module-boundaries: %s:%s: %s: %s\n' "$file" "$line" "$message" "$source" >&2
    FAILED=1
}

scan_forbidden_dependency() {
    local file="$1"
    local owner="$2"
    local dependency="$3"
    local pattern
    pattern="(crate|super)::[[:space:]]*${dependency}([^[:alnum:]_]|$)"

    while IFS=: read -r line source; do
        [[ -n "$line" ]] || continue
        finding "$file" "$line" "forbidden $owner -> $dependency dependency" "$source"
    done < <(grep -nE "$pattern" "$file" || true)

    while IFS=: read -r line source; do
        [[ -n "$line" ]] || continue
        finding "$file" "$line" "forbidden $owner -> $dependency grouped dependency" "$source"
    done < <(awk -v dependency="$dependency" '
        function is_forbidden_root(token, normalized) {
            normalized = token
            gsub(/^[[:space:]]+|[[:space:]]+$/, "", normalized)
            return normalized == dependency \
                || normalized ~ ("^" dependency "[[:space:]]+as[[:space:]]") \
                || normalized ~ ("^" dependency "::")
        }

        function contains_forbidden_outer_root(statement, start_brace, depth, token, i, char) {
            start_brace = index(statement, "{")
            depth = 0
            token = ""
            for (i = start_brace + 1; i <= length(statement); i++) {
                char = substr(statement, i, 1)
                if (char == "{") {
                    if (depth == 0 && is_forbidden_root(token)) {
                        return 1
                    }
                    depth++
                } else if (char == "}") {
                    if (depth == 0) {
                        return is_forbidden_root(token)
                    }
                    depth--
                } else if (depth == 0 && char == ",") {
                    if (is_forbidden_root(token)) {
                        return 1
                    }
                    token = ""
                } else if (depth == 0) {
                    token = token char
                }
            }
            return 0
        }

        !collecting && $0 ~ /(crate|super)::[[:space:]]*\{/ {
            start = NR
            statement = $0
            collecting = 1
        }
        collecting && NR != start {
            statement = statement " " $0
        }
        collecting && index($0, ";") {
            gsub(/[[:space:]]+/, " ", statement)
            if (contains_forbidden_outer_root(statement)) {
                print start ":" statement
            }
            collecting = 0
            statement = ""
        }
    ' "$file")
}

scan_output_dependency() {
    local file="$1"
    local pattern='(crate|super|matten|matten_data|matten_mlprep|serde|serde_json)::'

    while IFS=: read -r line source; do
        [[ -n "$line" ]] || continue
        finding "$file" "$line" "output may depend only on std" "$source"
    done < <(grep -nE "$pattern" "$file" || true)

    while IFS=: read -r line source; do
        [[ -n "$line" ]] || continue
        if [[ ! "$source" =~ ^[[:space:]]*use[[:space:]]+std:: ]]; then
            finding "$file" "$line" "output contains a non-std use" "$source"
        fi
    done < <(grep -nE '^[[:space:]]*use[[:space:]]+' "$file" || true)
}

scan_public_items() {
    local file="$1"
    local pattern='^[[:space:]]*pub([^[:alnum:]_]|$)'

    while IFS=: read -r line source; do
        [[ -n "$line" ]] || continue
        if [[ "$source" =~ ^[[:space:]]*pub[[:space:]]*\([[:space:]]*crate[[:space:]]*\) ]]; then
            continue
        fi
        finding "$file" "$line" "externally public item is forbidden; use pub(crate) when needed" "$source"
    done < <(grep -nE "$pattern" "$file" || true)
}

check_tree() {
    local root="$1"
    local src="$root/src"
    local manifest="$root/Cargo.toml"
    local file relative owner dependency line_count
    FAILED=0

    [[ -d "$src" ]] || fail "source directory is missing: $src"
    [[ -f "$manifest" ]] || fail "manifest is missing: $manifest"

    if [[ -e "$src/lib.rs" ]]; then
        finding "$src/lib.rs" 1 "src/lib.rs is forbidden for the private binary" "library target present"
    fi

    while IFS=: read -r line source; do
        [[ -n "$line" ]] || continue
        finding "$manifest" "$line" "Cargo [lib] target is forbidden" "$source"
    done < <(grep -nE '^[[:space:]]*\[lib\][[:space:]]*$' "$manifest" || true)

    if ! grep -Eq '^[[:space:]]*publish[[:space:]]*=[[:space:]]*false([[:space:]]*#.*)?$' "$manifest"; then
        finding "$manifest" 1 "package must declare publish = false" "missing exact private-package policy"
    fi

    while IFS= read -r file; do
        relative="${file#"$src"/}"
        line_count="$(wc -l <"$file")"
        if ((line_count > MAX_RUST_LINES)); then
            finding "$file" "$line_count" "Rust source exceeds ${MAX_RUST_LINES}-line ceiling" "line count: $line_count"
        fi
        scan_public_items "$file"
        case "$relative" in
            report.rs|report/*)
                owner=report
                for dependency in app cli render output; do
                    scan_forbidden_dependency "$file" "$owner" "$dependency"
                done
                ;;
            render.rs|render/*)
                owner=render
                for dependency in app cli output; do
                    scan_forbidden_dependency "$file" "$owner" "$dependency"
                done
                ;;
            cli.rs|cli/*)
                owner=cli
                for dependency in app report render output; do
                    scan_forbidden_dependency "$file" "$owner" "$dependency"
                done
                ;;
            output.rs|output/*)
                scan_output_dependency "$file"
                ;;
            request.rs|request/*)
                owner=request
                for dependency in app cli report render output; do
                    scan_forbidden_dependency "$file" "$owner" "$dependency"
                done
                ;;
        esac
    done < <(find "$src" -type f -name '*.rs' -print | sort)

    [[ "$FAILED" -eq 0 ]]
}

self_test() {
    local self_target="$REPO_ROOT/target/matten-report-boundaries"
    local fixture="$self_target/fixture"
    local output="$self_target/check.stderr"
    local line

    mkdir -p -- "$self_target"
    self_target="$(realpath -- "$self_target")"
    case "$self_target" in
        "$REPO_ROOT"/target/*) ;;
        *) fail "self-test target escaped the repository target tree" ;;
    esac

    cleanup() {
        case "$self_target" in
            "$REPO_ROOT"/target/matten-report-boundaries)
                rm -rf -- "$self_target"
                ;;
            *)
                printf 'module-boundaries: refusing unsafe cleanup path: %s\n' "$self_target" >&2
                ;;
        esac
    }
    trap cleanup EXIT

    reset_fixture() {
        rm -rf -- "$fixture"
        mkdir -p -- "$fixture/src"
        printf '[package]\nname = "boundary-fixture"\npublish = false\n' >"$fixture/Cargo.toml"
        printf 'fn main() {}\n' >"$fixture/src/main.rs"
    }

    expect_failure() {
        local name="$1"
        local expected="$2"
        if check_tree "$fixture" 2>"$output"; then
            fail "self-test $name unexpectedly passed"
        fi
        grep -q "$expected" "$output" || fail "self-test $name lacked a direct diagnostic"
        printf 'module-boundaries: self-test %s: expected failure observed\n' "$name"
    }

    reset_fixture
    printf 'use crate::render;\n' >"$fixture/src/report.rs"
    expect_failure direct-import 'report -> render'

    reset_fixture
    printf 'use crate::{cli, render};\n' >"$fixture/src/report.rs"
    expect_failure grouped-import 'report -> render'

    reset_fixture
    printf 'use crate::{render, request};\n' >"$fixture/src/report.rs"
    expect_failure grouped-first 'report -> render'

    reset_fixture
    printf 'use crate::{render};\n' >"$fixture/src/report.rs"
    expect_failure grouped-single 'report -> render'

    reset_fixture
    printf 'use crate::{\n    render,\n    request,\n};\n' >"$fixture/src/report.rs"
    expect_failure grouped-multiline 'report -> render'

    reset_fixture
    printf 'use crate::{request::{Config}, render};\n' >"$fixture/src/report.rs"
    expect_failure grouped-nested 'report -> render'

    reset_fixture
    printf 'use crate::{\n    request::{Config},\n    render,\n};\n' >"$fixture/src/report.rs"
    expect_failure grouped-nested-multiline 'report -> render'

    reset_fixture
    printf 'use crate::{request::{render}, self};\n' >"$fixture/src/report.rs"
    check_tree "$fixture" 2>"$output" || fail "self-test nested allowed-module control unexpectedly failed"
    printf 'module-boundaries: self-test grouped-nested-control: PASS\n'

    reset_fixture
    printf 'fn build() { crate::output::write(); }\n' >"$fixture/src/report.rs"
    expect_failure qualified-path 'report -> output'

    reset_fixture
    printf 'pub struct Leaked;\n' >"$fixture/src/request.rs"
    expect_failure public-item 'externally public item is forbidden'

    reset_fixture
    printf 'pub unsafe fn leaked() {}\n' >"$fixture/src/request.rs"
    expect_failure public-unsafe-function 'externally public item is forbidden'

    reset_fixture
    printf 'pub extern "C" fn leaked() {}\n' >"$fixture/src/request.rs"
    expect_failure public-extern-function 'externally public item is forbidden'

    reset_fixture
    printf 'pub(super) struct Leaked;\n' >"$fixture/src/request.rs"
    expect_failure restricted-public-item 'externally public item is forbidden'

    reset_fixture
    printf 'pub(crate) struct Internal;\n' >"$fixture/src/request.rs"
    check_tree "$fixture" 2>"$output" || fail "self-test pub(crate) control unexpectedly failed"
    printf 'module-boundaries: self-test pub(crate)-control: PASS\n'

    reset_fixture
    : >"$fixture/src/main.rs"
    for ((line = 1; line <= MAX_RUST_LINES + 1; line++)); do
        printf '// line %s\n' "$line" >>"$fixture/src/main.rs"
    done
    expect_failure rust-file-size "exceeds ${MAX_RUST_LINES}-line ceiling"

    reset_fixture
    : >"$fixture/src/main.rs"
    for ((line = 1; line <= MAX_RUST_LINES; line++)); do
        printf '// line %s\n' "$line" >>"$fixture/src/main.rs"
    done
    check_tree "$fixture" 2>"$output" || fail "self-test Rust file size control unexpectedly failed"
    printf 'module-boundaries: self-test rust-file-size-control: PASS\n'
    printf 'module-boundaries: self-test PASS\n'
    cleanup
    trap - EXIT
}

case "${1-}" in
    "")
        check_tree "$REPORT_ROOT" || exit 1
        printf 'module-boundaries: PASS\n'
        ;;
    --self-test)
        [[ $# -eq 1 ]] || fail "--self-test accepts no additional arguments"
        self_test
        ;;
    *)
        fail "unknown argument: $1"
        ;;
esac
