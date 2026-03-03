# Balanced Parentheses — OCaml vs Rust Comparison

## Core Insight

The stack data structure is central to bracket matching. OCaml models the stack as an immutable list (push = cons, pop = pattern match on head). Rust uses `Vec<char>` with `push()`/`pop()`. The `Option` return from `pop()` naturally handles the empty-stack case.

## OCaml Approach

A recursive function carries the stack as a list parameter. Pattern matching destructures both the current character and the stack top simultaneously. The functional style means no mutation — each recursive call gets a new stack state.

## Rust Approach

Iterative with `Vec::push()` and `Vec::pop()`. `pop()` returns `Option<char>`, so comparing with `Some('(')` handles both the empty-stack and wrong-bracket cases in one expression. The imperative style with early `return false` is idiomatic.

## Comparison Table

| Aspect        | OCaml                          | Rust                              |
|---------------|--------------------------------|-----------------------------------|
| **Memory**    | List stack (cons cells)        | Vec stack (contiguous memory)     |
| **Null safety** | Pattern match on list       | `Option<char>` from `pop()`       |
| **Errors**    | Returns false                  | Returns false                     |
| **Iteration** | Recursive with index           | `for c in s.chars()`              |
| **Stack op**  | `c :: stack` / `h :: t`        | `push(c)` / `pop()`              |

## Things Rust Learners Should Notice

1. **`Vec::pop()` returns `Option<T>`** — no panics on empty, unlike some languages
2. **Pattern matching on `Option`** — `if stack.pop() != Some('(')` is concise and safe
3. **`Vec` is a great stack** — O(1) amortized push/pop, cache-friendly, unlike linked list
4. **Early return** — `return false` inside a `for` loop is idiomatic Rust for short-circuiting

## Further Reading

- [Vec as a stack](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
- [Exercism: Matching Brackets](https://exercism.org/tracks/rust/exercises/matching-brackets)
