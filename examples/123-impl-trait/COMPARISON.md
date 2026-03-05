# OCaml vs Rust: impl Trait

## Side-by-Side Code

### OCaml — polymorphic argument (parametric polymorphism)
```ocaml
(* OCaml infers the most general type automatically *)
let stringify_all to_s items = List.map to_s items

(* Returning a function — concrete type is always visible in OCaml *)
let make_adder n = fun x -> x + n
```

### Rust — impl Trait in argument position
```rust
// Explicit trait bound; compiler monomorphizes per call site
pub fn stringify_all(items: &[impl Display]) -> Vec<String> {
    items.iter().map(|x| x.to_string()).collect()
}
```

### Rust — impl Trait in return position (opaque type)
```rust
// Concrete closure type is hidden; zero heap allocation
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// Returning a complex iterator chain without naming its type
pub fn even_squares(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|n| n % 2 == 0).map(|n| n * n)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic argument | `'a -> string` (inferred) | `fn f(x: impl Display)` |
| Returning a function | `int -> int` (always concrete) | `impl Fn(i32) -> i32` (opaque) |
| Returning an iterator | must name the module type | `impl Iterator<Item = u32>` |
| Multiple bounds | `('a : S1) ('a : S2)` (modules) | `impl Trait1 + Trait2` |

## Key Insights

1. **Argument position = sugar for generics.** `fn f(x: impl Display)` and `fn f<T: Display>(x: T)` are identical after monomorphization; `impl Trait` simply drops the explicit type-parameter name when you don't need to reference `T` elsewhere in the signature.

2. **Return position creates an opaque type.** OCaml always exposes the concrete type in its inferred type signature (e.g., `int -> int`). Rust's `impl Trait` hides it: the caller only knows the trait bound, not the underlying struct or closure type. This is an *existential* type from the caller's view.

3. **Zero-cost abstraction.** Unlike `Box<dyn Trait>`, `impl Trait` is resolved at compile time with no heap allocation and no vtable dispatch. The compiler monomorphizes each call site.

4. **Single concrete type required in return position.** If different branches return different concrete types, the compiler rejects `impl Trait`. Use `Box<dyn Trait>` for runtime polymorphism across branches; `impl Trait` is a compile-time mechanism only.

5. **Iterator ergonomics.** The iterator adaptor chain `(0..n).filter(...).map(...)` produces a type like `Map<Filter<Range<u32>, …>, …>` — impossible to write by hand. `impl Iterator<Item = u32>` makes it trivial to return such chains from public APIs.

## When to Use Each Style

**Use `impl Trait` in argument position when:** you have a simple trait bound and don't need to reference the type parameter elsewhere in the signature — keeps the function header clean.

**Use `impl Trait` in return position when:** you want to return a closure, a complex iterator chain, or any type that is private / unnamed, and all code paths return the *same* concrete type.

**Use `Box<dyn Trait>` instead when:** different branches need to return different concrete types, or you need to store the value in a struct field without making the struct generic.
