# OCaml vs Rust: Merge Sort

## OCaml
```ocaml
let rec merge = function
  | [], ys -> ys
  | xs, [] -> xs
  | x :: xs, y :: ys -> if x <= y then x :: merge (xs, y::ys) ...

let rec merge_sort = function
  | [] | [_] as lst -> lst
  | lst -> let (l, r) = split lst in merge (merge_sort l, merge_sort r)
```

## Rust
```rust
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    // merge...
}
```

## Key Difference
OCaml: Natural for linked lists, pure functional
Rust: Array-based, in-place with temporary allocation
