#!/usr/bin/env python3
import re
import os
import sys

# Read the queue file
with open('/home/node/.openclaw/workspace/functional-rust/QUEUE.md.backup', 'r', encoding='utf-8') as f:
    content = f.read()

# Split into entries
entries = re.split(r'^---\s*$', content, flags=re.MULTILINE)

pending_items = []

for entry in entries:
    if '**Status:** [ ]' in entry:
        # Extract number and title
        m = re.search(r'^### (\d+): (.+)$', entry, re.MULTILINE)
        if not m:
            continue
        num = m.group(1).zfill(3)
        title = m.group(2).strip()
        
        # Extract OCaml code
        ocaml_match = re.search(r'```ocaml\n(.*?)```', entry, re.DOTALL)
        ocaml_code = ocaml_match.group(1).strip() if ocaml_match else ''
        
        if ocaml_code:
            # Check if directory already exists
            dir_prefix = f"{num}-"
            dir_exists = any(d.startswith(dir_prefix) for d in os.listdir('/home/node/.openclaw/workspace/functional-rust/examples'))
            
            if not dir_exists:
                pending_items.append({
                    'number': num,
                    'title': title,
                    'ocaml_code': ocaml_code,
                    'full_text': entry
                })

print(f"Found {len(pending_items)} pending items without existing directories")
for i, item in enumerate(pending_items[:10]):
    print(f"\n--- Item {i+1} ---")
    print(f"Number: {item['number']}")
    print(f"Title: {item['title']}")
    print(f"OCaml code length: {len(item['ocaml_code'])}")

# Return first 5
top_five = pending_items[:5]
print(f"\n=== TOP 5 TO PROCESS ===")
for i, item in enumerate(top_five):
    print(f"{i+1}. {item['number']}: {item['title']}")

# Prepare for processing
with open('/tmp/top_five.txt', 'w') as f:
    for i, item in enumerate(top_five):
        f.write(f"Item {i+1}:\n")
        f.write(f"Number: {item['number']}\n")
        f.write(f"Title: {item['title']}\n")
        f.write(f"OCAML:\n{item['ocaml_code']}\n")
        f.write("-" * 80 + "\n")

sys.exit(0)