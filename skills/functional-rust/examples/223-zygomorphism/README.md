# 223: Zygomorphism

**Difficulty:** ⭐⭐⭐  **Level:** Recursion Schemes

Run two fold algebras simultaneously in one pass, where the main algebra can see both its own results and the helper algebra's results at each step.

## The Problem This Solves

Sometimes you need two things from one fold, and they're not independent — one depends on the other. Computing the *average* of a list in one pass requires both the sum and the count. But if you write two separate folds, you traverse the structure twice. And if you just pair up the results with a plain fold, the two computations can't see each other's intermediate values.

Zygomorphism solves this: run a *helper* fold and a *main* fold simultaneously. The helper runs independently. The main fold sees its own accumulated values **and** the helper's values at every node. They share one traversal, and the main algebra gets the richer information it needs.

The name comes from Greek *zygo* (yoke) — two computations yoked together, sharing the burden of a single traversal.

## The Intuition

Picture a spreadsheet with two computed columns. Column B uses only the raw data. Column A uses both the raw data and column B's values. Zygomorphism computes both columns in one left-to-right pass.

Classic example: pretty-printing an expression tree with correct precedence. The *helper* computes the numeric value (or precedence level) of each subexpression. The *main* printer uses those values to decide whether to add parentheses. Without zygomorphism, you'd traverse twice or thread awkward state.

Another example: safety-checking a calculation. The helper evaluates the expression to get actual numbers. The main checker looks at those numbers to decide if any division-by-zero risk exists. The helper's intermediate evaluations guide the main checker's decisions.

**Algebra shapes:**
- Helper: `ExprF<B> -> B` — sees only helper results below it
- Main: `ExprF<(A, B)> -> A` — sees (main_result, helper_result) pairs below it

## How It Works in Rust

```rust
// zygo: compute (main_result, helper_result) simultaneously
fn zygo_both<A: Clone, B: Clone>(
    helper: &dyn Fn(ExprF<B>) -> B,      // independent helper fold
    main:   &dyn Fn(ExprF<(A, B)>) -> A, // main fold sees both
    fix: &Fix,
) -> (A, B) {
    // For each child: recurse to get (A, B) pair
    let paired: ExprF<(A, B)> = fix.0.map_ref(|child| zygo_both(helper, main, child));

    // Extract only B values for the helper
    let b_layer = paired.map_ref(|(_, b)| b.clone());

    // Run both algebras at this node
    let a = main(paired.clone());   // main sees (A, B) pairs
    let b = helper(b_layer);        // helper sees only B values
    (a, b)
}
```

Safety-check example — helper evaluates, main checks for danger:
```rust
// Helper: plain evaluator
fn eval_helper(e: ExprF<i64>) -> i64 {
    match e {
        ExprF::LitF(n)     => n,
        ExprF::AddF(a, b)  => a + b,
        ExprF::MulF(a, b)  => a * b,
        ExprF::NegF(a)     => -a,
    }
}

// Main: safety check — uses helper's computed values to detect overflow risk
fn safe_check(e: ExprF<(String, i64)>) -> String {
    match e {
        ExprF::LitF(n) => format!("safe({})", n),
        ExprF::AddF((sa, va), (sb, vb)) =>
            if va.saturating_add(vb) != va + vb {
                format!("OVERFLOW({} + {})", sa, sb)  // va available from helper!
            } else {
                format!("safe({} + {})", sa, sb)
            },
        // ...
    }
}
```

The main algebra uses `va` and `vb` — the helper's computed values — to make decisions a pure structural fold couldn't make.

## What This Unlocks

- **One-pass average** — helper computes `(sum, count)`, main formats the result. No second traversal.
- **Pretty-printing with precedence** — helper computes precedence level, main uses it to decide parentheses. No separate precedence pass.
- **Validation + transformation** — helper validates, main transforms using validation results. Errors and transformations interleaved in one pass.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Helper algebra | `ExprF 'b -> 'b` | `ExprF<B> -> B` |
| Main algebra | `ExprF ('a * 'b) -> 'a` | `ExprF<(A, B)> -> A` |
| Pair extraction | `map_f snd` | `.map_ref(\|(_, b)\| b.clone())` |
| Clone overhead | None (GC) | Clone paired layer for split |
| vs two separate folds | Two traversals | One traversal, shared |
