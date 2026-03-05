# OCaml vs Rust: Partial Application

## OCaml
```ocaml
(* Functions are curried by default *)
let add x y = x + y
let add5 = add 5       (* automatic partial application *)
let clamp lo hi x = max lo (min hi x)
let clamp_0_100 = clamp 0 100
```

## Rust
```rust
fn add(x: i32, y: i32) -> i32 { x + y }
let add5 = |y| add(5, y);  // explicit closure required
let clamp_0_100 = |x| clamp(0, 100, x);
```

## Key Differences

1. **OCaml**: Curried by default — partial application is automatic
2. **Rust**: Requires explicit closure to capture fixed arguments
3. **OCaml**: `let f = g arg1` creates partial
4. **Rust**: `let f = move |rest| g(arg1, rest)` creates partial
5. Both integrate cleanly with map/filter pipelines
