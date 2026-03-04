# 237: Day Convolution

**Difficulty:** ⭐⭐⭐⭐  **Level:** Category Theory

Combine two functors into a new functor by pairing their contents with a combining function — the foundation of applicative functors.

## The Problem This Solves

You have two containers (functors) and you want to combine their contents in all possible ways. `Vec<fn(A)->B>` applied to `Vec<A>` should give `Vec<B>` — every function applied to every element. `Option<i32>` combined with `Option<i32>` via addition gives `Option<i32>` — `Some` only if both are `Some`. But what's the *general* structure behind all these "combining two functors" patterns?

Day convolution is that structure. It says: to combine functors `F` and `G`, hold three things together: a value from `F<B>`, a value from `G<C>`, and a function `B -> C -> A`. The result is a value that represents "all ways to combine one element from F and one element from G using the combining function." Run it (`lower()`) and you get the concrete `A` values.

This is the formal foundation of applicative functors: a functor is applicative (has `<*>`) if and only if Day convolution over it forms a monoid. In practice, this explains why `Vec`'s applicative is the cartesian product and `Option`'s applicative is the `zip` (both present = result, otherwise nothing).

## The Intuition

Convolution in signal processing: to combine two signals, you slide one over the other and sum the products at each offset. Day convolution is the categorical analogue: to combine two functors, you pair all elements from one with all elements from the other via a combining function.

Concrete: `Day(Vec<B>, Vec<C>, B -> C -> A)` means "for every `b` in the left Vec and every `c` in the right Vec, apply the combiner to get an `A`." Lower it and you get `Vec<A>` containing all `b×c` combinations.

For `Option`: `Day(Some(3), Some(4), +)` = `Some(7)`. `Day(Some(3), None, +)` = `None`. Because with `Option`, "all combinations" means 0 combinations if either side is empty.

The "monoidal" structure: Day convolution is *associative* — `Day(Day(F, G), H) ≅ Day(F, Day(G, H))`. And there's a unit: the *identity functor* (just wrapping a value). A monoid in the category of functors under Day convolution = an applicative functor. That's the theorem.

## How It Works in Rust

```rust
/// Day convolution: holds two functor values and a combining function
pub struct Day<B, C, A> {
    left:    Vec<B>,
    right:   Vec<C>,
    combine: Box<dyn Fn(B, C) -> A>,
}

impl<B: Clone, C: Clone, A> Day<B, C, A> {
    pub fn new(left: Vec<B>, right: Vec<C>, combine: impl Fn(B, C) -> A + 'static) -> Self {
        Day { left, right, combine: Box::new(combine) }
    }

    /// Lower: compute the cartesian product applying the combiner
    pub fn lower(self) -> Vec<A> {
        let mut result = Vec::new();
        for b in &self.left {
            for c in &self.right {
                result.push((self.combine)(b.clone(), c.clone()));
            }
        }
        result  // all combinations
    }

    /// fmap: post-compose a function on the result type
    pub fn fmap<D>(self, f: impl Fn(A) -> D + 'static) -> Day<B, C, D> {
        let old_combine = self.combine;
        Day {
            left: self.left,
            right: self.right,
            combine: Box::new(move |b, c| f(old_combine(b, c))),
        }
    }
}
```

The list applicative IS Day convolution:
```rust
// Apply list of functions to list of values — this is exactly Day
fn apply_list<A: Clone, B>(fs: &[fn(A) -> B], xs: &[A]) -> Vec<B> {
    fs.iter().flat_map(|f| xs.iter().map(|x| f(x.clone()))).collect()
    // Same as Day::new(fs.to_vec(), xs.to_vec(), |f, x| f(x)).lower()
}

// Option convolution — both must be Some
fn day_option<B, C, A>(lb: Option<B>, rc: Option<C>, f: impl Fn(B, C) -> A) -> Option<A> {
    lb.zip(rc).map(|(b, c)| f(b, c))
}
```

## What This Unlocks

- **Understanding applicative functors** — `<*>` in any applicative is Day convolution specialized to that functor. The cartesian product for `Vec`, zipping for `Option`, and parallel combination for `Result` all have the same categorical explanation.
- **Composable effects** — Day convolution lets you combine two independent effects into one. This is the correct structure for "run A and B independently, then combine results" (as opposed to monadic sequencing, which is dependent).
- **Profunctor optics** — Day convolution appears in the construction of `Conjoined` in Haskell's `lens` library and in the theoretical foundations of profunctor optics.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Day type | GADT or existential module | Struct with `Box<dyn Fn>` |
| `lower` | Fold via module interface | Nested loops, explicit clone |
| List applicative | `List.concat_map` | `flat_map` / `Day::lower` |
| Option applicative | `Option.bind` / custom | `.zip().map()` |
| Unit functor | Identity module | Implicit (wrapping `A` directly) |
