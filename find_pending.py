#!/usr/bin/env python3
import re
import os
import sys

examples_dir = 'examples'
queue_file = 'QUEUE.md'

# Get all existing example directories with their numbers
existing_numbers = {}
for entry in os.listdir(examples_dir):
    if entry.startswith('.'):
        continue
    dir_path = os.path.join(examples_dir, entry)
    if os.path.isdir(dir_path):
        cargo_path = os.path.join(dir_path, 'Cargo.toml')
        has_cargo = os.path.exists(cargo_path)
        # Extract number
        match = re.match(r'^(\d{3,})', entry)
        if match:
            num = match.group(1)
            existing_numbers[num] = has_cargo

print(f"Total numbered directories: {len(existing_numbers)}")
print(f"With Cargo.toml: {sum(1 for v in existing_numbers.values() if v)}")
print(f"Without Cargo.toml: {sum(1 for v in existing_numbers.values() if not v)}")

# Read QUEUE.md
with open(queue_file, 'r') as f:
    content = f.read()

# Find all pending sections
pending = []
pattern = re.compile(r'^### (\d{3}):.*?\*{2}Status:\*{2}\s*\[(.)\]', re.MULTILINE | re.DOTALL)
for match in re.finditer(pattern, content):
    num = match.group(1)
    status = match.group(2)
    if status == ' ':
        # Get section
        start_pos = content.rfind('\n\n', 0, match.start()) + 2
        end_pos = content.find('\n\n---', match.end())
        if end_pos == -1:
            end_pos = content.find('\n\n###', match.end() + 3)
        if end_pos == -1:
            end_pos = len(content)
        
        section = content[start_pos:end_pos]
        
        # Extract info
        title_line = section.split('\n')[0]
        title = title_line.split(':', 1)[1].strip() if ':' in title_line else title_line[4:].strip()
        
        topic_match = re.search(r'\*\*Topic:\*\*\s*(.*?)\n', section, re.IGNORECASE)
        topic = topic_match.group(1).strip() if topic_match else ''
        
        ocaml_match = re.search(r'\*\*OCaml:\*\*\s*```ocaml\n(.*?)\n```', section, re.DOTALL)
        ocaml_code = ocaml_match.group(1) if ocaml_match else ''
        
        if ocaml_code:
            # Check if already has a directory
            has_dir = num in existing_numbers
            has_cargo = existing_numbers.get(num, False)
            
            pending.append({
                'num': num,
                'title': title,
                'topic': topic,
                'ocaml_code': ocaml_code,
                'has_dir': has_dir,
                'has_cargo': has_cargo
            })

print(f"\nTotal pending items in queue: {len(pending)}")

# Print first 10
print("\nFirst 10 pending items:")
for i, p in enumerate(pending[:10]):
    print(f"{i+1}. {p['num']}: {p['title'][:60]}...")
    print(f"   Has dir: {p['has_dir']}, Has Cargo.toml: {p['has_cargo']}")
    print(f"   OCaml lines: {len(p['ocaml_code'].split(chr(10)))}")
    print()

# Find items without Cargo.toml
need_cargo = [p for p in pending if not p['has_cargo']]
print(f"\nItems needing Cargo.toml (dir may or may not exist): {len(need_cargo)}")

# Pick first 5 to process
to_process = []
for p in need_cargo:
    if len(to_process) >= 5:
        break
    to_process.append(p)

print("\nWill process these 5:")
for i, p in enumerate(to_process):
    print(f"{i+1}. {p['num']}: {p['title'][:60]}...")
    
# Also check the special 1199 directories
special_dirs = []
for entry in os.listdir(examples_dir):
    if entry.startswith('1199') and os.path.isdir(os.path.join(examples_dir, entry)):
        cargo_path = os.path.join(examples_dir, entry, 'Cargo.toml')
        if not os.path.exists(cargo_path):
            special_dirs.append(entry)

print(f"\nSpecial 1199 directories without Cargo.toml: {len(special_dirs)}")
for d in special_dirs:
    print(f"  - {d}")