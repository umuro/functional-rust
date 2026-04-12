📖 **[View on hightechmind.io →](https://hightechmind.io/rust/277-iterator-count)**

---

# 277: Counting Elements with count()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Counting elements — total count, conditional count, count of distinct values — is one of the most frequent operations in data processing. While `len()` is available for sized collections, `count()` works on any iterator, including those from lazy chains of `filter()`, `map()`, and `flat_map()`. It consumes the iterator in a single pass, returning the total number of elements yielded.

## Learning Outcomes

- Understand that `count()` consumes the iterator and returns the number of elements as `usize`
- Use `filter().count()` as the idiomatic way to count elements satisfying a condition
- Recognize that `count()` on sized collections may not call `next()` at all (size hint optimization)
- Combine `count()` with other reductions like `sum()` for multi-statistic computation

## Rust Application

`Iterator::count()` drains the iterator and returns the count. Composed with `filter()`, it counts elements matching a condition in a single pass:

```rust
// Count total
assert_eq!((1..=10).count(), 10);

// Count evens in range
let evens = (1..=10).filter(|x| x % 2 == 0).count();
assert_eq!(evens, 5);

// Count empty
let empty: Vec<i32> = vec![];
assert_eq!(empty.iter().count(), 0);

// Count after transformation
let words = ["hello", "world", "hi"];
let long_words = words.iter().filter(|w| w.len() > 3).count(); // 2
```

## OCaml Approach

OCaml uses `List.length` for full list length, and `List.length (List.filter pred xs)` or `List.fold_left` with a counter for conditional counting:

```ocaml
let count pred xs = List.fold_left (fun acc x -> if pred x then acc + 1 else acc) 0 xs
let evens = count (fun x -> x mod 2 = 0) (List.init 10 (fun i -> i+1))  (* 5 *)
```

The fold-based approach avoids building a filtered list just to count it — matching Rust's single-pass `filter().count()`.

## Key Differences

1. **Efficiency**: Rust's `count()` on `ExactSizeIterator` types may return the known size without iteration; OCaml's `List.length` always traverses the list.
2. **Composability**: `count()` is the terminal operation in a pipeline; OCaml's equivalent requires wrapping in `fold_left` or `filter` + `length`.
3. **No size hint in closures**: Rust respects `size_hint()` for `count()` optimization on known-size iterators but always processes unknown-size ones fully.
4. **Use cases**: Log analysis (count warnings), test coverage (count assertions), statistics (count non-null values).

## Exercises

1. Count the number of words in a string that start with a capital letter using `filter()` and `count()`.
2. Count the frequency of each character in a string using a `HashMap` updated via iteration, without using `count()` directly.
3. Verify that `count()` and `fold(0, |acc, _| acc + 1)` produce identical results on the same iterator.
