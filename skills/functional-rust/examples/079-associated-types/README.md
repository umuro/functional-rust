# 079: Associated Types

**Difficulty:** 2  **Level:** Intermediate

Let a trait declare a placeholder type that each implementor fills in — the implementor decides what type it produces, not the caller.

## The Problem This Solves

When you implement `Iterator` for a custom type, you need to declare what type of values it yields. You could make this an extra type parameter: `trait Iterator<Item>` — but then every function that accepts an iterator must carry `Item` in its signature too, and you'd need to write `fn sum<I: Iterator<i32>>` everywhere.

Associated types solve this by binding the output type *to the implementation*. A `Counter` iterator always yields `u64`. That's not something the caller chooses — it's a property of `Counter`. So `type Item = u64` lives inside the `impl Iterator for Counter` block, and callers just write `I: Iterator` without needing to track the item type separately.

This is the same idea as OCaml module types: a module signature declares abstract type members that the implementing module fills in. The consumer of the module doesn't need to know the concrete type — they just use whatever the module provides.

## The Intuition

Type parameters say "the caller chooses." Associated types say "the implementor chooses."

```rust
// Extra type parameter — CALLER must specify what Item is
trait BadIterator<Item> { fn next(&mut self) -> Option<Item>; }

// Associated type — implementor specifies Item, caller doesn't need to
trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
```

In Python, this is roughly the difference between a function that takes an explicit type argument versus one that returns a type that's implicit from the object itself. In Java, it's like a generic interface where the type is fixed by the implementing class rather than left open.

## How It Works in Rust

```rust
trait Container {
    type Item;            // implementor declares what the item type is

    fn push(&mut self, item: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

struct Stack<T> { items: Vec<T> }

impl<T> Container for Stack<T> {
    type Item = T;        // Stack decides: its items are of type T

    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
}
```

```rust
// Multiple associated types — input and output can differ
trait Transformer {
    type Input;
    type Output;
    fn transform(&self, input: Self::Input) -> Self::Output;
}

struct StringLength;
impl Transformer for StringLength {
    type Input = String;
    type Output = usize;
    fn transform(&self, input: String) -> usize { input.len() }
}

// Generic function: use T::Input and T::Output — no extra type params needed
fn apply<T: Transformer>(t: &T, input: T::Input) -> T::Output {
    t.transform(input)
}
```

## What This Unlocks

- **The standard library's `Iterator` trait**: every iterator specifies its own `Item` type — `map`, `filter`, `collect` all work generically because they're written in terms of `Iterator::Item`.
- **The `Add` trait**: `impl Add for Vector2D` declares `type Output = Vector2D` — the result type is fixed by the implementor, not caller-specified.
- **Clean trait APIs**: when a trait has a natural "output type" concept (what a parser produces, what a serializer needs as input), associated types make the API read naturally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Abstract type in interface | `type t` in module signature | `type Item` in trait |
| Implementor fills in type | Module provides `type t = ...` | `impl Trait for Foo { type Item = ... }` |
| Access from outside | `M.t` | `T::Item` or `<T as Trait>::Item` |
| vs type parameter | Module functor parameter | Extra `<Item>` on the trait |
| Standard library use | `Map.S` with `type key` | `Iterator` with `type Item`, `Add` with `type Output` |
