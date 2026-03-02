# Comparison: Function Composition

## OCaml — Curried compose

```ocaml
let compose f g x = f (g x)
let square_then_double = compose double square
(* square_then_double 3 = 18 *)
```

## Rust — Generic compose (direct translation)

```rust
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

let square_then_double = compose(double, square);
// square_then_double(3) == 18
```

## Rust — Pipeline style (left-to-right argument order)

```rust
pub fn pipe<A, B, C, F, G>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

let square_then_double = pipe(square, double);
```

## Rust — Trait extension

```rust
let square_then_double = square.then_apply(double);
// square_then_double(3) == 18
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Return type | Inferred polymorphic `'a -> 'b` | Explicit `impl Fn(A) -> C` |
| Closure capture | Automatic | Requires `move` keyword |
| Partial application | Built-in currying | Returns explicit closure |
| Type parameters | Implicit (`'a`, `'b`) | Explicit (`A`, `B`, `C`) |
| Method chaining | Not built-in | Via trait extension |
| Argument order | Mathematical (f∘g) | Two conventions: compose or pipe |
