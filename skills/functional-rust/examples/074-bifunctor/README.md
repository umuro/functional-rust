# 074: Bifunctor — Mapping Over Two Type Parameters

**Difficulty:** Intermediate  **Level:** Intermediate

When your container holds two different things, a Bifunctor lets you transform either one — or both — independently.

## The Problem This Solves

`Result<T, E>` holds two types: a success value `T` and an error value `E`. You often need to transform just one of them:

```rust
// I have Result<i32, String> and want Result<i64, String>
// This works — map transforms the Ok side:
let r: Result<i64, String> = result.map(|n| n as i64);

// But what if I want to transform just the error side?
// There's map_err for that. But what about a custom Either type?
// You'd have to match manually every single time.
```

Once you build your own `Either<A, B>` type (useful for representing one-of-two outcomes), you immediately hit this: you want to transform the `Left` side, or the `Right` side, or both at once. Without a systematic approach, you write a `map_left`, a `map_right`, and forget they should compose nicely together.

A Bifunctor is simply a container with *two* type slots, both of which support independent mapping. It's a Functor, but for pairs of types instead of single types. The concept exists to solve exactly that pain: "I have two things inside a container and I want to transform either one without touching the other."

## The Intuition

Imagine a shipping label with two fields: sender address and recipient address. You want to:
- Update just the recipient (forwarding a package)
- Update just the sender (returning a package)
- Update both (reshipping between two new parties)

A Bifunctor is that label: a container with two slots, where you can independently transform either slot, or both at once with a single operation.

In code:
```
Either::Left(42) --bimap(double, uppercase)--> Either::Left(84)
Either::Right("hello") --bimap(double, uppercase)--> Either::Right("HELLO")
```

`bimap` takes *two* functions: one for the `Left` slot and one for the `Right` slot. It applies the appropriate function based on which variant is active. Neither function is called if its slot isn't populated.

The two simpler operations fall out naturally:
- `map_left(f)` = `bimap(f, identity)` — transform only the Left, leave Right as-is
- `map_right(g)` = `bimap(identity, g)` — transform only the Right, leave Left as-is

## How It Works in Rust

**Step 1 — The Either type (the canonical Bifunctor):**

```rust
#[derive(Debug, Clone, PartialEq)]
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

**Step 2 — bimap: transform both slots with two functions:**

```rust
impl<A, B> Either<A, B> {
    fn bimap<C, D, F: FnOnce(A) -> C, G: FnOnce(B) -> D>(
        self, f: F, g: G,
    ) -> Either<C, D> {
        match self {
            Either::Left(a)  => Either::Left(f(a)),   // apply f to Left
            Either::Right(b) => Either::Right(g(b)),  // apply g to Right
        }
    }
}
```

**Step 3 — map_left and map_right as special cases:**

```rust
fn map_left<C, F: FnOnce(A) -> C>(self, f: F) -> Either<C, B> {
    self.bimap(f, |b| b)   // identity for the Right slot
}

fn map_right<D, G: FnOnce(B) -> D>(self, g: G) -> Either<A, D> {
    self.bimap(|a| a, g)   // identity for the Left slot
}
```

Usage:

```rust
let e: Either<i32, &str> = Either::Left(42);
let r = e.bimap(|n| n * 2, |s| s.to_uppercase());
// Either::Left(84) — only f was called; g was never invoked
```

**Step 4 — Bifunctor for pairs (both slots always populated):**

```rust
struct Pair<A, B>(A, B);

impl<A, B> Pair<A, B> {
    fn bimap<C, D, F: FnOnce(A)->C, G: FnOnce(B)->D>(self, f: F, g: G) -> Pair<C, D> {
        Pair(f(self.0), g(self.1))  // both f and g always called
    }
}

let pair = Pair("hello", 42_i32);
let result = pair.bimap(|s: &str| s.len(), |n| n * 2);
// Pair(5, 84)
```

**Bifunctor laws** (what `bimap` must satisfy):
1. `bimap(id, id)` == identity — doing nothing does nothing
2. `bimap(f∘g, h∘k)` == `bimap(f,h).bimap(g,k)` — order of composition doesn't matter

## What This Unlocks

- **Independent transformation of either side.** You can evolve error types or success types in `Result`-like structures without touching the other side. This is exactly how `Result::map` and `Result::map_err` work — they're the `map_right` and `map_left` of `Result`'s Bifunctor.
- **Symmetric error handling.** `Either<AppError, Value>` lets you transform errors independently of values. Many Rust error-handling libraries (like `either` crate) are built on this pattern.
- **Generic algorithms over two-slot containers.** Write functions that operate on any Bifunctor regardless of the concrete type.

Real codebases where this pattern appears: `Result<T,E>` in the standard library (which is a Bifunctor — `map` and `map_err` are its `map_right` and `map_left`), the `either` crate, parser combinators that separate parse errors from semantic errors, and RPC systems that tag responses as success or failure.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Either type | `type ('a,'b) either = Left of 'a \| Right of 'b` | `enum Either<A,B> { Left(A), Right(B) }` |
| bimap signature | `val bimap : ('a->'c) -> ('b->'d) -> ('a,'b) either -> ('c,'d) either` | `fn bimap<C,D,F,G>(self, f:F, g:G) -> Either<C,D>` |
| Calling style | `bimap f g e` (curried, function-first) | `e.bimap(f, g)` (method syntax) |
| Pair / tuple | `(a, b)` — built-in tuple, no newtype needed | `struct Pair<A,B>(A,B)` — newtype to add `bimap` method |
| `Result` as Bifunctor | `Result.map` + `Result.map_error` | `Result::map` + `Result::map_err` |
| Generic Bifunctor trait | Module signature with `type ('a,'b) t` | Trait with GATs: `type Target<C,D>` |
| Identity in bimap | `fun x -> x` | `\|x\| x` |
