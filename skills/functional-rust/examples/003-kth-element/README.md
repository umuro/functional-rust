# 003: K-th Element

**Difficulty:** ⭐  **Level:** Beginner

Access any element by position — safely, with no crashes on out-of-bounds.

## The Problem This Solves

Indexing into a list by position is one of the most common operations in programming. It's also one of the most common sources of runtime crashes: `IndexError` in Python, `ArrayIndexOutOfBoundsException` in Java, silent `undefined` in JavaScript.

Every time you write `arr[i]`, you're betting that `i` is in range. If that bet fails, your program crashes. In security-sensitive code, out-of-bounds reads are a vulnerability. In web servers, they're a 500 error. In scripts, they're a confusing message at 3am.

Rust doesn't let you make that bet silently. `slice.get(i)` returns `Option<&T>` — either the element exists (`Some`) or it doesn't (`None`). The crash is impossible by construction.

## The Intuition

You already know how to do this:

```python
# Python — crashes if i >= len(lst)
element = lst[i]

# Python — safe version, verbose
element = lst[i] if i < len(lst) else None
```

```javascript
// JavaScript — returns undefined for out-of-range (silent bug)
const element = arr[i];
```

Rust's `.get(i)` is the safe version, built in:

```rust
// Rust — always safe, returns Option<&T>
let element = slice.get(i);  // Some(&value) or None
```

One difference to watch: Rust uses **0-based indexing** (like Python and JavaScript). The OCaml original uses 1-based indexing. Both styles are shown in the example — use the one that matches your context.

## How It Works in Rust

```rust
// 0-indexed (idiomatic Rust)
fn at<T>(k: usize, list: &[T]) -> Option<&T> {
    list.get(k)   // returns None instead of panicking
}

// 1-indexed (matching OCaml convention)
fn at_one_indexed<T>(k: usize, list: &[T]) -> Option<&T> {
    if k == 0 {
        None           // 1-indexed has no position 0
    } else {
        list.get(k - 1)
    }
}
```

The `usize` type for the index is Rust's unsigned integer — it can never be negative, so you don't need to guard against that. But it *can* be larger than the list, which `.get()` handles gracefully.

⚠️ **One gotcha:** if you write the 1-indexed version with subtraction and forget the `k == 0` guard, `k - 1` will *underflow* (wrap around to a huge number) since `usize` is unsigned. Always check `k == 0` before subtracting from a `usize`.

```rust
// Using it:
let list = vec![10, 20, 30, 40, 50];

match at(2, &list) {
    Some(x) => println!("Element: {}", x),  // prints 30
    None    => println!("Out of bounds"),
}

// Or with a default:
let x = at(2, &list).copied().unwrap_or(0);  // 30
let y = at(99, &list).copied().unwrap_or(0); // 0 (safe default)
```

## What This Unlocks

- **Safe array access anywhere** — config values, CLI args, command history without panic guards
- **Grid navigation** — `grid.get(row * width + col)` returns `None` at edges, eliminating bounds checks
- **Parsing** — extracting fields from records without crashing on malformed input

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default indexing | 1-based | 0-based |
| Safe access | Pattern match on list | `slice.get(k)` |
| Out-of-bounds | Returns `None` via recursion | Returns `None` immediately |
| Access complexity | O(k) — walks linked list | O(1) — direct memory access |
| Negative index | Not applicable (recursive) | Not possible (`usize` is unsigned) |
