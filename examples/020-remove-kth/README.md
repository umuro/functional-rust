📖 **[View on hightechmind.io →](https://hightechmind.io/rust/020-remove-kth)**

---

# 020 — Remove the Kth Element
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Removing the element at position k from a list (OCaml 99 Problems #20) is a fundamental list surgery operation — the building block for deletion in arrays, linked lists, and sequences. It produces a new list with one element missing and returns both the removed element and the modified list.

This pattern appears everywhere: removing a player from a queue, deleting a row from a table, extracting the winner from a lottery draw (example 024), and implementing undo operations. The challenge is handling the removed element cleanly — returning it alongside the modified list as a tuple.

## Learning Outcomes

- Return a `(T, Vec<T>)` tuple to simultaneously yield the removed element and the new list
- Use `enumerate().filter()` or explicit split for position-based removal
- Handle out-of-bounds removal gracefully with `Option<(T, Vec<T>)>`
- Implement both the iterative and recursive versions
- Understand k as 1-based (OCaml 99) vs 0-based (Rust convention)

## Rust Application

The safe Rust version returns `Option<(T, Vec<T>)>`: check bounds, then construct the result. Using slice operations: `let elem = v[k].clone(); let result = [&v[..k], &v[k+1..]].concat(); Some((elem, result))`. The in-place version uses `Vec::remove(k)` which shifts elements and returns the removed value. For 1-based indexing (OCaml 99 convention), subtract 1 from the input k.

## OCaml Approach

OCaml's version: `let remove_at k lst = let rec aux acc n = function | [] -> failwith "index out of bounds" | x :: t -> if n = k then (x, List.rev_append acc t) else aux (x :: acc) (n + 1) t in aux [] 1 lst`. `List.rev_append acc t` concatenates the reversed accumulator with the tail — this is O(n) and avoids a separate reverse call.

## Key Differences

1. **`Vec::remove` vs list surgery**: Rust's `Vec::remove(i)` is O(n) due to shifting. OCaml's list removal reconstructs the prefix list, also O(n) but with different constant factors.
2. **Error handling**: Rust's `Vec::remove` panics on out-of-bounds. Return `Option` or check bounds first. OCaml uses `failwith` (raises an exception).
3. **`rev_append`**: OCaml's `List.rev_append acc t` is a key idiom — it combines reversing the accumulator with appending the tail in one pass. Rust achieves the same with `[&v[..k], &v[k+1..]].concat()`.
4. **Indexing**: OCaml 99 Problems uses 1-based indexing. Rust's standard library uses 0-based. Always be explicit about which convention is used.

## Exercises

1. **Remove all of value**: Write `remove_all(v: &[i32], x: i32) -> Vec<i32>` that removes all occurrences of `x` using `.filter()`. Compare with `retain()` (in-place filter).
2. **Remove first of value**: Write `remove_first(v: &[i32], x: i32) -> Option<Vec<i32>>` that removes only the first occurrence. Use `.iter().position()` to find the index.
3. **Remove and reinsert**: Write `move_to_end(v: &mut Vec<i32>, k: usize)` that removes the element at position k and appends it to the end, using `Vec::remove` and `Vec::push`.
