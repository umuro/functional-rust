# 071: Identity Monad

**Difficulty:** ⭐⭐  **Level:** Intermediate

A monad that does nothing — stripped bare so you can see the structure clearly.

## The Problem This Solves

When you learn about monads through `Option` and `Result`, the monad machinery is mixed together with the interesting behavior (short-circuiting on `None`, carrying error types). It's hard to tell what's "the monad part" and what's "the `Option` part."

Beginners often ask: "So what exactly does a monad *add*, independent of any specific effect?" It's a reasonable question. The answer is buried inside types that have a lot going on.

The Identity monad is a monad that does nothing extra. No short-circuiting. No error carrying. No effects whatsoever. It wraps a value, lets you chain operations on it, and unwraps cleanly. The only thing it provides is the monadic structure itself.

This sounds useless — and for production code, it mostly is. But for *understanding*, it's exactly what you need: the monad skeleton with nothing obscuring it. And in advanced Rust, it becomes useful as a base case for monad transformers (stacking multiple effects together).

The Identity monad exists to solve exactly that pain: it makes the abstract concrete by removing everything except the pattern itself.

## The Intuition

Imagine a box. You put a value in. You can apply functions to the value inside. You can chain those functions. At the end, you take the value out.

That's it. The box doesn't do anything else. It doesn't skip steps. It doesn't carry errors. It's just a box that supports chaining.

```
put in → transform → transform → transform → take out
Identity::of(10) → double → add_one → to_string → .run()
```

Compare to `Option`: the box might be empty, and if it's empty, all transforms are skipped. Compare to `Result`: the box might contain an error, and transforms are skipped. `Identity`: the box always has a value, transforms always run.

The minimal interface every monad has:
1. **`of(value)`** — put a value into the monad ("return" or "pure" in theory)
2. **`bind(f)`** — apply a function that returns a wrapped value, flatten the result ("bind" or `>>=`)
3. **`run()`** — extract the final value (only possible for Identity since there's no effect to unwrap)

## How It Works in Rust

**The struct — as simple as possible**

```rust
#[derive(Debug, Clone, PartialEq)]
struct Identity<A>(A);  // just a newtype wrapper
```

**The three core operations**

```rust
impl<A> Identity<A> {
    // "return" / "pure" — put a value into Identity
    fn of(x: A) -> Self {
        Identity(x)
    }

    // "bind" / ">>=" — apply f, get back Identity<B>
    // f must return Identity<B>, not plain B
    // This is what distinguishes bind from map
    fn bind<B, F: FnOnce(A) -> Identity<B>>(self, f: F) -> Identity<B> {
        f(self.0)   // unwrap, apply, return whatever f returned
    }

    // "map" — apply a plain function (not monadic)
    fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Identity<B> {
        Identity(f(self.0))   // unwrap, apply, re-wrap
    }

    // extract the final value
    fn run(self) -> A {
        self.0
    }
}
```

**Chaining operations**

```rust
let result = Identity::of(10)
    .bind(|x| Identity::of(x * 2))   // 10 → 20, wrapped in Identity
    .bind(|x| Identity::of(x + 1))   // 20 → 21, wrapped in Identity
    .run();                           // unwrap: 21

assert_eq!(result, 21);
```

**`bind` vs `map` — the key distinction**

```rust
// map: the function returns a plain value → Identity wraps it
Identity(5).map(|x| x * 2)                    // Identity(10)

// bind: the function returns Identity → no double-wrapping
Identity(5).bind(|x| Identity::of(x * 2))     // Identity(10), not Identity(Identity(10))
```

`bind` flattens the result. That flattening is the essence of "monad" — you can compose functions that each return a wrapped value, and the wrapping doesn't accumulate.

**The laws hold for Identity too**

```rust
// Left Identity: Identity::of(a).bind(f) == f(a)
let f = |x: i32| Identity::of(x * 3);
assert_eq!(Identity::of(5).bind(f), f(5));    // Identity(15) == Identity(15)

// Right Identity: m.bind(Identity::of) == m
let m = Identity(42);
assert_eq!(m.clone().bind(Identity::of), m);  // Identity(42) == Identity(42)
```

These feel trivially true for Identity — because there are no effects to get in the way. That's the point: Identity shows you the laws in their purest form.

## What This Unlocks

- **Monad transformer base case.** In advanced Rust, you can build `OptionT`, `ResultT` and other "stacked monad" types by transforming the Identity monad. Identity is the plain base you add effects on top of.
- **Understanding `bind` vs `map`.** The difference becomes obvious with Identity stripped of distractions: `map` lifts a plain function; `bind` chains functions that themselves produce wrapped values. Once you see it here, you'll recognize it in `Option`, `Result`, `Iterator`, and `Future`.
- **Testable monad laws.** Identity is the easiest monad to verify laws against — no edge cases like `None` or `Err`. Use it to validate your understanding of the three laws before applying them to more complex types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type definition | `type 'a t = Identity of 'a` | `struct Identity<A>(A)` (newtype) |
| `return` / `of` | `let return x = Identity x` | `fn of(x: A) -> Self` |
| `bind` | `let bind (Identity x) f = f x` | `fn bind<B, F>(self, f: F) -> Identity<B>` |
| Extraction | `let run (Identity x) = x` | `fn run(self) -> A` |
| Use in practice | Common as monad transformer base | Rare directly; useful for understanding |
