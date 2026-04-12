#!/usr/bin/env python3
import subprocess
import sys
import os

example_dir = sys.argv[1]
example_path = f"examples/{example_dir}"
ocaml_source = open(f"{example_path}/example.ml").read()

prompt = f"""Convert this OCaml example to idiomatic Rust.

Directory: {example_path}

## OCaml source
{ocaml_source}

## Topic
{example_dir}

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — {example_dir} — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]"""

print(f"Prompt length: {len(prompt)}")
print("Running claude via ssh...")
cmd = ['ssh', 'home-eu', 'cd /home/umur/workspaces/hightechmind2024/functional-rust && claude --dangerously-skip-permissions -p "$(cat)"']
proc = subprocess.run(cmd, input=prompt.encode(), capture_output=True, timeout=600)
print("STDOUT:", proc.stdout.decode())
print("STDERR:", proc.stderr.decode())
print("Return code:", proc.returncode)
if 'DONE —' in proc.stdout.decode() and 'cargo fmt ✓ clippy ✓ test ✓' in proc.stdout.decode():
    print("SUCCESS")
    sys.exit(0)
else:
    print("FAILURE")
    sys.exit(1)