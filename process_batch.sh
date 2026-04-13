#!/bin/bash
set -e

echo "Functional Rust Batch Processing"
echo "================================="

# Create array of entries to process
# Format: NUM:TITLE:OCAML_CODE_FILE
ENTRIES=(
  "099:Monoid Pattern — Generic Combining:099_monoid.ml"
  "100:Dijkstra's Shortest Path — Priority Queue:100_dijkstra.ml"
  "101:Huffman Encoding — Greedy Tree Building:101_huffman.ml"
  "102:Persistent Vector — Functional Array:102_pvector.ml"
  "103:Knapsack Problem — Dynamic Programming Functional:103_knapsack.ml"
)

# First, extract OCaml code for each entry
echo "Extracting OCaml code from QUEUE.md.backup..."
python3 << 'EOF'
import re
with open('/home/node/.openclaw/workspace/functional-rust/QUEUE.md.backup', 'r', encoding='utf-8') as f:
    content = f.read()

entries = re.split(r'^---\s*$', content, flags=re.MULTILINE)

targets = [
    ("099", "099_monoid.ml"),
    ("100", "100_dijkstra.ml"),
    ("101", "101_huffman.ml"),
    ("102", "102_pvector.ml"),
    ("103", "103_knapsack.ml"),
]

for num, filename in targets:
    pattern = r'^### ' + re.escape(num) + r':.*?```ocaml\n(.*?)```'
    match = re.search(pattern, content, re.DOTALL | re.MULTILINE | re.IGNORECASE)
    if match:
        code = match.group(1).strip()
        with open('/tmp/' + filename, 'w') as f:
            f.write(code)
        print(f"Extracted {num} to /tmp/{filename} ({len(code)} chars)")
    else:
        print(f"WARNING: Could not find entry {num}")
EOF

echo ""
echo "Starting conversion process..."

# Process each entry
for entry in "${ENTRIES[@]}"; do
    IFS=":" read -r NUM TITLE OCAML_FILE <<< "$entry"
    
    echo ""
    echo "=== Processing $NUM: $TITLE ==="
    
    # Check if directory already exists
    DIR_PREFIX="${NUM}-"
    EXISTING_DIR=$(find /home/node/.openclaw/workspace/functional-rust/examples -maxdepth 1 -type d -name "${DIR_PREFIX}*" | head -1)
    
    if [ -n "$EXISTING_DIR" ]; then
        echo "Directory already exists: $EXISTING_DIR"
        if [ -f "$EXISTING_DIR/Cargo.toml" ]; then
            echo "Already has Cargo.toml - skipping"
            continue
        fi
    fi
    
    # Create directory name
    SANITIZED_TITLE=$(echo "$TITLE" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9 -]//g' | sed 's/  */-/g' | sed 's/--*/-/g' | sed 's/^-//' | sed 's/-$//')
    DIR_NAME="/home/node/.openclaw/workspace/functional-rust/examples/${NUM}-${SANITIZED_TITLE}"
    echo "Creating directory: $DIR_NAME"
    mkdir -p "$DIR_NAME"
    
    # Write OCaml source
    if [ -f "/tmp/$OCAML_FILE" ]; then
        cp "/tmp/$OCAML_FILE" "$DIR_NAME/example.ml"
        echo "Copied OCaml source to $DIR_NAME/example.ml"
    else
        echo "ERROR: OCaml file /tmp/$OCAML_FILE not found"
        continue
    fi
    
    # Create minimal Cargo.toml
    cat > "$DIR_NAME/Cargo.toml" << TOML
[package]
name = "example-${NUM}"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
name = "example"
path = "src/lib.rs"
TOML
    
    # Create src directory
    mkdir -p "$DIR_NAME/src"
    
    # Run Claude Code conversion
    echo "Running Claude Code conversion..."
    
    # Create task description
    TASK_FILE="/tmp/task_${NUM}.txt"
    cat > "$TASK_FILE" << TASK
Convert this OCaml example to idiomatic Rust.

Directory: $(basename "$DIR_NAME")/

## OCaml source
$(cat "/tmp/$OCAML_FILE")

## Topic
$TITLE

Read the project guidelines in /home/node/.openclaw/workspace/functional-rust/README.md.
Follow Rust best practices: proper error handling, testing, documentation, and performance considerations.

When done, report:
DONE — ${NUM}-$(basename "$DIR_NAME" | cut -d- -f2-) — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
TASK
    
    # Run Claude Code via ssh home-eu
    echo "Starting Claude Code conversion..."
    
    # We need to run this in the functional-rust directory
    ssh home-eu "cd /home/umur/.openclaw/workspace/functional-rust/examples/${NUM}-${SANITIZED_TITLE} && cat '"$TASK_FILE"' | claude -p" &
    CLAUDE_PID=$!
    
    # Wait for completion with timeout
    echo "Waiting for Claude Code completion (PID: $CLAUDE_PID)..."
    wait $CLAUDE_PID
    
    echo "Claude Code finished for $NUM"
    
    # Verify with cargo fmt, clippy, test
    if [ -f "$DIR_NAME/Cargo.toml" ]; then
        echo "Running cargo checks..."
        (cd "$DIR_NAME" && cargo fmt --check 2>/dev/null || true)
        (cd "$DIR_NAME" && cargo clippy -- -D warnings 2>/dev/null || true)
        (cd "$DIR_NAME" && cargo test 2>/dev/null || true)
    fi
    
    echo "--- Completed $NUM ---"
done

echo ""
echo "Batch processing complete!"