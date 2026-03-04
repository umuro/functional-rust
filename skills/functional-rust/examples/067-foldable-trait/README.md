# 067: Foldable Trait

**Difficulty:** 2  **Level:** Intermediate

Define a `Foldable` trait so any container — list, tree, set — can be reduced to a single value using a binary function.

## The Problem This Solves

You have a custom linked list and a binary tree. Both can be summed. Both have a length. Both can be converted to a `Vec`. Without a shared abstraction, you write `sum_list`, `sum_tree`, `length_list`, `length_tree` as separate functions that share identical structure but different traversal paths.

The `Foldable` typeclass (trait in Rust) captures the *capability* of being reduced by a fold. Any type that implements `fold_left` and `fold_right` gets `sum`, `length`, `to_vec` — and any generic function that folds — for free through default methods.

This is the Haskell `Foldable` typeclass translated to Rust: the minimal interface is fold, and everything else derives from it.

## The Intuition

A `Foldable` is anything that can be "consumed" by a fold function — a binary operation and an initial value — to produce a single result. Think of it as "things you can iterate over and reduce."

Lists are obviously foldable. Trees are foldable (in-order traversal reduces to the same interface). Any container that has a meaningful linear traversal is foldable.

The power: write `sum` once, with a `Foldable` trait bound. It works on lists, trees, or any new container you add later.

## How It Works in Rust

```rust
trait Foldable {
    type Item;
    fn fold_left<B, F: FnMut(B, &Self::Item) -> B>(&self, init: B, f: F) -> B;
    fn fold_right<B, F: FnMut(&Self::Item, B) -> B>(&self, init: B, f: F) -> B;

    // Derived operations — free for all implementors:
    fn length(&self) -> usize {
        self.fold_left(0, |acc, _| acc + 1)
    }
    fn to_vec(&self) -> Vec<Self::Item> where Self::Item: Clone {
        self.fold_right(Vec::new(), |x, mut acc| { acc.insert(0, x.clone()); acc })
    }
}

// One generic function that works on any Foldable<Item = i32>:
fn sum<F: Foldable<Item = i32>>(foldable: &F) -> i32 {
    foldable.fold_left(0, |acc, x| acc + x)
}

// Both list and tree use the same sum():
sum(&list)  // works
sum(&tree)  // works — same function, different structure
```

The `associated type Item` lets the trait be generic over what's stored. Default methods in the trait provide derived operations — if you implement `fold_left` and `fold_right`, you get everything else.

## What This Unlocks

- **Generic algorithms over containers** — write `sum`, `any`, `all`, `count` once; apply to any `Foldable`.
- **The Haskell Foldable pattern in Rust** — understanding this bridges FP typeclasses to Rust traits.
- **Default method derivation** — traits with default methods as a mechanism to get free behaviour from a minimal interface.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Typeclass mechanism | Module type (`FOLDABLE`) | Trait with associated type |
| Default methods | Not in module types | `fn length(&self) { ... }` in trait body |
| Generic over Item | Parametric `'a t` | `type Item` associated type |
| First-class modules | Used for polymorphism | Trait bounds on generic functions |
| Tree nodes | `type 'a tree = Leaf \| Node of ...` | `enum Tree<T>` with `Box<Tree<T>>` |
