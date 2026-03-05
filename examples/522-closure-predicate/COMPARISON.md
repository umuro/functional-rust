# OCaml vs Rust: Predicate Composition

## OCaml
```ocaml
let pred_and p1 p2 x = p1 x && p2 x
let pred_or p1 p2 x = p1 x || p2 x
let pred_not p x = not (p x)

let is_positive x = x > 0
let is_even x = x mod 2 = 0
let is_positive_even = pred_and is_positive is_even
```

## Rust
```rust
pub fn pred_and<T, P1, P2>(p1: P1, p2: P2) -> impl Fn(&T) -> bool
where P1: Fn(&T) -> bool, P2: Fn(&T) -> bool {
    move |x| p1(x) && p2(x)
}

let is_positive_even = pred_and(is_positive(), is_even());
```

## Key Differences

1. **OCaml**: Simple function composition
2. **Rust**: Generic over predicate types with trait bounds
3. Both: Build complex predicates from simple ones
4. **Rust**: Closures need explicit lifetime/move handling
5. Both integrate with filter operations
