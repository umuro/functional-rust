# OCaml vs Rust: Map-Reduce

## OCaml
```ocaml
let map_reduce mapper reducer init items =
  List.fold_left (fun acc x -> reducer acc (mapper x)) init items

let word_count words =
  List.fold_left (fun acc word ->
    let count = try Hashtbl.find acc word with Not_found -> 0 in
    Hashtbl.replace acc word (count + 1); acc
  ) (Hashtbl.create 10) words
```

## Rust
```rust
pub fn map_reduce<T, U, V, M, R>(items: &[T], mapper: M, reducer: R, init: V) -> V
where M: Fn(&T) -> U, R: Fn(V, U) -> V {
    items.iter().map(mapper).fold(init, reducer)
}
```

## Key Differences

1. **OCaml**: fold_left/fold_right for reductions
2. **Rust**: .fold() on iterators, chainable with .map()
3. Both: Closures passed to map and reduce phases
4. **Rust**: HashMap via std::collections
5. Both support building complex aggregations
