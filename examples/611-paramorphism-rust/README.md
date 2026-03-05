# 611: Paramorphism

**Difficulty:** 5  **Level:** Master

A fold where the algebra receives both the computed result *and* the original sub-structure — enabling context-aware recursion like sorted insertion.

## The Problem This Solves

Catamorphism (`fold`) gives your algebra only the accumulated result so far. But some algorithms need to see the *original* recursive structure at each step, not just what it computed to. Consider inserting into a sorted list: at each `Cons(head, tail)` node, you need to compare with `head` and potentially prepend to the *original* tail — but cata has already replaced the tail with a folded value.

The workaround with `cata` is to carry both the original and the result as a tuple, but this is ad-hoc. A paramorphism makes it structural: the algebra receives `F<(original, result)>` — both the sub-structure and the fold result at each recursive position.

This is the formal version of "sometimes you need to look at what you're folding, not just what you've computed." The canonical example is `tails :: [a] -> [[a]]` — at each step you need the original tail to add to the results list.

## The Intuition

A paramorphism is like catamorphism, except the algebra gets a pair `(sub-structure, fold-result)` at each recursive position — it "keeps the original around" so context-aware decisions can use it. The trade-off: more powerful than cata but slightly harder to reason about; use cata when you only need the result, para when you need both.

## How It Works in Rust

```rust
// A simple natural number type as a recursive structure
enum Nat {
    Zero,
    Succ(Box<Nat>),
}

impl Nat {
    fn to_usize(&self) -> usize {
        match self {
            Nat::Zero    => 0,
            Nat::Succ(n) => 1 + n.to_usize(),
        }
    }

    // Paramorphism: algebra gets (original_sub, fold_result) pairs
    fn para<A>(self, zero_case: A, succ_case: impl Fn(Nat, A) -> A) -> A
    {
        match self {
            Nat::Zero       => zero_case,
            Nat::Succ(pred) => {
                // para the predecessor — get its result
                let pred_result = pred.para(zero_case, &succ_case);
                // But we've consumed pred — in a real para we'd clone or use &
                // The key idea: algebra receives (original_pred, pred_result)
                succ_case(*pred, pred_result)
                //         ^^^^^ original sub-structure
                //                       ^^^^^^^^^^^ computed result
            }
        }
    }
}

// Example: factorial — needs the original number at each step
// fact(n) = n * fact(n-1)
// With para: succ case gets (pred_nat, fact_of_pred)
// so we can do (pred_nat.to_usize() + 1) * fact_of_pred

// Contrast with cata — can only do running_product * 1 (loses the index)
```

For lists, paramorphism enables `tails`, sorted insertion, and "insert at position N" — all require the original tail at each step.

## What This Unlocks

- **Sorted insertion**: at each `Cons(head, tail)`, compare insertion value with `head` and either prepend using original `tail` or recurse — needs the original, not just the result.
- **`tails` function**: `[[1,2,3], [2,3], [3], []]` — each step prepends the original sub-list.
- **Index-aware folds**: algorithms where the fold result depends on the *depth* or *original size* at each position.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Paramorphism | `let rec para alg = function ...` | `fn para(self, alg: impl Fn(F<(T,R)>) -> R)` |
| Algebra input (cata) | `F<R>` — only fold result | `F<R>` |
| Algebra input (para) | `F<(T, R)>` — original + result | `F<(T, R)>` |
| Use case | Context-aware folds | Sorted insert, `tails`, index-dependent |
| Relationship to cata | Para is strictly more powerful | `cata alg = para (alg . fmap snd)` |
| Ownership challenge | GC handles copying | Need `Clone` or reference to keep original |
