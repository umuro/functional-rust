# Example 1110: ABC Words

**Difficulty:** ⭐
**Category:** Strings
**OCaml Source:** Rosetta Code — ABC words

## Problem Statement

Find all words in a dictionary that contain the letters 'a', 'b', and 'c' in that
order as a subsequence (not necessarily consecutive). For example, "abacus" qualifies
because 'a' appears before 'b', and 'b' appears before 'c'.

## Learning Outcomes

- How Rust's stateful iterators enable elegant subsequence search with zero extra allocation
- Using `Iterator::any()` as a consuming, position-advancing predicate
- The fold-accumulator pattern as an alternative to mutable state
- Slice pattern matching (`[ch, rest @ ..]`) to express OCaml-style recursion

## OCaml Approach

OCaml uses `String.index` and `String.index_from` to find each letter's position
sequentially, piping each found index into the next search. Wrapped in a `try/catch`
for `Not_found`, this is concise but relies on exception-based control flow.

## Rust Approach

Rust's `Iterator::any()` naturally consumes input up to (and including) the first
match, leaving the iterator positioned for the next search. Chaining three `.any()`
calls with `&&` short-circuits cleanly and requires no explicit position tracking,
no exceptions, and no allocation.

## Key Differences

1. **Control flow:** OCaml raises `Not_found` on failure (caught by `try`); Rust
   returns `bool` via short-circuit `&&` — no exceptions needed.
2. **Position tracking:** OCaml threads the found index explicitly through the
   pipeline; Rust's stateful iterator cursor tracks position implicitly.
3. **Allocation:** OCaml's `String.index_from` works on the original string by
   index; Rust's `chars()` iterator is zero-copy — no substring is allocated.
4. **Recursion vs fold:** The recursive Rust version mirrors OCaml's pattern-match
   style; the fold version shows how accumulated state replaces mutable variables.
