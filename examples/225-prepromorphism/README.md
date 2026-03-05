# 225: Prepromorphism

**Difficulty:** 5  **Level:** Expert

A catamorphism with a natural transformation applied at every recursive step — reshape the structure as you fold it.

## The Problem This Solves

A regular catamorphism (`cata`) folds a recursive structure bottom-up, applying an algebra at each node. But sometimes you want to *modify the structure itself* at each step before folding — not just the accumulated value.

Prepromorphism adds a natural transformation: a function `ExprF<Fix> → ExprF<Fix>` that runs on each layer's children *before* recursing into them. This lets you rewrite the structure on the way down. Replace every `Mul` with `Add`. Double every literal. Remove every `Neg`. These transformations are applied at every level — not just the top.

## The Intuition

Think of `cata` as reading a tree bottom-up. `prepro` is `cata` with a lens applied at each step: before you recurse into a subtree, you transform its structure. The transformation is applied repeatedly — at the top level, then at each child, then at each grandchild.

This means transformations *compose across depth*. If you apply `mul_to_add` and the tree is `Mul(Mul(2, 3), 4)`, the outer `Mul` becomes `Add` — and then when we recurse into the children, the inner `Mul(2, 3)` also gets transformed. All multiplications become additions throughout the tree.

Mathematically: `prepro(nat, alg, tree) = alg(fmap(child → prepro(nat, alg, Fix(nat(unfix(child))))) tree)`. The natural transformation `nat` reshapes each child before recursing.

## How It Works in Rust

```rust
fn prepro<A>(
    nat: &dyn Fn(ExprF<Fix>) -> ExprF<Fix>,  // natural transformation
    alg: &dyn Fn(ExprF<A>) -> A,             // algebra (same as cata)
    Fix(f): &Fix,
) -> A {
    alg(f.map_ref(|child| {
        // Apply nat to the child's layer, wrap back in Fix, then recurse
        let transformed = Fix(Box::new(nat(child.0.as_ref().clone())));
        prepro(nat, alg, &transformed)
    }))
}
```

Natural transformations:

```rust
// Replace every Mul with Add (at every level going down)
fn mul_to_add(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::MulF(a, b) => ExprF::AddF(a, b),
        other => other,
    }
}

// Double every literal value
fn double_lits(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::LitF(n) => ExprF::LitF(n * 2),
        other => other,
    }
}

// Remove negation (replace Neg(x) with x)
fn remove_neg(e: ExprF<Fix>) -> ExprF<Fix> {
    match e {
        ExprF::NegF(a) => a.0.as_ref().clone(),  // unwrap one level
        other => other,
    }
}
```

Examples:

```rust
let e = mul(add(lit(2), lit(3)), lit(4));

cata(&eval_alg, &e)                        // 20: (2+3)*4
prepro(&identity_nat, &eval_alg, &e)       // 20: identity = plain cata

prepro(&mul_to_add, &eval_alg, &e)         // 9: (2+3)+4
// The top Mul became Add. Children are already adds/lits — nat is identity on them.

prepro(&double_lits, &eval_alg, &add(lit(1), lit(2)))  // 6: 2 + 4

prepro(&remove_neg, &eval_alg, &add(neg(lit(5)), lit(3)))  // 8: 5 + 3
```

Identity check: `prepro(identity, alg, tree) = cata(alg, tree)` — always holds.

## What This Unlocks

- **Expression rewriting** — apply simplification rules as part of evaluation (constant folding, algebraic simplifications)
- **AST normalization** — transform an AST into a canonical form while computing a result
- **Layered interpretation** — preprocess each recursive level independently before evaluation

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Natural transform type | `fix expr_f -> fix expr_f` | `Fn(ExprF<Fix>) -> ExprF<Fix>` |
| Unwrapping | `unfix child` | `child.0.as_ref().clone()` |
| Re-wrapping | `Fix (nat ...)` | `Fix(Box::new(nat(...)))` |
| Layer cloning | Not needed (GC handles sharing) | `.clone()` required at each step |
| Identity check | `prepro (fun x -> x) alg = cata alg` | Same — verified in tests |
| vs cata | Just an algebra | Algebra + natural transformation |
| vs histo | Histo carries history down; prepro rewrites structure | Different axes of extension |
