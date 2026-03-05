# OCaml vs Rust: Function Composition

## OCaml
```ocaml
let compose f g x = f (g x)
let ( >> ) g f x = f (g x)  (* pipe operator *)

let pipeline = double >> inc >> square
let result = pipeline 3  (* 49 *)
```

## Rust
```rust
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(B) -> C, G: Fn(A) -> B {
    move |x| f(g(x))
}

let pipeline = compose(square, compose(inc, double));
let result = pipeline(3);  // 49
```

## Key Differences

1. **OCaml**: Custom operators `>>` and `<<` make composition readable
2. **Rust**: Generic compose function with explicit type bounds
3. **OCaml**: Currying makes composition natural
4. **Rust**: Move closures needed to own captured functions
5. Both support building complex transforms from simple pieces
