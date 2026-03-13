#!/usr/bin/env python3
import sys
import os

def parse_queue(filepath):
    with open(filepath, 'r') as f:
        lines = f.readlines()
    
    i = 0
    entries = []
    while i < len(lines):
        line = lines[i].strip()
        if line.startswith('### '):
            # start of entry
            entry_lines = []
            j = i
            while j < len(lines) and not (lines[j].strip() == '---' and j > i):
                entry_lines.append(lines[j])
                j += 1
            entry_text = ''.join(entry_lines)
            # check if pending
            if '**Status:** [ ]' in entry_text:
                # parse fields
                num = None
                title = None
                topic = None
                source = None
                code = None
                # extract number from ### line
                import re
                m = re.search(r'### (\d+):', entry_text)
                if m:
                    num = int(m.group(1))
                # title is the ### line
                title_match = re.search(r'### .+', entry_text)
                if title_match:
                    title = title_match.group(0).strip()
                # topic
                topic_match = re.search(r'\*\*Topic:\*\*\s*(.+)', entry_text)
                if topic_match:
                    topic = topic_match.group(1).strip()
                # source
                source_match = re.search(r'\*\*Source:\*\*\s*(.+)', entry_text)
                if source_match:
                    source = source_match.group(1).strip()
                # OCaml code block
                code_match = re.search(r'```ocaml\n(.*?)\n```', entry_text, re.DOTALL)
                if code_match:
                    code = code_match.group(1).strip()
                if code:
                    entries.append({
                        'num': num,
                        'title': title,
                        'topic': topic,
                        'source': source,
                        'code': code
                    })
            i = j  # move to after the separator
        else:
            i += 1
    return entries

if __name__ == '__main__':
    entries = parse_queue('QUEUE.md')
    print(f'Found {len(entries)} pending entries', file=sys.stderr)
    for idx, e in enumerate(entries[:10]):
        print(f'\n--- Entry {idx+1} ---', file=sys.stderr)
        print(f"Number: {e['num']}", file=sys.stderr)
        print(f"Title: {e['title']}", file=sys.stderr)
        print(f"Topic: {e['topic']}", file=sys.stderr)
        print(f"Source: {e['source']}", file=sys.stderr)
        print(f"Code preview: {e['code'][:200]}...", file=sys.stderr)
    # Output as machine-readable format (JSON)
    import json
    json.dump(entries[:10], sys.stdout, indent=2)