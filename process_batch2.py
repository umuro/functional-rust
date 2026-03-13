#!/usr/bin/env python3
import re
import os
import sys
import json
from pathlib import Path

def parse_queue(filepath):
    with open(filepath, 'r') as f:
        lines = f.readlines()
    entries = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith('### '):
            # start of entry
            block_lines = []
            j = i
            while j < len(lines) and not (lines[j].strip().startswith('---') and len(lines[j].strip()) >= 3 and j > i):
                block_lines.append(lines[j])
                j += 1
            block = ''.join(block_lines)
            # check status pending
            if '**Status:** [ ]' in block:
                # parse fields
                num_match = re.search(r'### (\d+):', block)
                num = int(num_match.group(1)) if num_match else None
                title_match = re.search(r'### .+', block)
                title = title_match.group(0).strip() if title_match else ''
                topic_match = re.search(r'\*\*Topic:\*\*\s*(.+)', block)
                topic = topic_match.group(1).strip() if topic_match else ''
                source_match = re.search(r'\*\*Source:\*\*\s*(.+)', block)
                source = source_match.group(1).strip() if source_match else ''
                diff_match = re.search(r'\*\*Difficulty:\*\*\s*(.+)', block)
                difficulty = diff_match.group(1).strip() if diff_match else ''
                cat_match = re.search(r'\*\*Category:\*\*\s*(.+)', block)
                category = cat_match.group(1).strip() if cat_match else ''
                code_match = re.search(r'```ocaml\n(.*?)\n```', block, re.DOTALL)
                code = code_match.group(1).strip() if code_match else ''
                if code:
                    entries.append({
                        'num': num,
                        'title': title,
                        'topic': topic,
                        'source': source,
                        'difficulty': difficulty,
                        'category': category,
                        'code': code
                    })
            i = j  # move to separator line
        else:
            i += 1
    entries.sort(key=lambda e: e['num'] if e['num'] is not None else 0)
    return entries

def kebab_case(s):
    # extract title after colon
    if ':' in s:
        s = s.split(':', 1)[1]
    s = re.sub(r'[^\w\s-]', '', s)
    s = s.strip().lower()
    s = re.sub(r'\s+', '-', s)
    return s

def main():
    root = Path(__file__).parent
    examples_dir = root / 'examples'
    entries = parse_queue('QUEUE.md')
    pending = [e for e in entries if e['num'] is not None]
    print(f'Found {len(pending)} pending entries', file=sys.stderr)
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
    # take first 5 pending entries
    selected = pending[:5]
    if not selected:
        print('No pending entries found', file=sys.stderr)
        sys.exit(1)
    # process each
    for idx, entry in enumerate(selected):
        dir_num = next_num + idx
        title_kebab = kebab_case(entry['title'])
        dir_name = f'{dir_num:03d}-{title_kebab}'
        dir_path = examples_dir / dir_name
        dir_path.mkdir(exist_ok=True)
        # write example.ml
        (dir_path / 'example.ml').write_text(entry['code'])
        # write metadata as JSON (optional)
        meta = {k: v for k, v in entry.items() if k != 'code'}
        (dir_path / 'metadata.json').write_text(json.dumps(meta, indent=2))
        print(f'Created {dir_name}', file=sys.stderr)
    print(f'Created {len(selected)} directories', file=sys.stderr)
    # output the list for bash processing
    for idx, entry in enumerate(selected):
        dir_num = next_num + idx
        title_kebab = kebab_case(entry['title'])
        dir_name = f'{dir_num:03d}-{title_kebab}'
        print(dir_name)

if __name__ == '__main__':
    main()