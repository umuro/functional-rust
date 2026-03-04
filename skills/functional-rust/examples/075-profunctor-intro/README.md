# 075: Profunctor — Contramap Input, Map Output

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

A type `P<A, B>` that you can adapt on both ends — remap the input (contravariant) and remap the output (covariant) — the abstraction that unifies all optic types.

## The Problem This Solves

You have a function that does exactly what you need — but its input and output types are wrong for your context. Say you have a function that converts `String → usize` (string to length). Now you need to feed it `i32` instead of `String`, and you want `bool` (is it long?) instead of `usize`.

The naive solution: write a new function, or write a wrapper every time:

```rust
// You already have this:
fn str_len(s: String) -> usize { s.len() }

// Now you need: i32 -> bool (is digit-representation long?)
// So you write yet another function:
fn int_has_long_repr(n: i32) -> bool {
    str_len(n.to_string()) > 3
}
```

That's fine for one case. But what if you have dozens of variations? What if you want to build adapter pipelines systematically, or store partially-adapted functions in data structures?

You end up with scattered wrapper functions, or you reach for trait objects and lose type information. Neither is elegant.

The profunctor abstraction formalises this "adapt both ends" operation as `dimap`, giving you a unified, composable way to wire up transformations. It exists to solve exactly that pain.

## The Intuition

A **profunctor** is any type `P<A, B>` that supports two kinds of mapping:

1. **`rmap` (covariant)** — adapt the output. If you have `P<A, B>` and a function `B → D`, you get `P<A, D>`. This is like `map` on a regular functor.

2. **`lmap` (contravariant)** — adapt the input. If you have `P<A, B>` and a function `C → A` (note: going *backward*), you get `P<C, B>`. This is "pre-compose" — you plug in a converter *before* the profunctor runs.

3. **`dimap`** — both at once. `C → A` on the input, `B → D` on the output. Gives you `P<C, D>` from `P<A, B>`.

The most natural profunctor is a plain function `fn(A) -> B`:
- `rmap` post-composes: `rmap(f, g)` means "run f, then apply g to the result"
- `lmap` pre-composes: `lmap(g, f)` means "apply g to the input, then run f"
- `dimap` does both: `dimap(pre, post, f)` means "convert input with pre, run f, convert output with post"

**Analogy:** Think of a profunctor like a pipe fitting adapter. The pipe does something in the middle. `lmap` adds an adapter at the inlet (converts the incoming type). `rmap` adds an adapter at the outlet (converts the outgoing type). `dimap` does both.

```
   C ──[lmap/pre]──▶ A ──[profunctor]──▶ B ──[rmap/post]──▶ D
       (contravariant)                       (covariant)
```

The word "contravariant" just means the input adapter runs in the *opposite* direction of what you'd expect — you provide `C → A` (not `A → C`) to make a `P<C, B>` from `P<A, B>`.

## How It Works in Rust

```rust
// The core profunctor type: wraps a function A -> B
pub struct Mapper<A, B> {
    f: Box<dyn Fn(A) -> B>,
}

impl<A: 'static, B: 'static> Mapper<A, B> {
    pub fn new<F: Fn(A) -> B + 'static>(f: F) -> Self {
        Mapper { f: Box::new(f) }
    }

    pub fn apply(&self, a: A) -> B {
        (self.f)(a)
    }

    // dimap: adapt BOTH input and output at once
    // pre:  converts new input type C into A (what the function expects)
    // post: converts B (what the function produces) into new output type D
    pub fn dimap<C: 'static, D: 'static>(
        self,
        pre: impl Fn(C) -> A + 'static,
        post: impl Fn(B) -> D + 'static,
    ) -> Mapper<C, D> {
        Mapper::new(move |c| post((self.f)(pre(c))))
        //                     ^^^^ post . self.f . pre
    }

    // lmap: adapt only the INPUT (contravariant) — dimap pre id
    pub fn lmap<C: 'static>(self, pre: impl Fn(C) -> A + 'static) -> Mapper<C, B> {
        Mapper::new(move |c| (self.f)(pre(c)))
    }

    // rmap: adapt only the OUTPUT (covariant) — dimap id post
    pub fn rmap<D: 'static>(self, post: impl Fn(B) -> D + 'static) -> Mapper<A, D> {
        Mapper::new(move |a| post((self.f)(a)))
    }
}

// Example: start with String -> String (uppercase)
let upper = Mapper::new(|s: String| s.to_uppercase());

// lmap: feed it i32 instead of String
let int_upper = Mapper::new(|s: String| s.to_uppercase())
    .lmap(|n: i32| n.to_string());
// int_upper: i32 -> String
int_upper.apply(42);  // "42"

// rmap: get length instead of String
let upper_len = Mapper::new(|s: String| s.to_uppercase())
    .rmap(|s: String| s.len());
// upper_len: String -> usize
upper_len.apply("hello".to_string());  // 5

// dimap: i32 in, length out
let int_upper_len = Mapper::new(|s: String| s.to_uppercase())
    .dimap(|n: i32| n.to_string(), |s: String| s.len());
// int_upper_len: i32 -> usize
int_upper_len.apply(42);  // 2  ("42".to_uppercase().len())

// Star: a profunctor whose output is wrapped in Option
// Useful for fallible transformations
pub struct Star<A, B> {
    run: Box<dyn Fn(A) -> Option<B>>,
}
// lmap/rmap work the same way — rmap uses .map() on the Option
let parse_int = Star::new(|s: String| s.parse::<i32>().ok());
let parse_double = parse_int.rmap(|n| n * 2);
parse_double.apply("21".to_string());  // Some(42)
parse_double.apply("x".to_string());   // None
```

## What This Unlocks

- **Optic encoding** — Profunctors are the foundation of "profunctor optics" (see examples 238, 621). A Lens is a Strong profunctor; a Prism is a Choice profunctor. Knowing `dimap` means understanding why all optic types are really the same thing.
- **Middleware / adapter pipelines** — When building processing pipelines (HTTP middleware, event handlers, codec adapters), profunctor `dimap` lets you compose adapters systematically instead of writing boilerplate wrappers.
- **Generic transformers** — A function parameterised over any profunctor `P` automatically works for plain functions (fast, zero-cost) and effectful wrappers like `Star` (with `Option`/`Result` output) without code duplication.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Profunctor trait | Type class `Profunctor` with `dimap` | Trait with `dimap` method (GAT limitations apply) |
| Function as profunctor | Natural: `(->) a b` is a profunctor instance | Explicit wrapper `Mapper<A, B>` around `Box<dyn Fn(A)->B>` |
| `lmap` / `rmap` | Derived from `dimap` in the type class | Implemented as separate methods for ergonomics |
| Star (lifted profunctor) | `Star f a b = Kleisli f a b` via `newtype` | `Star<A, B>` struct wrapping `Box<dyn Fn(A)->Option<B>>` |
| HKT / polymorphism | Full higher-kinded types via functors | No HKT; must specialise per concrete wrapper type |
