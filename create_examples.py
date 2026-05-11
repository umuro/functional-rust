#!/usr/bin/env python3
import re, os, sys
from pathlib import Path

def slugify(title):
    # simple slug: lowercase, replace spaces with hyphens, remove non-alnum
    slug = re.sub(r'[^a-z0-9]+', '-', title.lower()).strip('-')
    return slug

def main():
    with open('QUEUE.md', 'r') as f:
        content = f.read()
    entries = re.split(r'\n### ', content)
    pending = []
    for e in entries:
        if '**Status:** [ ]' not in e:
            continue
        first_line = e.split('\n')[0]
        match = re.match(r'(\d+): (.+)', first_line)
        if not match:
            continue
        num = int(match.group(1))
        title = match.group(2)
        code_match = re.search(r'```ocaml\n(.*?)```', e, re.DOTALL)
        if not code_match:
            continue
        code = code_match.group(1).strip()
        pending.append((num, title, code))
    
    # Find highest numeric prefix in examples directory
    examples_dir = Path('examples')
    max_num = 0
    for d in examples_dir.iterdir():
        if d.is_dir():
            name = d.name
            m = re.match(r'^(\d+)-', name)
            if m:
                n = int(m.group(1))
                if n > max_num:
                    max_num = n
    print(f'Highest existing number: {max_num}')
    
    # Process first 5 pending entries
    for i, (num, title, code) in enumerate(pending[:5]):
        new_num = max_num + i + 1
        slug = slugify(title)
        dir_name = f'{new_num:04d}-{slug}'
        dir_path = examples_dir / dir_name
        dir_path.mkdir(parents=True, exist_ok=True)
        example_ml = dir_path / 'example.ml'
        example_ml.write_text(code)
        print(f'Created {dir_name} with {len(code)} bytes')
        # Optionally create a placeholder README.md
        readme = dir_path / 'README.md'
        if not readme.exists():
            readme.write_text(f'# {title}\n\nOCaml example converted to Rust.\n')
    
    # Write list of created dirs to a file for later processing
    with open('created_dirs.txt', 'w') as f:
        for i, (num, title, code) in enumerate(pending[:5]):
            new_num = max_num + i + 1
            slug = slugify(title)
            dir_name = f'{new_num:04d}-{slug}'
            f.write(f'{dir_name}\n')
    print('Wrote created_dirs.txt')

if __name__ == '__main__':
    main()