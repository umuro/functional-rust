# 613: Futumorphism

**Difficulty:** 5  **Level:** Master

An anamorphism (unfold) that can produce multiple levels at once — the dual of histomorphism, using a `Free` comonad structure.

## The Problem This Solves

Anamorphism (`ana`) unfolds a seed value one step at a time: given a seed, produce one layer of the structure and a new seed for the next. This is fine for simple sequences, but some generation patterns are naturally multi-step. Generating Fibonacci numbers requires two previous values; generating bit patterns requires looking two positions ahead; tokenizing a string may produce multiple tokens per source character.

With `ana`, multi-step generation requires threading awkward state through the seed type. You end up encoding "I already generated the next element, please use it" in the seed, which is a code smell. The futumorphism makes this formal: the coalgebra can return a `Free F A` value that contains either a seed for further unfolding (`Pure(seed)`) or a pre-built layer with possibly more seeds embedded (`Free(layer)`).

This is the dual of histomorphism: histo gives the fold access to all *previous* results; futu gives the unfold the ability to produce *multiple future* steps. Together they form the "past and future" recursion scheme pair.

## The Intuition

A futumorphism is an anamorphism where the coalgebra can "pre-produce" multiple layers at once instead of just one seed — by returning a `Free` value that interleaves "I have a ready-made layer here" with "continue unfolding from this seed". The trade-off: more expressive than `ana` but harder to reason about; use `ana` when you generate one step at a time, `futu` when the natural decomposition produces multiple steps.

## How It Works in Rust

```rust
// Free monad over F: either a pure value (seed for continuation)
// or a layer with Free values in the recursive positions
enum Free<A> {
    Pure(A),              // "continue unfolding from seed A"
    Cons(i32, Box<Free<A>>),  // "here's a ready-made layer; recurse into inner Free"
}

// The unfolded list result
type List = Vec<i32>;

// Futumorphism: coalgebra returns Free<Seed> — can produce multiple steps
fn futu<S>(seed: S, coalg: &impl Fn(S) -> Free<S>) -> List {
    let mut result = Vec::new();
    futu_inner(coalg(seed), coalg, &mut result);
    result
}

fn futu_inner<S>(free: Free<S>, coalg: &impl Fn(S) -> Free<S>, out: &mut Vec<i32>) {
    match free {
        Free::Pure(seed) => {
            // Continue unfolding from the new seed
            futu_inner(coalg(seed), coalg, out);
        }
        Free::Cons(value, rest) => {
            // Pre-built layer — emit this value and continue with rest
            out.push(value);
            futu_inner(*rest, coalg, out);
        }
    }
}

// Example: generate pairs [n, n+1, n+2, ...] stepping by 2
// The coalgebra produces TWO elements at once
fn pair_coalg(seed: i32) -> Free<i32> {
    if seed > 10 {
        return Free::Pure(seed); // actually, terminate somehow
    }
    // Produce two elements in one coalgebra call
    Free::Cons(seed, Box::new(Free::Cons(seed + 1, Box::new(Free::Pure(seed + 2)))))
    //         ^^^^ first element  ^^^^^^^^^^^^^^^^^^^^^ second element pre-built
}
```

The key: a single coalgebra invocation can produce multiple `Cons` layers before bottoming out with `Pure(next_seed)`, enabling efficient multi-step generation.

## What This Unlocks

- **Efficient sequence generation**: produce Fibonacci pairs, byte pairs, or token groups in single coalgebra calls without re-entering the unfold machinery.
- **Streaming tokenizers**: one source character may produce multiple tokens — futu naturally expresses this as one coalgebra step yielding multiple `Cons` nodes.
- **Dual of dynamic programming**: while histo accumulates past results to avoid recomputation, futu pre-computes future steps to avoid redundant seeds.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Anamorphism | `let rec ana coalg seed = ...` | `fn ana<S>(seed: S, coalg: impl Fn(S) -> F<S>)` |
| Futumorphism | `futu coalg seed` — coalg returns `CoFree` | Coalgebra returns `Free<S>` |
| vs Ana | One layer per call | Multiple layers per call possible |
| `Free` monad | `type 'a free = Pure of 'a \| Free of 'a free f` | `enum Free<A> { Pure(A), Layer(...) }` |
| Dual of | Histomorphism | Histomorphism |
| Practical use | Batch generation, tokenizers | Same |
