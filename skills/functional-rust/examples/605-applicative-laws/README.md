# 605: Applicative Functor Laws

**Difficulty:** 5  **Level:** Master

The four rules every correct Applicative must follow — and how to verify them in Rust.

## The Problem This Solves

You've implemented an `apply` function for your custom type. It compiles. The tests pass. But is it *correct*? "Correct" for an Applicative means more than "returns something sensible." It means the implementation behaves consistently across all possible inputs in ways that let callers reason about it algebraically.

Consider: if you write a generic algorithm that works over "any Applicative," that algorithm's correctness depends on certain guarantees. The algorithm might rely on the fact that `pure(id).apply(v) == v` — that applying the identity function changes nothing. Or that applying a pure function to a pure value is the same as just computing the result. If your implementation breaks these guarantees, the algorithm breaks in subtle, hard-to-debug ways.

The four Applicative laws are the *contract* between an Applicative implementor and the callers who depend on it. They can't be enforced by the type system — they're semantic guarantees. But they can be tested. And understanding *why* each law exists tells you what Applicatives are actually for.

```rust
// Is this Applicative correct? Laws tell you.
fn ap_option<A, B>(mf: Option<fn(A)->B>, ma: Option<A>) -> Option<B> {
    match (mf, ma) { (Some(f), Some(a)) => Some(f(a)), _ => None }
}
// It looks right. But does it satisfy identity? Homomorphism? Let's verify.
```

This exists to solve exactly that pain: providing a checklist that separates "it seems to work" from "it is provably correct."

## The Intuition

Think of the Applicative laws as the same kind of guarantees you'd expect from addition:
- `0 + x = x` (identity element — adding zero changes nothing)
- `(a + b) + c = a + (b + c)` (associativity — grouping doesn't matter)

You'd be shocked if `0 + 5` returned `6`. Similarly, you should be shocked if `pure(id).ap(v)` returned anything other than `v`. The laws codify that shock into checkable properties.

**The four laws, in plain English:**

1. **Identity** — Applying the wrapped identity function changes nothing. `Some(id).ap(Some(5)) == Some(5)`. This verifies that `pure` doesn't add effects and `ap` doesn't do something weird when the function is identity.

2. **Homomorphism** — Applying a pure function to a pure value is the same as computing in the "normal" world and then wrapping. `Some(double).ap(Some(5)) == Some(double(5))`. The wrapper is transparent for pure values.

3. **Interchange** — You can swap which side is "pure". `u.ap(pure(y)) == pure(|f| f(y)).ap(u)`. The value `y` can move into the function's side. This ensures `ap` doesn't treat its arguments asymmetrically.

4. **Composition** — Applicatives compose. `pure(∘).ap(f).ap(g).ap(v)` equals `f.ap(g.ap(v))`. This is the hardest to grasp — it says function composition distributes over `ap`.

**Jargon decoded:**
- *`pure(x)`* — lift a plain value into the applicative context (`Some(x)`, `Ok(x)`)
- *`ap(mf, ma)`* — apply a wrapped function `mf` to a wrapped value `ma`
- *`id`* — the identity function: `fn id<T>(x: T) -> T { x }` — returns its argument unchanged
- *`∘` (compose)* — `(f ∘ g)(x) = f(g(x))` — apply `g` first, then `f` to the result

## How It Works in Rust

```rust
// The core primitives for Option as an Applicative:
fn pure_opt<A>(a: A) -> Option<A> { Some(a) }

fn ap_option<A, B>(mf: Option<fn(A)->B>, ma: Option<A>) -> Option<B> {
    match (mf, ma) {
        (Some(f), Some(a)) => Some(f(a)),
        _ => None,
    }
}
```

```rust
// Law 1: Identity
// pure(id) `ap` v == v
fn identity_law<A: PartialEq + Copy>(v: Option<A>) -> bool {
    fn id<T>(x: T) -> T { x }
    ap_option(pure_opt(id as fn(A)->A), v) == v
    // "applying the identity function does nothing"
}

assert!(identity_law(Some(42)));
assert!(identity_law::<i32>(None));
```

```rust
// Law 2: Homomorphism
// pure(f) `ap` pure(x) == pure(f(x))
fn homomorphism_law<A: PartialEq + Copy>(f: fn(A)->A, x: A) -> bool {
    ap_option(pure_opt(f), pure_opt(x)) == pure_opt(f(x))
    // "the wrapper is transparent for pure computations"
}

fn double(x: i32) -> i32 { x * 2 }
assert!(homomorphism_law(double, 5));  // Some(double(5)) == Some(10) ✓
```

```rust
// Law 3: Interchange
// u `ap` pure(y) == pure(|f| f(y)) `ap` u
fn interchange_law<A: PartialEq + Copy>(u: Option<fn(A)->A>, y: A) -> bool {
    let apply_to_y = move |f: fn(A)->A| f(y);
    ap_option(u, pure_opt(y))
        == ap_option(pure_opt(apply_to_y as fn(fn(A)->A)->A), u)
    // "y can be moved into the function side without changing the result"
}

assert!(interchange_law(Some(double as fn(i32)->i32), 10));
```

```rust
// Practical Applicative usage: combining independent Options
fn map2<A: Copy, B: Copy, C>(
    fa: Option<A>,
    fb: Option<B>,
    f: impl Fn(A, B) -> C,
) -> Option<C> {
    match (fa, fb) {
        (Some(a), Some(b)) => Some(f(a, b)),
        _ => None,
    }
}

map2(Some(3i32), Some(4i32), |a, b| a + b)  // Some(7)
map2(Some(3i32), None,       |a, b| a + b)  // None
```

**The key insight: independent effects.** Unlike monads, where each step can depend on the previous result, Applicatives combine *independent* effects. `map2(fa, fb, f)` doesn't let `fb` depend on the value inside `fa`. This is both the limitation and the power — because effects are independent, they can run in parallel, and the computation structure can be analyzed statically before running.

## What This Unlocks

- **Generic applicative algorithms:** A function parameterized over `impl Applicative` (if Rust had the trait) can work with `Option`, `Result`, `Vec`, futures — as long as each implementation satisfies the laws, the algorithm is correct for all of them.
- **Parallel execution:** Because Applicative effects are independent (unlike monad's sequential dependencies), a parallel executor can schedule them concurrently. `futures::join!` is the Applicative combinator for async Rust.
- **Property-based testing:** The laws are perfect candidates for property-based tests with `proptest` or `quickcheck`. Instead of checking specific inputs, you verify the laws hold for *any* input — catching bugs that hand-picked test cases would miss.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Applicative typeclass | `module type Applicative` with `pure` and `<*>` | No built-in trait — typically implement per-type |
| `<*>` syntax | Infix operator, chains naturally: `f <*> a <*> b` | Function call: `ap_option(ap_option(f, a), b)` |
| Law enforcement | Not enforced by type system (same as Rust) | Not enforced by type system — must test |
| Parallel effects | Supported via async Applicatives | `futures::join!` is the Applicative for `Future` |
| Identity law | `pure id <*> v = v` | `ap_option(pure_opt(id), v) == v` |
| Composition law | `pure (∘) <*> f <*> g <*> v = f <*> (g <*> v)` | Requires careful type annotation around `fn` pointers |
