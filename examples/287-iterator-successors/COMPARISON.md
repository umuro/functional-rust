# OCaml vs Rust: successors()

## Pattern 1: Powers of 2

### OCaml
```ocaml
let successors first f =
  Seq.unfold (fun state ->
    match state with
    | None -> None
    | Some x -> Some (x, f x)
  ) (Some first)

let powers_of_2 = successors 1 (fun x -> 
  if x < 256 then Some (x * 2) else None)
```

### Rust
```rust
let powers_of_2: Vec<u32> = std::iter::successors(Some(1u32), |&n| {
    if n < 512 { Some(n * 2) } else { None }
}).collect();
```

## Pattern 2: Collatz Sequence

### OCaml
```ocaml
let collatz n =
  successors n (fun x ->
    if x = 1 then None
    else if x mod 2 = 0 then Some (x / 2)
    else Some (3 * x + 1))
```

### Rust
```rust
let collatz: Vec<u64> = std::iter::successors(Some(6u64), |&n| {
    if n == 1 { None }
    else if n % 2 == 0 { Some(n / 2) }
    else { Some(3 * n + 1) }
}).collect();
```

## Pattern 3: Newton's Method

### Rust
```rust
let sqrt2: Vec<f64> = std::iter::successors(Some(1.0f64), |&x| {
    let next = 0.5 * (x + 2.0 / x);
    if (next - x).abs() < 1e-10 { None } else { Some(next) }
}).collect();
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Builtin | `Seq.unfold` (manual) | `std::iter::successors` |
| First element | Part of unfold state | `Option<T>` parameter |
| Closure receives | Value (via seed) | `&T` reference |
| Termination | Return `None` state | Return `None` |
| Infinite | Return `Some` always | Same, use `.take(n)` |
