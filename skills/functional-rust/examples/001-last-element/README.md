# 001: Last Element of a List

**Difficulty:** ⭐  **Level:** Beginner

Get the last item from a list — safely, without crashes.

## The Problem This Solves

You have a list of items — log entries, search results, a history of moves in a game — and you want the last one. Every programmer has written this code dozens of times.

In Python you'd write `my_list[-1]`, but that throws an `IndexError` on an empty list. In JavaScript, `arr[arr.length - 1]` returns `undefined` instead of crashing, which is arguably worse — a silent wrong value that causes bugs later.

Rust takes a different approach: the function *tells you* whether it found something. `list.last()` returns `Some(&value)` if the list has elements, and `None` if it's empty. You can't forget to check — the compiler won't let you use the value without handling the empty case.

## The Intuition

Think of `Option<T>` as a box that either contains something (`Some(x)`) or is explicitly empty (`None`). It's like Python's `Optional[T]` type hint, except Rust actually *enforces* it at compile time instead of just documenting intent.

| Language | "Get last item" | Empty list behavior |
|----------|----------------|---------------------|
| Python | `lst[-1]` | Raises `IndexError` |
| JavaScript | `arr.at(-1)` | Returns `undefined` |
| Java | `list.get(list.size()-1)` | Throws `IndexOutOfBoundsException` |
| Rust | `slice.last()` | Returns `None` |

The Rust version can't crash. There's no exception to forget catching.

## How It Works in Rust

```rust
// The idiomatic solution — one line, O(1)
fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}
```

That's it. The `&[T]` parameter means "a slice of any type T" — works with `Vec<i32>`, `&[&str]`, anything. The `&T` in the return means we borrow a reference to the element (no copying).

Three more ways to write it — all equivalent, all correct:

```rust
// Pattern matching on the slice structure
fn last_pattern<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,           // empty list → nothing
        [.., last] => Some(last), // any length → grab the last
    }
}

// Iterator style
fn last_iter<T>(list: &[T]) -> Option<&T> {
    list.iter().last()
}
```

Using the result:

```rust
match last(&[1, 2, 3, 4]) {
    Some(x) => println!("Last item: {}", x),
    None    => println!("List was empty"),
}

// Or with a default:
let x = last(&[1, 2, 3]).unwrap_or(&0);
```

## What This Unlocks

- **Safe API design** — any function that might not have an answer returns `Option`, never panics
- **Processing the last log entry, message, or record** — without defensive `if !empty` guards everywhere
- **Chaining with `.map()`** — `last(&events).map(|e| e.timestamp)` transforms the value only if it exists

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| "Maybe a value" type | `option` / `'a option` | `Option<T>` |
| Empty case | `None` | `None` |
| Present case | `Some x` | `Some(x)` |
| Idiomatic style | Recursive pattern match | `.last()` on slice |
| Data structure | Linked list | Contiguous slice/Vec |
| Recursive safety | TCO guaranteed | No TCO — prefer iterators |
