[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 095 — Balanced Parentheses

## Problem Statement

Check whether a string of brackets is balanced: every opening bracket has a matching closing bracket in the correct order. Implement both an imperative `Vec` stack version and a functional `try_fold` version. Compare with OCaml's recursive list-as-stack approach.

## Learning Outcomes

- Use `Vec<char>` as an explicit stack for bracket matching
- Use `stack.pop() != Some(expected)` for match checking with early return
- Use `try_fold` to propagate failure: return `None` on mismatch, `Some(stack)` on success
- Understand `try_fold` as a short-circuiting fold returning `Option`
- Map Rust's `Vec` stack to OCaml's immutable list used as a stack
- Recognise the stack-based matching algorithm as the canonical solution

## Rust Application

The imperative version iterates `s.chars()`, pushes opening brackets, and pops + compares on closing brackets. If `pop()` returns the wrong bracket or `None`, it returns `false` immediately. After the loop, `stack.is_empty()` confirms no unclosed brackets remain. The functional `is_balanced_fold` uses `try_fold` with `Vec<char>` as state — returning `None` on mismatch short-circuits the fold.

## OCaml Approach

OCaml uses a recursive `check stack i` function. The `matching` helper maps closing brackets to their expected opening. The list stack `top :: rest` is pattern-matched, and recursion continues with `rest` when matched correctly. Lists serve as stacks naturally in OCaml — `::` is O(1) push and pattern matching on `top :: rest` is O(1) pop.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Stack | `Vec<char>` | `char list` |
| Push | `stack.push(c)` | `c :: stack` |
| Pop | `stack.pop()` | Pattern `top :: rest` |
| Empty check | `stack.is_empty()` | `stack = []` |
| Functional style | `try_fold` | Tail recursion |
| Early exit | `return false` | Pattern match fails |

The stack-based balanced bracket algorithm is O(n) time and O(n) space. The functional `try_fold` version is equivalent in complexity but more composable — the stack is the entire state, and failure is represented by `None` rather than an early return.

## Exercises

1. Extend to also balance HTML-style tags: `<div>…</div>` where you must match tag names, not just single characters.
2. Return the position of the first unmatched bracket instead of just `bool`.
3. Handle nested string literals: brackets inside `"…"` should be ignored.
4. Implement `balance_report(s: &str) -> (usize, usize)` returning counts of unmatched opens and closes.
5. In OCaml, implement `is_balanced` using `Seq.fold_left` with an `option char list` accumulator.
