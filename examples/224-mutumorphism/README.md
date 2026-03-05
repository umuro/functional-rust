# 224: Mutumorphism

**Difficulty:** 5  **Level:** Expert

Two folds that depend on each other — compute two results simultaneously where each depends on the other's intermediate values.

## The Problem This Solves

Zygo (example 221) runs one "helper" fold alongside a main fold, but the helper is independent — the main fold uses helper results, not vice versa. Mutumorphism is genuinely mutual: both folds receive *each other's* results at every step.

The canonical example: `isEven` and `isOdd` are mutually recursive by definition — `isEven(n) = isOdd(n-1)` and `isOdd(n) = isEven(n-1)`. You can't compute one without the other. Mutumorphism encodes this as two algebras that receive a paired `(A, B)` layer.

A more compelling use case: simultaneously evaluate an expression *and* type-check it. Both the value and the type of each subexpression are needed to compute the value and type of the parent.

## The Intuition

In `mutu(alg_a, alg_b, tree)`, both algebras receive `F<(A, B)>` — the functor layer where each recursive position has already been expanded into the pair of both results. Each algebra picks what it needs from the pair.

Compare to `cata`: `cata(alg, tree)` gives `alg` an `F<A>` — just one result per position. `mutu` gives both algebras an `F<(A, B)>` — the full pair at every position. This is why `isEven` can call `isOdd` at each step without a separate traversal.

The result is a pair `(A, B)` produced in a single pass, even though both sides depend on each other.

## How It Works in Rust

```rust
fn mutu<A: Clone, B: Clone>(
    alg_a: &dyn Fn(NatF<(A, B)>) -> A,   // sees BOTH results at each position
    alg_b: &dyn Fn(NatF<(A, B)>) -> B,
    fix: &FixNat,
) -> (A, B) {
    // Recursively expand each child into its (A, B) pair
    let paired: NatF<(A, B)> = fix.0.map_ref(|child| mutu(alg_a, alg_b, child));
    // Both algebras receive the same paired layer — must clone to share
    (alg_a(paired.clone()), alg_b(paired))
}
```

isEven / isOdd — the mutual dependency in action:

```rust
fn is_even_alg(n: NatF<(bool, bool)>) -> bool {
    match n {
        NatF::ZeroF => true,                  // 0 is even
        NatF::SuccF((_even, odd)) => odd,     // n+1 is even iff n is odd
    }
}

fn is_odd_alg(n: NatF<(bool, bool)>) -> bool {
    match n {
        NatF::ZeroF => false,                 // 0 is not odd
        NatF::SuccF((even, _odd)) => even,    // n+1 is odd iff n is even
    }
}

// Both computed in a single traversal:
let (even, odd) = mutu(&is_even_alg, &is_odd_alg, &nat(4));
// even = true, odd = false
```

Simultaneous evaluation + type-checking:

```rust
fn val_alg(e: ExprF<(Value, Typ)>) -> Value {
    match e {
        ExprF::Add((Value::VInt(a), _), (Value::VInt(b), _)) => Value::VInt(a + b),
        ExprF::If((Value::VBool(true), _), (v, _), _) => v,  // use type to guide eval
        _ => Value::VError,
    }
}

fn typ_alg(e: ExprF<(Value, Typ)>) -> Typ {
    match e {
        ExprF::Add((_, Typ::TInt), (_, Typ::TInt)) => Typ::TInt,
        ExprF::If((_, Typ::TBool), (_, t1), (_, t2)) if t1 == t2 => t1,
        _ => Typ::TError,
    }
}

// Type and value computed together — one pass, mutual dependency
let (value, typ) = mutu_expr(&val_alg, &typ_alg, &expr);
```

## What This Unlocks

- **Simultaneous evaluation + type-checking** — one traversal instead of two separate passes
- **Even/odd parity tracking** — any mutually-defined predicate on recursive structures
- **Co-dependent accumulations** — wherever two summary values at a node depend on each other's child values

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Algebra signature | `'f ('a * 'b) -> 'a` | `Fn(F<(A, B)>) -> A` |
| Pair type | `'a * 'b` (native tuple) | `(A, B)` |
| Layer sharing | Single evaluation, GC handles aliasing | `.clone()` required to share paired layer between two algebras |
| Pattern matching | `VInt a, _` on pair | `(Value::VInt(a), _)` — same structure |
| vs zygo | Helper is one-way dependency | Both algebras are mutually dependent |
