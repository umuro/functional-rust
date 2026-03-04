# 053: Applicative Functor Basics

**Difficulty:** 2  **Level:** Intermediate

Apply a wrapped function to a wrapped value — the missing link between `map` and `and_then`.

## The Problem This Solves

You know `Option::map`. You give it a plain function and it applies it inside the `Option`. Simple. But what happens when the *function itself* is wrapped in an `Option`? Maybe it came from a lookup table, maybe it's user-configured, maybe it could be absent. `map` can't handle that — it only takes plain functions.

You also know `and_then` (Rust's flatMap). That chains steps where each step depends on the *result* of the previous one. But sometimes your values are **independent** — you have two separate `Option<i32>` values and want to add them. You don't need to chain them; you need to combine them in parallel.

The gap: `map` applies one plain function, `and_then` chains dependent steps, but there's no clean built-in for "combine two or three independent wrapped values using a plain function." You end up writing the same nested `match` pattern over and over: `match (a, b) { (Some(x), Some(y)) => Some(f(x, y)), _ => None }`. Every time. For every function. For every pair of values.

Applicative fills this gap. It gives you `apply` (for wrapped functions) and `lift2`/`lift3` (for combining 2 or 3 independent wrapped values). This exists to solve exactly that pain.

## The Intuition

Think of `Option` as a box. `map` takes a regular function and applies it to whatever is in the box. But what if the function is *also in a box*? Applicative says: if you have a box containing a function, and a box containing a value, you can produce a box containing the result — or `Nothing` if either box is empty.

`lift2` is even simpler to understand: "give me a regular two-argument function and two wrapped values, and I'll apply the function if both are present." It's like `zip` + `map` in one shot.

```rust
// You have these:
let maybe_name: Maybe<String> = Maybe::Just("Alice".to_string());
let maybe_age:  Maybe<i32>    = Maybe::Just(30);

// You want this — but both inputs are optional:
// User { name: "Alice", age: 30 }

// Without applicative (nested match):
let user = match (maybe_name, maybe_age) {
    (Maybe::Just(name), Maybe::Just(age)) => Maybe::Just(User { name, age }),
    _ => Maybe::Nothing,
};

// With lift2:
let user = lift2_simple(
    |name, age| User { name, age },
    maybe_name,
    maybe_age,
);
// Same result, half the noise
```

**Jargon decoded:**
- *Functor* — a type you can `map` over (`Option`, `Vec`, `Result`)
- *Applicative* — a Functor where you can also apply wrapped functions and combine independent wrapped values
- *`pure`* — wrap a plain value in the context (`Some(x)`, `Ok(x)`)
- *`apply`* — given a wrapped function `Maybe<F>` and a wrapped value `Maybe<A>`, produce `Maybe<B>`
- *`lift2`* — given a plain `fn(A, B) -> C` and `Maybe<A>` and `Maybe<B>`, produce `Maybe<C>`

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

// apply: the wrapped function is stored in Maybe<F>
// If either is Nothing, the whole thing is Nothing
impl<F> Maybe<F> {
    fn apply<A, B>(self, ma: Maybe<A>) -> Maybe<B>
    where
        F: FnOnce(A) -> B,
    {
        match (self, ma) {
            (Maybe::Just(f), Maybe::Just(a)) => Maybe::Just(f(a)),
            _ => Maybe::Nothing,
        }
    }
}

// Usage:
let double = Maybe::Just(|x: i32| x * 2);
let five   = Maybe::Just(5);
println!("{:?}", double.apply(five)); // Just(10)

let no_fn: Maybe<fn(i32) -> i32> = Maybe::Nothing;
println!("{:?}", no_fn.apply(five)); // Nothing — function was absent
```

```rust
// lift2: combine two independent Maybe values with a plain function
// No currying needed — Rust takes multi-argument closures directly
fn lift2_simple<A, B, C, F: FnOnce(A, B) -> C>(
    f: F,
    a: Maybe<A>,
    b: Maybe<B>,
) -> Maybe<C> {
    match (a, b) {
        (Maybe::Just(a), Maybe::Just(b)) => Maybe::Just(f(a, b)),
        _ => Maybe::Nothing,  // Either was absent — result is absent
    }
}

// lift3: same idea for three independent values
fn lift3_simple<A, B, C, D, F: FnOnce(A, B, C) -> D>(
    f: F, a: Maybe<A>, b: Maybe<B>, c: Maybe<C>,
) -> Maybe<D> {
    match (a, b, c) {
        (Maybe::Just(a), Maybe::Just(b), Maybe::Just(c)) => Maybe::Just(f(a, b, c)),
        _ => Maybe::Nothing,
    }
}
```

```rust
// Real example: parse two numbers and add them
let sum = lift2_simple(
    |a, b| a + b,
    parse_int("42"),   // Maybe::Just(42)
    parse_int("bad"),  // Maybe::Nothing
);
// Result: Maybe::Nothing  — one bad parse, result is Nothing
```

```rust
// Rust's Option already has zip() as a built-in applicative combinator:
let a = "42".parse::<i32>().ok();
let b = "7".parse::<i32>().ok();
let pair = a.zip(b); // Some((42, 7))
// zip gives you the pair; you still need map to apply the function after
let sum = a.zip(b).map(|(x, y)| x + y); // Some(49)
```

**Note on currying:** OCaml functions are curried by default, which lets you write elegant `pure f <*> a <*> b` chains. Rust doesn't curry, so `lift2`/`lift3` take the full multi-argument closure directly. This is actually cleaner for most Rust use cases.

## What This Unlocks

- **Config combination:** When you have 3 optional config fields and need all 3 to build a connection — `lift3` builds the connection or returns `Nothing` cleanly, without nested `match`.
- **Parallel validation (without error accumulation):** When all checks are independent and you just want success-or-absent, applicative combination is cleaner than chaining with `and_then`.
- **Option::zip chains:** Understanding why `zip` exists — it's Rust's built-in applicative combinator for `Option`. Knowing that `a.zip(b).map(f)` is the same as `lift2(f, a, b)` helps you recognize the pattern in the wild.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Syntax | `pure f <*> a <*> b` (infix operators) | `lift2_simple(f, a, b)` (free functions) |
| Currying | Functions are curried by default; `f a` returns a function | No currying; closures take all args at once |
| Built-in applicative | No `zip` equivalent in stdlib | `Option::zip` is a built-in applicative combinator |
| Wrapped function | `Maybe<'a -> 'b>` — function is a value | `Maybe<F> where F: FnOnce(A) -> B` — needs generic bounds |
| `pure` | `pure x` wraps a value | `Maybe::Just(x)` or `Some(x)` |
