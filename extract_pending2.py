#!/usr/bin/env python3
import re
import sys

with open('QUEUE.md', 'r') as f:
    content = f.read()

entries = re.split(r'\n---\n', content)

pending = []
for entry in entries:
    if '**Status:** [ ]' in entry:
        lines = entry.strip().split('\n')
        title_line = None
        for line in lines:
            if line.startswith('###'):
                title_line = line
                break
        if not title_line:
            continue
        match = re.match(r'### (\d+): (.*)', title_line)
        if not match:
            continue
        num = int(match.group(1))
        title = match.group(2)
        if num <= 24:
            continue  # skip already processed
        code_match = re.search(r'```ocaml\n(.*?)```', entry, re.DOTALL)
        if not code_match:
            continue
        code = code_match.group(1).strip()
        pending.append((num, title, code))
        if len(pending) >= 4:
            break

for i, (num, title, code) in enumerate(pending):
    print(f'Item {i+1}: {num:03d}: {title}')
    print(f'Code length: {len(code)}')
    with open(f'temp_{num:03d}.ml', 'w') as f:
        f.write(code)
    print(f'Saved to temp_{num:03d}.ml')
    print()