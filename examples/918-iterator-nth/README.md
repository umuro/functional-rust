📖 **[View on hightechmind.io →](https://hightechmind.io/rust/918-iterator-nth)**

---

# 918-iterator-nth — Iterator nth
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Accessing the nth element of an iterator requires consuming all preceding elements — the iterator has no random access. `Iterator::nth(n)` does exactly this: it skips n elements and returns the (n+1)th as `Option<T>`. For slices, `.get(n)` is O(1) and preferred. For filtered or chained iterators, `.nth()` provides controlled positional access without collecting. Understanding nth's consumption semantics — it advances the iterator past n elements — is essential for using it correctly in parsing and protocol implementations.

## Learning Outcomes

- Use `.nth(n)` to access the nth element of an iterator, consuming 0..n
- Understand that `.nth()` advances the iterator — subsequent calls continue from after the consumed position
- Return `None` safely for out-of-bounds access
- Use slice `.get(n)` as the O(1) alternative when available
- Compare with OCaml's `List.nth` which raises on out-of-bounds

## Rust Application

Tests demonstrate: `[10,20,30,40].iter().nth(2)` = `Some(&30)`. Out of bounds: `[1,2].iter().nth(5)` = `None` (safe). Advancing state: `it.nth(1)` consumes indices 0 and 1, returns `Some(&2)`; then `it.nth(0)` returns `Some(&3)` (iterator is now at position 2). `[99].iter().nth(0)` = `Some(&99)`. The advancing-state behavior is useful for protocol parsers that read a header field, then skip reserved bytes, then read payload.

## OCaml Approach

`List.nth: 'a list -> int -> 'a` raises `Not_found` or `Invalid_argument` on out-of-bounds — not safe. `List.nth_opt: 'a list -> int -> 'a option` (since 4.05) is the safe version. Both are O(n). `Array.get: 'a array -> int -> 'a` raises on bounds; `Array.get` with bounds check is O(1). Unlike Rust's `.nth()`, OCaml's does not advance a cursor — each call to `List.nth` starts from the beginning.

## Key Differences

1. **Stateful cursor**: Rust `.nth()` advances the iterator — it is a cursor operation; OCaml `List.nth_opt` always starts from the beginning of the list.
2. **Safety**: Rust returns `Option<T>` for out-of-bounds; OCaml `List.nth` raises — use `List.nth_opt` for safe access.
3. **Efficiency**: Both are O(n) for list/iterator access; slice `.get(n)` is O(1) in Rust, `Array.get arr i` is O(1) in OCaml.
4. **Multiple calls**: Rust `it.nth(0); it.nth(0); it.nth(0)` accesses elements 0, 1, 2 sequentially; OCaml `List.nth_opt xs 0; List.nth_opt xs 1; List.nth_opt xs 2` accesses 0, 1, 2 independently.

## Exercises

1. Use `.nth()` to implement a simple command-line argument parser that reads options at specific positions in a `Vec<String>`.
2. Write `every_third<T: Clone>(data: &[T]) -> Vec<T>` using a loop calling `.nth(2)` repeatedly (advancing by 3 each time).
3. Implement `parse_protocol(bytes: &[u8]) -> Option<(u8, u16, Vec<u8>)>` using `iter.nth()` to read `(version, length, payload)` fields.
