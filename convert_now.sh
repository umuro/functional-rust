#!/bin/bash
set -e

EXAMPLES=(
    "1214-binary-tree-with-pattern-matching"
    "1221-currying-and-partial-application"
    "1222-module-signature-stack"
    "1223-pattern-matching-variants"
    "804-tarjan-scc"
)

for example in "${EXAMPLES[@]}"; do
    echo "=== Converting $example ==="
    
    # Read OCaml source
    OCAML_SRC=$(cat "/home/node/hightechmind2024/functional-rust/examples/$example/example.ml")
    
    # Create prompt file
    PROMPT_FILE="/tmp/prompt_$example.txt"
    cat > "$PROMPT_FILE" <<EOF
Convert this OCaml example to idiomatic Rust.

Directory: examples/$example/

## OCaml source
$OCAML_SRC

## Topic
$(echo "$example" | sed 's/-/ /g')

## Requirements
- Create 'example.rs' with a \`main\` function to demonstrate usage.
- Create 'src/lib.rs' for the library implementation and modules.
- Implement comprehensive tests within 'src/lib.rs' using \`#[cfg(test)]\`.
- Use idiomatic Rust, including Option/Result for error handling.
- Avoid external crates; use only the standard library.
- Add documentation comments (\`///\`) to public items.

When done, provide the content of 'example.rs' and 'src/lib.rs' in separate markdown code blocks. Do not add any other text or commentary. Format your output as follows:

\`\`\`rust
// example.rs content
fn main() { ... }
\`\`\`

\`\`\`rust
// src/lib.rs content
pub mod lib_module; // Or similar
\`\`\`
EOF
    
    # Run Claude Code on host
    echo "Running Claude Code for $example..."
    ssh home-eu "cd ~/workspaces/hightechmind2024/functional-rust/examples/$example && cat /tmp/prompt_$example.txt | claude -p --dangerously-skip-permissions" > /tmp/claude_output_$example.txt 2>&1
    
    # Parse output and write files
    python3 <<EOF
import re
import os
import sys

with open('/tmp/claude_output_$example.txt', 'r') as f:
    output = f.read()

# Look for Rust code blocks
blocks = re.findall(r'```rust\\n(.*?)```', output, re.DOTALL)
if len(blocks) >= 2:
    example_rs = blocks[0].strip()
    lib_rs = blocks[1].strip()
    
    # Create directory if needed
    os.makedirs("/home/node/hightechmind2024/functional-rust/examples/$example/src", exist_ok=True)
    
    # Write files
    with open("/home/node/hightechmind2024/functional-rust/examples/$example/example.rs", "w") as f:
        f.write(example_rs)
    with open("/home/node/hightechmind2024/functional-rust/examples/$example/src/lib.rs", "w") as f:
        f.write(lib_rs)
    
    # Create minimal Cargo.toml if not exists
    cargo_path = "/home/node/hightechmind2024/functional-rust/examples/$example/Cargo.toml"
    if not os.path.exists(cargo_path):
        with open(cargo_path, "w") as f:
            f.write("""[package]
name = "example-${example//-/_}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
""")
    
    print("✓ Files written for $example")
else:
    print("✗ Failed to parse Claude output for $example")
    sys.exit(1)
EOF
    
    # Run cargo checks
    echo "Running cargo checks for $example..."
    ssh home-eu "cd ~/workspaces/hightechmind2024/functional-rust/examples/$example && cargo fmt && cargo clippy -- -Dwarnings && cargo test" 2>&1 | tee /tmp/cargo_$example.txt
    
    if grep -q "error:" /tmp/cargo_$example.txt; then
        echo "✗ Cargo checks failed for $example"
        # Retry logic could go here
    else
        echo "✓ $example — cargo fmt ✓ clippy ✓ test ✓"
    fi
    
    echo ""
done