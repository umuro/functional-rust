#!/bin/bash
# Convert a batch of OCaml examples to Rust using Claude Code (docker).
# Usage:
#   bash run_conversions.sh                     # convert the hardcoded list below
#   bash run_conversions.sh dir1 dir2 ...       # convert specific dirs (bare names, no examples/ prefix)
# After each conversion, scripts/check-one-example.sh validates all quality gates.
set -e
cd /home/umur/workspaces/hightechmind2024/functional-rust

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Allow passing dirs as args; fall back to hardcoded batch
if [[ $# -gt 0 ]]; then
    DIRS=("$@")
else
    DIRS=(
        1181-listpartition-divide-list-by-predicate
        1182-listflatten-flatten-nested-lists
        1183-arrayblit-copy-subarray
        1184-arraymake-and-arraymake_matrix-multi-dimensional-arrays
        1185-stringsplit_on_char-tokenize-a-string
    )
fi

PASSED=()
FAILED=()

for dir in "${DIRS[@]}"; do
    echo ""
    echo "=== Converting $dir ==="
    if [ ! -d "examples/$dir" ]; then
        echo "Directory examples/$dir not found, skipping."
        continue
    fi
    # Extract topic from dir name (remove number and dashes)
    topic=$(echo "$dir" | sed 's/^[0-9]*-//' | sed 's/-/ /g')
    # Read OCaml source
    ocaml_source=$(cat "examples/$dir/example.ml")
    # Build prompt
    prompt="Convert this OCaml example to idiomatic Rust.

Directory: examples/$dir/

## OCaml source
$ocaml_source

## Topic
$topic

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — $dir — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]"
    echo "Running Claude Code..."
    docker run --rm \
      -v /home/umur/workspaces/hightechmind2024:/work/hightechmind2024 \
      -v /home/umur/.claude:/home/dev/.claude \
      -w /work/hightechmind2024/functional-rust \
      dev-tools:latest \
      claude -p "$prompt"
    echo ""
    echo "--- Post-conversion quality check ---"
    if bash "$SCRIPT_DIR/scripts/check-one-example.sh" "examples/$dir"; then
        PASSED+=("$dir")
        echo "=== $dir: PASS ==="
    else
        FAILED+=("$dir")
        echo "=== $dir: FAIL (see issues above) ==="
        echo "Fix the issues and re-run this script for just this example:"
        echo "  bash run_conversions.sh $dir"
    fi
done

echo ""
echo "=============================="
echo "Batch summary"
echo "=============================="
echo "Passed: ${#PASSED[@]}"
for d in "${PASSED[@]}"; do echo "  ✓ $d"; done
echo "Failed: ${#FAILED[@]}"
for d in "${FAILED[@]}"; do echo "  ✗ $d"; done

if (( ${#FAILED[@]} > 0 )); then
    exit 1
fi