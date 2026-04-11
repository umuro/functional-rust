📖 **[View on hightechmind.io →](https://hightechmind.io/rust/006-function-composition)**

---

# 006 — Function Composition

## Problem Statement

Function composition is the mathematical operation of combining two functions `f` and `g` into `f ∘ g`, where `(f ∘ g)(x) = f(g(x))`. It is the fundamental mechanism by which complex transformations are built from simple, reusable parts. Unix pipes (`ls | grep foo | sort`) are function composition in a shell. Promise chaining in JavaScript, method chaining in jQuery, and Spark's transformation DAGs are all manifestations of the same idea.

In purely functional languages like Haskell, `(.)` is a built-in operator: `(f . g) x = f (g x)`. This enables point-free style where you define functions by composition without naming intermediate values. While Rust does not have a built-in composition operator, closures and iterator method chaining achieve the same expressive power.

## Learning Outcomes

- Implement `compose(f, g)` that returns a new function applying `g` then `f`
- Implement `pipe(f, g)` that applies `f` then `g` (left-to-right reading order)
- Use Rust's iterator method chaining as idiomatic composition
- Build a `pipeline` function that applies a list of transformations in sequence
- Understand the mathematical definition: `(f ∘ g)(x) = f(g(x))`

- Implement a `pipeline` function that applies a slice of `&dyn Fn(i64) -> i64` transformations sequentially using `fold`

## Rust Application

`compose(f, g)` takes two functions and returns `Box<dyn Fn(A) -> C>` that calls `f(g(x))`. The `Box<dyn Fn>` is needed because the composed closure has a unique, unnameable type. `pipe` reverses the order to match data-flow reading direction. The `process` function demonstrates that Rust's `.map().filter().sum()` chain is function composition in practice — each method returns a lazy `Iterator` that the next method transforms. The `pipeline` function uses `fold` over a slice of `&dyn Fn` to apply transformations sequentially.

## OCaml Approach

OCaml does not have a built-in `(.)` composition operator in the standard library (unlike Haskell), but it is trivially defined: `let (%) f g = fun x -> f (g x)`. The `|>` pipe operator serves as left-to-right composition for single values. Composing transformations on lists is done by piping: `list |> List.map f |> List.filter g`. Function composition is pervasive in OCaml because functions are naturally curried — `List.map (fun x -> x + 1) |> List.filter (fun x -> x > 0)` partially applies to create reusable pipeline stages.

## Key Differences

1. **Built-in operator**: Haskell has `(.)`, OCaml does not by default, Rust does not. All three require explicit definition or use of method chaining.
2. **`Box<dyn Fn>` overhead**: Rust needs a `Box` (heap allocation + vtable) to return composed closures because each closure is a different type. OCaml closures are heap-allocated by the GC with no explicit boxing.
3. **Lifetime**: Rust's composed function requires `'static` bounds on the input functions if stored. OCaml closures capture environment via GC references with no lifetime constraints.
4. **Method chaining vs composition**: Rust's idiomatic style is iterator method chaining (`.map().filter()`), which reads left-to-right like OCaml's `|>`. Mathematical composition (`f ∘ g`) is less common in idiomatic Rust code.

1. **Built-in operator:** Haskell has `(.)`, OCaml has no built-in compose (though `|>` serves as pipe). Rust has neither — both must be built from closures.
2. **`Box<dyn Fn>`:** Rust's composed closure has an unnameable type, requiring `Box<dyn Fn(A) -> C>` for storage and return. OCaml closures always have a uniform runtime representation.
3. **`'static` bound:** Rust's `compose` requires `'static` because the closure may outlive its enclosing scope. OCaml closures capture environment automatically.
4. **Method chaining as composition:** Rust's `.map().filter().sum()` is function composition without naming intermediate values — the most idiomatic form.
5. **Argument order:** Mathematical `f ∘ g` applies `g` first, then `f`. OCaml's `|>` applies left-to-right. Rust's `pipe` also applies left-to-right — check which order your `compose` function uses.

## Exercises

1. **Compose three**: Write `compose3(f, g, h)` that produces a function equivalent to `f(g(h(x)))`. Then generalize to `compose_all(fns: Vec<Box<dyn Fn(i64) -> i64>>) -> Box<dyn Fn(i64) -> i64>` using fold.
2. **Memoize**: Write a `memoize(f: impl Fn(i32) -> i32) -> impl FnMut(i32) -> i32` wrapper that caches results in a `HashMap`. How does this interact with composition?
3. **Point-free pipeline**: Define three small functions `double`, `increment`, `square` and compose them into a single `transform: Box<dyn Fn(i64) -> i64>` without calling any of them directly.

4. **Memoize**: Implement `memoize<A: Hash + Eq + Clone, B: Clone>(f: impl Fn(A) -> B) -> impl Fn(A) -> B` using a `HashMap` cache to avoid recomputing results for inputs already seen.
5. **Retry combinator**: Write `retry<T, E>(f: impl Fn() -> Result<T, E>, n: usize) -> Result<T, E>` that calls `f` up to n times, returning the first `Ok` or the last `Err`.
