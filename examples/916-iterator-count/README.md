📖 **[View on hightechmind.io →](https://hightechmind.io/rust/916-iterator-count)**

---

# 916-iterator-count — Iterator Count
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Counting how many elements satisfy a condition is a fundamental operation: how many words are longer than 5 characters, how many numbers are prime, how many transactions exceeded a threshold. Rust's `Iterator::count()` consumes the iterator and returns the number of elements. Combined with `.filter()`, it counts matching elements. For slices, `.len()` is O(1) and preferred; for filtered or transformed iterators, `.count()` is the correct consumer. OCaml uses `List.length (List.filter pred xs)` — two passes — while Rust's `.filter().count()` is a single pass.

## Learning Outcomes

- Use `.count()` to consume an iterator and return the element count
- Combine `.filter().count()` for conditional counting in one pass
- Understand when `.len()` (O(1) for slices) is preferred over `.count()`
- Use `count()` after transformations that change element count (flat_map, filter)
- Compare with OCaml's `List.length (List.filter ...)` two-pass approach

## Rust Application

The tests demonstrate: `[1,2,3,4,5].iter().count()` = 5. Filter count: `(1..=10).filter(|x| x % 2 == 0).count()` = 5. After flat_map: `[[1,2],[3],[4,5,6]].iter().flatten().count()` = 6. Count with take_while: `(1..).take_while(|&x| x * x < 100).count()` = 9. The key distinction: `.len()` is O(1) on slices and arrays; `.count()` is O(n) for any iterator (it must traverse all elements).

## OCaml Approach

`List.length: 'a list -> int` counts all elements. `List.filter` then `List.length` for conditional count: `List.length (List.filter (fun x -> x mod 2 = 0) xs)` — two passes. `List.fold_left (fun acc x -> if pred x then acc + 1 else acc) 0 xs` — single pass. `Array.fold_left` similarly. Standard OCaml lacks a `List.count` function — it must be expressed as filter+length or fold.

## Key Differences

1. **Single pass**: Rust `.filter().count()` is a single traversal; OCaml `List.filter + List.length` is two passes over the data.
2. **len vs count**: Rust distinguishes O(1) `.len()` (slice) from O(n) `.count()` (iterator); OCaml `List.length` is always O(n).
3. **No standard count**: OCaml standard library has no `List.count pred xs`; Rust's iterator method is universally available.
4. **Infinite safety**: Rust `.take_while().count()` safely bounds counting over infinite ranges; OCaml requires explicit `Seq.take_while`.

## Exercises

1. Implement `count_by_key<T, K: Eq + Hash>(data: &[T], key: impl Fn(&T) -> K) -> HashMap<K, usize>` using a single pass over the data.
2. Write `count_runs(data: &[i32]) -> usize` that counts the number of runs of consecutive equal elements using windows.
3. Find the length of the longest run of consecutive primes in the first 1000 natural numbers using count with take_while.
