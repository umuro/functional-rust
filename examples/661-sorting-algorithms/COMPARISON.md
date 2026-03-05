# OCaml vs Rust: Sorting Algorithms

## Quick Sort

### OCaml (functional)
```ocaml
let rec quick_sort = function
  | [] -> []
  | pivot :: rest ->
      let left = List.filter (fun x -> x < pivot) rest in
      let right = List.filter (fun x -> x >= pivot) rest in
      quick_sort left @ [pivot] @ quick_sort right
```

### Rust (in-place)
```rust
fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Style | Functional, immutable | Imperative, in-place |
| Memory | Allocates new lists | Modifies in place |
| Readability | More concise | More explicit |
