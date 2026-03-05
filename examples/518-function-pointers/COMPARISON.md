# OCaml vs Rust: Function Pointers

## OCaml
```ocaml
(* No distinction — all functions are uniform *)
let square x = x * x
let apply f x = f x
let _ = apply square 5
```

## Rust
```rust
fn square(x: i32) -> i32 { x * x }

// fn pointer: thin, no captured data
fn apply_fn_ptr(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }

// Generic Fn: works with closures too
fn apply_generic<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
```

## Key Differences

1. **OCaml**: Uniform function representation
2. **Rust**: fn pointers are thin (1 pointer), closures may carry data
3. **Rust**: Non-capturing closures are zero-sized
4. **Rust**: fn pointers needed for C FFI
5. Both support higher-order functions
