# OCaml vs Rust: Recursive Closures

## OCaml
```ocaml
(* let rec makes recursion natural *)
let rec factorial n =
  if n <= 1 then 1 else n * factorial (n - 1)

(* Y combinator for fun *)
let y f = (fun x -> f (fun v -> x x v))
          (fun x -> f (fun v -> x x v))
```

## Rust
```rust
// Named recursion is straightforward
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

// Y combinator needs boxing for type recursion
struct Y<A, B>(Box<dyn Fn(&Y<A, B>, A) -> B>);
```

## Key Differences

1. **OCaml**: `let rec` enables natural recursion
2. **Rust**: Named functions recurse naturally; closures need tricks
3. **OCaml**: Y combinator is a simple lambda expression
4. **Rust**: Y combinator requires Box to break type recursion
5. Both: Open recursion passes "self" as explicit parameter
