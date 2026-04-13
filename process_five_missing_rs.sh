#!/bin/bash
set -e

echo "Processing 5 examples missing Rust files (example.rs)..."
echo "=========================================================="

# Get first 5 directories with example.ml but no example.rs
cd /home/node/.openclaw/workspace/functional-rust
DIRS=$(python3 -c "
import os
examples = []
for d in sorted(os.listdir('examples')):
    if not os.path.isdir(f'examples/{d}'):
        continue
    ml = f'examples/{d}/example.ml'
    rs = f'examples/{d}/example.rs'
    if os.path.exists(ml) and not os.path.exists(rs):
        examples.append(d)
        if len(examples) >= 5:
            break
for d in examples:
    print(d)
")

IFS=$'\n' read -d '' -r -a DIR_ARRAY <<< "$DIRS"

echo "Directories to process:"
for i in "${!DIR_ARRAY[@]}"; do
    echo "$((i+1)). ${DIR_ARRAY[$i]}"
done

echo ""
echo "Processing..."

for dir in "${DIR_ARRAY[@]}"; do
    echo ""
    echo "=== Processing $dir ==="
    
    # Read OCaml source
    OCAML_FILE="examples/${dir}/example.ml"
    OCAML_SRC=$(cat "$OCAML_FILE")
    
    echo "Read OCaml source (${#OCAML_SRC} chars)"
    
    # Create task for Claude Code
    TASK_FILE="/tmp/task_${dir}.txt"
    cat > "$TASK_FILE" << TASK
Convert this OCaml example to idiomatic Rust.

Directory: $dir/

## OCaml source
$OCAML_SRC

## Topic
$(echo "$dir" | cut -d- -f2- | sed 's/-/ /g')

Follow Rust best practices:
- Use proper error handling (Option/Result instead of exceptions or sentinel values)
- Write comprehensive tests in src/lib.rs
- Add documentation comments for public functions
- Consider performance implications
- Use appropriate data structures

Generate:
1. A complete src/lib.rs file with the implementation and #[test] blocks
2. An example.rs file with a main() function demonstrating usage
3. Ensure it passes cargo fmt, clippy, and test

IMPORTANT: The OCaml source contains both implementation and test assertions.
Translate the OCaml tests to Rust #[test] blocks in src/lib.rs.

When done, report:
DONE — $dir — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
TASK
    
    echo "Task file created: $TASK_FILE"
    
    # Check that Cargo.toml exists
    if [ ! -f "examples/${dir}/Cargo.toml" ]; then
        echo "ERROR: No Cargo.toml in $dir"
        continue
    fi
    
    # Check src directory exists
    if [ ! -d "examples/${dir}/src" ]; then
        mkdir -p "examples/${dir}/src"
        echo "Created src directory"
    fi
    
    # Run Claude Code via ssh home-eu
    echo "Running Claude Code conversion..."
    
    # Use a simpler approach: run Claude Code non-interactively with timeout
    TIMEOUT=30
    echo "Starting conversion (timeout: ${TIMEOUT}s)..."
    
    # Create a wrapper script to run claude
    WRAPPER="/tmp/claude_wrapper_${dir}.sh"
    cat > "$WRAPPER" << WRAPPER
#!/bin/bash
cd "/home/umur/.openclaw/workspace/functional-rust/examples/${dir}"
cat "$TASK_FILE" | claude -p
WRAPPER
    chmod +x "$WRAPPER"
    
    # Run via ssh home-eu with timeout
    timeout $TIMEOUT ssh home-eu "bash -s" < "$WRAPPER" 2>&1 | tee "/tmp/claude_output_${dir}.txt" &
    CLAUDE_PID=$!
    
    # Wait for completion
    wait $CLAUDE_PID 2>/dev/null || true
    
    echo "Claude Code finished or timed out"
    
    # Check if example.rs was created
    if [ -f "examples/${dir}/example.rs" ]; then
        echo "✓ Successfully created example.rs"
        
        # Also check for src/lib.rs
        if [ -f "examples/${dir}/src/lib.rs" ]; then
            echo "✓ Found src/lib.rs"
            
            # Run cargo checks
            echo "Running cargo checks..."
            
            # Format check
            echo "-> cargo fmt --check"
            (cd "examples/${dir}" && cargo fmt --check 2>&1 | head -5) || echo "  fmt check failed"
            
            # Clippy  
            echo "-> cargo clippy -- -D warnings"
            (cd "examples/${dir}" && cargo clippy -- -D warnings 2>&1 | tail -5) || echo "  clippy failed/skipped"
            
            # Tests
            echo "-> cargo test"
            (cd "examples/${dir}" && cargo test 2>&1 | tail -10) || echo "  tests failed"
        else
            echo "✗ No src/lib.rs created"
        fi
    else
        echo "✗ No example.rs created"
        # Check for any .rs files
        find "examples/${dir}" -name "*.rs" -type f | while read f; do
            echo "  Found: $f"
        done
    fi
    
    echo "--- Completed $dir ---"
done

echo ""
echo "Batch processing complete!"