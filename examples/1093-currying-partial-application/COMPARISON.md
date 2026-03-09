# OCaml vs Rust: Currying and Partial Application

## Side-by-Side Code

### OCaml
```ocaml
(* All functions are automatically curried *)
let add x y = x + y

(* Partial application — just supply fewer arguments *)
let add5 = add 5

(* Equivalent desugared form *)
let add' = fun x -> fun y -> x + y
```

### Rust (idiomatic)
```rust
/// Plain two-argument function.
fn add(x: i64, y: i64) -> i64 {
    x + y
}

/// Partial application: return a closure capturing `x`.
fn add_partial(x: i64) -> impl Fn(i64) -> i64 {
    move |y| x + y
}

let add5 = add_partial(5);
assert_eq!(add5(3), 8);
```

### Rust (functional/recursive)
```rust
/// Fully curried — mirrors OCaml's `fun x -> fun y -> x + y`.
fn add_curried(x: i64) -> impl Fn(i64) -> i64 {
    move |y| x + y
}

/// Generic version with trait bounds.
fn add_curried_generic<T: std::ops::Add<Output = T> + Copy>(x: T) -> impl Fn(T) -> T {
    move |y| x + y
}

// One-shot call reads like OCaml: add_curried(2)(3)
assert_eq!(add_curried(2)(3), 5);
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val add : int -> int -> int` | `fn add(x: i64, y: i64) -> i64` |
| Curried signature | `int -> int -> int` (same!) | `fn(i64) -> impl Fn(i64) -> i64` |
| Partial application | `let add5 = add 5` | `let add5 = add_partial(5)` |
| Polymorphic | `val add : 'a -> 'a -> 'a` (with `(+)`) | `fn add<T: Add<Output=T> + Copy>(x: T) -> impl Fn(T) -> T` |
| Closure type | Implicit, GC-managed | `impl Fn(i64) -> i64` (stack or heap via `Box<dyn Fn>`) |
| Curry combinator | `val curry : ('a * 'b -> 'c) -> 'a -> 'b -> 'c` | `fn curry<A,B,C,F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>` |

## Key Insights

1. **OCaml currying is invisible; Rust currying is explicit.** In OCaml, `let add x y = x + y` is already `fun x -> fun y -> x + y`. In Rust, you must explicitly return a closure to achieve the same effect.

2. **`move` closures are Rust's capture mechanism.** Where OCaml's GC handles closure environments automatically, Rust's `move |y| x + y` transfers ownership of `x` into the closure — no garbage collection needed.

3. **`impl Fn` vs `Box<dyn Fn>` — static vs dynamic dispatch.** When the caller knows the concrete closure type, `impl Fn` gives zero-cost abstraction. When you need to store or return closures of varying types (e.g., from the `curry` combinator), `Box<dyn Fn>` provides dynamic dispatch.

4. **Trait bounds replace OCaml's implicit polymorphism.** OCaml's `(+)` works on any numeric type via ad-hoc polymorphism. Rust requires `T: Add<Output = T> + Copy` to express the same constraint — more verbose but explicit.

5. **Rust's closures are zero-cost.** Unlike OCaml's heap-allocated closure environments, Rust's `move` closures with `impl Fn` return types are monomorphized at compile time — no allocation, no indirection.

## When to Use Each Style

**Use idiomatic Rust when:** You have a fixed-arity function and occasionally want partial application — `add_partial(5)` is clear and efficient.

**Use curried Rust when:** You're building combinator libraries or DSLs where function composition and point-free style improve readability, e.g., `let transform = compose(scale(2.0), translate(1.0))`.
