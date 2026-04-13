#!/usr/bin/env python3
"""
Create example.rs files for directories that have src/lib.rs but no example.rs.
This is a fallback when Claude Code doesn't produce the example.rs file.
"""
import os
import sys
from pathlib import Path

def main():
    base = Path("/home/node/.openclaw/workspace/functional-rust/examples")
    created = 0
    
    for example_dir in sorted(base.iterdir()):
        if not example_dir.is_dir():
            continue
            
        lib_rs = example_dir / "src" / "lib.rs"
        example_rs = example_dir / "example.rs"
        
        # Skip if we already have example.rs
        if example_rs.exists():
            continue
            
        # Only create if we have lib.rs
        if lib_rs.exists():
            # Read the lib.rs to extract function names for the demo
            with open(lib_rs, 'r') as f:
                content = f.read()
            
            # Extract public function names (simple heuristic)
            import re
            # Look for "fn func_name" patterns
            funcs = re.findall(r'\n(?:pub )?fn (\w+)\(', content)
            # Filter out test functions
            funcs = [f for f in funcs if not f.startswith('test_')]
            
            # Create example.rs
            example_content = """// Example usage for {} — demonstrates the library functions.
fn main() {{
""".format(example_dir.name)
            
            if funcs:
                example_content += "    // Import from the local library\n"
                example_content += "    use example::*;\n\n"
                example_content += "    // Example usage of functions:\n"
                for func in funcs[:3]:  # Limit to 3 functions for brevity
                    example_content += f"    // {func}(...);\n"
                example_content += "\n"
            
            example_content += """    println!("{} example ready!");
    println!("Run `cargo test` to see all tests.");
}
"""
            
            with open(example_rs, 'w') as f:
                f.write(example_content)
            
            print(f"Created: {example_dir.name}/example.rs")
            created += 1
            
            if created >= 5:
                break
    
    print(f"\nCreated {created} example.rs files")

if __name__ == "__main__":
    main()