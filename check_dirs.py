#!/usr/bin/env python3
import os
import re

examples_dir = '/home/node/.openclaw/workspace/functional-rust/examples'
all_dirs = [d for d in os.listdir(examples_dir) if os.path.isdir(os.path.join(examples_dir, d))]
print(f"Total directories: {len(all_dirs)}")

# Check for OCaml files and Cargo.toml
without_toml = []
with_toml = []

for d in all_dirs:
    path = os.path.join(examples_dir, d)
    ml = os.path.join(path, 'example.ml')
    toml = os.path.join(path, 'Cargo.toml')
    
    has_ml = os.path.exists(ml)
    has_toml = os.path.exists(toml)
    
    if has_ml and not has_toml:
        without_toml.append(d)
    elif has_toml:
        with_toml.append(d)

print(f"\nDirectories with OCaml but no Cargo.toml: {len(without_toml)}")
print(f"Directories with Cargo.toml: {len(with_toml)}")

print("\nFirst 10 without Cargo.toml:")
for d in sorted(without_toml)[:10]:
    print(f"  {d}")

# Now check the QUEUE for truly pending items
queue_file = '/home/node/.openclaw/workspace/functional-rust/QUEUE.md.backup'
with open(queue_file, 'r', encoding='utf-8') as f:
    content = f.read()

entries = re.split(r'^---\s*$', content, flags=re.MULTILINE)
pending_in_queue = []

for entry in entries:
    if '**Status:** [ ]' in entry:
        m = re.search(r'^### (\d+): (.+)$', entry, re.MULTILINE)
        if m:
            num = m.group(1).zfill(3)
            title = m.group(2).strip()
            pending_in_queue.append((num, title))

print(f"\nPending entries in QUEUE.md.backup: {len(pending_in_queue)}")

# Filter out those that already have directories (even without Cargo.toml)
pending_no_dir = []
for num, title in pending_in_queue:
    dir_prefix = f"{num}-"
    has_dir = any(d.startswith(dir_prefix) for d in all_dirs)
    if not has_dir:
        pending_no_dir.append((num, title))

print(f"Pending entries WITHOUT any directory: {len(pending_no_dir)}")
for i, (num, title) in enumerate(pending_no_dir[:10]):
    print(f"  {num}: {title}")

return_dirs = without_toml[:5] if without_toml else pending_no_dir[:5]
return_dirs = [d[0] if isinstance(d, tuple) else d for d in return_dirs]
print(f"\nSelected 5 to process:")
for i, d in enumerate(return_dirs[:5]):
    print(f"  {i+1}. {d}")