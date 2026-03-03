# OCaml vs Rust: Quicksort

## Side-by-Side Code

### OCaml
```ocaml
let rec quicksort = function
  | [] -> []
  | pivot :: rest ->
    let left, right = List.partition (fun x -> x < pivot) rest in
    quicksort left @ [pivot] @ quicksort right
```

### Rust (functional/recursive — mirrors OCaml)
```rust
pub fn quicksort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] => vec![],
        [pivot, rest @ ..] => {
            let (left, right): (Vec<T>, Vec<T>) =
                rest.iter().cloned().partition(|x| x < pivot);
            let mut result = quicksort(&left);
            result.push(pivot.clone());
            result.extend(quicksort(&right));
            result
        }
    }
}
```

### Rust (in-place recursive — Lomuto partition scheme)
```rust
pub fn quicksort_recursive<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 { return; }
    let pivot_idx = partition(data);
    quicksort_recursive(&mut data[..pivot_idx]);
    quicksort_recursive(&mut data[pivot_idx + 1..]);
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let last = data.len() - 1;
    let mut store = 0;
    for i in 0..last {
        if data[i] <= data[last] {
            data.swap(i, store);
            store += 1;
        }
    }
    data.swap(store, last);
    store
}
```

### Rust (idiomatic — stdlib introsort)
```rust
pub fn quicksort_inplace<T: Ord>(data: &mut [T]) {
    data.sort();
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Functional sort | `val quicksort : 'a list -> 'a list` | `fn quicksort<T: Ord + Clone>(list: &[T]) -> Vec<T>` |
| In-place sort | N/A (immutable lists) | `fn quicksort_recursive<T: Ord>(data: &mut [T])` |
| List/slice type | `'a list` | `&[T]` (slice) or `Vec<T>` |
| Partition result | `'a list * 'a list` | `(Vec<T>, Vec<T>)` |
| Comparability | `'a` with polymorphic `<` | `T: Ord` (explicit trait bound) |

## Key Insights

1. **List head destructuring:** OCaml's `pivot :: rest` maps directly to Rust's `[pivot, rest @ ..]` slice pattern — both bind the first element and the remainder in a single match arm.

2. **`Clone` is explicit:** OCaml's GC lets the runtime share or copy values transparently. Rust requires `T: Clone` in the signature and explicit `.cloned()` / `.clone()` calls, making allocation visible at the type level.

3. **Allocation model:** Every OCaml recursive call produces new list nodes implicitly. The Rust functional version makes the same allocation pattern explicit with `Vec<T>`. The in-place Lomuto variant eliminates per-call allocation entirely by splitting `&mut [T]` borrows.

4. **Borrow splitting for safe recursion:** `&mut data[..pivot_idx]` and `&mut data[pivot_idx + 1..]` are non-overlapping mutable borrows — the borrow checker verifies this statically. OCaml has no equivalent concept; mutation would require `ref` or mutable records.

5. **Production choice:** Rust's `slice::sort` is an introsort (quicksort + heapsort fallback + insertion sort for small slices), giving O(n log n) worst-case. The naïve functional quicksort degrades to O(n²) on already-sorted input when always picking the first element as pivot.

## When to Use Each Style

**Use idiomatic Rust (`data.sort()`) when:** sorting in production code where correctness and performance are paramount — it is cache-friendly, allocation-free, and O(n log n) worst-case.

**Use functional Rust (`quicksort`) when:** teaching the OCaml→Rust translation, demonstrating slice patterns and iterator-based partitioning, or when you need to return a new sorted copy without mutating the input.

**Use in-place recursive Rust (`quicksort_recursive`) when:** studying how Rust's borrow checker enables safe recursive mutation through non-overlapping slice splits, or implementing custom partition strategies.
