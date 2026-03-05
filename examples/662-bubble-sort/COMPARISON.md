# OCaml vs Rust: Bubble Sort

## Implementation

### OCaml
```ocaml
let bubble_sort arr =
  let a = Array.copy arr in
  for i = 0 to n - 1 do
    for j = 0 to n - 2 - i do
      if a.(j) > a.(j + 1) then swap
    done
  done; a
```

### Rust
```rust
fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] { arr.swap(j, j + 1); }
        }
    }
}
```

## Key Difference

- OCaml copies array for immutability
- Rust mutates in place
