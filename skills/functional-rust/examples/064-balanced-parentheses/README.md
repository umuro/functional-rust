# 064: Balanced Parentheses

**Difficulty:** ⭐  **Level:** Foundations

Check whether every opening bracket has a matching closing bracket in the right order — the classic stack algorithm.

## The Problem This Solves

You're writing a linter, a simple expression parser, or checking whether a JSON/HTML template has unclosed tags. The problem always reduces to the same question: are the brackets balanced?

The naive approach — counting opens vs closes — fails for `"([)]"`: same count, wrong order. You need a stack. The algorithm is simple: push opening brackets, pop on closing ones, and check the popped value matches. At the end, the stack must be empty.

This is one of the most common interview problems precisely because it's clean and teaches an important concept: some problems need memory of the past, and the right data structure (a stack) makes the solution obvious.

## The Intuition

Think of it like checking parentheses in a math textbook. When you see `(`, you mentally "open a tab." When you see `)`, you close the most recent open tab. If the tab you're closing doesn't match what you opened, something is wrong. If you reach the end with open tabs remaining, also wrong.

In Python you'd use a list as a stack (`stack.append()` / `stack.pop()`). In JavaScript, same thing with an array. Rust uses `Vec<char>` with `.push()` and `.pop()`.

The Rust version shines in the pattern match — each closing bracket is handled in its own branch, and `pop()` returns `Option<char>`, forcing you to handle the "nothing on the stack" case explicitly.

## How It Works in Rust

```rust
pub fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),  // push opening brackets
            ')' => {
                if stack.pop() != Some('(') { return false; }
            }
            ']' => {
                if stack.pop() != Some('[') { return false; }
            }
            '}' => {
                if stack.pop() != Some('{') { return false; }
            }
            _ => {}  // ignore letters, spaces, etc.
        }
    }
    stack.is_empty()  // unmatched opens would remain
}
```

`pop()` returns `Option<char>` — `None` if the stack is empty (unmatched close bracket), `Some(c)` otherwise. Comparing directly to `Some('(')` handles both the empty-stack case and the wrong-bracket case in one check.

## What This Unlocks

- **Parsing** — any recursive/nested structure (HTML, JSON, code) can be validated with a stack
- **Expression evaluation** — extend this to evaluate arithmetic with operator precedence using two stacks
- **State machines** — the "push on open, pop on close" pattern generalizes to any context where you need to match paired events

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Stack data structure | `list` (immutable, cons/hd) | `Vec<char>` with `.push()` / `.pop()` |
| Pop result | Pattern match on list head | `pop()` returns `Option<char>` |
| Empty stack check | `= []` | `.is_empty()` |
| Match on char | `match c with \| '(' -> ...` | `match c { '(' \| '[' \| '{' => ... }` |
