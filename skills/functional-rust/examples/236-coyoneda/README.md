# 236: Coyoneda

**Difficulty:** 4  **Level:** Expert

Make any type constructor act like a functor for free, with automatic map fusion as a bonus.

## The Problem This Solves

Suppose you have a custom type — say a priority queue, a graph node, or a database cursor — that doesn't implement `map`. You can't call `.map()` on it. You have to manually unpack it, transform the values, and repack. That's annoying, but manageable.

Now suppose you need to apply several transformations in sequence. Each one unpacks and repacks. Worse, if the container is expensive to traverse (a lazy I/O stream, a tree with millions of nodes), you've paid the traversal cost multiple times for what is conceptually one operation.

The deeper problem: you'd like to write generic code that works with "any container that can be mapped" — but your type doesn't have a `Functor` instance. You can't use any of the generic infrastructure.

Coyoneda solves both problems at once. It wraps any type constructor (whether or not it has `map`) and gives you a `fmap` that defers all work. You compose as many maps as you want — zero traversals. Only when you call `lower()` does the container get traversed exactly once, with all transformations fused into a single function. This exists to solve exactly that pain.

## The Intuition

Think of Coyoneda as a "pending receipt" for a series of operations on a container.

When you buy something and request a refund, the store doesn't immediately restock the item and re-issue currency. They give you a receipt that says "you are owed X." You can endorse that receipt (add conditions), give it to someone else — all without touching the actual inventory.

`Coyoneda::lift(data)` creates the receipt. Each `.fmap(f)` adds an endorsement (composes `f` into a pending function). `.lower()` is when you cash it in — the container is traversed *once* with the fully composed function applied.

The "free functor" part: even if your original container type has no `map`, Coyoneda wraps it and provides one. You get functor behavior for free, just by wrapping in Coyoneda.

```
Original data:  [1, 2, 3, 4, 5]
                     ↓ lift
Coyoneda:       (data=[1..5], transform=identity)
                     ↓ .fmap(|x| x*2)      — no traversal yet
Coyoneda:       (data=[1..5], transform=|x| x*2)
                     ↓ .fmap(|x| x+1)      — no traversal yet
Coyoneda:       (data=[1..5], transform=|x| x*2+1)
                     ↓ .lower()            — ONE traversal
Result:         [3, 5, 7, 9, 11]
```

## How It Works in Rust

```rust
// Coyoneda wraps any Vec<A> and holds the pending transformation
pub struct Coyoneda<A> {
    // A boxed closure: when called, applies ALL pending fmaps and returns Vec<A>
    lower_fn: Box<dyn FnOnce() -> Vec<A>>,
}

impl<A: 'static> Coyoneda<A> {
    // Lift: wrap plain data — transformation is identity (do nothing yet)
    pub fn lift(data: Vec<A>) -> Self {
        Coyoneda {
            lower_fn: Box::new(move || data),  // closure owns the data
        }
    }

    // fmap: compose new function AROUND the existing closure — no traversal
    // The key: this is O(1), regardless of how large the data is
    pub fn fmap<B: 'static>(self, f: impl Fn(A) -> B + 'static) -> Coyoneda<B> {
        Coyoneda {
            lower_fn: Box::new(move || {
                let data = (self.lower_fn)();       // get data from inner closure
                data.into_iter().map(f).collect()   // apply f
            }),
            // NOTE: the inner closure isn't called yet — this is still deferred
        }
    }

    // Lower: execute everything — the one and only traversal
    pub fn lower(self) -> Vec<A> {
        (self.lower_fn)()
    }
}

fn main() {
    // Three fmaps — but data is only traversed ONCE at .lower()
    let result = Coyoneda::lift(vec![1_i32, 2, 3, 4, 5])
        .fmap(|x| x * 2)          // pending
        .fmap(|x| x + 1)          // pending
        .fmap(|x| x.to_string())  // pending — note: type changes from i32 to String
        .lower();                  // executed: ["3", "5", "7", "9", "11"]
}
```

The "explicit" version in the code makes the structure even clearer — it stores the original data and a composed `i32 -> A` function as separate fields, so you can see exactly what's accumulated before `.lower()` runs.

**Rust limitation:** The true Coyoneda type is `∃B. (B -> A, F<B>)` — an *existential* type hiding the intermediate type `B`. Rust doesn't have native existential types, so we use trait objects (`Box<dyn Fn>`) to erase the intermediate type. This works correctly but loses some type information.

## What This Unlocks

- **Lazy transformation pipelines:** Build a sequence of transforms on expensive containers (files, network streams, trees) and pay the traversal cost exactly once.
- **Generic functor infrastructure:** Wrap any type in `Coyoneda` to use it with code that requires `fmap` — even types you don't control.
- **Understanding Rust's iterator fusion:** `Iterator` chains in Rust work by exactly this principle — `.map().map().filter()` fuses into one loop because each adapter wraps the previous in a closure.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Full Coyoneda | `type ('f,'a) coyoneda = Coyoneda: ('b -> 'a) * 'f 'b -> ('f,'a) coyoneda` — native existential | Simulated via `Box<dyn FnOnce() -> Vec<A>>` |
| Type `F` | Polymorphic over any functor `F` | Fixed to `Vec` (HKT limitation) |
| Existential `∃B` | Native GADT encoding | Type-erased via trait objects |
| `fmap` cost | O(1) — composes functions | O(1) — wraps closure |
| `lower` cost | O(n) — one traversal | O(n) — one traversal |
| Use in ecosystem | Haskell: `Data.Functor.Coyoneda` | Rust iterator adapters use same principle |
