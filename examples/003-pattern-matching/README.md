📖 **[View on hightechmind.io →](https://hightechmind.io/rust/003-pattern-matching)**

---

# 003 — Pattern Matching
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Pattern matching originated in ML (1973) and is now recognized as one of the most productive features in statically typed programming. It combines structural decomposition of data with exhaustiveness checking: the compiler verifies that every possible shape of a value is handled, preventing entire classes of runtime errors that plague switch/case in other languages.

Without pattern matching, processing tree-structured data (ASTs, JSON, XML) or discriminated unions requires chains of `instanceof` checks and casts. Pattern matching makes this both safe and concise, which is why it has spread to Swift, Kotlin, Python 3.10, C# 7, and even Java 21.

## Learning Outcomes

- Use `match` for tuples, enums, and nested data structures
- Write match guards (`if n < 0`) to add conditions beyond structural patterns
- Understand how Rust's exhaustiveness checking catches missing cases at compile time
- Build recursive algebraic data types with `enum` and `Box`
- Evaluate expression trees using pattern matching recursion

## Rust Application

The code shows three escalating uses of `match`. `describe_pair` matches on a `(i32, i32)` tuple with literal and binding patterns. The `Shape` enum with `area()` and `shape_name()` demonstrates how enums with data replace class hierarchies. The `Expr` enum with `eval()` shows recursive pattern matching: `Expr::Add(a, b)` recursively evaluates both branches, and `Expr::Mul` does the same — this is the canonical interpreter pattern used in compilers. Guards in `classify_number` show how to add conditions: `n if n < 0 => "negative"`.

## OCaml Approach

OCaml uses the same `match` syntax and the same structural patterns. Variant types (`type shape = Circle of float | Rectangle of float * float`) are OCaml's algebraic data types. The `function` keyword is shorthand for `fun x -> match x with ...`. OCaml's pattern matching is exhaustiveness-checked at compile time just like Rust's — a missing arm is a warning or error, not a silent bug.

## Key Differences

1. **Box for recursion**: Rust requires `Box<Expr>` in recursive enum variants because the compiler needs to know the size. OCaml allocates on the heap automatically via the GC; no boxing syntax needed.
2. **Guard syntax**: Both use guards (`if condition`) but OCaml omits the variable binding in guards when already in scope: `| n when n < 0 -> "negative"`. Rust requires `n if n < 0`.
3. **Wildcard binding**: Rust uses `_` for unused bindings and `..` to ignore multiple tuple fields. OCaml uses `_` the same way.
4. **OR patterns**: OCaml allows `| A | B -> expr` in one arm. Rust also supports `A | B => expr` since Rust 1.53.

## Exercises

1. **Extend Shape**: Add a `Triangle` variant with base and height to the `Shape` enum and add a `perimeter` function. Verify the compiler forces you to handle the new case everywhere.
2. **Depth of Expr**: Write `depth(expr: &Expr) -> usize` that returns the maximum nesting depth of an expression tree, using pattern matching recursion.
3. **Simplify**: Write `simplify(expr: Expr) -> Expr` that eliminates `Add(Num(0), x)` and `Mul(Num(1), x)` identity cases — the kind of optimization a real compiler's peephole pass performs.
