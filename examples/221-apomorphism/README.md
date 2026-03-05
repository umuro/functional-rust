📖 **[View on hightechmind.io →](https://hightechmind.io/rust/221-apomorphism)**

---

# 221: Apomorphism

**Difficulty:** ⭐⭐⭐  **Level:** Recursion Schemes

Anamorphism upgraded: unfold a structure but short-circuit by injecting a pre-built subtree instead of continuing.

## The Problem This Solves

An anamorphism (plain unfold) builds a recursive structure step by step — at each point you return a seed for the next step. But sometimes you want to *stop early* and embed an already-built subtree instead of continuing to unfold. The classic example is inserting into a sorted list: once you find the right position, you want to splice in the entire remaining tail as-is, not re-unfold it element by element.

With plain `ana`, there's no way to say "stop here and use this pre-existing structure." You'd have to keep unfolding until you reach the end even when you already have the rest of the list ready. This is wasteful and — more importantly — loses the intent of the algorithm.

Apomorphism solves this with an `Either` type in the coalgebra's output: return `Right(new_seed)` to continue unfolding as normal, or return `Left(existing_fix)` to short-circuit and embed a pre-built subtree immediately.

## The Intuition

Think of building a list like laying bricks one by one. Anamorphism forces you to lay each brick from your raw materials. Apomorphism lets you grab a pre-assembled wall section (an existing `FixList`) and slot it in directly.

When inserting `3` into the sorted list `[1, 2, 4, 5]`:
- At `1`: `3 > 1`, so continue — `Right([2, 4, 5])` (keep unfolding)
- At `2`: `3 > 2`, so continue — `Right([4, 5])` (keep unfolding)
- At `4`: `3 ≤ 4`, so we found the position — output `3` and then inject `[4, 5]` directly as `Left([4, 5])`

Without short-circuit, you'd have to "unfold" `[4, 5]` element by element even though you already have it. With apomorphism, you say "stop — here's the rest."

`Either::Left(fix)` = "done, embed this"  
`Either::Right(seed)` = "continue from this seed"

## How It Works in Rust

```rust
// Either: Left = pre-built Fix (stop), Right = seed (continue)
enum Either<L, R> { Left(L), Right(R) }

// apo: coalgebra returns F<Either<Fix, Seed>>
fn apo<S>(coalg: &dyn Fn(S) -> ListF<Either<FixList, S>>, seed: S) -> FixList {
    FixList(Box::new(coalg(seed).map(|either| match either {
        Either::Left(fix) => fix,          // short-circuit: use pre-built subtree
        Either::Right(s) => apo(coalg, s), // continue unfolding from new seed
    })))
}
```

Insert into sorted list:
```rust
fn insert(x: i64, lst: FixList) -> FixList {
    apo(&|fl: FixList| match fl.0.as_ref() {
        ListF::NilF =>
            // Reached the end: insert x before the empty list
            ListF::ConsF(x, Either::Left(nil())),
        ListF::ConsF(y, rest) => {
            if x <= *y {
                // Found position: output x, then inject the rest unchanged
                ListF::ConsF(x, Either::Left(fl.clone()))  // ← short-circuit here
            } else {
                // Keep going: output y, continue from rest
                ListF::ConsF(*y, Either::Right(rest.clone())) // ← continue
            }
        }
    }, lst)
}
```

The `Either::Left(fl.clone())` is the key: instead of re-unfolding the tail, we hand back the existing `FixList` directly.

## What This Unlocks

- **Sorted insertion** — the prototypical use: insert one element without disturbing the already-sorted tail.
- **`take` / `replace-first`** — any operation that wants to stop after a condition is met and preserve the rest structurally.
- **Efficient list transformations** — avoid redundant unfolding when a suffix is already computed; share structure explicitly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Either type | Custom `('a, 'b) either` sum type | Custom `Either<L, R>` enum |
| Short-circuit syntax | `Left fix` (shared reference) | `Left(fix.clone())` (clone required) |
| Coalgebra output | `'f (fix either seed)` | `ListF<Either<FixList, S>>` |
| vs anamorphism | No early exit possible | `Either::Left` = early exit |
| Memory model | GC shares the injected subtree | Clone at injection point |
