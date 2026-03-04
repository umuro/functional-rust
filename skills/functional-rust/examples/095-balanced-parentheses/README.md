# 095: Balanced Parentheses

**Difficulty:** 1  **Level:** Foundations

Check whether every opening bracket has a matching close — using a stack.

## The Problem This Solves

Compilers, linters, and code editors all need to verify bracket matching. When you type `([{}])` in an IDE and it highlights mismatches in real time, bracket checking is running on every keystroke.

The core challenge: brackets must nest correctly. `([)]` fails even though both pairs exist — the nesting is wrong. You can't just count opens and closes; you need to remember *which* bracket you opened.

A stack is the natural data structure: push when you open, pop and verify when you close. If the popped bracket doesn't match, fail immediately.

## The Intuition

Imagine a stack of receipts. Every time you open a bracket, you place it on the pile. Every time you close a bracket, you check the top of the pile — it should be the matching opener. If the pile is empty at the end, everything balanced.

`Vec<char>` in Rust works exactly like a stack: `.push(c)` to put something on top, `.pop()` to take it off. The `match` statement handles the three bracket pairs cleanly.

The functional variant uses `try_fold` — it folds over the characters, threading the stack as accumulator, and returns `None` (early exit) on the first mismatch.

## How It Works in Rust

```rust
pub fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),   // open: push
            ')' | ']' | '}' => {
                let expected = match c {
                    ')' => '(', ']' => '[', '}' => '{',
                    _ => unreachable!(),
                };
                if stack.pop() != Some(expected) {
                    return false;   // wrong or missing opener
                }
            }
            _ => {}  // ignore non-bracket characters
        }
    }
    stack.is_empty()  // true iff all openers were closed
}
```

The functional style uses `try_fold` for early exit:

```rust
pub fn is_balanced_fold(s: &str) -> bool {
    let result = s.chars().try_fold(Vec::new(), |mut stack, c| {
        match c {
            '(' | '[' | '{' => { stack.push(c); Some(stack) }
            ')' | ']' | '}' => {
                // pop and verify — return None to short-circuit
                if stack.pop() == Some(matching(c)) { Some(stack) } else { None }
            }
            _ => Some(stack),
        }
    });
    matches!(result, Some(s) if s.is_empty())
}
```

`try_fold` stops as soon as `None` is returned — no wasted iterations.

## What This Unlocks

- **Editor tooling** — real-time bracket mismatch highlighting
- **Parser validation** — pre-check input before handing it to a full parser
- **Config/data validation** — check JSON, XML, TOML structure before parsing

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Stack type | `char list` (functional list) | `Vec<char>` |
| Push | `c :: stack` (prepend) | `stack.push(c)` |
| Pop | Pattern match on head | `stack.pop()` → `Option<char>` |
| Early exit | `Option` in recursive call | `try_fold` returning `None` |
| Mutual recursion | Natural with `let rec` | Simple loop — no recursion needed |
