# OCaml vs Rust: Fibonacci Memoization vs Tabulation

## Memoization (Top-Down)

### Rust
```rust
pub fn fib_memo(n: u64) -> u64 {
    fn helper(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&result) = cache.get(&n) {
            return result;
        }
        let result = match n {
            0 => 0, 1 => 1,
            _ => helper(n - 1, cache) + helper(n - 2, cache),
        };
        cache.insert(n, result);
        result
    }
    helper(n, &mut HashMap::new())
}
```

### OCaml
```ocaml
let fib_memo n =
  let cache = Hashtbl.create 100 in
  let rec helper n =
    match Hashtbl.find_opt cache n with
    | Some v -> v
    | None ->
        let result = match n with
          | 0 -> 0 | 1 -> 1
          | _ -> helper (n - 1) + helper (n - 2)
        in
        Hashtbl.add cache n result;
        result
  in
  helper n
```

## Tabulation (Bottom-Up)

### Rust
```rust
pub fn fib_optimized(n: u64) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
```

### OCaml
```ocaml
let fib_optimized n =
  let rec loop a b i =
    if i >= n then b
    else loop b (a + b) (i + 1)
  in
  if n = 0 then 0 else loop 0 1 1
```

## Complexity Comparison

| Approach | Time | Space |
|----------|------|-------|
| Naive | O(2^n) | O(n) stack |
| Memoization | O(n) | O(n) |
| Tabulation | O(n) | O(n) |
| Optimized | O(n) | O(1) |
| Matrix | O(log n) | O(1) |
