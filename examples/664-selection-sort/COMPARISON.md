# OCaml vs Rust: Selection Sort

## OCaml
```ocaml
let rec selection_sort = function
  | [] -> []
  | lst -> let (min, rest) = find_min lst in min :: selection_sort rest
```

## Rust
```rust
fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let min_idx = (i..arr.len()).min_by_key(|&j| &arr[j]).unwrap();
        arr.swap(i, min_idx);
    }
}
```

## Key Difference
Rust: O(n) swaps (optimal)
OCaml: Creates new lists (allocates)
