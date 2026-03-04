# 051: Functors Introduction

**Difficulty:** ⭐⭐ Intermediate  **Level:** Intermediate

How to make any container type "mappable" — and why you'd want to.

## The Problem This Solves

You have a `Maybe<i32>` (a value that might not exist). You want to double it. So you write:

```rust
let doubled = match maybe_value {
    None => None,
    Some(x) => Some(x * 2),
};
```

Fine. But now you want to convert it to a string. Another match. Then you want to add 1. Another match. Every transformation forces you to unwrap, do the thing, re-wrap. The boilerplate multiplies with every step.

Now imagine the same problem with a binary tree: you want to multiply every node value by 10. You write a recursive function. Then you want to convert every node to a string. Another recursive function. The structure of the traversal is identical every time — only the operation changes — yet you rewrite it from scratch each time.

The insight is that "apply a function to the value(s) inside a container, without changing the container's shape" is a single idea that appears constantly. A Functor is just a container type that makes this idea explicit with a `map` method. You already use Functors every day: `Option::map`, `Vec::map` (via iterators), `Result::map`. This example shows you what's happening under the hood and how to add that power to your own types.

## The Intuition

Think of a shipping box. The box doesn't care what's inside — you can put books in, take books out, or swap books for DVDs. The box stays the same; only the contents change.

A Functor is exactly that: a "box" (or container) where you can transform the contents without touching the box itself. The `map` method is the operation that reaches inside, applies your function, and hands you back a box of the same shape with transformed contents.

```
Maybe::Just(5) --[multiply by 2]--> Maybe::Just(10)
Maybe::Nothing --[multiply by 2]--> Maybe::Nothing   ← box shape preserved
```

The `Nothing` case shows why this is powerful: `map` handles the "empty box" case automatically. You never have to check manually. The container's logic (is it empty? is it a node or a leaf?) lives in `map`, not scattered through your code.

In Rust terms: a type `T<A>` is a Functor when it has a `map` method that takes a function `fn(A) -> B` and returns `T<B>`. Same container shape, different contents.

## How It Works in Rust

**Step 1 — The simplest functor: a Maybe type**

```rust
enum Maybe<T> { Nothing, Just(T) }

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Nothing => Maybe::Nothing,  // empty stays empty
            Maybe::Just(x) => Maybe::Just(f(x)), // apply f to the inside
        }
    }
}
```

That's all a Functor is: a `map` that applies a function to the inside. Now you can chain:

```rust
Maybe::Just(3)
    .map(|x| x + 1)   // Just(4)
    .map(|x| x * 2)   // Just(8)
    .map(|x| x.to_string()); // Just("8")
```

No unwrapping. No re-wrapping. Just transformations.

**Step 2 — A Functor trait (so any type can participate)**

```rust
trait Functor {
    type Inner;           // the A in T<A>
    type Mapped<U>;       // what T<B> looks like after mapping
    fn fmap<U, F: FnOnce(Self::Inner) -> U>(self, f: F) -> Self::Mapped<U>;
}
```

The `type Mapped<U>` is a Generic Associated Type (GAT) — Rust's way of expressing "a version of this container holding a different type." It's what makes `Maybe<i32>` become `Maybe<String>` after a map.

**Step 3 — Implementing it for a tree**

```rust
enum Tree<T> { Leaf, Node(Box<Tree<T>>, T, Box<Tree<T>>) }

impl<T> Tree<T> {
    fn map<U, F: Fn(&T) -> U>(&self, f: &F) -> Tree<U> {
        match self {
            Tree::Leaf => Tree::Leaf,
            Tree::Node(l, v, r) => Tree::Node(
                Box::new(l.map(f)),  // recurse left
                f(v),                // transform value
                Box::new(r.map(f)),  // recurse right
            ),
        }
    }
}
```

The recursion is written once. Every future transformation on a tree is just `.map(|x| ...)`. The traversal logic never needs to be repeated.

**What breaks if you skip the pattern:** You end up with `map_tree_to_double`, `map_tree_to_string`, `map_tree_to_whatever` — one function per transformation. The container's traversal logic gets duplicated across your codebase.

## What This Unlocks

- **Chain transformations without manual unwrapping.** `option.map(f).map(g)` is readable and safe — no nested matches.
- **Write traversal logic once.** Any new operation on `Tree` or `Maybe` is just a lambda passed to `.map()`.
- **Generic code over any Functor.** Once you have the trait, you can write algorithms that work on `Maybe`, `Tree`, `Vec`, and your own types without specializing for each.

Real codebases where this pattern appears: Rust's `Iterator` (the entire iterator adapter chain is functor composition), `Option`/`Result` error handling, parser combinators (`nom`, `winnow`), and async `Future` transformations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Functor interface | `module type FUNCTOR` with `type 'a t` and `val map` | `trait Functor` with GATs (`type Mapped<U>`) |
| Higher-kinded types | Native — `'a list`, `'a option` are type constructors | Simulated with GATs — not first-class in the type system |
| Ownership | Values are immutable and shared; `map` is referentially transparent | `map` consumes `self` by default (moves the value inside) |
| Heap allocation | Trees/linked structures are automatic; no explicit boxing | Recursive types need `Box<T>` to have a known size |
| Calling convention | `Maybe.map f x` (function first, then container) | `x.map(f)` (method syntax, container first) |
