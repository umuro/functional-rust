# 390: Type Alias impl Trait (TAIT)

**Difficulty:** 4  **Level:** Expert

Name an opaque type so you can refer to it in multiple places — the named version of `impl Trait`.

## The Problem This Solves

`impl Trait` in return position lets you hide a concrete type: `fn make_iter() -> impl Iterator<Item = i32>`. That's great for one function. But what if two functions return *the same* opaque type and you want to store them in a struct, pass them to a third function, or use the type in a trait definition? Anonymous `impl Trait` can't be named — every occurrence is a fresh, distinct opaque type.

Type Alias impl Trait (TAIT) solves this: `type MyIter = impl Iterator<Item = i32>`. Now `MyIter` is a stable name for one concrete type that satisfies `Iterator<Item = i32>`. Multiple functions can return `MyIter`, structs can store `MyIter` fields, and the compiler infers the concrete type from the definitions — you never write it explicitly.

TAIT also unlocks self-referential and recursive type definitions that were impossible with anonymous `impl Trait`. The feature stabilized in Rust 1.75 as part of RPITIT (return-position impl trait in traits), making it available in stable Rust.

## The Intuition

Think of TAIT as giving a nickname to "whatever concrete type the compiler figures out here." The compiler still knows the exact type — it's just you, the programmer, who doesn't write it. The alias is opaque to callers (they see the trait bound) but transparent to the compiler (it resolves to the monomorphic type).

The key distinction from `Box<dyn Iterator>`: TAIT is still zero-cost. No heap allocation, no dynamic dispatch. The concrete type is fully known at compile time — it's just hidden behind an alias.

## How It Works in Rust

```rust
// Without TAIT: each function returns a distinct anonymous type
fn counter() -> impl Iterator<Item = i32> { 0..10 }
fn evens() -> impl Iterator<Item = i32> { (0..20).filter(|x| x % 2 == 0) }
// Can't unify these — different opaque types

// With TAIT (Rust 1.75+):
type Counts = impl Iterator<Item = i32>;

fn counter() -> Counts { 0..10 }
// Another function can also return Counts — same concrete type

// Stable approximation with Box<dyn>:
type BoxedIter<T> = Box<dyn Iterator<Item = T>>;

fn range_iter(n: i32) -> BoxedIter<i32> { Box::new(0..n) }
fn evens_iter(n: i32) -> BoxedIter<i32> { Box::new((0..n).filter(|x| x % 2 == 0)) }
// Now both share the BoxedIter<i32> type — storable, passable, consistent
```

## What This Unlocks

- **Stable naming for opaque types** — structs and traits can reference the same opaque iterator/closure type across multiple functions without boxing
- **Zero-cost abstraction** — unlike `Box<dyn Trait>`, TAIT involves no heap allocation and no dynamic dispatch; the concrete type is monomorphized
- **Trait method return types** — RPIT in traits (impl Trait in trait method signatures) became stable in Rust 1.75, making TAIT the foundation for async and iterator traits

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Opaque type | `module type S = sig type t end` with ascription | `type Alias = impl Trait` |
| Multiple uses | Module sharing / functor argument | Type alias reused across functions |
| Concrete type | Known inside module, hidden outside | Inferred by compiler, hidden from programmer |
| Dynamic dispatch | N/A (no vtable by default) | `Box<dyn Trait>` (heap, dynamic) |
| Zero-cost opaque | Module ascription (stack, monomorphic) | TAIT (stack, monomorphic) |
