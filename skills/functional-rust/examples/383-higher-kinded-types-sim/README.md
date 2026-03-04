# 383: Simulating Higher-Kinded Types with GATs

**Difficulty:** 4  **Level:** Expert

Write generic code that works over `Option`, `Vec`, and `Result` as containers — without repeating yourself — using Generic Associated Types to simulate what Rust's type system can't express natively.

## The Problem This Solves

You want to write a function that works for "any mappable container." In most languages this is straightforward: a `Functor` typeclass or interface that says "you can call `map` on me." In Rust, you hit a wall:

```rust
// What you WANT to write:
fn double_all<F: Functor>(container: F<i32>) -> F<i32> {
    container.map(|x| x * 2)
}

// But Rust says: ERROR — can't use `F<i32>` (F is not a type constructor)
```

Rust's trait system only talks about *concrete types*, not *type constructors*. You can say `T: Clone` (T is some type that can be cloned), but you cannot say `F: Functor` where `F` is something like `Option` or `Vec` that still needs a type parameter.

This means if you want `map` to work generically, you write it three times: once for `Option<A>`, once for `Vec<A>`, once for `Result<A, E>`. Any library function that should work for "any mappable container" gets copied for each container. Combinators that compose functors require even more boilerplate.

Higher-Kinded Types (HKT) solve this by letting you abstract over type constructors, not just types. Rust doesn't have native HKT, but Generic Associated Types (GATs, stabilized in Rust 1.65) let you simulate it — with some verbosity. This exists to solve exactly that pain.

## The Intuition

**The problem in one sentence:** Rust traits can be generic over *types* (`T: Clone`) but not *type constructors* (`F<_>: Functor`).

Think of the difference between a box and a box shape:
- A *type* is a specific box with specific contents: `Option<i32>`, `Vec<String>`.
- A *type constructor* is the box shape without contents yet: `Option<_>`, `Vec<_>`.

Rust traits work with specific boxes. What we want is to write rules about box shapes — "any box shape that has a `map` operation."

**GATs (Generic Associated Types) as the workaround:** Instead of saying "F is a type constructor," we say "any type that has an associated type `Mapped<B>` describing what it becomes after mapping to `B`."

```
trait Functor {
    type Unwrapped;         // the A in Container<A>
    type Mapped<B>;         // what this becomes when you map A -> B
    
    fn map<B>(self, f: fn(Self::Unwrapped) -> B) -> Self::Mapped<B>;
}
```

The `Mapped<B>` is the GAT. It's an associated type that is *itself* generic. Before GATs (Rust < 1.65), this was impossible. With GATs, each implementor says "when you map me to B, you get [this specific type]":

- `Option<A>` says `Mapped<B> = Option<B>`
- `Vec<A>` says `Mapped<B> = Vec<B>`
- `Result<A, E>` says `Mapped<B> = Result<B, E>`

## How It Works in Rust

```rust
// The Functor trait — simulating HKT via GATs
trait Functor {
    type Unwrapped;           // the inner type A
    type Mapped<B>;           // GAT: what we become after mapping to B

    fn map<B, F: Fn(Self::Unwrapped) -> B>(self, f: F) -> Self::Mapped<B>;
}

// Implementation for Option<A>
impl<A> Functor for Option<A> {
    type Unwrapped = A;
    type Mapped<B> = Option<B>;  // Option<A>.map(...) -> Option<B>

    fn map<B, F: Fn(A) -> B>(self, f: F) -> Option<B> {
        self.map(f)  // delegate to Option's built-in map
    }
}

// Implementation for Vec<A>
impl<A> Functor for Vec<A> {
    type Unwrapped = A;
    type Mapped<B> = Vec<B>;  // Vec<A>.map(...) -> Vec<B>

    fn map<B, F: Fn(A) -> B>(self, f: F) -> Vec<B> {
        self.into_iter().map(f).collect()
    }
}

// Implementation for Result<A, E>
impl<A, E> Functor for Result<A, E> {
    type Unwrapped = A;
    type Mapped<B> = Result<B, E>;  // Result<A,E>.map(...) -> Result<B,E>

    fn map<B, F: Fn(A) -> B>(self, f: F) -> Result<B, E> {
        self.map(f)
    }
}

fn main() {
    // All three use the SAME Functor trait — generic, not repeated
    let opt: Option<i32> = Some(21);
    let doubled = Functor::map(opt, |x| x * 2);  // Some(42)

    let v: Vec<i32> = vec![1, 2, 3, 4];
    let tripled = Functor::map(v, |x| x * 3);    // [3, 6, 9, 12]

    let r: Result<i32, &str> = Ok(10);
    let stringified = Functor::map(r, |x| x.to_string()); // Ok("10")
}
```

**The limitation:** You still can't write `fn map_twice<F: Functor>(container: F) -> F` because `F` at that point is a concrete type, not a type constructor. GATs let you describe the *output* type, but you can't yet write fully generic code like `fn works_for_any_functor<F<_>: Functor>(...)`. That would require native HKT.

## What This Unlocks

- **Generic combinators:** Write adapters, validators, transformers that work for `Option`, `Vec`, and `Result` through one trait implementation — eliminates copy-paste.
- **Monad-like abstractions:** GATs are the stepping stone toward simulating `Monad` traits (with `bind`/`flat_map`) generically — used in libraries like `higher` and `fp-core`.
- **Understanding Rust's future:** HKT has been a long-requested Rust feature. GATs are the current approximation. Understanding GATs deeply prepares you for the type-level abstractions that Rust is moving toward.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| HKT | Native: `'a t` — type constructor is first-class | Not native; simulated via GATs (`type Mapped<B>`) |
| Functor | `module type FUNCTOR = sig type 'a t; val fmap: ('a -> 'b) -> 'a t -> 'b t end` | `trait Functor { type Unwrapped; type Mapped<B>; fn map... }` |
| Abstraction over F | `functor Make(F: FUNCTOR) = ...` | Partially possible; fully generic code over F still limited |
| Stability | Native since OCaml 1.0 | GATs stabilized in Rust 1.65 (2022) |
| Output type | Inferred from `'b t` | Explicit: `Self::Mapped<B>` must be declared per impl |
| Complexity | Moderate — standard ML module system | High — GAT lifetime constraints add verbosity |
