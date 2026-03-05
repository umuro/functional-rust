# Example 277: Circular Buffer — Functional Queue

**Difficulty:** ⭐⭐
**Category:** Data Structures
**OCaml Source:** https://dev.realworldocaml.org/

## Problem Statement

Implement a functional queue with amortized O(1) enqueue and dequeue operations, using two lists (or vectors). This is the classic "banker's queue" from purely functional data structures.

## Learning Outcomes

- Implementing persistent-style data structures in Rust using ownership transfer
- Amortized analysis: reversing the back list into the front list
- Translating OCaml's immutable record updates (`{ q with ... }`) to Rust's `mut self` pattern
- Using `Option<(T, Self)>` to return both a value and the updated structure

## OCaml Approach

OCaml uses an immutable record with two lists. `enqueue` creates a new record with the element prepended to `back`. `dequeue` pattern-matches on `front`; when empty, reverses `back` into `front`. All operations return new values — the original queue is unchanged.

## Rust Approach

Rust uses `Vec<T>` instead of linked lists. The queue takes ownership of `self` in `enqueue`/`dequeue` (consuming the old queue), which mirrors OCaml's functional semantics while allowing in-place mutation. The `remove(0)` on `front` is O(n) but happens rarely due to the amortized reversal strategy.

## Key Differences

1. **Ownership model:** OCaml returns new immutable records; Rust consumes `self` and returns the modified queue — same semantics, but Rust reuses the allocation
2. **List vs Vec:** OCaml's linked lists have O(1) prepend; Rust's `Vec` has O(1) push but O(n) remove-from-front — a VecDeque would be more efficient but less pedagogical
3. **Pattern matching:** OCaml matches on list constructors (`h :: t`); Rust checks `is_empty()` and uses `remove(0)`
4. **Record update:** OCaml's `{ q with back = x :: q.back }` creates a new record; Rust's `mut self` modifies in place
