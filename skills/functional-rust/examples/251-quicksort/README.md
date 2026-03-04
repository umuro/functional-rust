# Example 251: Quicksort

**Difficulty:** ⭐⭐
**Category:** Sorting algorithms

Three implementations of quicksort in Rust: functional (mirroring OCaml's style), in-place recursive (Lomuto partition), and the standard library sort.

## The Problem This Solves

Sorting is the most fundamental operation in computer science. Any time you need ordered data — binary search, rank queries, deduplication, merge joins — you need a sort. Quicksort is the practical baseline: average-case O(n log n), in-place (O(log n) stack space), and cache-friendly due to sequential memory access patterns during partitioning. Understanding quicksort also teaches partitioning, divide-and-conquer, and the difference between *allocating* functional style and *mutating* imperative style.

In Rust, the standard library's `sort` uses a hybrid introsort (quicksort + heapsort + insertion sort), and `sort_unstable` is even faster when stability isn't required. For most production code you'd use `sort_unstable`. But understanding the recursive implementation clarifies the algorithmic structure and the ownership implications of each approach.

## The Intuition

Pick a pivot. Partition all other elements into "less than pivot" and "greater than or equal to pivot". Recursively sort each partition. Concatenate. The average case achieves O(n log n) because a random pivot splits the array roughly in half; worst case (sorted input with first-element pivot) degrades to O(n²). OCaml's functional version creates new lists via `partition` — clean and elegant but O(n log n) *space* as well. Rust's in-place version mutates the array directly, achieving O(log n) space for the recursive call stack.

## How It Works in Rust

```rust
// Functional style: mirrors OCaml exactly — pattern matching on slice
// O(n log n) time, O(n log n) space (allocating)
pub fn quicksort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] => vec![],
        [pivot, rest @ ..] => {
            // partition() returns two new Vecs — allocating, like OCaml
            let (left, right): (Vec<T>, Vec<T>) = rest.iter().cloned().partition(|x| x < pivot);
            let mut result = quicksort(&left);
            result.push(pivot.clone());
            result.extend(quicksort(&right));
            result
        }
    }
}

// In-place recursive: Lomuto partition scheme
// O(n log n) average time, O(log n) stack space
pub fn quicksort_recursive<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 { return; }
    let pivot_idx = partition(data);
    quicksort_recursive(&mut data[..pivot_idx]);      // left partition
    quicksort_recursive(&mut data[pivot_idx + 1..]);  // right partition
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let last = data.len() - 1;
    let mut store = 0;
    for i in 0..last {
        if data[i] <= data[last] {  // data[last] is pivot
            data.swap(i, store);
            store += 1;
        }
    }
    data.swap(store, last);  // place pivot at final position
    store
}

// Standard library: introsort — fastest in practice
pub fn quicksort_inplace<T: Ord>(data: &mut [T]) {
    data.sort_unstable();  // prefer sort_unstable when stability not needed
}
```

The slice pattern `[pivot, rest @ ..]` in the functional version is a Rust 1.42+ feature that matches the first element and the remainder — directly mirroring OCaml's `| pivot :: rest ->` pattern.

## What This Unlocks

- **Understanding ownership in algorithms**: the functional version uses `Clone`; the in-place version uses mutable references to slices — two different ownership strategies for the same algorithm.
- **Slice patterns**: the `[pivot, rest @ ..]` destructuring technique applies broadly to any recursive slice processing, from parsing to signal processing.
- **Sort algorithm selection**: knowing when to use `sort` vs `sort_unstable` vs a custom comparator (`sort_by`, `sort_by_key`) makes your Rust code both correct and fast.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Functional partition | `List.partition (fun x -> x < pivot)` | `rest.iter().cloned().partition(\|x\| x < pivot)` |
| Slice pattern | `\| pivot :: rest ->` | `[pivot, rest @ ..]` |
| In-place mutation | Rare — functional style preferred | `&mut [T]` — idiomatic Rust |
| Pivot selection | First element (common OCaml style) | First or last element (Lomuto) |
| Standard sort | `List.sort compare` | `slice.sort_unstable()` — introsort hybrid |
