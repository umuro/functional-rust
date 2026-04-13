#!/usr/bin/env python3
import re
import os

with open('/home/node/.openclaw/workspace/functional-rust/QUEUE.md.backup', 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Find all entry numbers
entry_numbers = []
i = 0
while i < len(lines):
    if lines[i].startswith('### '):
        # Extract number
        m = re.match(r'^### (\d+):', lines[i])
        if m:
            num = int(m.group(1))
            entry_numbers.append((i, num))  # line index, number
    i += 1

print(f"Total entries found: {len(entry_numbers)}")
print(f"First 5: {entry_numbers[:5]}")
print(f"Last 5: {entry_numbers[-5:]}")

# Find the next 5 after 1212
target_start_idx = None
for idx, (line_idx, num) in enumerate(entry_numbers):
    if num > 1212:
        target_start_idx = idx
        break

if target_start_idx is None:
    print("No entries with number > 1212")
    # Go to beginning
    target_start_idx = 0

print(f"\nStarting at index {target_start_idx}, number {entry_numbers[target_start_idx][1]}")

# Collect 5 entries
selected = []
for j in range(target_start_idx, min(target_start_idx + 5, len(entry_numbers))):
    line_idx, num = entry_numbers[j]
    # Find the end of this entry (next '---' or end of file)
    end_idx = line_idx + 1
    while end_idx < len(lines) and not lines[end_idx].startswith('---'):
        end_idx += 1
    
    # Extract the entry block
    entry_lines = lines[line_idx:end_idx]
    entry_text = ''.join(entry_lines)
    
    # Check status
    if '**Status:** [ ]' in entry_text:
        # Extract title
        title_match = re.search(r'^### \d+: (.+)$', lines[line_idx], re.MULTILINE)
        title = title_match.group(1).strip() if title_match else f"Entry {num}"
        
        # Extract OCaml code
        ocaml_match = re.search(r'```ocaml\n(.*?)```', entry_text, re.DOTALL)
        if ocaml_match:
            ocaml_code = ocaml_match.group(1).strip()
            if ocaml_code != '[code]':
                selected.append((num, title, ocaml_code, entry_text))
                print(f"Selected {num}: {title}")
        else:
            print(f"Entry {num} has no OCaml code block")
    else:
        print(f"Entry {num} is already marked [x]")

print(f"\nSelected {len(selected)} entries")
if len(selected) < 5:
    print("Need more entries, looking further...")
    # Could look for entries with numbers < 1212 that are pending
    # But for now, just return what we have

# For each selected, create directory name
for i, (num, title, ocaml_code, entry_text) in enumerate(selected):
    # Sanitize title for directory name
    sanitized = re.sub(r'[^\w\s-]', '', title.lower())
    sanitized = re.sub(r'[-\s]+', '-', sanitized)
    dir_name = f"{num:03d}-{sanitized[:50]}"
    
    # Check if directory exists
    examples_dir = '/home/node/.openclaw/workspace/functional-rust/examples'
    exists = any(d.startswith(f"{num:03d}-") for d in os.listdir(examples_dir))
    
    print(f"\n{i+1}. {num:03d}: {title}")
    print(f"   Dir will be: {dir_name}")
    print(f"   Already exists: {exists}")
    print(f"   OCaml code length: {len(ocaml_code)} chars")
    if len(ocaml_code) > 200:
        print(f"   Preview: {ocaml_code[:200]}...")

if not selected:
    print("\nNo pending entries found. Checking for any '[ ]' status...")
    # Fallback: find any '[ ]' in the file
    for i, line in enumerate(lines):
        if '**Status:** [ ]' in line:
            # Find the preceding '###' line
            j = i - 1
            while j >= 0 and not lines[j].startswith('### '):
                j -= 1
            if j >= 0 and lines[j].startswith('### '):
                m = re.match(r'^### (\d+):', lines[j])
                if m:
                    num = int(m.group(1))
                    print(f"Found pending entry at line {j+1}: {lines[j].strip()}")
                    # Get entry block
                    k = j
                    while k < len(lines) and not (k > j and lines[k].startswith('---')):
                        k += 1
                    entry_text = ''.join(lines[j:k])
                    # Extract OCaml
                    ocaml_match = re.search(r'```ocaml\n(.*?)```', entry_text, re.DOTALL)
                    if ocaml_match:
                        ocaml_code = ocaml_match.group(1).strip()
                        if ocaml_code != '[code]':
                            selected.append((num, "Extracted", ocaml_code, entry_text))
                            if len(selected) >= 5:
                                break

print(f"\nTotal selected for processing: {len(selected)}")
if selected:
    print("\nWill process these:") 
    for i, (num, title, ocaml_code, _) in enumerate(selected):
        print(f"  {i+1}. {num:03d}")