# OCaml vs Rust: unzip and partition

## Pattern 1: Partition by Predicate

### OCaml
```ocaml
let nums = [-3; 0; 1; -1; 0; 5] in
let (neg, non_neg) = List.partition (fun x -> x < 0) nums
```

### Rust
```rust
let nums = vec![-3, 0, 1, -1, 0, 5];
let (neg, non_neg): (Vec<i32>, Vec<i32>) = 
    nums.into_iter().partition(|&x| x < 0);
```

## Pattern 2: Unzip Pairs

### OCaml
```ocaml
let pairs = [(1, 'a'); (2, 'b'); (3, 'c')] in
let (nums, chars) = List.split pairs
```

### Rust
```rust
let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
```

## Pattern 3: Multi-way Split with Fold

### OCaml
```ocaml
type ('a, 'b) either = Left of 'a | Right of 'b
let partition_map f lst =
  List.fold_left (fun (ls, rs) x ->
    match f x with
    | Left l -> (l :: ls, rs)
    | Right r -> (ls, r :: rs)
  ) ([], []) lst |> fun (ls, rs) -> (List.rev ls, List.rev rs)
```

### Rust
```rust
let (nums, words) = data.iter().fold(
    (Vec::new(), Vec::new()),
    |(mut ns, mut ws), s| {
        match s.parse::<i32>() {
            Ok(n) => ns.push(n),
            Err(_) => ws.push(s),
        }
        (ns, ws)
    }
);
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Partition | `List.partition` | `.partition()` |
| Unzip | `List.split` | `.unzip()` |
| N-way split | Nested `partition` or fold | `fold` with tuple accumulator |
| Single-pass | Depends on laziness | Guaranteed |
| Type hint | Inferred | Often needed for collect |
