import re
import sys

with open('QUEUE.md', 'r') as f:
    content = f.read()

# Split by '---' horizontal rule? Actually entries are separated by '---' lines.
# Let's split by '\n---\n'
entries = []
parts = re.split(r'\n---\n', content)
for part in parts:
    if not part.strip():
        continue
    # Find status
    status_match = re.search(r'\*\*Status:\*\* \[(.)\]', part)
    if not status_match:
        continue
    status = status_match.group(1)
    # Extract number and title
    match = re.match(r'### (\d{3}): (.+?)\n', part)
    if not match:
        continue
    num = match.group(1)
    title = match.group(2).strip()
    # Extract OCaml code block
    code_match = re.search(r'```ocaml\n(.*?)```', part, re.DOTALL)
    code = code_match.group(1).strip() if code_match else ''
    entries.append({
        'num': num,
        'title': title,
        'status': status,
        'code': code,
        'raw': part
    })

pending = [e for e in entries if e['status'] == ' ']
print(f'Total pending: {len(pending)}')
# Take first 5
selected = pending[:5]
for i, e in enumerate(selected):
    print(f'\n--- Entry {i+1} ---')
    print(f'Number: {e[\"num\"]}')
    print(f'Title: {e[\"title\"]}')
    print(f'Code:\n{e[\"code\"]}')