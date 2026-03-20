📖 **[View on hightechmind.io →](https://hightechmind.io/rust/064-balanced-parentheses)**

---

# 064 — Balanced Parentheses
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Checking whether brackets are balanced is the classic stack application problem. Every `(`, `[`, or `{` must have a matching closing bracket in the correct order. The algorithm is: push opening brackets; on closing brackets, pop and verify the match; at end, the stack must be empty.

This problem appears in: compilers (syntax checking), text editors (bracket highlighting), math expression parsing, HTML/XML validation, and shell scripts (unmatched quotes). It is also the typical introductory interview problem for stacks.

## Learning Outcomes

- Use `Vec<char>` as a stack for bracket matching
- Push opening brackets, verify and pop on closing brackets
- Return `false` immediately on mismatch (early exit)
- Return `false` at end if the stack is non-empty (unclosed brackets)
- Implement a recursive version using immutable slice-as-stack (functional style)

## Rust Application

`is_balanced` uses `Vec<char>` as a stack. Opening brackets `(`, `[`, `{` are pushed. For closing brackets, `stack.pop()` is checked against the expected matching opener — if the stack is empty or mismatches, return `false`. After processing all characters, `stack.is_empty()` is the final check. The recursive version `is_balanced_recursive` passes the stack as a `Vec` accumulator to avoid mutation at the call site.

## OCaml Approach

OCaml's version: `let is_balanced s = let stack = Stack.create () in try String.iter (fun c -> match c with | '(' | '[' | '{' -> Stack.push c stack | ')' -> if Stack.pop stack <> '(' then raise Exit | ']' -> if Stack.pop stack <> '[' then raise Exit | '}' -> if Stack.pop stack <> '{' then raise Exit | _ -> ()) s; Stack.is_empty stack with Exit -> false | Stack.Empty -> false`. The `try/with` handles both the mismatch and the empty-stack cases.

## Key Differences

1. **Exception vs early return**: OCaml uses exceptions (`raise Exit`) for early exit from iteration. Rust uses `return false` inside a `for` loop — more explicit.
2. **`Stack` module vs `Vec`**: OCaml's `Stack` module is a mutable stack. Rust uses `Vec<char>` as a stack — `push` appends, `pop` removes from end.
3. **`String.iter` vs `chars()`**: OCaml's `String.iter f s` calls `f` on each byte. Rust's `s.chars()` iterates Unicode scalar values. For ASCII input (brackets), both are equivalent.
4. **Functional recursive**: The recursive approach treats the remaining input and current stack as function arguments, enabling pure functional style without mutation.

## Exercises

1. **Return mismatch position**: Return `Result<(), (usize, char)>` where the error contains the position and character where balancing failed. Use `enumerate()` on the char iterator.
2. **Nesting depth**: Write `max_nesting_depth(s: &str) -> usize` that returns the maximum nesting depth of parentheses without checking for balance.
3. **Generate balanced**: Write `generate_balanced(n: usize) -> Vec<String>` that generates all balanced parenthesis strings of exactly n pairs. This is the Catalan number problem (C(n) = (2n choose n) / (n+1)).
