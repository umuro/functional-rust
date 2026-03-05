📖 **[View on hightechmind.io →](https://hightechmind.io/rust/612-zygomorphism-rust)**

---

# 612: Zygomorphism

**Difficulty:** 5  **Level:** Master

Run two mutually dependent folds in a single traversal — when fold B needs fold A's result at each step.

## The Problem This Solves

Sometimes you need two values from a single fold, where one depends on the other. Computing the average of a list requires both the sum and the count. Computing variance requires both the mean and the sum of squared deviations. If you run them as two separate folds, you traverse the structure twice and miss the opportunity to share intermediate results.

The naive fix is to fold a tuple `(sum, count)`. That works, but it's informal. The problem is deeper: what if fold B doesn't just need the *result* of fold A, but needs fold A's *intermediate results at each step*? A zygomorphism formalizes this: algebra B receives its own result-so-far *and* the result of algebra A at each node.

This pattern appears in compilers (type inference that depends on a size computation done simultaneously), dynamic programming (a DP fold that depends on an auxiliary DP table built in the same pass), and statistics (streaming computation of mean and variance together).

## The Intuition

A zygomorphism runs two algebras simultaneously — algebra A computes one value, algebra B computes another while having access to A's result at each step — giving you two outputs in one traversal of the structure. The trade-off: more complex algebra signature, but you get single-pass efficiency when two computations share sub-expressions.

## How It Works in Rust

```rust
// Zygomorphism over a list: fold A and fold B together
// alg_a: F<A> → A  (the "helper" fold)
// alg_b: F<(A, B)> → B  (the "main" fold, has access to A's current result)

fn zygo_list<A, B, AlgA, AlgB>(
    list: &[f64],
    init_a: A,
    init_b: B,
    alg_a: AlgA,   // A fold: computes running A value
    alg_b: AlgB,   // B fold: uses A's result at each step
) -> (A, B)
where
    A: Clone,
    AlgA: Fn(f64, A) -> A,
    AlgB: Fn(f64, A, B) -> B,  // B receives: current element, A's result, B's result
{
    let mut acc_a = init_a;
    let mut acc_b = init_b;
    for &x in list {
        let new_a = alg_a(x, acc_a.clone());
        acc_b = alg_b(x, acc_a, acc_b);  // B sees A's result BEFORE updating A
        acc_a = new_a;
    }
    (acc_a, acc_b)
}

let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

// Compute count and sum simultaneously
let (count, sum) = zygo_list(
    &data,
    0.0_f64, 0.0_f64,
    |_, c| c + 1.0,         // alg_a: count
    |x, _c, s| s + x,       // alg_b: sum (doesn't need count, but could)
);
let mean = sum / count;

// Compute mean AND variance in one pass
// Variance fold needs running mean — so fold B depends on fold A at each step
let (_, variance) = zygo_list(
    &data,
    mean,  // pre-computed mean as "A"
    0.0,
    |_, m| m,                       // alg_a: identity (mean is fixed)
    |x, m, acc| acc + (x - m).powi(2),  // alg_b: sum of squared deviations
);
println!("variance = {}", variance / count);
```

## What This Unlocks

- **Streaming statistics**: compute mean and variance (or standard deviation) in one pass — no intermediate storage.
- **Parallel DP tables**: one DP fold provides lookup values that a second DP fold reads, both in one traversal.
- **Size-weighted folds**: fold B computes a weighted value where weights come from fold A (e.g., normalize by total weight computed simultaneously).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Zygomorphism | `zygo alg_a alg_b` (two algebra args) | Function with two algebra closures |
| vs two catas | Two traversals | One traversal, shared intermediate state |
| Algebra B signature | `F<(A, B)> → B` | `Fn(element, A, B) -> B` |
| Efficiency | Single traversal | Single traversal |
| Classic example | Average (sum + count) | Mean + variance together |
| Generalizes | Paramorphism (A = original sub-structure) | Same — para is a special zygo |
