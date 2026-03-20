📖 **[View on hightechmind.io →](https://hightechmind.io/rust/270-iterator-position)**

---

# 270: Finding Index with position()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Locating the position of an element matching a condition is a fundamental operation: finding where a token appears in a list, locating a delimiter in a byte sequence, or finding the insertion point in a sorted array. Unlike `find()` which returns the element, `position()` returns the zero-based index — essential when the index itself is needed for slicing, bounds computation, or further navigation.

## Learning Outcomes

- Understand that `position(pred)` returns `Option<usize>` — the index of the first matching element
- Distinguish `position()` from `find()`: index vs element value
- Use `rposition()` to find the last matching index from the right (on slices)
- Recognize that `position()` consumes up to the matching element and stops

## Rust Application

`Iterator::position(pred)` short-circuits at the first match, returning `Some(index)`, or `None` if no element satisfies the predicate. The index is zero-based:

```rust
let v = [10i32, 20, 30, 40];
assert_eq!(v.iter().position(|&x| x == 30), Some(2));
assert_eq!(v.iter().position(|&x| x == 99), None);

// rposition: last matching index (only on slice iterators)
let v = [1i32, 2, 3, 2, 1];
assert_eq!(v.iter().rposition(|&x| x == 2), Some(3));
```

The common pattern `position()` → slice with `&arr[..pos]` splits a sequence at a delimiter without copying.

## OCaml Approach

OCaml's `List.find_index` (recent versions) or a manual fold:

```ocaml
let position pred lst =
  let rec go i = function
    | [] -> None
    | x :: xs -> if pred x then Some i else go (i+1) xs
  in go 0 lst
```

For arrays, `Array.find_index` (OCaml 4.14+) returns `Option<int * 'a>` with both index and element.

## Key Differences

1. **Return type**: Rust returns `Option<usize>` (index only); OCaml 4.14's `Array.find_index` returns `Option<int * 'a>` (index and element).
2. **Standard library**: `position()` is built into Rust's `Iterator`; OCaml required manual implementation or recent library additions.
3. **Reverse search**: `rposition()` is available on slice iterators; OCaml requires `List.rev` then `find_index` or a right fold.
4. **Use cases**: Parser delimiter detection, binary search insertion points, finding newlines in byte buffers.

## Exercises

1. Find the index of the first vowel in a string's character iterator using `position()`.
2. Use `position()` to split a string at the first occurrence of `":"` — return `None` if no colon is found.
3. Find all positions of a value in a slice (not just the first) by using `position()` iteratively on a shrinking sub-slice, collecting all indices.
