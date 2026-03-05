# OCaml vs Rust: Currying

## OCaml
```ocaml
(* All functions are curried by default *)
let add x y = x + y
let add5 = add 5  (* automatic partial application *)

let curry f x y = f (x, y)    (* tuple to curried *)
let uncurry f (x, y) = f x y  (* curried to tuple *)
```

## Rust
```rust
// Explicit currying via nested closures
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let add5 = add(5);

fn curry<A, B, C, F>(f: F) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
```

## Key Differences

1. **OCaml**: All multi-arg functions are curried by default
2. **Rust**: Must explicitly return closures for currying
3. **OCaml**: `let f x y = ...` is automatically `f: a -> b -> c`
4. **Rust**: Need `Box<dyn Fn>` for nested impl Fn returns
5. Both support curry/uncurry conversion utilities
