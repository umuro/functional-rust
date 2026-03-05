# OCaml vs Rust: Const Fibonacci

## Compile-Time Computation

### Rust
```rust
pub const fn fib_iterative(n: u64) -> u64 {
    let mut a = 0u64;
    let mut b = 1u64;
    let mut i = 1;
    while i < n {
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }
    b
}

// Computed at compile time!
pub const FIB_20: u64 = fib_iterative(20);
```

### OCaml
```ocaml
(* Computed at module initialization *)
let fib_iterative n =
  let rec loop a b i =
    if i >= n then b
    else loop b (a + b) (i + 1)
  in
  if n = 0 then 0 else loop 0 1 1

let fib_20 = fib_iterative 20
```

## Compile-Time Arrays

### Rust
```rust
pub const fn fib_array<const N: usize>() -> [u64; N] {
    let mut arr = [0u64; N];
    // ... fill at compile time
    arr
}

pub const FIB_FIRST_20: [u64; 20] = fib_array();
```

### OCaml
```ocaml
(* Runtime array creation *)
let fib_first_20 = Array.init 20 fib_iterative
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Evaluation time | Module init | Compile time |
| Binary size | Code + runtime | Embedded values |
| Startup cost | Computation | None |
| Verification | Runtime tests | `const { assert!() }` |
