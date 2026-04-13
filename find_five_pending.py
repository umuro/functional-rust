#!/usr/bin/env python3
import re
import os
import sys

# Read the queue file
with open('/home/node/.openclaw/workspace/functional-rust/QUEUE.md.backup', 'r', encoding='utf-8') as f:
    content = f.read()

# Split into entries
entries = re.split(r'^---\s*$', content, flags=re.MULTILINE)

pending_entries = []
existing_dirs = set(os.listdir('/home/node/.openclaw/workspace/functional-rust/examples'))

for entry in entries:
    if '**Status:** [ ]' in entry:
        # Extract number and title
        m = re.search(r'^### (\d+): (.+)$', entry, re.MULTILINE)
        if not m:
            # Try alternative pattern
            m = re.search(r'^### (\d+)\.?\s*(.+)$', entry, re.MULTILINE)
            if not m:
                continue
        num = int(m.group(1))
        title = m.group(2).strip()
        
        # Extract OCaml code
        ocaml_match = re.search(r'```ocaml\n(.*?)```', entry, re.DOTALL)
        ocaml_code = ocaml_match.group(1).strip() if ocaml_match else ''
        
        if ocaml_code and ocaml_code != '[code]':
            pending_entries.append((num, title, ocaml_code, entry))

print(f"Found {len(pending_entries)} pending entries")

# Filter out those that might have directories (check existing dirs by number prefix)
def dir_exists(num):
    prefix = f"{num}-"
    for d in existing_dirs:
        if d.startswith(prefix):
            return True
    return False

pending_without_dir = []
for num, title, ocaml_code, entry in pending_entries:
    if not dir_exists(num):
        pending_without_dir.append((num, title, ocaml_code, entry))

print(f"Found {len(pending_without_dir)} pending entries without directories")

# Sort by number (we want to process in order)
pending_without_dir.sort(key=lambda x: x[0])

# Take 5 (preferably with consecutive numbers > 1212)
candidates = []
for num, title, ocaml_code, entry in pending_without_dir:
    if num > 1212:  # Higher than existing directories
        candidates.append((num, title, ocaml_code, entry))
    if len(candidates) >= 5:
        break

# If not enough > 1212, take any
if len(candidates) < 5:
    candidates = pending_without_dir[:5]

print(f"\nSelected {len(candidates)} entries to process:")
for i, (num, title, ocaml_code, entry) in enumerate(candidates):
    print(f"\n{i+1}. {num}: {title}")
    print(f"   OCaml code length: {len(ocaml_code)} chars")
    # Create directory name
    dir_name = f"{num:03d}-{title.lower().replace(' ', '-').replace('—', '-').replace('–', '-').replace('/', '-').replace(':', '').replace('(', '').replace(')', '').replace('---', '-')[:50]}"
    print(f"   Directory: {dir_name}")

# Write task files for processing
for i, (num, title, ocaml_code, entry) in enumerate(candidates):
    task_file = f"/tmp/task_{num:03d}.txt"
    with open(task_file, 'w') as f:
        f.write(f"Convert this OCaml example to idiomatic Rust.\n\n")
        f.write(f"Directory: examples/{num:03d}-[title_here]/\n\n")
        f.write(f"## OCaml source\n{ocaml_code}\n\n")
        f.write(f"## Topic\n{title}\n\n")
        f.write(f"Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.\n\n")
        f.write(f"When done, report:\nDONE — {num:03d}-[name] — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]")
    print(f"   Task file written: {task_file}")

sys.exit(0)