📖 **[View on hightechmind.io →](https://hightechmind.io/rust/614-histomorphism-rust)**

---

# 614: Histomorphism

**Difficulty:** 5  **Level:** Master

A fold where the algebra sees the *entire history* of previously computed results — enabling dynamic programming without a memoization table.

## The Problem This Solves

Computing Fibonacci numbers with a naive catamorphism is O(2^n): `fib(n) = fib(n-1) + fib(n-2)` recomputes every sub-problem exponentially. The standard fix is memoization — a lookup table you build alongside the recursion. But maintaining a separate `HashMap` is boilerplate, error-prone, and not composable.

The histomorphism makes memoization structural: the algebra receives a `Cofree F A` value at each step, which is a *comonad* that carries the full history of all previously computed results. To compute `fib(n)`, the algebra simply looks up `fib(n-1)` and `fib(n-2)` from the history — both are already computed at that point because `histo` processes bottom-up.

This is the categorical basis of dynamic programming: the `Cofree` structure is the memoization table, built automatically by the recursion scheme. Any DP problem that fills a table from base cases upward maps directly to a histomorphism.

## The Intuition

A histomorphism is a fold (like `cata`) where the algebra receives a `Cofree` comonad instead of a plain value — `Cofree` is a pair of `(current_result, history_of_all_sub_results)`, so the algebra can look back at any previously computed result in O(1) without a separate memo table. The trade-off: you pay O(n) memory for the history, but get O(n) time instead of O(2^n).

## How It Works in Rust

```rust
// Cofree F A: the history comonad
// At each step, contains the current computed value AND a pointer to previous history
struct Cofree<A> {
    value: A,
    prev: Option<Box<Cofree<A>>>,  // full chain of previous results
}

impl<A: Clone> Cofree<A> {
    // Extract the current value
    fn extract(&self) -> &A { &self.value }

    // Look back N steps
    fn lookback(&self, n: usize) -> Option<&A> {
        if n == 0 { return Some(&self.value); }
        self.prev.as_ref()?.lookback(n - 1)
    }
}

// Histomorphism over natural numbers
// Algebra receives Cofree<A> — can access all previous results
fn histo<A: Clone>(n: usize, base: A, alg: &impl Fn(&Cofree<A>) -> A) -> A {
    let mut history = Cofree { value: base, prev: None };
    for _ in 0..n {
        let new_val = alg(&history);
        history = Cofree {
            value: new_val,
            prev: Some(Box::new(history)),
        };
    }
    history.value
}

// Fibonacci — O(n), no HashMap needed
// The algebra looks back 1 and 2 steps — both are in the Cofree history
let fib_100 = histo(100, 1u64, &|hist| {
    let prev1 = hist.extract();           // fib(n-1) — current top
    let prev2 = hist.lookback(1)          // fib(n-2) — one step back
        .unwrap_or(prev1);
    prev1 + prev2
});
println!("fib(100) = {}", fib_100);

// Tribonacci — needs 3 previous values — same pattern, look back 0, 1, 2
let trib = histo(20, 1u64, &|hist| {
    let a = hist.lookback(0).copied().unwrap_or(1);
    let b = hist.lookback(1).copied().unwrap_or(1);
    let c = hist.lookback(2).copied().unwrap_or(1);
    a + b + c
});
```

## What This Unlocks

- **DP without memoization tables**: longest common subsequence, edit distance, coin change — the `Cofree` history IS the DP table, populated automatically.
- **Sliding window computations**: `Cofree` lets the algebra peek back N steps — moving average, rolling max, and similar patterns become direct lookups.
- **Dual to futumorphism**: while `futu` generates multiple future steps in one call, `histo` consumes all past results in one lookup.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Catamorphism algebra | `F<A> → A` | `F<A> → A` |
| Histo algebra | `Cofree<F, A> → A` | `&Cofree<A> → A` |
| `Cofree F A` | `type ('f, 'a) cofree = { extract: 'a; unwrap: ('f, 'a) cofree 'f }` | `struct Cofree<A> { value: A, prev: Option<Box<Cofree<A>>> }` |
| Fibonacci | O(2^n) with naive cata | O(n) with histo |
| Memo table | Explicit `Hashtbl` | Implicit in `Cofree` chain |
| Dual | Futumorphism | Futumorphism |
