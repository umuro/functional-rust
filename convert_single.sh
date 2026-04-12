#!/bin/bash
set -e
dir=$1
topic=$(echo "$dir" | sed 's/^[0-9]*-//' | sed 's/-/ /g')
ocaml_source=$(cat "examples/$dir/example.ml")
prompt="Convert this OCaml example to idiomatic Rust.

Directory: examples/$dir/

## OCaml source
$ocaml_source

## Topic
$topic

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — $dir — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]"
echo "Converting $dir..."
ssh home-eu "cd ~/workspaces/hightechmind2024/functional-rust && echo '$prompt' | claude -p"
