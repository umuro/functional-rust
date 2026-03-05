# OCaml vs Rust: Quick Sort

## OCaml
```ocaml
let rec quick_sort = function
  | [] -> []
  | pivot :: rest ->
      quick_sort (filter (< pivot) rest) @ [pivot] @ quick_sort (filter (>= pivot) rest)
```

## Rust
```rust
fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}
```

## Key Difference
OCaml: Elegant but allocates O(n) per level
Rust: In-place with O(log n) stack space
