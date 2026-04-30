#!/usr/bin/env python3
import subprocess
import re
import os
import tempfile
import sys

def convert_example(example_name):
    print(f"=== Converting {example_name} ===")
    
    # Read OCaml source
    ocaml_path = f"/home/node/hightechmind2024/functional-rust/examples/{example_name}/example.ml"
    if not os.path.exists(ocaml_path):
        print(f"Error: {ocaml_path} not found")
        return False
    
    with open(ocaml_path, 'r') as f:
        ocaml_src = f.read()
    
    prompt = f"""Convert this OCaml example to idiomatic Rust.

Directory: examples/{example_name}/

## OCaml source
{ocaml_src}

## Topic
{example_name.replace('-', ' ')}

## Requirements
- Create 'example.rs' with a `main` function to demonstrate usage.
- Create 'src/lib.rs' for the library implementation and modules.
- Implement comprehensive tests within 'src/lib.rs' using `#[cfg(test)]`.
- Use idiomatic Rust, including Option/Result for error handling.
- Avoid external crates; use only the standard library.
- Add documentation comments (`///`) to public items.

When done, provide the content of 'example.rs' and 'src/lib.rs' in separate markdown code blocks. Do not add any other text or commentary. Format your output as follows:

```rust
// example.rs content
fn main() {{ ... }}
```

```rust
// src/lib.rs content
pub mod lib_module; // Or similar
```
"""
    
    # Write prompt to temp file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
        f.write(prompt)
        prompt_path = f.name
    
    try:
        # Run Claude Code on host
        print("Running Claude Code...")
        claude_cmd = f"cat {prompt_path} | claude -p --dangerously-skip-permissions"
        result = subprocess.run(
            ['ssh', 'home-eu', claude_cmd],
            capture_output=True,
            text=True,
            timeout=300
        )
        
        if result.returncode != 0:
            print(f"Claude failed: {result.stderr}")
            return False
        
        output = result.stdout
        print(f"Claude output length: {len(output)} chars")
        
        # Parse Rust code blocks
        blocks = re.findall(r'```rust\n(.*?)\n```', output, re.DOTALL)
        if len(blocks) < 2:
            print(f"Could not find 2 Rust code blocks, found {len(blocks)}")
            # Try alternative pattern
            blocks = re.findall(r'```rust\s*\n(.*?)\n```', output, re.DOTALL)
            if len(blocks) < 2:
                print("Alternative pattern also failed")
                print("Output preview:", output[:1000])
                return False
        
        example_rs = blocks[0].strip()
        lib_rs = blocks[1].strip()
        
        # Create directory structure
        example_dir = f"/home/node/hightechmind2024/functional-rust/examples/{example_name}"
        src_dir = os.path.join(example_dir, "src")
        os.makedirs(src_dir, exist_ok=True)
        
        # Write files
        with open(os.path.join(example_dir, "example.rs"), "w") as f:
            f.write(example_rs)
        with open(os.path.join(src_dir, "lib.rs"), "w") as f:
            f.write(lib_rs)
        
        # Create minimal Cargo.toml if not exists
        cargo_path = os.path.join(example_dir, "Cargo.toml")
        if not os.path.exists(cargo_path):
            safe_name = example_name.replace('-', '_')
            with open(cargo_path, "w") as f:
                f.write(f"""[package]
name = "example_{safe_name}"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
""")
        
        print("✓ Files written")
        
        # Run cargo checks
        print("Running cargo checks...")
        cargo_cmd = f"cd ~/workspaces/hightechmind2024/functional-rust/examples/{example_name} && cargo fmt && cargo clippy -- -Dwarnings && cargo test"
        cargo = subprocess.run(
            ['ssh', 'home-eu', cargo_cmd],
            capture_output=True,
            text=True,
            timeout=180
        )
        
        if cargo.returncode == 0:
            print(f"✓ {example_name} — cargo fmt ✓ clippy ✓ test ✓")
            return True
        else:
            print(f"✗ Cargo checks failed")
            print("Stderr:", cargo.stderr[:500])
            return False
            
    except subprocess.TimeoutExpired:
        print("Timeout expired")
        return False
    finally:
        os.unlink(prompt_path)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 convert_one.py <example-name>")
        sys.exit(1)
    
    success = convert_example(sys.argv[1])
    sys.exit(0 if success else 1)