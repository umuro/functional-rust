import re, sys
with open('QUEUE.md', 'r') as f:
    content = f.read()
# split by '---'
parts = re.split(r'\n---\n', content)
for part in parts:
    status_match = re.search(r'\*\*Status:\*\* \[(.)\]', part)
    if not status_match:
        continue
    status = status_match.group(1)
    match = re.match(r'### (\d{3}): (.+?)\n', part)
    if not match:
        continue
    num = match.group(1)
    title = match.group(2).strip()
    if num not in ['029', '030', '036', '037', '038']:
        continue
    code_match = re.search(r'```ocaml\n(.*?)```', part, re.DOTALL)
    code = code_match.group(1).strip() if code_match else ''
    print(f'--- {num}: {title} ---')
    print(code)
    print()