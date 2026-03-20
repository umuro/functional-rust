#!/bin/bash
set -e
cd /home/umur/workspaces/hightechmind2024/functional-rust

for dir in 1181-listpartition-divide-list-by-predicate \
           1182-listflatten-flatten-nested-lists \
           1183-arrayblit-copy-subarray \
           1184-arraymake-and-arraymake_matrix-multi-dimensional-arrays \
           1185-stringsplit_on_char-tokenize-a-string; do
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
    echo "=== Finished $dir ==="
done