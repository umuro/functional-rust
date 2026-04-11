📖 **[View on hightechmind.io →](https://hightechmind.io/rust/021-insert-at)**

---

# 021 — Insert an Element at a Given Position

## Problem Statement

Inserting an element at position k (OCaml 99 Problems #21) is the inverse of removal — splicing a value into the middle of a sequence. Together with removal, it forms the primitive operations for maintaining ordered sequences, implementing undo/redo buffers, and building persistent data structures.

The problem teaches position-based list manipulation without higher-level abstractions: build the prefix, prepend the new element, then append the suffix. In databases, this is analogous to a B-tree node split. In text editors, every character insertion is a version of this operation.

## Learning Outcomes

- Use `Vec::insert(k, value)` for O(n) in-place insertion
- Construct insertion by combining prefix slice, element, and suffix slice
- Handle the special cases: insert at 0 (prepend), insert at len (append)
- Return a new `Vec` without mutating the input (functional style)
- Implement recursive insertion via count-down accumulator

- Use `Vec::insert(k, elem)` for O(n) in-place insertion that shifts elements right
- Handle boundary cases: k=0 is prepend (insert at front), k=len is append (insert at back)

## Rust Application

The in-place approach: `let mut result = v.to_vec(); result.insert(k, elem); result`. This shifts all elements after k right by one — O(n). The functional approach: `[&v[..k], &[elem], &v[k..]].concat()`. Edge cases: `k=0` is prepend, `k=v.len()` is append. For 1-based indexing (OCaml convention), convert with `k-1`. The recursive approach counts down from k, prepending to an accumulator, then inserting when the counter reaches zero.

## OCaml Approach

OCaml's version: `let insert_at x k lst = let rec aux acc n = function | [] -> if n <= 0 then List.rev (x :: acc) else List.rev acc (* out of bounds — append *) | t when n = 0 -> List.rev_append acc (x :: t) | h :: t -> aux (h :: acc) (n - 1) t in aux [] (k - 1) lst`. The counter starts at `k-1`; when it reaches 0, the element is inserted before the current head.

OCaml's insert: `let rec insert_at x k = function | [] -> if k = 1 then [x] else [] (* or error *) | h :: t -> if k = 1 then x :: h :: t else h :: insert_at x (k-1) t`. This builds a new list — OCaml lists are immutable. The result shares the suffix with the original via structural sharing.

## Key Differences

1. **`Vec::insert` vs list surgery**: Rust's `Vec::insert(k, v)` shifts elements right — O(n). OCaml's list insertion walks to position k and splices — also O(k), same complexity.
2. **Functional vs mutable**: Rust's `Vec::insert` mutates. Return `v.to_vec()` then insert for a functional style that does not modify the input.
3. **Bounds behavior**: Rust panics if k > v.len(). OCaml's implementation can silently append if k > list length — clarify the contract.
4. **`concat` with slices**: Rust's `[&v[..k], &[elem], &v[k..]].concat()` creates a single allocation. OCaml's `List.rev_append acc (x :: t)` avoids one reverse call per step.

1. **`Vec::insert` shifts:** Rust's `v.insert(k, elem)` shifts all elements at index k and beyond one position right — O(n). Returns nothing (mutates in place).
2. **Functional via concat:** The immutable version `[&v[..k], &[elem], &v[k..]].concat()` creates a new `Vec` — O(n). OCaml's recursive version also creates a new list — but shares the suffix via cons-cell reuse.
3. **Edge cases:** Inserting at 0 is prepend; inserting at `len` is append. Both are O(n) for `Vec` but O(1) for linked lists in OCaml.

## Exercises

1. **Insert sorted**: Write `insert_sorted(v: &mut Vec<i32>, x: i32)` that inserts `x` into a sorted `Vec` at the correct position to maintain sort order. Use `v.binary_search(&x).unwrap_or_else(|e| e)` to find the insertion point.
2. **Multi-insert**: Write `insert_many(v: &[i32], insertions: &[(usize, i32)]) -> Vec<i32>` that applies multiple insertions in position order. Sort insertions by position first.
3. **Rope data structure**: Research how a rope (balanced binary tree of strings) represents text as a sequence of insertions, and explain why `Vec::insert` is O(n) while rope insertion is O(log n).

4. **Ordered insert**: Implement `insert_sorted<T: Ord>(v: &mut Vec<T>, elem: T)` that inserts `elem` into an already-sorted `Vec` at the correct position using binary search (`v.binary_search`).
5. **Multi-insert**: Implement `insert_all(v: &[T], elems: &[(usize, T)]) -> Vec<T>` that inserts multiple elements at given positions in one pass, adjusting positions as elements are inserted.
