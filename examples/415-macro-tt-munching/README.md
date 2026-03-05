📖 **[View on hightechmind.io →](https://hightechmind.io/rust/415-macro-tt-munching)**

---

# 415: Token Tree Munching

**Difficulty:** 4  **Level:** Expert

Consume an arbitrary token stream one token at a time to implement complex DSLs and parsers inside `macro_rules!`.

## The Problem This Solves

Standard `macro_rules!` patterns work well when input has a fixed structure. But what if you're implementing a DSL where the grammar is complex or context-sensitive? What if you want to define a struct with field defaults (`port: u16 = 8080,`), or parse a mini arithmetic expression (`2 + 3 * 4`), or process a list of heterogeneous token sequences?

Simple repetition patterns (`$(...)*`) match uniform sequences. They can't handle inputs where each element has a different structure, or where you need to accumulate state across elements in complex ways. Token tree munching (TTM) is the technique that handles this: use `$head:tt` to consume one token tree at a time, match on its shape, and recurse with the remaining tokens. It's essentially hand-written parser combinators, operating at compile time, inside `macro_rules!`.

This is how complex macro libraries work: `serde` attribute parsing, `clap` command-line DSLs, async frameworks. When the grammar is too rich for simple patterns, TTM is the tool.

## The Intuition

Token tree munching gets its name from the pattern: eat one token tree (`tt`), decide what to do with it, emit output, then recurse on the rest. Each recursive call "munches" one more token tree from the front of the input.

The key insight: `tt` matches any single token (identifier, operator, literal) OR any balanced group (`(...)`, `[...]`, `{...}` with everything inside). By matching `$head:tt $($rest:tt)*`, you can peel off the front of any token stream and process it.

State between recursive calls is carried in accumulated output (internal arms build up a result). The entry point sets up the initial accumulator; the base case returns the accumulated result.

## How It Works in Rust

```rust
// Define a struct from a DSL: "field: type = default,"
// TTM processes one field at a time
macro_rules! define_struct {
    // Base case: no more fields — emit the struct
    (@fields $name:ident {} -> { $($fields:tt)* }) => {
        #[derive(Debug, Default)]
        struct $name {
            $($fields)*
        }
    };

    // Recursive case: consume one "field: ty = default," and continue
    (@fields $name:ident {
        $field:ident : $ty:ty = $default:expr ,  // munch one field
        $($rest:tt)*                              // remaining input
    } -> { $($fields:tt)* }) => {
        define_struct!(@fields $name { $($rest)* } -> {
            $($fields)*
            $field: $ty,  // accumulate field declarations
        });
    };

    // Entry point
    (struct $name:ident { $($body:tt)* }) => {
        define_struct!(@fields $name { $($body)* } -> {});
    };
}

define_struct!(struct Config {
    port: u16 = 8080,
    debug: bool = false,
    max_connections: u32 = 100,
});

// Simple arithmetic DSL — munch one operator+operand at a time
macro_rules! calc {
    ($n:literal) => { $n };                           // base: single number
    ($a:literal + $($rest:tt)+) => { $a + calc!($($rest)+) };
    ($a:literal - $($rest:tt)+) => { $a - calc!($($rest)+) };
    ($a:literal * $b:literal) => { $a * $b };
    ($a:literal * $b:literal + $($rest:tt)+) => { ($a * $b) + calc!($($rest)+) };
}

fn main() {
    let c = Config { port: 9090, ..Default::default() };
    println!("{:?}", c);

    // calc DSL — evaluated at compile time
    println!("2 + 3 * 4 = {}", calc!(2 + 3 * 4));  // 14 (left-to-right)
    println!("10 - 3 + 2 = {}", calc!(10 - 3 + 2));
    println!("3 * 4 = {}", calc!(3 * 4));
}
```

**The TTM pattern anatomy:**
1. Internal arm tagged with `@tag` — carries accumulated state
2. Match `$next:tt` (or a more specific pattern) — consume one token tree
3. Emit output for this token
4. Recurse with `$($rest:tt)*` — process remaining tokens
5. Base case — when input is empty, emit the final accumulated result

## What This Unlocks

- **Custom DSL grammars** — parse `routes! { GET /api => handler }`, `sql! { SELECT * FROM users WHERE age > 18 }` — any syntax you can describe with pattern matching.
- **Struct/enum generation with metadata** — `define_struct!` with types, defaults, documentation, validation attributes — all from a compact macro call.
- **Complex repetition with context** — process pairs, triples, or context-dependent sequences that `$(...)*` can't handle uniformly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Token-based parsing | Lexer/parser functions — runtime | TTM — compile time, inside `macro_rules!` |
| Accumulator | `let rec f acc tokens = ...` — runtime function | `@acc` internal arm — compile-time recursion |
| Token types | `token` variants in a type | `tt` fragment — any single token or balanced group |
| Parser combinators | `angstrom`, `menhir` — runtime or code-gen | TTM — in-language, no dependencies, zero runtime cost |
