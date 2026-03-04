# 140: Type-Safe Printf

**Difficulty:** 4  **Level:** Advanced

Encode the arity and types of a format string in the type system so mismatched arguments are a compile error, not a runtime crash.

## The Problem This Solves

C's `printf` is famously unsafe: pass the wrong number of arguments, or the wrong type for a format specifier, and you get undefined behavior at runtime. The format string and the argument list are completely decoupled — the compiler sees a `char*` and a variadic list and trusts you got it right.

OCaml solved this properly: `Printf.printf "%s: %d"` has type `string -> int -> unit`. The format string is parsed at compile time and its type reflects exactly how many arguments are needed and what types they must be. Pass an `int` where a `string` is expected and you get a type error, not a segfault.

Rust's `format!` macro achieves the same result differently: the compiler parses the format string in the macro expansion and verifies argument count and types at compile time. Beyond the standard macro, this example shows how to build a *manual* type-level format builder using phantom types — a linked list of type tags that threads through the builder API, ensuring each argument is provided in the right order and at the right type.

## The Intuition

Encode "what arguments are still expected" as a phantom type parameter on a builder struct — each `.str_arg()` or `.int_arg()` call advances the type state, and the compiler rejects calls in the wrong order or of the wrong type.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Type-level "what's still expected" — a linked list of argument types
pub struct End;
pub struct ArgStr<Rest>(PhantomData<Rest>);
pub struct ArgInt<Rest>(PhantomData<Rest>);

// A format builder whose type tracks remaining arguments
pub struct TypedFmt<Spec> {
    parts: Vec<String>,
    _spec: PhantomData<Spec>,
}

// When Spec = ArgStr<Rest>, the next call must be .str_arg()
impl<Rest> TypedFmt<ArgStr<Rest>> {
    pub fn str_arg(mut self, s: &str) -> TypedFmt<Rest> {
        // Consuming ArgStr<Rest>, returning TypedFmt<Rest>
        // The type advances — one slot consumed
        self.parts.push(s.to_string());
        TypedFmt { parts: self.parts, _spec: PhantomData }
    }
    pub fn lit(mut self, s: &str) -> Self {
        self.parts.push(s.to_string()); self
    }
}

// When Spec = ArgInt<Rest>, the next call must be .int_arg()
impl<Rest> TypedFmt<ArgInt<Rest>> {
    pub fn int_arg(mut self, n: i64) -> TypedFmt<Rest> {
        self.parts.push(n.to_string());
        TypedFmt { parts: self.parts, _spec: PhantomData }
    }
}

// When Spec = End, .build() is available — all args provided
impl TypedFmt<End> {
    pub fn build(self) -> String { self.parts.join("") }
}

// Usage: type parameter ensures "str then int" order
let result: TypedFmt<End> =
    TypedFmt::<ArgStr<ArgInt<End>>>::new_with_str()
        .lit("Name: ")
        .str_arg("Bob")    // ArgStr consumed → TypedFmt<ArgInt<End>>
        .lit(", Age: ")
        .int_arg(25)       // ArgInt consumed → TypedFmt<End>
        .build();
// .int_arg() before .str_arg() would be a compile error!
```

## What This Unlocks

- **Arity-safe format APIs** — library authors can expose formatting functions where wrong argument counts or types are rejected at compile time.
- **Type-state machines** — the phantom type pattern applies anywhere you want sequential steps enforced by the compiler (parsers, protocol builders, SQL query builders).
- **Understanding OCaml's `Printf` module** — OCaml's format strings use a GADT internally; this example shows the Rust equivalent and makes the design legible.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type-safe format strings | GADT phantom format string type | `format!` macro + manual phantom builder |
| Argument type checking | Compile-time via GADT | `format!`: compile-time via macro; manual: via type state |
| Mechanism | Format types built into the type system | Phantom type parameters on a builder struct |
| Ergonomics | `Printf.printf "%s: %d" name age` | `format!("{}: {}", name, age)` |
