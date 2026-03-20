📖 **[View on hightechmind.io →](https://hightechmind.io/rust/016-drop-every-nth)**

---

# 016 — Drop Every Nth Element
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Dropping every nth element from a list (OCaml 99 Problems #16) — keeping all elements except those at positions 0, n-1, 2n-1, ... (1-indexed: drop positions n, 2n, 3n, ...) — is a simple but instructive filtering problem. It requires maintaining a counter alongside the data, which is the job of `enumerate`.

This pattern appears in subsampling (audio downsampling, video frame dropping), RAID stripe removal, round-robin scheduling (skip every nth slot), and data thinning. The `enumerate().filter().map()` chain is idiomatic Rust for position-based filtering.

## Learning Outcomes

- Use `.enumerate()` to pair each element with its 1-based index
- Filter by position using `(index + 1) % n != 0`
- Understand 1-indexed vs 0-indexed counting and why the off-by-one matters
- Compare iterative and recursive implementations
- Recognize position-based filtering as a common pattern for subsampling

## Rust Application

The idiomatic Rust approach uses `.iter().enumerate().filter(|(i, _)| (i + 1) % n != 0).map(|(_, x)| x.clone()).collect()`. The `enumerate()` adds a 0-based index, so `(i + 1) % n != 0` skips every position that is a multiple of n in 1-based counting. A loop-based version maintains an explicit counter, incrementing and resetting at n. The recursive version passes the counter as an accumulator argument.

## OCaml Approach

OCaml's version: `let drop lst n = let rec aux acc count = function | [] -> List.rev acc | x :: t -> if count = n then aux acc 1 t else aux (x :: acc) (count + 1) t in aux [] 1 lst`. The counter starts at 1; when it reaches n, the element is skipped and the counter resets to 1. This is the idiomatic recursive accumulator pattern.

## Key Differences

1. **`enumerate` vs counter**: Rust's `.enumerate()` is the standard way to add position information. OCaml threads a counter through recursive calls as a function argument.
2. **1-indexed convention**: OCaml 99 Problems uses 1-based indexing (drop positions 1, n+1, 2n+1, ...). Be careful: the implementation must match this convention.
3. **Filter vs skip**: Rust's `filter` is declarative — express the condition for keeping elements. OCaml's recursive approach is imperative in spirit (check counter, decide, recurse).
4. **Output order**: Both approaches preserve the order of kept elements. The accumulator-based OCaml version builds in reverse and must `List.rev` at the end.

## Exercises

1. **Keep every nth**: Write the complement — `keep_every_nth(list: &[i32], n: usize) -> Vec<i32>` that keeps only elements at positions n, 2n, 3n, ... This is useful for downsampling.
2. **Drop ranges**: Write `drop_range(list: &[i32], start: usize, end: usize) -> Vec<i32>` that removes elements from index `start` to `end` inclusive.
3. **Stride iterator**: Implement a custom `Stride<I>` iterator adapter that yields every nth element of the underlying iterator without collecting to a `Vec`.
