# Example 005: Currying and Partial Application

## Concept

Currying transforms multi-argument functions into chains of single-argument functions. OCaml curries by default. Rust requires manual implementation via closures.

## Key Differences

**OCaml (automatic):**
```ocaml
let add x y = x + y  (* Already curried: int -> int -> int *)
let add5 = add 5     (* Partial application *)
```

**Rust (manual):**
```rust
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let add5 = add(5);
```

## Why Rust Doesn't Auto-Curry

1. **Ownership complexity** - Currying captures variables
2. **Performance** - Function pointers vs closure indirection
3. **Ergonomics** - Method chaining preferred over currying

## Rust Alternative: Closures

```rust
let add5 = |y| 5 + y;
let numbers: Vec<_> = vec![1, 2, 3].iter().map(add5).collect();
```

## Next Steps

Example 006 explores function composition.
