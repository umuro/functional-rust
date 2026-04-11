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

- Understand why `Vec::append` is O(|a|) while OCaml's `@` copies only the left list (structural sharing of the right)
- Apply tail-recursive accumulator as the template for stack-safe recursion on large inputs

## Rust Application

The code demonstrates three implementation styles for each operation. `head` and `tail` use `v.first()` and slice indexing, returning `Option` for safety. The recursive `rec_length` and `rec_reverse` mirror OCaml's pattern-matching style directly. The `rev_acc` function introduces the accumulator pattern: instead of building the result on the way back up the call stack, it passes a growing accumulator forward, making the recursion equivalent to a loop.

## OCaml Approach

OCaml's list operations use pattern matching on the `x :: rest` constructor. `let rec length = function [] -> 0 | _ :: t -> 1 + length t` is the canonical recursive form. The tail-recursive version uses an explicit accumulator: `let rec length_aux acc = function [] -> acc | _ :: t -> length_aux (acc + 1) t`. OCaml does guarantee tail-call optimization, so this form is safe for large lists.

OCaml's standard library (`List` module) provides all these operations but they operate on singly-linked lists. `List.rev` uses an accumulator internally: `let rec rev_append l acc = match l with [] -> acc | h :: t -> rev_append t (h :: acc)`. The `List.append` operator `@` is O(|left|) because it must copy the entire left list — appending to the back of a linked list always requires traversal.

## Key Differences

1. **Stack safety**: OCaml guarantees tail-call optimization (TCO); Rust does not. Large recursive functions in Rust require manual conversion to loops or use of `fold`.
2. **Nil representation**: OCaml's `[]` is a built-in list constructor. Rust uses `Vec` (heap-allocated) or slices; there is no built-in singly-linked list in the standard library.
3. **Pattern matching**: OCaml matches `| [] -> ... | x :: t -> ...` directly. Rust uses `v.split_first()` or matches on slice patterns `[head, tail @ ..]`.
4. **Reverse complexity**: Both languages have O(n) reverse with an accumulator. Naive recursive reverse (building the result via append on the way back up) is O(n²) in both.

5. **Mutation:** Rust can reverse a `Vec` in-place with `v.reverse()` — no allocation. OCaml lists are immutable; every "modification" produces a new list. This immutability is what makes OCaml code easy to reason about, but it means garbage collection bears the cost of intermediate structures.
6. **Length caching:** Rust's `Vec` caches its length as a `usize` field, so `.len()` is O(1). OCaml's `List.length` is O(n) — it must walk the entire list.

5. **Structural sharing in OCaml:** OCaml's `@` operator shares the right list — the second argument is not copied, only the left argument is reconstructed. Rust's `extend_from_slice` always copies all elements from both sides.

## Exercises

1. **Last element**: Write `last(v: &[i32]) -> Option<i32>` that returns the last element, implemented both with `.last()` and with tail-recursive decomposition.
2. **Flatten**: Write `flatten(vecs: &[Vec<i32>]) -> Vec<i32>` that concatenates all inner lists into one, using `iter().flatten().collect()` and a manual fold-based version.
3. **Zip**: Write `zip(a: &[i32], b: &[i32]) -> Vec<(i32, i32)>` that pairs elements position-by-position, stopping at the shorter list, using `.zip()` and then recursively.

4. **Interleave**: Write `interleave(a: &[i32], b: &[i32]) -> Vec<i32>` that alternates elements from two lists: `[1,2,3]` + `[a,b,c]` → `[1,a,2,b,3,c]`. Stop at the shorter list.
5. **Take and drop**: Implement `take(n, v: &[i32]) -> Vec<i32>` and `drop(n, v: &[i32]) -> Vec<i32>` using slice patterns and iterators.
