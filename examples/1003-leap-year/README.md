# Example #4: Leap Year Validator

**Topic:** Boolean Logic with Divisibility Rules  
**Category:** Pattern Matching & Logic  
**Difficulty:** Beginner

## Overview

This example demonstrates converting a simple boolean expression from OCaml to Rust. A leap year is determined by a clear set of rules:

1. If divisible by **400** → leap year
2. Else if divisible by **100** → not a leap year
3. Else if divisible by **4** → leap year
4. Otherwise → not a leap year

## Learning Outcomes

After studying this example, you will understand:

- How to express complex boolean logic idiomatically in Rust
- Two different approaches to the same problem (expression vs. guards)
- The equivalence between OCaml's `mod` operator and Rust's `%` operator
- How to write comprehensive unit tests in Rust
- How Rust's type system and borrowing rules affect simple functions
- The similarities and differences in how OCaml and Rust express predicates

## OCaml Approach

```ocaml
let leap_year year =
  (year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)
```

**Characteristics:**
- Declarative: expresses the condition directly as a boolean expression
- Uses `mod` for modulo operation
- Uses `||` for logical OR and `&&` for logical AND
- Uses `<>` for "not equal" comparison
- No explicit type annotation (inferred as `int -> bool`)
- Very compact and readable

## Rust Approach

### Idiomatic Expression (Primary)

```rust
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}
```

**Characteristics:**
- Nearly identical to OCaml in structure
- Uses `%` operator for modulo (same semantics as OCaml's `mod`)
- Uses `==` for equality and `!=` for "not equal"
- Explicit return type annotation (`-> bool`)
- Function parameters require type annotations (`year: u32`)
- Clean and direct, minimal overhead

### Alternative Implementation (Guard Clauses)

```rust
pub fn is_leap_year_guards(year: u32) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}
```

**Characteristics:**
- Uses early returns and guard clauses (imperative style)
- More verbose but sometimes clearer intent
- Guards fail-fast: if first condition is true, return immediately
- The last line is an implicit return (idiomatic Rust)
- Easier to add side effects (logging, debugging) if needed

## Key Differences Between OCaml and Rust

### 1. **Type System Explicitness**
   - **OCaml:** Types are inferred; no annotation needed
   - **Rust:** Function signatures require explicit types for parameters and return values
   - **Benefit in Rust:** Code is self-documenting and compiler catches type errors early

### 2. **Operators and Syntax**
   - **OCaml:** `mod` (word), `<>` (not equal), `||` (or), `&&` (and)
   - **Rust:** `%` (symbol), `!=` (not equal), `||` (or), `&&` (and)
   - **Similarity:** Both use short-circuit evaluation for `&&` and `||`
   - **Note:** Rust's `%` works identically to OCaml's `mod` for positive integers

### 3. **Integer Types**
   - **OCaml:** Uses a single `int` type (typically 63-bit on 64-bit systems)
   - **Rust:** Requires explicit integer type (`u32`, `i32`, `u64`, etc.)
   - **Benefit in Rust:** Prevents accidental overflow and clarifies intent (unsigned vs. signed)

### 4. **Function Definitions**
   - **OCaml:** `let` keyword, inferred types, minimal ceremony
   - **Rust:** `fn` keyword, explicit types, visibility modifiers (`pub`)
   - **Visibility:** Rust requires explicit `pub` to export; OCaml functions are public by default

### 5. **Testing Approach**
   - **OCaml:** Uses `assert` statements scattered in code or in separate test files
   - **Rust:** Has a built-in test framework (`#[test]` attribute, `assert!` macros)
   - **Benefit in Rust:** Tests are integrated into the library, run with `cargo test`

## Code Structure

- **`src/lib.rs`** - Main implementation with both approaches and comprehensive tests
- **`example.rs`** - Standalone executable demonstrating both functions
- **`example.ml`** - OCaml version with equivalent tests
- **`Cargo.toml`** - Package manifest for Rust
- **`COMPARISON.md`** - Detailed side-by-side analysis

## Running the Code

### Rust

```bash
# Run tests
cargo test -p example-1003-leap-year

# Run the example executable
rustc --edition 2021 example.rs && ./example
```

### OCaml

```bash
# Compile and run with assertions
ocamlopt example.ml -o leap_year_ml && ./leap_year_ml

# Or with the bytecode compiler
ocamlc example.ml -o leap_year_ml && ./leap_year_ml
```

## Key Insights for Functional Programmers

1. **Expression-Oriented:** Both languages excel at expressing predicates as boolean expressions
2. **Pattern Matching:** While this example doesn't use it, both languages support pattern matching for more complex conditions
3. **Immutability:** Both languages handle immutable values naturally; `year` is never modified
4. **No Side Effects:** The function is pure—same input always produces same output
5. **Type Safety:** Rust's explicit types catch errors earlier; OCaml's inference is more concise
