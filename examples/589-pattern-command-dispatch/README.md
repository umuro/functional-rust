📖 **[View on hightechmind.io →](https://hightechmind.io/rust/589-pattern-command-dispatch)**

---

# Pattern Command Dispatch
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Pattern matching in Rust goes beyond simple value checks — it enables powerful dispatch mechanisms for type-safe command processing, visitor-pattern traversals, state machine transitions, and recursive data structure manipulation. This example demonstrates advanced pattern matching techniques that arise in compiler construction, game engines, protocol implementations, and functional programming idioms applied to real systems code.

## Learning Outcomes

- Advanced pattern matching constructs specific to this example's domain
- How Rust's exhaustiveness checking prevents missed cases in complex dispatch
- How patterns interact with ownership — matching by value vs by reference
- How recursive enum patterns (trees, ASTs) work with  variants
- Where this technique appears in real-world Rust: compilers, game engines, CLI tools

## Rust Application

The source demonstrates the core technique with working examples that can be run directly. Key constructs include enum variant matching, guard conditions, nested destructuring, and composition of multiple pattern types. The match expressions are exhaustive — adding new variants requires updating all match sites, making the code refactor-safe.

Key patterns demonstrated:
- Named constant patterns using `const` values in match arms
- Type-dispatch via enum variants carrying different payload types
- `Box<T>` deref patterns for recursive data structures
- Or-pattern grouping for related variants in dispatch tables

## OCaml Approach

OCaml's ML heritage makes it the reference implementation for these patterns. Variant types, exhaustive matching, and recursive type handling in OCaml are equivalent in power:

```ocaml
(* Pattern matching in OCaml handles:
   - Variant constructors with data: Cmd (arg1, arg2) -> ...
   - Guards: | x when x > threshold -> ...  
   - Nested patterns: Node { left; right } -> ...
   - Recursive cases: the natural form for tree traversal *)
```

## Key Differences

1. **Box deref**: Rust requires `Box<T>` for recursive types and Rust's patterns transparently deref through `Box`; OCaml's GC manages recursive variant pointers automatically.
2. **Const patterns**: Rust allows named `const` values in patterns; OCaml can use `let open Consts in` to bring constants into scope for pattern matching.
3. **Visitor pattern**: OCaml's idiomatic style uses recursive functions directly; Rust often uses both direct recursion and the trait-based visitor pattern for separation of concerns.
4. **State machines**: Both languages naturally express state machines with variant enums + match — this is one of the strongest arguments for algebraic types over OOP class hierarchies.

## Exercises

1. **Extend the data type**: Add a new variant or field to the main data structure and trace all the match expressions that need updating — practice the exhaustiveness feedback loop.
2. **Accumulating visitor**: Write a traversal function that collects all leaf values into a `Vec<T>` using only pattern matching and recursion.
3. **State machine validation**: Implement an invalid-transition error: when the state/event combination is unexpected, return `Err("invalid transition")` instead of panicking.
