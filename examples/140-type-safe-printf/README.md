📖 **[View on hightechmind.io →](https://hightechmind.io/rust/140-type-safe-printf)**

---

# Type-Safe Printf
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

C's `printf` is famously unsafe: `printf("%d", "hello")` compiles but causes undefined behavior at runtime. A type-safe printf encodes the format string's type signature in the type system, so passing the wrong type of argument is a compile error. This is one of the motivating examples for GADTs in OCaml and for HLists in Haskell, demonstrating how the type system can enforce format-argument correspondence at zero runtime cost.

## Learning Outcomes

- Understand how format strings can be encoded as type-level data structures
- Learn to use phantom types or HLists to thread argument types through a format specification
- See how Rust's `format_args!` macro achieves a limited form of type-safe formatting at compile time
- Appreciate the gap between Rust's macro-based approach and GADTs-based type-safe printf

## Rust Application

Rust's `format!` macro achieves compile-time type checking by parsing the format string as a macro invocation, not a runtime string. The compiler sees `format!("{} is {}", name, age)` and generates code that calls `Display::fmt` on each argument in order, with type checking at the macro expansion site. A GADT-style type-safe printf in Rust requires encoding format specifiers as types (`Fmt<Int, Fmt<Str, Done>>`) and building argument lists as HLists matched to the format type.

## OCaml Approach

OCaml's `Printf.printf` is genuinely type-safe via a clever encoding in the standard library:
```ocaml
Printf.printf "%d is %s\n" 42 "hello"  (* type: int -> string -> unit *)
Printf.printf "%d is %s\n" "oops"      (* type error at compile time *)
```
OCaml encodes format strings as GADTs where the phantom type encodes the argument sequence. The `format6` type in OCaml's standard library is the result of decades of refinement to make this ergonomic.

## Key Differences

1. **Macro vs. types**: Rust uses macros to parse format strings at compile time; OCaml encodes format strings as values of a GADT type that carries type information.
2. **Runtime format strings**: OCaml's type-safe printf works with statically known format strings only (like Rust's); dynamic format strings in both languages lose type safety.
3. **Ergonomics**: OCaml's approach integrates naturally into the language; Rust's `format!` macro achieves safety through a different (macro-based) mechanism.
4. **Extensibility**: OCaml's GADT format encoding is user-extensible; Rust's `format!` macro is closed (only built-in specifiers).

## Exercises

1. Implement a simplified type-safe format using HLists: `Fmt<Int, Fmt<Str, Done>>` and a `format` function that takes the matching `HCons<i32, HCons<String, HNil>>`.
2. Use Rust's `format_args!` to verify that passing the wrong type for a `{}` placeholder is caught at compile time (not runtime).
3. Design a DSL for SQL query parameters using a similar type-encoding to ensure `WHERE id = ?` receives an integer, not a string.
