# OCaml vs Rust: Higher-Order Functions

## OCaml
```ocaml
let zip_with f a b = List.map2 f a b
let scan_left f init = List.fold_left (fun (acc, xs) x ->
  let next = f acc x in (next, xs @ [next])) (init, [init])

List.map (fun x -> x * 2) [1; 2; 3]
List.filter (fun x -> x mod 2 = 0) [1; 2; 3; 4]
List.fold_left (+) 0 [1; 2; 3; 4; 5]
```

## Rust
```rust
pub fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where F: Fn(&A, &B) -> C {
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

vec![1, 2, 3].iter().map(|&x| x * 2).collect::<Vec<_>>()
```

## Key Differences

1. **OCaml**: List module with List.map, List.filter, etc.
2. **Rust**: Iterator trait methods: .map(), .filter(), .fold()
3. **OCaml**: Lazy by default in some contexts
4. **Rust**: Iterators are lazy, collect() to materialize
5. Both support chaining multiple HOFs
