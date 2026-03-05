# OCaml vs Rust: Iterator from_fn

## Pattern 1: Counter with Mutable State

### OCaml
```ocaml
let counter =
  let n = ref 0 in
  Seq.unfold (fun () ->
    if !n >= 5 then None
    else begin incr n; Some (!n, ()) end
  ) ()
```

### Rust
```rust
let mut n = 0i32;
let counter = std::iter::from_fn(move || {
    n += 1;
    if n <= 5 { Some(n) } else { None }
});
```

## Pattern 2: Fibonacci Sequence

### OCaml
```ocaml
let fib =
  let a = ref 0 and b = ref 1 in
  Seq.forever (fun () ->
    let v = !a in
    let next = !a + !b in
    a := !b; b := next; v
  )
```

### Rust
```rust
let fib = {
    let (mut a, mut b) = (0u64, 1u64);
    std::iter::from_fn(move || {
        let val = a;
        let next = a + b;
        a = b;
        b = next;
        Some(val)
    })
};
```

## Pattern 3: Parsing Tokens

### Rust
```rust
let mut words = input.split_whitespace();
let numbers = std::iter::from_fn(|| {
    words.next().and_then(|w| w.parse().ok())
});
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Constructor | `Seq.unfold` / `Seq.of_dispenser` | `std::iter::from_fn` |
| State | `ref` cells in closure | `move` closure captures |
| Termination | Return `None` | Return `None` |
| Infinite | `Seq.forever` | Always return `Some`, use `.take(n)` |
| Use case | General unfold pattern | Stateful generator without struct |
