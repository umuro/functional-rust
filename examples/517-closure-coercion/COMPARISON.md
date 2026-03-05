# OCaml vs Rust: Closure/Function Pointer Coercion

## OCaml
```ocaml
(* All functions have the same representation *)
let double x = x * 2
let add_n n x = x + n

(* No distinction between fn pointers and closures *)
let apply f x = f x
let _ = apply double 5
let _ = apply (add_n 3) 5
```

## Rust
```rust
// Non-capturing closure coerces to fn pointer
let f: fn(i32) -> i32 = |x| x * 2;  // OK

// Capturing closure CANNOT coerce to fn pointer
let n = 3;
// let f: fn(i32) -> i32 = |x| x + n;  // ERROR!

// Must use Box<dyn Fn> for capturing closures
let f: Box<dyn Fn(i32) -> i32> = Box::new(move |x| x + n);
```

## Key Differences

1. **OCaml**: Uniform function representation — no distinction
2. **Rust**: fn pointers are thin, closures may carry captured data
3. **Rust**: Non-capturing closures are zero-sized, coerce to fn ptr
4. **Rust**: Capturing closures need Box<dyn Fn> for storage
5. **Rust**: fn pointers required for C FFI interop
