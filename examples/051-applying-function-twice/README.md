📖 **[View on hightechmind.io →](https://hightechmind.io/rust/051-applying-function-twice)**

---

# Example 051: Applying a Function Twice

**Difficulty:** ⭐ **Category:** Higher-Order Functions **OCaml Source:** CS3110

## Problem

Define `twice f x = f (f x)`, then use partial application to build `quad`
(double twice) and `fourth` (square twice) without mentioning the argument.

## Learning Outcomes

- Higher-order functions: passing functions as values
- Partial application: binding one argument, receiving a new function
- Generic `Fn` trait bounds in Rust vs OCaml's polymorphic arrow types
- Difference between `Fn`, `FnMut`, `FnOnce` and when each applies
- Bare function pointers (`fn(T) -> T`) vs closures (`impl Fn(T) -> T`)

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Partial application | Built-in (all functions are curried) | Explicit closure: `move \|x\| f(f(x))` |
| Type of `twice` | `('a -> 'a) -> 'a -> 'a` | `fn<T, F: Fn(T)->T>(f: F, x: T) -> T` |
| Returning a function | `let quad = twice double` | `let quad = twice_partial(double)` |
| Function pointer | Not distinguished | `fn(i32) -> i32` (no captures) |
| Ownership | Garbage collected | `f` moved into returned closure |

## Implementations

- **`twice`** — generic over `T` and `F: Fn(T) -> T`; direct two-argument form
- **`twice_partial`** — returns `impl Fn(T) -> T`; enables OCaml-style partial application
- **`twice_fp`** — bare function pointer variant; zero-overhead, less general
