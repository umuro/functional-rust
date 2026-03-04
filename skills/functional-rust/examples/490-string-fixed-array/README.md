# 490: Fixed-Size String Arrays

**Difficulty:** 1  **Level:** Beginner

Store and work with collections of strings — both fixed-size arrays and growable vectors.

## The Problem This Solves

When you have a known set of strings — days of the week, command names, error messages — you don't need a heap-allocated dynamic collection. A fixed-size array `[&str; 7]` lives on the stack, costs nothing to allocate, and conveys your intent: this list doesn't grow.

But often you need to sort, filter, or search string collections. Whether you use `[&str; N]`, `Vec<&str>`, or `Vec<String>` depends on ownership: do you own the strings or just borrow them?

This example shows the idiomatic patterns: when to use each type, how to sort case-insensitively, how to filter, and how to join a collection back into a single string.

## The Intuition

Three string collection types:
- `[&str; N]` — fixed-size, stack-allocated, borrowed — great for static data
- `Vec<&str>` — growable, borrowed — when you slice from owned data
- `Vec<String>` — growable, owned — when you produce or mutate strings

The key operation is slicing: both arrays and Vecs deref to `&[T]`, so all slice methods work on both.

## How It Works in Rust

```rust
// Fixed array — stack allocated, no heap
let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

// Slice operations work on both arrays and Vecs
println!("{}", days.contains(&"Wed")); // true

// Sort a Vec (arrays can't be sorted in-place while borrowed)
let mut words = vec!["banana", "apple", "cherry"];
words.sort(); // lexicographic

// Case-insensitive sort
words.sort_by_key(|s| s.to_lowercase());

// Filter: retain keeps elements matching a predicate
let mut items: Vec<&str> = vec!["rust", "ruby", "go", "python"];
items.retain(|s| s.starts_with('r'));
// items == ["rust", "ruby"]

// Join: collect a slice into a single string
let joined = words.join(", "); // "apple, banana, cherry"

// Search
let pos = words.iter().position(|&s| s == "banana");
```

## What This Unlocks

- Build lookup tables and menus as zero-cost `[&str; N]` arrays
- Sort and filter lists of labels, tags, or command names idiomatically
- Join display strings with `join()` — replacing manual string concatenation loops

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fixed string array | `let a = [| "a"; "b" |]` | `let a: [&str; 2] = ["a", "b"]` |
| Sort | `Array.sort compare a` | `vec.sort()` (in-place) |
| Filter | `Array.to_list a \|> List.filter f` | `vec.retain(\|s\| pred(s))` |
| Join | `String.concat sep list` | `slice.join(sep)` |
| Contains | `Array.mem x a` | `slice.contains(&x)` |
