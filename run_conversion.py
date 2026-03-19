#!/usr/bin/env python3
import subprocess
import sys
import os

example_dir = "1163-024-currying-partial-application-and-sections"
example_path = f"examples/{example_dir}"
ocaml_source = open(f"{example_path}/example.ml").read()

prompt = f"""Convert this OCaml example to idiomatic Rust.

Directory: {example_path}

## OCaml source
{ocaml_source}

## Topic
Currying, partial application, and operator sections

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — {example_dir} — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]"""

# Write prompt to a temporary file to avoid shell escaping issues
with open("/tmp/prompt.txt", "w") as f:
    f.write(prompt)

cmd = [
    "docker", "run", "--rm",
    "-v", "/home/umur/workspaces/hightechmind2024:/work/hightechmind2024",
    "-v", "/home/umur/.claude:/home/dev/.claude",
    "-w", "/work/hightechmind2024/functional-rust",
    "dev-tools:latest",
    "claude", "-p", prompt
]

print("Running docker command...")
result = subprocess.run(cmd, capture_output=True, text=True)
print("STDOUT:", result.stdout)
print("STDERR:", result.stderr)
print("Return code:", result.returncode)