📖 **[View on hightechmind.io →](https://hightechmind.io/rust/138-type-witnesses)**

---

# Type Witnesses / GADT Encoding

## Problem Statement

A type witness is a value that carries proof of a type relationship at runtime, allowing safe downcasting or refinement of a type. GADTs (Generalized Algebraic Data Types) use type witnesses at the constructor level to encode invariants like "this expression evaluates to an int" or "this list has exactly n elements." Without type witnesses, type-safe evaluation of heterogeneous expression trees requires runtime type tags and potential panics on mismatch.

## Learning Outcomes

- Understand what a type witness is and how it encodes type-level proofs
- Learn to simulate GADT-style typed expression trees with phantom types and smart constructors
- See how the phantom type parameter eliminates impossible branches in evaluation
- Implement a typed heterogeneous map where each key witnesses its value's type

## Rust Application

`TypedExpr<T>` wraps a `RawExpr` with a phantom `T` that witnesses the result type. Smart constructors like `int_lit(n: i32) -> TypedExpr<i32>` and `bool_lit(b: bool) -> TypedExpr<bool>` set `T` correctly. `eval_int(e: &TypedExpr<i32>) -> i32` can only be called on integer expressions — passing a boolean expression is a compile error. The `unreachable!()` branches in `eval` are dead by the phantom-type invariant; the compiler cannot eliminate them without GADTs, but they never execute.

## OCaml Approach

OCaml's GADTs express this pattern directly:
```ocaml
type _ expr =
  | IntLit : int -> int expr
  | BoolLit : bool -> bool expr
  | Add : int expr * int expr -> int expr
  | Eq : int expr * int expr -> bool expr
let rec eval : type a. a expr -> a = function
  | IntLit n -> n
  | BoolLit b -> b
  | Add (l, r) -> eval l + eval r
  | Eq (l, r) -> eval l = eval r
```
OCaml's GADT pattern matching exhausts cases correctly — the compiler knows `eval` on `int expr` cannot match `BoolLit`. There are no `unreachable!()` branches.

## Key Differences

1. **Exhaustiveness**: OCaml's GADT matcher eliminates impossible cases in each branch; Rust must include `unreachable!()` branches that the phantom type guarantees are dead.
2. **Smart constructors**: Rust requires smart constructors to set `T` correctly; OCaml sets the index type directly in the constructor declaration.
3. **Typed maps**: Both languages can build typed heterogeneous maps; Rust's uses `TypeId` at runtime for downcast; OCaml uses GADT keys for static dispatch.
4. **Error messages**: OCaml's GADT errors point to incorrect type indices directly; Rust's phantom-type errors manifest as mismatched type parameters.

## Exercises

1. Add an `If(TypedExpr<bool>, TypedExpr<i32>, TypedExpr<i32>) -> TypedExpr<i32>` constructor and implement its evaluation.
2. Build a typed key-value map where `Key<T>` witnesses that the associated value has type `T`, with `get<T>(key: Key<T>) -> Option<T>`.
3. Try constructing an ill-typed expression (adding a bool to an int) and observe the compile error.
