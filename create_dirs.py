#!/usr/bin/env python3
import re
import sys
import json
import os
from pathlib import Path

def kebab_case(s):
    # extract title after colon
    if ':' in s:
        s = s.split(':', 1)[1]
    s = re.sub(r'[^\w\s-]', '', s)
    s = s.strip().lower()
    s = re.sub(r'\s+', '-', s)
    return s

def extract_pending():
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
        if len(pending) >= 15:
            break
    # filter out template (num is None)
    real = [p for p in pending if p['num'] is not None]
    real.sort(key=lambda x: x['num'])
    return real

def main():
    root = Path(__file__).parent
    examples_dir = root / 'examples'
    # get highest existing directory number
    max_num = 0
    for d in examples_dir.iterdir():
        if d.is_dir():
            name = d.name
            if '-' in name:
                prefix = name.split('-')[0]
                if prefix.isdigit():
                    n = int(prefix)
                    if n > max_num:
                        max_num = n
    next_num = max_num + 1
    print(f'Next directory number: {next_num}', file=sys.stderr)
    
    entries = extract_pending()
    print(f'Found {len(entries)} pending entries', file=sys.stderr)
    selected = entries[:5]
    if not selected:
        print('No pending entries', file=sys.stderr)
        sys.exit(1)
    
    created = []
    for idx, entry in enumerate(selected):
        dir_num = next_num + idx
        title_kebab = kebab_case(entry['title'])
        dir_name = f'{dir_num:03d}-{title_kebab}'
        dir_path = examples_dir / dir_name
        dir_path.mkdir(exist_ok=True)
        # write example.ml
        (dir_path / 'example.ml').write_text(entry['code'])
        # write metadata.json (optional)
        meta = {k: v for k, v in entry.items() if k != 'code'}
        (dir_path / 'metadata.json').write_text(json.dumps(meta, indent=2))
        created.append(dir_name)
        print(f'Created {dir_name}', file=sys.stderr)
    # output created dirs
    for d in created:
        print(d)

if __name__ == '__main__':
    main()