📖 **[View on hightechmind.io →](https://hightechmind.io/rust/177-gadt-expr)**

---

# GADT Typed Expression Evaluator
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Building on the GADT introduction, this example shows a full typed expression evaluator where the type system guarantees that evaluation never fails with a type mismatch. Each node type in the AST is a separate Rust struct implementing an `Expr` trait with an associated `Value` type. This approach ensures that `eval` on an `Add` node always produces an integer, and on a `Compare` node always produces a boolean — type safety is structural, not checked at runtime.

## Learning Outcomes

- Implement a typed expression tree using a trait with an associated `Value` type
- See how separate structs per node type (rather than one enum) provide GADT-like safety
- Understand the trade-off: trait-based approach vs. phantom-type enum approach
- Learn to build composite expressions: `IfExpr<B, V>` parameterized by condition and value types

## Rust Application

Each node is a struct implementing `trait Expr { type Value; fn eval(&self) -> Self::Value; }`. `Lit(i64)` has `Value = i64`. `BLit(bool)` has `Value = bool`. `Add<L: Expr<Value=i64>, R: Expr<Value=i64>>` has `Value = i64`. `Compare<L: Expr<Value=i64>, R: Expr<Value=i64>>` has `Value = bool`. The type bounds on `Add` prevent adding booleans — the type checker enforces it. `IfExpr<C: Expr<Value=bool>, V: Expr>` selects between two branches of the same type.

## OCaml Approach

OCaml's GADT evaluator is more concise:
```ocaml
type _ expr =
  | Lit : int -> int expr
  | BLit : bool -> bool expr
  | Add : int expr * int expr -> int expr
  | Compare : int expr * int expr -> bool expr
  | If : bool expr * 'a expr * 'a expr -> 'a expr
let rec eval : type a. a expr -> a = function ...
```
OCaml's single recursive type with indexed constructors is more unified than Rust's per-struct approach, and `eval` is a single function with exhaustive pattern matching.

## Key Differences

1. **Unification**: OCaml uses one GADT type with multiple constructors; Rust uses one struct per node, all implementing the same trait — a different structural choice.
2. **Recursive evaluation**: OCaml's `eval` is `let rec` over the unified GADT; Rust's `eval` calls `self.cond.eval()`, `self.then_.eval()` as method calls on constituent structs.
3. **Type inference**: OCaml infers all types in the GADT; Rust requires explicit type bounds on generic parameters of each node struct.
4. **Verbosity**: Rust's approach requires N struct definitions and N trait implementations; OCaml's requires one type and one function.

## Exercises

1. Add a `Mul<L: Expr<Value=i64>, R: Expr<Value=i64>>` node for multiplication.
2. Add a `Not<E: Expr<Value=bool>>` node for boolean negation.
3. Implement a `pretty_print` method on each node type via a separate `PrettyPrint` trait.
