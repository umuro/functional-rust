#!/usr/bin/env python3
import re
import sys
import json

with open('QUEUE.md', 'r') as f:
    lines = f.readlines()

i = 0
pending = []
while i < len(lines):
    line = lines[i]
    if line.startswith('### '):
        # check if status pending within next 20 lines
        found_pending = False
        for j in range(i, min(i+20, len(lines))):
            if lines[j].startswith('**Status:** [ ]'):
                found_pending = True
                break
        if found_pending:
            # find OCaml block
            for k in range(i, min(i+30, len(lines))):
                if lines[k].strip() == '```ocaml':
                    code_lines = []
                    l = k + 1
                    while l < len(lines) and lines[l].strip() != '```':
                        code_lines.append(lines[l])
                        l += 1
                    code = ''.join(code_lines)
                    # parse number and title
                    num_match = re.search(r'### (\d+):', line)
                    num = int(num_match.group(1)) if num_match else None
                    # topic and source (look ahead)
                    topic = ''
                    source = ''
                    for m in range(i, l):
                        if lines[m].startswith('**Topic:**'):
                            topic = lines[m].split('**Topic:**',1)[1].strip()
                        if lines[m].startswith('**Source:**'):
                            source = lines[m].split('**Source:**',1)[1].strip()
                    pending.append({
                        'num': num,
                        'title': line.strip(),
                        'topic': topic,
                        'source': source,
                        'code': code
                    })
                    i = l  # skip to end of code block
                    break
    i += 1
    if len(pending) >= 10:
        break

# filter out template (num is None)
real = [p for p in pending if p['num'] is not None]
print(f'Found {len(real)} real pending entries', file=sys.stderr)
# sort by num
real.sort(key=lambda x: x['num'])
selected = real[:5]
for idx, e in enumerate(selected):
    print(f'{idx+1}: {e["num"]} - {e["title"]}', file=sys.stderr)
# output as JSON
json.dump(selected, sys.stdout, indent=2)