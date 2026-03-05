# OCaml vs Rust: collect()

## Pattern 1: Collect to List/Vec

### OCaml
```ocaml
let nums = List.init 5 (fun i -> i * i)
(* List is the natural collection type *)
```

### Rust
```rust
let squares: Vec<u32> = (0..5).map(|x| x * x).collect();
// Type annotation tells collect() what to produce
```

## Pattern 2: Collect to Set

### OCaml
```ocaml
module StringSet = Set.Make(String)
let words = ["apple"; "banana"; "apple"; "cherry"]
let set = List.fold_left (fun s w -> StringSet.add w s) 
          StringSet.empty words
```

### Rust
```rust
let words = vec!["apple", "banana", "apple", "cherry"];
let set: HashSet<&str> = words.into_iter().collect();
// Automatically deduplicates
```

## Pattern 3: Fallible Collection

### Rust
```rust
// Collect Result<T> into Result<Vec<T>>
let nums: Result<Vec<i32>, _> = ["1", "2", "3"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();
// Ok([1, 2, 3]) or Err on first failure
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Default | `List` is natural | Must specify type |
| To set | Manual `fold_left` | `.collect::<HashSet<_>>()` |
| To map | Manual `fold_left` | `.collect::<HashMap<K,V>>()` |
| Type inference | From context | Annotation or turbofish |
| Fallible | Manual error handling | `.collect::<Result<Vec<_>,_>>()` |
