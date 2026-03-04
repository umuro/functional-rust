# 144: GAT Basics — Generic Associated Types

**Difficulty:** 4  **Level:** Advanced

Associated types that are themselves generic over lifetimes or type parameters — enabling lending iterators, typed expression trees, and iterator-of-iterators patterns.

## The Problem This Solves

Standard associated types have a fundamental limitation: they can't borrow from `self`. Consider trying to write a `LendingIterator` trait where `next()` returns a reference into the iterator's own buffer. With a regular associated type `type Item`, you can't express "Item borrows from self with lifetime 'a" — the lifetime has nowhere to live. You'd be forced to return owned data or clone.

GATs (Generic Associated Types, stable since Rust 1.65) solve this by letting you write `type Item<'a> where Self: 'a`. Now `Item<'a>` is a type that depends on a lifetime — a "higher-kinded" associated type. An iterator over window slices can yield `&'a [T]` that borrows from its own internal slice. A mutable iterator can yield `&'a mut T`. These weren't expressible before GATs.

The second problem this example addresses is Rust's equivalent of OCaml GADTs: typed expression trees where different constructors return different types. OCaml's `type _ expr = Int : int -> int expr | Bool : bool -> bool expr` guarantees at the type level that `eval_int` returns `int` and `eval_bool` returns `bool`. Rust achieves the same separation with two distinct enum types (`IntExpr` and `BoolExpr`) that reference each other — the compiler enforces the same guarantee without language-level GADTs.

## The Intuition

GATs let an associated type be generic over a lifetime — so it can borrow from `self`, which regular associated types cannot do.

## How It Works in Rust

```rust
// Regular Iterator can't yield references into self.
// LendingIterator can — because Item<'a> borrows for lifetime 'a.

pub trait LendingIterator {
    type Item<'a> where Self: 'a;  // 'a is the GAT lifetime parameter
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

// Window iterator: yields overlapping slices of the source data
pub struct Windows<'s, T> {
    slice: &'s [T],
    size: usize,
    pos: usize,
}

impl<'s, T> LendingIterator for Windows<'s, T> {
    type Item<'a> = &'a [T] where Self: 'a;
    // Item borrows from self — impossible without GATs

    fn next(&mut self) -> Option<&[T]> {
        if self.pos + self.size <= self.slice.len() {
            let window = &self.slice[self.pos..self.pos + self.size];
            self.pos += 1;
            Some(window)
        } else {
            None
        }
    }
}

let data = vec![1, 2, 3, 4, 5];
let mut win = Windows::new(&data, 3);
while let Some(w) = win.next() {
    println!("{:?}", w);  // [1,2,3], [2,3,4], [3,4,5]
}

// Typed expression tree (OCaml GADT equivalent):
// eval_int can ONLY accept IntExpr — BoolExpr is a type error.
// eval_bool can ONLY accept BoolExpr.
pub enum IntExpr {
    Lit(i64),
    Add(Box<IntExpr>, Box<IntExpr>),
    If(Box<BoolExpr>, Box<IntExpr>, Box<IntExpr>),  // condition is BoolExpr!
}
pub enum BoolExpr {
    Lit(bool),
    Eq(Box<IntExpr>, Box<IntExpr>),   // compares integers, returns bool
    Not(Box<BoolExpr>),
}

fn eval_int(e: &IntExpr) -> i64 { /* ... */ }
fn eval_bool(e: &BoolExpr) -> bool { /* ... */ }
// eval_int(BoolExpr::Lit(true))  // ERROR — type mismatch, just like OCaml GADTs
```

## What This Unlocks

- **Lending iterators** — yield references into internal buffers, windows, or cursors without forcing clones.
- **Async iterators** — GATs are the foundation of `async fn` in traits and the async iterator design.
- **Typed ASTs** — separate `IntExpr` / `BoolExpr` types give the same compile-time guarantees as OCaml GADTs without a language extension.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Typed expression trees | GADT (`type _ expr`) | Separate enum types referencing each other |
| Associated type over lifetime | Not applicable (GC) | GAT: `type Item<'a> where Self: 'a` |
| Lending iterator | Standard iterator works (GC) | Requires GAT — regular associated type can't borrow from self |
| Stable availability | Always | GATs stable since Rust 1.65 (2022) |
