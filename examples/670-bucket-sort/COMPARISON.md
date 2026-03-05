# OCaml vs Rust: Bucket Sort

## Algorithm
1. Create n empty buckets
2. Distribute elements into buckets
3. Sort each bucket
4. Concatenate buckets

## Rust
```rust
let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];
for &x in arr.iter() {
    buckets[idx].push(x);
}
```

## OCaml
```ocaml
let buckets = Array.make n [] in
Array.iter (fun x -> buckets.(idx) <- x :: buckets.(idx)) arr
```

## Key Difference
Rust: Vec<Vec<T>> for dynamic buckets
OCaml: Array of lists (cons prepends)
