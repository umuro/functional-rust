#!/bin/bash
# Find pending queue entries
cd /home/node/.openclaw/workspace/functional-rust
QUEUE="QUEUE.md.backup"
echo "Looking for pending entries in $QUEUE"
echo "Number of lines: $(wc -l < "$QUEUE")"
echo "Number of '[ ]': $(grep -c '\[ \]' "$QUEUE")"

# Find line numbers of actual entries (not the legend)
grep -n '^### ' "$QUEUE" | while read linenum title; do
    # Get the block until next '---'
    # Not implementing full extraction but we can at least see titles
    echo "$linenum: $title"
done | head -20

echo ""
echo "First few '[ ]' lines (excluding legend):"
grep -n '\[ \]' "$QUEUE" | grep -v '^6:' | head -10