# 590: Advanced Enum Pattern Cookbook

**Difficulty:** 3  **Level:** Intermediate

Model complex domain types with recursive enums, mixed variant forms, and exhaustive pattern matching — one enum that replaces a whole class hierarchy.

## The Problem This Solves

In object-oriented languages, a "type that can be one of several shapes" requires an abstract base class, concrete subclasses, a visitor pattern for exhaustive dispatch, and a runtime type tag you can't inspect statically. It's a lot of ceremony for a very common idea.

Rust enums collapse all of this into one construct. A single `enum` can have unit variants (no data), tuple variants (positional fields), and struct variants (named fields), all in the same type. The compiler generates the discriminant, enforces exhaustive matching, and can even derive common traits like `Debug`, `Clone`, `PartialEq`, and `Hash`. No base class, no visitor, no runtime type erasure.

Recursive types — trees, JSON, S-expressions, ASTs — are the canonical use case. A `Json` enum with an `Array(Vec<Json>)` variant is a recursive type: `Vec<Json>` stores owned `Json` values, each of which may themselves be arrays. The recursion terminates because `Vec` is heap-allocated — Rust knows the stack frame for `Json` is finite because the `Vec` is a fixed-size pointer, not an inline array of unknown depth.

## The Intuition

An enum variant is a named constructor. `Json::Null` constructs the null value. `Json::Num(3.14)` constructs a number. `Json::Array(vec![...])` constructs an array. The `match` expression is the inverse constructor — it deconstructs the value and lets you act on each case. The compiler verifies at compile time that you've covered every case. This is *exhaustive dispatch* — no forgotten branches, no default-fallthrough bugs.

Phantom types (`PhantomData<T>`) add a compile-time type marker to an enum without storing any data for it at runtime. This is useful for state machines where you want the type system to prevent invalid state transitions.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum Json {
    Null,                              // unit variant — no data
    Bool(bool),                        // tuple variant — one field
    Num(f64),                          // tuple variant
    Str(String),                       // tuple variant
    Array(Vec<Json>),                  // recursive via heap (Vec)
    Object(Vec<(String, Json)>),       // recursive via heap (Vec)
}

impl Json {
    fn depth(&self) -> usize {
        match self {
            Json::Array(xs)  => 1 + xs.iter().map(Json::depth).max().unwrap_or(0),
            Json::Object(kv) => 1 + kv.iter().map(|(_, v)| v.depth()).max().unwrap_or(0),
            _                => 0,   // _ catches all remaining unit/scalar variants
        }
    }

    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _                => None,
        }
    }
}

// Mixed variant forms in one enum:
enum Expr {
    Lit(i64),                          // tuple
    Var { name: String },              // struct variant
    Add(Box<Expr>, Box<Expr>),         // recursive — Box breaks infinite size
    Unit,                              // unit
}
```

`Box<Expr>` in the recursive `Add` variant is required: without it, Rust can't compute the size of `Expr` on the stack (it would be infinitely large). `Box` indirects through the heap, making the size finite.

## What This Unlocks

- **Complete domain models in one type**: A compiler AST, a JSON value, a network protocol packet — expressed as one enum without inheritance hierarchies or visitor boilerplate.
- **Compile-time exhaustiveness**: Add a new variant and every `match` that doesn't cover it becomes a compile error — the compiler finds every place that needs updating.
- **Zero-cost dispatch**: The compiler generates a jump table or branch tree from your `match` — no virtual dispatch, no pointer indirection through vtables.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mixed variant kinds | All use `of` syntax | Unit / tuple / struct variants in one `enum` |
| Recursive type | `type t = A \| B of t` | `Box<T>` required to break size cycle |
| Exhaustive match | `match` — compiler warns on missing | `match` — compiler errors on missing |
| Derived traits | `deriving` ppx | `#[derive(Debug, Clone, PartialEq)]` |
| Phantom type marker | GADT | `PhantomData<T>` |
| Dispatch cost | Compiled to jump table | Compiled to jump table — no vtable |
