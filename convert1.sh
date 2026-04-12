#!/bin/bash
set -e
cd /hostroot/home/umur/workspaces/hightechmind2024/functional-rust
dir="1181-listpartition-divide-list-by-predicate"
topic="list partition divide list by predicate"
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
echo "Running Claude Code for $dir..."
docker run --rm \
  -v /home/umur/workspaces/hightechmind2024:/work/hightechmind2024 \
  -v /home/umur/.claude:/home/dev/.claude \
  -w /work/hightechmind2024/functional-rust \
  dev-tools:latest \
  claude -p "$prompt"