#!/bin/bash
set -e

echo "Functional Rust Batch Conversion"
echo "================================="

# List of examples to process (first 5 directories with example.ml but no example.rs)
EXAMPLES=(
    "001-higher-order-functions"
    "002-list-operations" 
    "003-pattern-matching"
    "004-option-result"
    "005-reverse-list"
)

for dir in "${EXAMPLES[@]}"; do
    echo ""
    echo "=== Processing $dir ==="
    
    # Check files exist
    if [ ! -f "examples/${dir}/example.ml" ]; then
        echo "ERROR: example.ml not found in $dir"
        continue
    fi
    
    # Read OCaml source
    OCAML_SRC=$(cat "examples/${dir}/example.ml")
    
    # Create task file
    TASK_FILE="/tmp/task_${dir}.txt"
    TOPIC=$(echo "$dir" | cut -d- -f2- | sed 's/-/ /g')
    
    cat > "$TASK_FILE" << TASK
Convert this OCaml example to idiomatic Rust.

Directory: $dir/

## OCaml source
$OCAML_SRC

## Topic
$TOPIC

Follow Rust best practices:
- Use proper error handling (Option/Result instead of sentinel values)
- Write comprehensive tests
- Add documentation comments
- Consider performance implications
- Use appropriate data structures

Generate:
1. A src/lib.rs file with the implementation and tests
2. An example.rs file with a main() function demonstrating usage
3. Ensure it passes cargo fmt, clippy, and test

When done, report:
DONE — $dir — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
TASK
    
    echo "Task file created for $dir"
    
    # Check Cargo.toml exists
    if [ ! -f "examples/${dir}/Cargo.toml" ]; then
        echo "WARNING: No Cargo.toml found, creating minimal one..."
        cat > "examples/${dir}/Cargo.toml" << TOML
[package]
name = "example-$(echo "$dir" | cut -d- -f1)"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
name = "example"
path = "src/lib.rs"
TOML
    fi
    
    # Ensure src directory exists
    mkdir -p "examples/${dir}/src"
    
    # Run Claude Code via ssh home-eu
    echo "Running Claude Code conversion (via ssh home-eu)..."
    
    # Create a temporary script to run on host
    RUN_SCRIPT="/tmp/run_claude_${dir}.sh"
    cat > "$RUN_SCRIPT" << 'RUN'
#!/bin/bash
set -e
WORK_DIR="/home/umur/.openclaw/workspace/functional-rust/examples/$1"
cd "$WORK_DIR"
cat "$2" | claude -p
RUN
    
    chmod +x "$RUN_SCRIPT"
    
    # Run with timeout
    TIMEOUT=45
    echo "Starting conversion (timeout: ${TIMEOUT}s)..."
    
    timeout $TIMEOUT ssh home-eu "cd '/home/umur/.openclaw/workspace/functional-rust' && cd 'examples/$dir' && cat '$TASK_FILE' | claude -p" 2>&1 | tee "/tmp/claude_${dir}.log" || {
        echo "Claude Code timed out or failed for $dir"
        # Check if any output was generated
        if [ -f "examples/${dir}/example.rs" ]; then
            echo "But example.rs was created before timeout"
        fi
    }
    
    # Verify results
    if [ -f "examples/${dir}/example.rs" ]; then
        echo "✓ Successfully created example.rs for $dir"
        # Run cargo checks
        echo "Running cargo checks..."
        (cd "examples/${dir}" && cargo fmt --check 2>&1 | head -5 || echo "fmt check failed")
        (cd "examples/${dir}" && cargo clippy -- -D warnings 2>&1 | tail -5 || echo "clippy failed")
        (cd "examples/${dir}" && cargo test 2>&1 | tail -5 || echo "tests failed")
    else
        echo "✗ Failed to create example.rs for $dir"
        # Check for any .rs files
        find "examples/${dir}" -name "*.rs" -type f 2>/dev/null | while read f; do
            echo "  Found: $f"
        done
    fi
    
    echo "--- Completed $dir ---"
done

echo ""
echo "Batch processing complete!"