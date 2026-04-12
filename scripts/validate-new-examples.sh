#!/usr/bin/env bash
# Validate new/changed examples pass cargo clippy -D warnings and cargo test.
# Usage: bash scripts/validate-new-examples.sh [dir1 dir2 ...]
# If no args, validates all examples that have src/lib.rs.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

FAILED=()
PASSED=0

if [[ $# -gt 0 ]]; then
    DIRS=("$@")
else
    # Auto-detect: all examples with src/lib.rs
    mapfile -t DIRS < <(find examples -maxdepth 2 -name "lib.rs" | sed 's|/src/lib.rs||' | sort)
fi

for dir in "${DIRS[@]}"; do
    name=$(basename "$dir")
    pkg="example-${name}"

    # Check Cargo.toml exists
    if [[ ! -f "$dir/Cargo.toml" ]]; then
        echo "  SKIP $name (no Cargo.toml)"
        continue
    fi

    echo -n "  $name ... "

    # clippy
    if ! cargo clippy -p "$pkg" -- -D warnings -q 2>/dev/null; then
        echo "FAIL (clippy)"
        FAILED+=("$name")
        continue
    fi

    # test
    if ! cargo test -p "$pkg" -q 2>/dev/null; then
        echo "FAIL (test)"
        FAILED+=("$name")
        continue
    fi

    echo "ok"
    ((PASSED++)) || true
done

echo ""
echo "Passed: $PASSED  Failed: ${#FAILED[@]}"

if [[ ${#FAILED[@]} -gt 0 ]]; then
    echo ""
    echo "Fix these before committing:"
    for f in "${FAILED[@]}"; do
        echo "  $f"
    done
    exit 1
fi
