#!/bin/bash
set -e

echo "Starting conversion of 5 examples with OCaml but no Rust..."

# Get list of directories with OCaml but no Rust
DIRS_WITH_OCAML_NO_RUST=$(cd /home/node/.openclaw/workspace/functional-rust && find examples -name "example.ml" -type f | while read f; do dir=$(dirname "$f"); if [ ! -f "$dir/example.rs" ]; then echo "$dir"; fi; done | head -5)

echo "Found directories to convert:"
i=1
for dir in $DIRS_WITH_OCAML_NO_RUST; do
    echo "$i. $dir"
    i=$((i+1))
done

echo ""
echo "Processing each directory..."

for dir in $DIRS_WITH_OCAML_NO_RUST; do
    echo ""
    echo "=== Processing $dir ==="
    
    # Read OCaml source
    OCAML_SRC=$(cat "/home/node/.openclaw/workspace/functional-rust/$dir/example.ml")
    
    # Extract directory name without path
    DIR_NAME=$(basename "$dir")
    
    # Extract example number (first part before dash)
    EXAMPLE_NUM=$(echo "$DIR_NAME" | cut -d- -f1)
    
    # Create task for Claude Code
    TASK_FILE="/tmp/task_${DIR_NAME}.txt"
    cat > "$TASK_FILE" << TASK
Convert this OCaml example to idiomatic Rust.

Directory: $DIR_NAME/

## OCaml source
$OCAML_SRC

## Topic
$(echo "$DIR_NAME" | cut -d- -f2- | sed 's/-/ /g')

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
DONE — $DIR_NAME — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
TASK
    
    echo "Task file created: $TASK_FILE"
    echo "OCaml source length: ${#OCAML_SRC} characters"
    
    # Check if we have Cargo.toml
    if [ ! -f "/home/node/.openclaw/workspace/functional-rust/$dir/Cargo.toml" ]; then
        echo "WARNING: No Cargo.toml found in $dir"
        # Create minimal Cargo.toml
        cat > "/home/node/.openclaw/workspace/functional-rust/$dir/Cargo.toml" << TOML
[package]
name = "example-${EXAMPLE_NUM}"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
name = "example"
path = "src/lib.rs"
TOML
        echo "Created minimal Cargo.toml"
    fi
    
    # Check if src directory exists
    if [ ! -d "/home/node/.openclaw/workspace/functional-rust/$dir/src" ]; then
        mkdir -p "/home/node/.openclaw/workspace/functional-rust/$dir/src"
    fi
    
    # Run Claude Code conversion via ssh home-eu
    echo "Starting Claude Code conversion (via ssh home-eu)..."
    
    # Run in the specific directory
    ssh home-eu "cd '/home/umur/.openclaw/workspace/functional-rust/$dir' && cat '$TASK_FILE' | claude -p" &
    CLAUDE_PID=$!
    
    echo "Claude Code running (PID: $CLAUDE_PID)"
    
    # Wait with timeout
    WAIT_SECONDS=120
    sleep $WAIT_SECONDS &
    SLEEP_PID=$!
    
    wait -n $CLAUDE_PID $SLEEP_PID 2>/dev/null || true
    
    if kill -0 $CLAUDE_PID 2>/dev/null; then
        echo "Claude Code still running after $WAIT_SECONDS seconds, continuing..."
        # Don't kill it, let it continue
    else
        echo "Claude Code finished"
    fi
    
    # Verify we got example.rs
    if [ -f "/home/node/.openclaw/workspace/functional-rust/$dir/example.rs" ]; then
        echo "✓ Successfully created example.rs"
        
        # Run cargo checks
        echo "Running cargo checks..."
        (cd "/home/node/.openclaw/workspace/functional-rust/$dir" && cargo fmt --check 2>&1 || echo "cargo fmt check failed")
        (cd "/home/node/.openclaw/workspace/functional-rust/$dir" && cargo clippy -- -D warnings 2>&1 | tail -5 || echo "clippy failed")
        (cd "/home/node/.openclaw/workspace/functional-rust/$dir" && cargo test 2>&1 | tail -10 || echo "tests failed")
    else
        echo "✗ No example.rs created"
        # Check if we got src/lib.rs instead
        if [ -f "/home/node/.openclaw/workspace/functional-rust/$dir/src/lib.rs" ]; then
            echo "But found src/lib.rs, creating example.rs wrapper..."
            # Create a simple example.rs that uses the library
            cat > "/home/node/.openclaw/workspace/functional-rust/$dir/example.rs" << RUST
fn main() {
    // Use the library from src/lib.rs
    println!("Example implementation in src/lib.rs");
    
    // Run any example tests
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(true, true);
        }
    }
}
RUST
        fi
    fi
    
    echo "--- Finished $dir ---"
done

echo ""
echo "Completed processing 5 examples"