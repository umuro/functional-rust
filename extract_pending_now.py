#!/usr/bin/env python3
import re
import sys

with open('/home/node/.openclaw/workspace/functional-rust/QUEUE.md.backup', 'r', encoding='utf-8') as f:
    content = f.read()

# Pattern for a queue entry
entries = re.split(r'^---\s*$', content, flags=re.MULTILINE)
pending = []
for entry in entries:
    if '[ ]' in entry:
        # Extract number and title
        m = re.search(r'^###\s*(\d+):\s*(.+)$', entry, re.MULTILINE)
        if m:
            num = m.group(1).zfill(3)
            title = m.group(2).strip()
        else:
            # maybe the line before OCaml block
            lines = entry.strip().split('\n')
            for line in lines:
                if line.startswith('###'):
                    parts = line[4:].split(':', 1)
                    if len(parts) == 2:
                        num = parts[0].strip().zfill(3)
                        title = parts[1].strip()
                        break
            else:
                num = '???'
                title = 'Unknown'
        # Extract OCaml code
        ocaml_match = re.search(r'```ocaml\n(.*?)```', entry, re.DOTALL)
        ocaml_code = ocaml_match.group(1).strip() if ocaml_match else ''
        if ocaml_code:
            pending.append((num, title, ocaml_code))

print(f"Found {len(pending)} pending entries")
for i, (num, title, code) in enumerate(pending[:10]):
    print(f"\n--- Entry {i+1} ---")
    print(f"Number: {num}")
    print(f"Title: {title}")
    print(f"OCAML code length: {len(code)} chars")
    print(f"First 100 chars: {code[:100]}...")

sys.exit(0)