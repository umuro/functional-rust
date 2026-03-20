📖 **[View on hightechmind.io →](https://hightechmind.io/rust/900-iterator-chain)**

---

# 900-iterator-chain — Iterator Chain
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Concatenating sequences without allocating a combined container is a common optimization. Iterating over "all items from set A, then all items from set B" should not require copying both into a new buffer. SQL's `UNION ALL`, Haskell's `(++)`, and OCaml's `List.append` all solve this problem — with different allocation behaviors. Rust's `.chain()` adapter concatenates two iterators lazily: no allocation occurs until the combined sequence is consumed. This enables zero-cost concatenation of slices, ranges, filtered views, and any other iterator.

## Learning Outcomes

- Use `.chain()` to concatenate two iterators lazily without allocation
- Chain more than two iterators using consecutive `.chain()` calls
- Combine filtered sub-sequences with chain (evens-then-odds pattern)
- Use `chain` with sum and other consumers without collecting intermediate results
- Compare with OCaml's `List.append` which always allocates

## Rust Application

`chain_slices` uses `first.iter().chain(second.iter()).copied().collect()`. `evens_then_odds` separates even and odd numbers into two filtered iterators then chains them. `chain_three` demonstrates `a.iter().chain(b.iter()).chain(c.iter())` for three-way concatenation. `sum_chained` chains two slices and sums without collecting — fully lazy. The generic `chain_iters<I, J, T>` works over any two iterators yielding the same item type, enabling flexible composition.

## OCaml Approach

`List.append: 'a list -> 'a list -> 'a list` creates a new list by copying the first list's spine. `(@)` is the infix alias. For lazy concatenation, `Seq.append: 'a Seq.t -> 'a Seq.t -> 'a Seq.t` works like Rust's `.chain()`. `List.append` is O(n) where n is the length of the first list; it allocates a new list spine. Chaining multiple lists: `List.concat: 'a list list -> 'a list` flattens a list of lists.

## Key Differences

1. **Allocation**: Rust `.chain()` is zero-allocation until consumed; OCaml `List.append` allocates the first list's spine immediately.
2. **Laziness**: Rust chain is lazy; `Seq.append` is also lazy; `List.append` is eager.
3. **Type requirements**: Both ends of `.chain()` must yield the same `Item` type; OCaml `List.append` requires the same list element type.
4. **Sum without collect**: Rust `chain().sum()` avoids materializing the combined list; OCaml requires `List.append` then `List.fold_left (+) 0`.

## Exercises

1. Write `interleave<T: Copy>(a: &[T], b: &[T]) -> Vec<T>` using `zip` and `chain` that produces `[a0, b0, a1, b1, ...]`.
2. Implement `unique_chain(a: &[i32], b: &[i32]) -> Vec<i32>` using chain followed by a deduplication pass.
3. Use `chain` to implement a priority iterator that yields all high-priority items first, then normal-priority items.
