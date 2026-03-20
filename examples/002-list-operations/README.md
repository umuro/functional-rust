📖 **[View on hightechmind.io →](https://hightechmind.io/rust/002-list-operations)**

---

# 002 — List Operations

## Problem Statement

Linked lists are the canonical data structure of functional programming. In Haskell, OCaml, and Lisp, the list is the primary collection type and virtually all standard library functions operate on it. Even in languages where arrays are preferred at runtime, understanding the core list operations — `head`, `tail`, `length`, `append`, `reverse` — gives insight into structural recursion and the accumulator pattern.

The recursive structure of lists (an element prepended to another list) maps directly to recursive function definitions. This correspondence between data shape and function shape is the essence of pattern matching on algebraic data types, used extensively in compilers, interpreters, and proof assistants.

## Learning Outcomes

- Understand head/tail decomposition as the basis for structural recursion
- Implement list operations iteratively, recursively, and with tail-recursive accumulators
- Recognize the performance implications of naive recursion vs accumulator style
- Use `Vec` methods that correspond to functional list operations
- Understand why `rev_acc` (reverse with accumulator) is O(n) while naive reverse is O(n²)

## Rust Application

The code demonstrates three implementation styles for each operation. `head` and `tail` use `v.first()` and slice indexing, returning `Option` for safety. The recursive `rec_length` and `rec_reverse` mirror OCaml's pattern-matching style directly. The `rev_acc` function introduces the accumulator pattern: instead of building the result on the way back up the call stack, it passes a growing accumulator forward, making the recursion equivalent to a loop.

## OCaml Approach

OCaml's list operations use pattern matching on the `x :: rest` constructor. `let rec length = function [] -> 0 | _ :: t -> 1 + length t` is the canonical recursive form. The tail-recursive version uses an explicit accumulator: `let rec length_aux acc = function [] -> acc | _ :: t -> length_aux (acc + 1) t`. OCaml does guarantee tail-call optimization, so this form is safe for large lists.

## Key Differences

1. **Stack safety**: OCaml guarantees tail-call optimization (TCO); Rust does not. Large recursive functions in Rust require manual conversion to loops or use of `fold`.
2. **Nil representation**: OCaml's `[]` is a built-in list constructor. Rust uses `Vec` (heap-allocated) or slices; there is no built-in singly-linked list in the standard library.
3. **Pattern matching**: OCaml matches `| [] -> ... | x :: t -> ...` directly. Rust uses `v.split_first()` or matches on slice patterns `[head, tail @ ..]`.
4. **Reverse complexity**: Both languages have O(n) reverse with an accumulator. Naive recursive reverse (building the result via append on the way back up) is O(n²) in both.

## Exercises

1. **Last element**: Write `last(v: &[i32]) -> Option<i32>` that returns the last element, implemented both with `.last()` and with tail-recursive decomposition.
2. **Flatten**: Write `flatten(vecs: &[Vec<i32>]) -> Vec<i32>` that concatenates all inner lists into one, using `iter().flatten().collect()` and a manual fold-based version.
3. **Zip**: Write `zip(a: &[i32], b: &[i32]) -> Vec<(i32, i32)>` that pairs elements position-by-position, stopping at the shorter list, using `.zip()` and then recursively.
