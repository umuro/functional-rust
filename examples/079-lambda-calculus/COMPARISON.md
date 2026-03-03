# Lambda Calculus Interpreter — Comparison

## Core Insight
Building an interpreter reveals the fundamental difference in how OCaml and Rust handle recursive data and environment capture. OCaml's GC allows free sharing of environments; Rust requires explicit `Box` for recursion and `clone()` for environment capture.

## OCaml Approach
- `type expr = Int of int | Lam of string * expr | ...` — recursive variants, no boxing needed
- `type value = VClosure of string * expr * env` — closures capture environment by reference (GC)
- `List.assoc` for environment lookup
- Pattern matching on nested variants is concise

## Rust Approach
- `enum Expr { Lam(String, Box<Expr>), ... }` — `Box` required for recursive types
- `Value::VClosure(String, Box<Expr>, Env)` — environment must be cloned
- `env.iter().rev().find()` for lookup (last binding wins)
- `Result<Value, String>` for error handling vs OCaml exceptions

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursive types | Direct | Requires `Box<T>` |
| Environment | Shared via GC | Cloned on capture |
| Error handling | `failwith` exception | `Result<T, E>` |
| Lookup | `List.assoc` | `.iter().rev().find()` |
| Memory | GC collects cycles | Ownership prevents cycles |

## Learner Notes
- `Box<Expr>` is the Rust idiom for recursive enum variants
- Cloning environments is the cost of no-GC; consider `Rc` for sharing
- OCaml `and` keyword for mutually recursive types; Rust just uses the type name
- Helper constructors (`fn int(n)`, `fn app(f, a)`) make test code readable
