#!/usr/bin/env python3
import re
import os
import sys
import json
from pathlib import Path

def parse_queue(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    # split by horizontal rule, but careful with leading/trailing newlines
    # entries are separated by --- on its own line
    blocks = re.split(r'\n---+\n', content)
    entries = []
    for block in blocks:
        if '**Status:** [ ]' not in block:
            continue
        # extract number
        m = re.search(r'### (\d+):', block)
        if not m:
            continue
        num = int(m.group(1))
        # title line
        title_match = re.search(r'### .+', block)
        title = title_match.group(0).strip() if title_match else ''
        # topic
        topic_match = re.search(r'\*\*Topic:\*\*\s*(.+)', block)
        topic = topic_match.group(1).strip() if topic_match else ''
        # source
        source_match = re.search(r'\*\*Source:\*\*\s*(.+)', block)
        source = source_match.group(1).strip() if source_match else ''
        # difficulty
        diff_match = re.search(r'\*\*Difficulty:\*\*\s*(.+)', block)
        difficulty = diff_match.group(1).strip() if diff_match else ''
        # category
        cat_match = re.search(r'\*\*Category:\*\*\s*(.+)', block)
        category = cat_match.group(1).strip() if cat_match else ''
        # OCaml code
        code_match = re.search(r'```ocaml\n(.*?)\n```', block, re.DOTALL)
        code = code_match.group(1).strip() if code_match else ''
        if not code:
            continue
        entries.append({
            'num': num,
            'title': title,
            'topic': topic,
            'source': source,
            'difficulty': difficulty,
            'category': category,
            'code': code
        })
    # sort by number
    entries.sort(key=lambda e: e['num'])
    return entries

def kebab_case(s):
    # convert title line like "### 025: List.map — Transform Every Element"
    # extract the title part after colon and optional space
    # remove leading ### and number
    # keep only letters, digits, spaces, hyphens; replace spaces with hyphens
    # lowercase
    # remove any trailing punctuation
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