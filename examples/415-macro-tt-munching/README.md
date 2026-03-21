📖 **[View on hightechmind.io →](https://hightechmind.io/rust/415-macro-tt-munching)**

---

# 415: Token Tree Munching
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Complex macro DSLs need to parse arbitrary syntax that doesn't fit standard repetition patterns. Token tree (TT) munching processes input one token tree at a time: one arm peels off the first `$tt` and processes it, recursing with the remainder. This enables parsing heterogeneous sequences, implementing mini-parsers within macros, and supporting complex field definition syntax like `field: Type = default`. TT munching is the technique behind the `bitflags!`, `clap::arg!`, and `pest` grammar macros.

Understanding TT munching unlocks the ability to write DSL macros that parse natural, human-readable syntax rather than forcing callers into rigid comma-separated patterns.

## Learning Outcomes

- Understand the token tree munching technique: peel one `$tt` per recursive step
- Learn how `@internal_name` naming conventions mark internal macro arms
- See how `define_config!` parses `field: Type = default,` syntax incrementally
- Understand when TT munching is needed vs. simpler repetition patterns
- Learn the trade-off: TT munching is powerful but slower to compile than simple repetition

## Rust Application

In `src/lib.rs`, `define_config!` uses TT munching to parse a struct definition with defaults. The `@fields` internal arm processes one `$field: $ty = $default,` at a time, accumulating field declarations and default values separately. The base case emits the struct and `Default` impl. The public entry point `(struct $name:ident { $($body:tt)* })` captures the entire body as `tt` tokens, which the internal arms then parse incrementally.

## OCaml Approach

OCaml's PPX approach to DSL parsing is more direct: it operates on the already-parsed OCaml AST. A PPX extension receives a `Parsetree.structure` (sequence of items) and transforms it. For custom syntax, OCaml uses Menhir parser generators or `angstrom` combinator parsers at runtime. True DSL parsing during OCaml compilation requires `camlp5` or ppx extensions, which are more powerful than `macro_rules!` TT munching but require more infrastructure.

## Key Differences

1. **Token level**: Rust TT munching operates on raw tokens; OCaml PPX operates on parsed AST nodes, making it more structured.
2. **Accumulation**: Rust uses `$($acc:tt)*` accumulator arms; OCaml PPX uses mutable buffers or immutable list accumulation in OCaml code.
3. **Compile time**: TT munching macros can be slow to compile for large inputs due to recursive expansion; OCaml PPX runs as a separate program once.
4. **Error messages**: TT munching errors appear as "no rules matched" at the point of failure; OCaml PPX can emit custom error messages using `Location.error`.

## Exercises

1. **Enum with methods**: Write `define_enum!(Status { Active => "active", Inactive => "inactive" })` that generates an enum and a `fn as_str(&self) -> &str` method using TT munching to parse each variant-to-string mapping.
2. **Builder DSL**: Implement `build_struct!(Point { x: f64 required, y: f64 required, label: String optional = "".to_string() })` where `required` fields must be set and `optional` fields have defaults.
3. **State machine**: Create `state_machine!(start: Idle { on(Event::Start) => Running }, Running { on(Event::Stop) => Idle })` using TT munching to generate a state enum and `transition` method.
