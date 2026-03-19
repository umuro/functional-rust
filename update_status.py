#!/usr/bin/env python3
import re
import sys

with open('QUEUE.md', 'r') as f:
    content = f.read()

# Find entry for 024
pattern = r'(### 024:.*?)(\*\*Status:\*\*) \[ \]'
replacement = r'\1\2 [x]'
new_content = re.sub(pattern, replacement, content, flags=re.DOTALL)

if new_content == content:
    print("No change made. Pattern not found.")
    sys.exit(1)

with open('QUEUE.md', 'w') as f:
    f.write(new_content)
print("Updated status for 024 to [x].")