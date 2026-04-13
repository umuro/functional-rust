#!/bin/bash
set -e

DIR=$1
if [ -z "$DIR" ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

cd /home/node/.openclaw/workspace/functional-rust

if [ ! -d "examples/$DIR" ]; then
    echo "Directory examples/$DIR not found"
    exit 1
fi

OCAML_SRC=$(cat "examples/$DIR/example.ml")

# Create task file
cat > /tmp/task.txt << TASK
Convert this OCaml example to idiomatic Rust.

Directory: $DIR/

## OCaml source
$OCAML_SRC

## Topic
$(echo "$DIR" | cut -d- -f2- | sed 's/-/ /g')

Create:
1. A complete src/lib.rs file with implementation and tests
2. An example.rs file with a main() function
3. Ensure it passes cargo fmt, clippy, and test

When done, report:
DONE — $DIR — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
TASK

echo "Running Claude Code for $DIR..."
ssh home-eu "cd '/home/umur/.openclaw/workspace/functional-rust/examples/$DIR' && cat '/tmp/task.txt' | claude -p --dangerously-skip-permissions"

echo "Checking results..."
if [ -f "examples/$DIR/example.rs" ]; then
    echo "✓ example.rs created"
else
    echo "✗ No example.rs"
fi

if [ -f "examples/$DIR/src/lib.rs" ]; then
    echo "✓ src/lib.rs created"
else
    echo "✗ No src/lib.rs"
fi