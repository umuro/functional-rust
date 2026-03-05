📖 **[View on hightechmind.io →](https://hightechmind.io/rust/604-monad-laws-rust)**

---

# 604: Monad Laws — Rust Deep Dive

**Difficulty:** 5  **Level:** Master

Encode monadic bind as a Rust trait, verify the three monad laws with generic functions, and understand why the laws matter for compiler-assisted composition.

## The Problem This Solves

You've used `Option::and_then` and `Result::and_then`. You've seen the three monad laws stated informally. But at the Master level, there's a harder problem: can you express the monad pattern *generically* in Rust's type system?

The challenge is significant. Monads require what Rust calls "higher-kinded types" — the ability to abstract over type constructors like `Option<_>` or `Result<_, E>`, not just over concrete types. Rust doesn't support this natively (unlike Haskell or OCaml). Working around it reveals a lot about how Rust's type system differs from functional languages.

Beyond the type system puzzle, there's a practical concern: once you write a `Monad` trait implementation, how do you *prove* it's correct? The compiler won't enforce the laws. You need to verify them programmatically — with concrete functions that check each law holds for specific values.

This exists to solve exactly that pain: it shows you how far you can push Rust's type system toward the monad abstraction, and gives you runnable law proofs for `Option` and `Result`.

## The Intuition

Think of the `Monad` trait like a contract for a production line:
- `unit(value)` — start the line with a value (like `Some(x)` or `Ok(x)`)
- `bind(f)` — add a station: if the line is running, apply `f`; if it's stopped, propagate the stopped state

The three laws are quality guarantees for that contract:

**Law 1 — Left Identity:** Starting the line with a value and immediately handing it to station `f` is the same as just running `f` directly. The startup step is transparent.
```
unit(a).bind(f)  ==  f(a)
```

**Law 2 — Right Identity:** If the only station is "wrap in unit", the line produces what it started with. The unit wrapper is transparent.
```
m.bind(unit)  ==  m
```

**Law 3 — Associativity:** Grouping doesn't matter. Running station `f` then `g` sequentially is the same as running `f` where `f` itself runs `g`. You can refactor pipelines into sub-pipelines freely.
```
m.bind(f).bind(g)  ==  m.bind(|x| f(x).bind(g))
```

Without Law 3, extracting a sub-pipeline into its own function might change the result. That would make monads unreliable for building abstractions.

## How It Works in Rust

**A `Monad` trait using associated types**

Rust can't directly express "a type constructor `M<_>` where `M<A>` and `M<B>` are related." The workaround: use an associated type `Wrapped<B>` to name the output type:

```rust
trait Monad: Sized {
    type Inner;               // the A in Option<A>
    type Wrapped<B>: Monad<Inner=B>;  // the Option<B> produced by bind
    fn unit(a: Self::Inner) -> Self;
    fn bind<B>(self, f: impl FnOnce(Self::Inner) -> Self::Wrapped<B>) -> Self::Wrapped<B>;
}
```

**Implementing for Option**

```rust
impl<A> Monad for Option<A> {
    type Inner = A;
    type Wrapped<B> = Option<B>;
    fn unit(a: A) -> Option<A> { Some(a) }
    fn bind<B>(self, f: impl FnOnce(A) -> Option<B>) -> Option<B> {
        self.and_then(f)   // bind IS and_then — no magic
    }
}
```

**Implementing for Result**

```rust
impl<A, E: Clone> Monad for Result<A, E> {
    type Inner = A;
    type Wrapped<B> = Result<B, E>;
    fn unit(a: A) -> Result<A, E> { Ok(a) }
    fn bind<B>(self, f: impl FnOnce(A) -> Result<B, E>) -> Result<B, E> {
        self.and_then(f)   // same: bind IS and_then
    }
}
```

**Generic law verification functions**

```rust
// Law 1: unit(a).bind(f) == f(a)
fn left_identity<A: Clone, B: PartialEq>(a: A, f: impl Fn(A) -> Option<B> + Clone) -> bool {
    let left  = Option::unit(a.clone()).bind(f.clone());
    let right = f(a);
    left == right
}

// Law 2: m.bind(unit) == m
fn right_identity<A: PartialEq + Clone>(m: Option<A>) -> bool {
    m.clone().bind(Option::unit) == m
}

// Law 3: m.bind(f).bind(g) == m.bind(|x| f(x).bind(g))
fn associativity<A: Clone, B: Clone, C: PartialEq>(
    m: Option<A>,
    f: impl Fn(A) -> Option<B> + Clone,
    g: impl Fn(B) -> Option<C> + Clone,
) -> bool {
    let left  = m.clone().bind(f.clone()).bind(g.clone());
    let right = m.bind(move |x| f(x).bind(g.clone()));
    left == right
}
```

**Running the proofs**

```rust
let f = |x: i32| if x > 0 { Some(x*2) } else { None };
let g = |x: i32| if x < 100 { Some(x+1) } else { None };

assert!(left_identity(5, f));            // true: Some(5).bind(f) == f(5)
assert!(right_identity(Some(5)));        // true: Some(5).bind(Some) == Some(5)
assert!(right_identity(None::<i32>));    // true: None.bind(Some) == None
assert!(associativity(Some(5), f, g));   // true: grouping doesn't matter
assert!(associativity(None, f, g));      // true: None is transparent
```

**Simulated do-notation with `?`**

```rust
// Rust's equivalent of Haskell's do-notation or OCaml's let*
fn compute(s: &str) -> Option<i32> {
    let n      = s.parse::<i32>().ok()?;         // bind step 1
    let doubled = if n > 0 { Some(n*2) } else { None }?;  // bind step 2
    Some(doubled + 1)                             // unit at the end
}
// Each ? is one monadic bind. This IS the do-notation pattern.
```

## What This Unlocks

- **Generic monadic code.** With the `Monad` trait, you can write algorithms that work over any monad — `Option`, `Result`, or your own types — and the laws guarantee they'll compose correctly.
- **Confident refactoring of pipelines.** Law 3 (associativity) means you can extract any sub-chain from a larger pipeline into its own function without changing the result. This is the theoretical foundation for why `?`-based code refactors safely.
- **Foundation for monad transformers.** Stacking effects (e.g., `Option` + logging, or `Result` + state) requires knowing the laws hold for each layer. This pattern is the entry point to advanced effect composition in Rust.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic monad | Type class / functor interface | Trait with associated types (limited HKT) |
| Higher-kinded types | Native (`'a t`, `'a monad`) | Workaround via `type Wrapped<B>` |
| `bind` name | `>>=` / `bind` | `and_then` (stdlib) / `bind` (trait) |
| `return` / `unit` | `return` keyword in monad context | `unit` method or `Some`/`Ok` directly |
| Law enforcement | Convention (no type-level proof) | Convention (verified via `assert!` tests) |
| Do-notation | `let*` (OCaml 4.08+) | `?` operator (for `Option`/`Result`) |
