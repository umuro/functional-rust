**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[merge-sort on hightechmind.io](https://hightechmind.io/posts/functional-rust/merge-sort)

---

## Problem Statement

Implement functional merge sort in Rust: recursively split a slice in half, sort each half, then merge two sorted slices. Implement two `merge` variants — one with explicit index cursors and one using peekable iterators — to demonstrate the functional style. Compare with OCaml's list-based merge sort that uses pattern matching.

## Learning Outcomes

- Implement recursive `merge_sort<T: Ord + Clone>(list: &[T]) -> Vec<T>` using slice splitting
- Implement `merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T>` with two-pointer merge
- Implement `merge_iter` using `Peekable` iterators for a more functional-style merge
- Understand that `Clone` is required because merge sort produces new `Vec` allocations at each level
- Connect to OCaml's pattern-matched list merge sort: `[] | [_]` base case, `List.init` split

## Rust Application

```rust
pub fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] { result.push(left[i].clone()); i += 1; }
        else { result.push(right[j].clone()); j += 1; }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

pub fn merge_sort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    if list.len() <= 1 { return list.to_vec(); }
    let mid = list.len() / 2;
    let left  = merge_sort(&list[..mid]);
    let right = merge_sort(&list[mid..]);
    merge(&left, &right)
}

// Peekable iterator version
pub fn merge_iter<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut li = left.iter().peekable();
    let mut ri = right.iter().peekable();
    loop {
        match (li.peek(), ri.peek()) {
            (Some(l), Some(r)) => {
                if l <= r { result.push((*li.next().unwrap()).clone()); }
                else { result.push((*ri.next().unwrap()).clone()); }
            }
            (Some(_), None) => { result.extend(li.cloned()); break; }
            (None, Some(_)) => { result.extend(ri.cloned()); break; }
            (None, None) => break,
        }
    }
    result
}
```

`Vec::with_capacity` pre-allocates the output to avoid reallocations during merge. `extend_from_slice` copies the tail of whichever side is exhausted first. The `T: Clone` bound is unavoidable because merge sort works on slices (borrowed) and must produce new owned `Vec` values at each recursive level.

The peekable variant uses `match (left.peek(), right.peek())` — a four-case match that covers all combinations without index arithmetic.

## OCaml Approach

```ocaml
let rec merge xs ys = match xs, ys with
  | [], ys -> ys
  | xs, [] -> xs
  | x :: xs', y :: ys' ->
    if x <= y then x :: merge xs' ys
    else y :: merge xs ys'

let split xs =
  let n = List.length xs in
  let k = n / 2 in
  (List.filteri (fun i _ -> i < k) xs,
   List.filteri (fun i _ -> i >= k) xs)

let rec merge_sort = function
  | [] | [_] as xs -> xs
  | xs ->
    let (left, right) = split xs in
    merge (merge_sort left) (merge_sort right)
```

OCaml's pattern match on `(xs, ys)` pairs makes the merge function read as three cases: one side empty, other side empty, or both non-empty. The recursive `merge` in OCaml consumes the lists directly — no index tracking needed.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Input type | `&[T]` slice — zero-copy split at mid | `'a list` — must copy for split |
| Clone requirement | `T: Clone` — explicit | GC handles sharing transparently |
| Base case | `list.len() <= 1` | Pattern match `[] \| [_]` |
| Merge style | Index cursor or `Peekable` | Pattern match on pairs |
| Memory | One `Vec` per merge step | One list cons per element |

Merge sort is naturally recursive and allocation-heavy. For production use, Rust's `slice::sort` (pdqsort) is significantly faster. Functional merge sort here is for algorithmic clarity.

## Exercises

1. Implement `merge_sort_by<T, F: Fn(&T, &T) -> Ordering>(list, cmp)` with a custom comparator.
2. Make `merge_sort` work on `Vec<T>` in-place with O(n log n) time and O(n) auxiliary space.
3. Implement a `natural_merge_sort` that detects existing sorted runs and merges them (Timsort inspiration).
4. Benchmark `merge_sort` vs `slice::sort` on a 10,000-element `Vec<i32>`.
5. Implement `merge_k_sorted(lists: Vec<Vec<T>>)` that merges k sorted lists in O(n log k) using a `BinaryHeap`.
