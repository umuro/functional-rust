# 222: Histomorphism

**Difficulty:** ⭐⭐⭐  **Level:** Recursion Schemes

Fold a recursive structure with access to the *entire computation history* at each step — not just the previous result, but every result computed so far.

## The Problem This Solves

Computing Fibonacci numbers naively is exponential — `fib(n)` calls `fib(n-1)` and `fib(n-2)`, each of which re-computes everything below them. The standard fix is a memoization table. But there's a deeper fix: use a fold that structurally carries all previous answers along with it.

A catamorphism gives you one result per recursive call. A paramorphism gives you the result plus the original subterm. A histomorphism goes further: at each step you see a *chain* of all results computed on the way here. This is the structure of dynamic programming made explicit — instead of a mutable cache, you have an immutable `Cofree` chain that IS the memo table.

The result: Fibonacci in O(n) with no memoization table, no mutation, and no imperative loops — just a fold that carries its own history.

## The Intuition

Imagine computing `fib(5)`. You need `fib(4)` and `fib(3)`. But by the time you're computing `fib(5)`, you've already computed `fib(4)`, `fib(3)`, `fib(2)`, `fib(1)`, and `fib(0)` — they're all sitting in a chain below you.

A histomorphism threads this chain through the fold as a `Cofree` structure. `Cofree` means "free comonad" but you can read it as: **a value with its history attached**. Each node in the `Cofree` chain has:
- `head`: the result computed here
- `tail`: the rest of the history (what was computed before this)

At `fib(5)`, the algebra looks at the `Cofree` chain and can reach back to `fib(4)` (one step) and `fib(3)` (two steps) by navigating `.tail`.

```
Cofree chain for nat(5):
  head=fib(5)
  tail → head=fib(4)
         tail → head=fib(3)
                tail → head=fib(2)
                       tail → head=fib(1)
                              tail → head=fib(0)
                                     tail → ZeroF
```

## How It Works in Rust

```rust
// Cofree: result + history
struct Cofree<A> {
    head: A,                      // result computed at this position
    tail: Box<NatF<Cofree<A>>>,   // the history: NatF wrapping more Cofree nodes
}

// histo: build cofree bottom-up, algebra sees history chain
fn histo<A: Clone>(alg: &dyn Fn(NatF<Cofree<A>>) -> A, fix: &FixNat) -> A {
    histo_build(alg, fix).head
}

fn histo_build<A: Clone>(alg: &dyn Fn(NatF<Cofree<A>>) -> A, fix: &FixNat) -> Cofree<A> {
    let layer = fix.0.map_ref(|child| histo_build(alg, child)); // recurse first
    let result = alg(layer.clone());                             // compute result
    Cofree::new(result, layer)                                   // attach history
}
```

Fibonacci algebra — looks back 2 steps via the `Cofree` chain:
```rust
fn fib_alg(n: NatF<Cofree<u64>>) -> u64 {
    match n {
        NatF::ZeroF => 0,                // fib(0) = 0
        NatF::SuccF(prev) => match prev.tail.as_ref() {
            NatF::ZeroF => 1,            // fib(1) = 1
            NatF::SuccF(prev2) =>
                prev.head + prev2.head,  // fib(n) = fib(n-1) + fib(n-2)
                                         // prev.head = fib(n-1), prev2.head = fib(n-2)
        }
    }
}
```

Each `Cofree` node carries all previous `head` values in the chain — the algebra just navigates as far back as needed.

## What This Unlocks

- **Dynamic programming without mutation** — any DP recurrence (Fibonacci, tribonacci, Pascal's triangle) becomes a histomorphism; the `Cofree` chain is the memo table.
- **Sliding-window computations** — when you need the last `k` computed values at each step, navigate `k` levels into the `Cofree` chain.
- **Proof of O(n)** — because the `Cofree` chain shares nodes built during the recursion, each value is computed exactly once.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Cofree type | Recursive `'a cofree_nat = CF of 'a * ...` | `struct Cofree<A> { head, tail: Box<NatF<...>> }` |
| History access | Pattern match nested `CF` constructors | `.tail.as_ref()` then match |
| vs catamorphism | Only current result | Entire history chain |
| vs paramorphism | One level of original | All levels of computed results |
| Performance | O(n), GC shares history | O(n), but clones at each layer |
