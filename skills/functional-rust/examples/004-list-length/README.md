# 004: List Length

**Difficulty:** ⭐  **Level:** Beginner

Count how many elements are in a list — and understand why this is interesting in Rust.

## The Problem This Solves

`len()` seems trivial. Every language has it. But *how* it works reveals a lot about the language's data model — and explains why Rust's iterators and slices are designed the way they are.

In Python and JavaScript, `len(arr)` and `arr.length` are O(1) because arrays store their length. In OCaml, linked lists don't store their length — you have to *walk the whole list* to count it, which is O(n). That's why tail-recursive list-length is a classic OCaml exercise: you need to be careful not to blow the stack.

In Rust, slices and `Vec` store their length, so `.len()` is always O(1) and completely safe. No stack concerns, no recursion needed. The exercise becomes: understand *why* that's the case, and learn the fold pattern for when you do need to accumulate over a collection.

## The Intuition

Think of a Rust `Vec<T>` like Python's `list`: it's a contiguous block of memory with a stored length. Asking for its size is just reading a number, not counting.

A Rust slice `&[T]` is even simpler: it's a pointer + a length. The length is *part of the value itself* — it travels with the data. So `.len()` is literally just returning a field.

```python
# Python — O(1), length stored
len(my_list)

# JavaScript — O(1), length stored  
arr.length

# OCaml — O(n)! walks the linked list
List.length lst  (* traverses every element *)

# Rust — O(1), length stored with the slice
slice.len()
```

## How It Works in Rust

```rust
// The idiomatic way — O(1), use this always
fn length<T>(list: &[T]) -> usize {
    list.len()
}

// Iterator version — also O(1) for slices (ExactSizeIterator)
fn length_iter<T>(list: &[T]) -> usize {
    list.iter().count()
}
```

For learning purposes, here's the recursive version (mirrors OCaml's naive approach):

```rust
// Educational only — don't use for large lists (no tail-call optimization!)
fn length_recursive<T>(list: &[T]) -> usize {
    match list {
        [] => 0,
        [_, rest @ ..] => 1 + length_recursive(rest),
    }
}
```

⚠️ **Important:** Rust does **not** guarantee tail-call optimization. The naive recursive version above will stack-overflow on large lists, just like OCaml's naive version. In OCaml, you'd use the tail-recursive accumulator pattern. In Rust, just use `.len()` — the problem is already solved.

If you *do* need to fold over a collection to compute a value:

```rust
// The functional accumulator pattern in Rust
let sum: i32 = vec![1, 2, 3, 4].iter().fold(0, |acc, x| acc + x);
```

## What This Unlocks

- **Bounds checking before indexing** — `if i < list.len() { list[i] }` (though `.get(i)` is cleaner)
- **Splitting work evenly** — `list.len() / num_threads` for parallelism
- **Validating input sizes** — "this record must have exactly 8 fields"

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `length` complexity | O(n) — walks linked list | O(1) — stored with slice |
| Stack-safe recursion | TCO guaranteed for tail calls | No TCO — use iterators |
| Accumulator pattern | `aux n list` with inner recursive fn | `.fold(0, \|acc, _\| acc + 1)` |
| Type of length | `int` (signed) | `usize` (unsigned, pointer-sized) |
| Large lists | Tail-recursive version required | `.len()` always safe |
