# Example 001: Last Element of a List

**Difficulty:** ⭐ Beginner  
**Category:** Lists & Higher-Order Functions  
**OCaml Source:** 99 Problems #1  

## Problem Statement

Find the last element of a list.

## Learning Outcomes

- Pattern matching on lists
- Recursive thinking
- Handling edge cases (empty list)
- Multiple solution approaches

## OCaml Approach

OCaml uses pattern matching to destructure lists. The standard library provides `List.rev` + `List.hd`, but the idiomatic solution is recursive pattern matching.

## Rust Approach

Rust provides `.last()` method on slices for O(1) access. For functional approach, we can use pattern matching with slice patterns or recursion.

## Key Differences

1. **Ownership:** Rust returns `Option<&T>` (reference), OCaml returns `'a option` (value)
2. **Performance:** Rust O(1) with `.last()`, OCaml O(n) with recursive solution
3. **Safety:** Both handle empty list safely via Option type
4. **Ergonomics:** Rust's slice indexing is simpler for this case

## Related Concepts

- Pattern matching
- Option/Maybe type
- List traversal
- Tail recursion (see example.ml)
