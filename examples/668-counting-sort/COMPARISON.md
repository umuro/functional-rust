# OCaml vs Rust: Counting Sort

## Rust
```rust
fn counting_sort(arr: &mut [usize]) {
    let max = *arr.iter().max().unwrap();
    let mut count = vec![0; max + 1];
    for &x in arr.iter() { count[x] += 1; }
    // reconstruct
}
```

## OCaml
```ocaml
let counting_sort arr =
  let max_val = Array.fold_left max arr.(0) arr in
  let count = Array.make (max_val + 1) 0 in
  Array.iter (fun x -> count.(x) <- count.(x) + 1) arr
```

## Key Insight
Both use similar imperative approach - counting sort is naturally imperative.
