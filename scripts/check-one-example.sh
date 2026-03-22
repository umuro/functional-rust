#!/usr/bin/env bash
# Check a single example directory meets all quality gates.
# Usage: bash scripts/check-one-example.sh examples/NNNN-name
# Exit 0 = all checks passed. Exit 1 = one or more checks failed.
set -euo pipefail

dir="${1:?Usage: $0 examples/NNNN-name}"
name=$(basename "$dir")
pkg="example-${name}"
PASS=0
FAIL=0
ISSUES=()

ok()   { echo "  ✓ $1"; ((PASS++)) || true; }
fail() { echo "  ✗ $1"; ISSUES+=("$1"); ((FAIL++)) || true; }

echo "=== Checking $name ==="

# --- File presence ---
if [[ -f "$dir/README.md" ]]; then
    readme_size=$(wc -c < "$dir/README.md")
    if (( readme_size >= 300 )); then
        ok "README.md (${readme_size}B)"
    else
        fail "README.md too thin (${readme_size}B, need ≥300B)"
    fi
else
    fail "README.md missing"
fi

if [[ -f "$dir/COMPARISON.md" ]]; then
    cmp_size=$(wc -c < "$dir/COMPARISON.md")
    if (( cmp_size >= 500 )); then
        ok "COMPARISON.md (${cmp_size}B)"
    else
        fail "COMPARISON.md too thin (${cmp_size}B, need ≥500B)"
    fi
else
    fail "COMPARISON.md missing"
fi

if [[ -f "$dir/example.ml" ]]; then
    if grep -q 'print_endline' "$dir/example.ml"; then
        ok "example.ml (has print_endline)"
    else
        fail "example.ml missing print_endline \"ok\" at end"
    fi
else
    fail "example.ml missing"
fi

if [[ -f "$dir/src/lib.rs" ]]; then
    ok "src/lib.rs"
else
    fail "src/lib.rs missing"
fi

if [[ -f "$dir/example.rs" ]]; then
    ok "example.rs"
else
    fail "example.rs missing"
fi

# --- Cargo.toml ---
if [[ ! -f "$dir/Cargo.toml" ]]; then
    fail "Cargo.toml missing — cannot run cargo checks"
    echo ""
    echo "Result: $PASS passed, $FAIL FAILED"
    exit 1
fi

# --- Clippy ---
if cargo clippy -p "$pkg" -- -D warnings -q 2>/dev/null; then
    ok "cargo clippy -D warnings"
else
    fail "cargo clippy -D warnings"
fi

# --- Tests ---
test_output=$(cargo test -p "$pkg" -q 2>&1 || true)
if echo "$test_output" | grep -q "test result: ok"; then
    test_count=$(echo "$test_output" | grep -oP '\d+ passed' | head -1)
    ok "cargo test ($test_count)"
elif echo "$test_output" | grep -q "^test result:"; then
    fail "cargo test — $(echo "$test_output" | grep '^test result:' | head -1)"
else
    fail "cargo test — could not determine result"
fi

echo ""
if (( FAIL == 0 )); then
    echo "  PASS — $PASS checks passed"
    exit 0
else
    echo "  FAIL — $FAIL issue(s):"
    for issue in "${ISSUES[@]}"; do
        echo "    - $issue"
    done
    exit 1
fi
