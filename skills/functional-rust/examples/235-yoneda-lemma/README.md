# 235: Yoneda Lemma

**Difficulty:** 4  **Level:** Expert

Represent a container as a "pending computation" so that multiple map operations fuse into a single pass.

## The Problem This Solves

Imagine processing a large list with several transformations:

```rust
data.iter()
    .map(|x| x * 2)
    .map(|x| x + 1)
    .map(|x| x * x)
    .collect::<Vec<_>>()
```

Rust's iterator is lazy, so these three `.map()` calls DO fuse into one pass — but only because the standard library is built for this. What if you're working with a custom container type that isn't lazy? Or you want to accumulate transformations and decide later when to apply them? You'd end up with intermediate allocations or multiple traversals.

More fundamentally: what if you want a generic "deferred map" that works for *any* container type? The moment you store the container to apply functions later, you've fixed the type. The moment you try to make it generic, Rust's type system says no.

The Yoneda Lemma offers a precise mathematical answer to this problem: any container `F<A>` can be represented equivalently as a function `(A -> B) -> F<B>` for all possible `B`. Instead of storing the data directly, store a closure that accepts any mapping function and returns the mapped container. The Yoneda Lemma says these two representations are *exactly equivalent* — no information is lost.

The practical payoff: you can chain multiple map calls by composing closures (zero-cost), then apply everything in one pass at the end. This exists to solve exactly that pain.

## The Intuition

Imagine you're a travel agent. Instead of booking a specific flight, you give the client a voucher: "bring me any destination and I'll book you there." The voucher IS the trip, in a deferred form. You can modify the voucher (attach conditions: "only economy class") without booking anything. When the client finally chooses, you do exactly one booking.

In Rust terms: instead of storing `Vec<A>`, store a closure `Box<dyn Fn(impl Fn(A) -> B) -> Vec<B>>`. This closure *is* the data, in deferred form. "Mapping" means wrapping the closure, not iterating. Only when you call `.lower()` does actual work happen.

The **Yoneda Lemma** says these two are isomorphic:
- Storing `Vec<A>` directly  
- Storing a closure that maps any function over it

`to_yoneda` converts left→right (wraps data in closure). `from_yoneda` converts right→left (applies identity function to recover data). They're inverses.

## How It Works in Rust

```rust
// The Yoneda representation: stores original data + accumulated composed function
struct FusedYoneda {
    original: Vec<i64>,
    transform: Box<dyn Fn(i64) -> i64>,  // composed from all fmap calls
}

impl FusedYoneda {
    // Lift: wrap plain data into the Yoneda representation
    // Start with identity transform (do nothing yet)
    fn lift(data: Vec<i64>) -> Self {
        FusedYoneda {
            original: data,
            transform: Box::new(|x| x),  // identity — no-op
        }
    }

    // fmap: compose new function onto the existing one — NO ITERATION
    // This is O(1) per call regardless of data size
    fn fmap(self, f: impl Fn(i64) -> i64 + 'static) -> Self {
        let prev = self.transform;
        FusedYoneda {
            original: self.original,  // data untouched
            // compose: new_f(old_f(x)) — just closure composition
            transform: Box::new(move |x| f(prev(x))),
        }
    }

    // Lower: apply ALL composed functions in ONE traversal
    fn lower(self) -> Vec<i64> {
        self.original.into_iter().map(|x| (self.transform)(x)).collect()
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // Three fmaps — zero iterations until .lower()
    let result = FusedYoneda::lift(data)
        .fmap(|x| x * 2)   // just composes closure
        .fmap(|x| x + 1)   // just composes closure
        .fmap(|x| x * x)   // just composes closure
        .lower();           // ONE pass: applies all three at once

    // [9, 25, 49, 81, 121]
}
```

**Why Rust can't do full Yoneda:** The true Yoneda type needs `forall B. (A -> B) -> F<B>`, universal quantification over `B`. Rust can't express this directly (no higher-ranked type parameters over type constructors). So this example fixes `F = Vec` and demonstrates the core operational benefit — lazy map fusion.

## What This Unlocks

- **Custom lazy containers:** Build a container that accumulates transformations and applies them in one pass, without relying on `Iterator` laziness.
- **Builder patterns with deferred computation:** Accumulate configuration/transformation steps cheaply, execute once when "built."
- **Understanding iterator fusion:** Rust's `Iterator::map` works this way internally — Yoneda explains *why* chained `.map()` calls don't allocate intermediate `Vec`s.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Full Yoneda type | `type ('f, 'a) yoneda = { run: 'b. ('a -> 'b) -> 'f 'b }` — native HKT | Not expressible; fix `F = Vec`, simulate with closures |
| `forall b` | Natively polymorphic record field | Requires concrete type or trait object workaround |
| Function composition | `fun f g x -> f (g x)` | `move \|x\| f(prev(x))` — closure capturing `prev` |
| Roundtrip law | `runYoneda (toYoneda fa) id = fa` | `from_yoneda(to_yoneda(data)) == data` ✓ |
| Practical value | Haskell/OCaml use full Yoneda for generic fusion | Rust achieves same via `Iterator` trait; Yoneda explains the principle |
