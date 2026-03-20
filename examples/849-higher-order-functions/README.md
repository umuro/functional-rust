📖 **[View on hightechmind.io →](https://hightechmind.io/rust/849-higher-order-functions)**

---

# Higher-Order Functions
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Higher-order functions — functions that take other functions as arguments or return functions as results — are the foundation of functional programming. They enable abstraction over behavior: instead of writing five slightly different loops, write one `map`, `filter`, `fold`, `compose`, or `apply` and pass the varying behavior as a function. This is how real-world Rust uses iterators (`map`, `filter`, `flat_map`, `fold`) and how callback-based APIs work. Higher-order functions also enable point-free style, function composition pipelines, and building domain-specific languages from combinators. OCaml's `|>` pipe operator and Haskell's `$` are syntactic sugar for function application.

## Learning Outcomes

- Implement `apply(f, x)`: call a function on a value — the simplest higher-order function
- Implement `twice(f, x)`: apply f twice — demonstrating function composition
- Implement `compose(f, g)`: return a new function that applies g then f
- Use closures to capture context when returning functions from functions
- Recognize Rust's iterator methods as higher-order functions over collections

## Rust Application

```rust
pub fn apply<T, R, F: Fn(T) -> R>(f: F, x: T) -> R { f(x) }
pub fn twice<T: Clone, F: Fn(T) -> T>(f: F, x: T) -> T { f(f(x)) }
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(B) -> C, G: Fn(A) -> B {
    move |x| f(g(x))
}
```

`apply` is trivially useful but demonstrates that `F: Fn(T) -> R` is the trait bound for callable types. `twice` requires `T: Clone` because `f(x)` moves `x`, then `f(result)` needs to work on the result — actually `T: Clone` is not needed if `f` is `FnOnce`; it's needed only if `f` is reused. `compose` uses `move |x|` to capture both `f` and `g` by value into the returned closure. The `impl Fn(A) -> C` return type avoids boxing — the compiler generates a concrete type.

## OCaml Approach

OCaml's higher-order functions are first-class: `let apply f x = f x`, `let twice f x = f (f x)`, `let compose f g x = f (g x)`. The `|>` operator `x |> f` is `f x` — pipeline application. OCaml's `Fun.compose` is in the standard library. Partial application (`let add_one = (+) 1`) creates new functions by fixing one argument. OCaml's type inference deduces `('a -> 'b) -> 'a -> 'b` for `apply` without annotations. Currying (all functions take one argument and return a function for the rest) is the default, not an exception.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Function type | `Fn(T) -> R` trait | `'a -> 'b` type |
| Currying | Manual (tuples or multiple params) | Automatic (all functions curried) |
| Composition | `impl Fn(A) -> C` return | `fun x -> f (g x)` or `Fun.compose` |
| Pipe operator | `.` method chain or `|>` (unstable) | `\|>` built in |
| Closure capture | `move` to capture by value | Captures by default |
| Zero-cost | `impl Fn` avoids boxing | Closures heap-allocated |

## Exercises

1. Implement `flip(f)`: returns a function that calls f with its arguments reversed — `flip(f)(x, y) = f(y, x)`.
2. Implement function memoization as a higher-order function: `memoize(f)` returns a version of f that caches results.
3. Implement `pipe(functions: &[impl Fn(T) -> T], initial: T) -> T` that applies functions in sequence.
4. Implement `curry` and `uncurry` for 2-argument functions using closures.
5. Demonstrate that Rust's iterator `.map(f).filter(g).fold(init, h)` is a higher-order pipeline equivalent to OCaml's `List.fold_left h init (List.filter g (List.map f xs))`.
