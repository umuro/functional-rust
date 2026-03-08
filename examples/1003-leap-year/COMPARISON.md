# Detailed Comparison: OCaml vs Rust — Leap Year Validator

## Side-by-Side Code Comparison

### OCaml Implementation

```ocaml
let leap_year year =
  (year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)

(* Usage: leap_year 2000 *)
(* Result: true *)
```

### Rust Implementation (Idiomatic)

```rust
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

// Usage: is_leap_year(2000)
// Result: true
```

### Rust Implementation (Guard Clauses)

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

## Type Signature Comparison

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Function Definition** | `let leap_year year =` | `fn is_leap_year(year: u32) -> bool` |
| **Parameter Type** | Inferred (any numeric type supporting `mod`) | Explicit: `u32` (unsigned 32-bit integer) |
| **Return Type** | Inferred: `bool` | Explicit: `bool` |
| **Type Annotation** | Optional/Inferred | Required |
| **Visibility** | Public by default | Requires `pub` keyword |
| **Full Signature** | `int -> bool` | `fn(u32) -> bool` |

## Operator Mapping

| Operation | OCaml | Rust | Semantics |
|-----------|-------|------|-----------|
| **Modulo** | `mod` | `%` | Remainder after division |
| **Equal** | `=` | `==` | Comparison (not assignment) |
| **Not Equal** | `<>` | `!=` | Inequality check |
| **Logical AND** | `&&` | `&&` | Short-circuit AND |
| **Logical OR** | `\|\|` | `\|\|` | Short-circuit OR |
| **Assignment** | `<-` (mutable) | `=` | Bind/assign value |

## Execution Flow Analysis

### Leap Year Check: 2000

**OCaml:**
```
(2000 mod 400 = 0) || (2000 mod 4 = 0 && 2000 mod 100 <> 0)
↓
(0 = 0) || (...)
↓
true || (...)        ← Short-circuit: stops here
↓
true
```

**Rust:**
```
(2000 % 400 == 0) || (2000 % 4 == 0 && 2000 % 100 != 0)
↓
(0 == 0) || (...)
↓
true || (...)        ← Short-circuit: stops here
↓
true
```

### Leap Year Check: 1900

**OCaml:**
```
(1900 mod 400 = 0) || (1900 mod 4 = 0 && 1900 mod 100 <> 0)
↓
(300 = 0) || (0 = 0 && 0 <> 0)
↓
false || (true && false)
↓
false || false
↓
false
```

**Rust:**
```
(1900 % 400 == 0) || (1900 % 4 == 0 && 1900 % 100 != 0)
↓
(300 == 0) || (0 == 0 && 0 != 0)
↓
false || (true && false)
↓
false || false
↓
false
```

## Memory and Performance Considerations

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Integer Size** | 63-bit signed (on 64-bit systems) | 32-bit unsigned (u32) or specified type |
| **Stack Allocation** | Automatic | On stack (no allocation needed) |
| **Register Usage** | Likely one register per operation | Single register, highly optimized |
| **Generated Code** | 3-4 mod operations + 2 comparisons | 3-4 mod operations + 2 comparisons |
| **Optimization** | OCaml compiler optimizes well | LLVM backend optimizes aggressively |
| **Inlining** | Inline if called from optimized code | Always inlined in release builds |

## Testing Approach

### OCaml
```ocaml
let () = assert (leap_year 2000 = true)
let () = assert (leap_year 1900 = false)
let () = assert (leap_year 2004 = true)
let () = assert (leap_year 2001 = false)
```

**Characteristics:**
- Uses assertions in module scope
- Tests executed at module initialization
- No structured test framework
- Failed assertions raise `Assert_failure` exception

### Rust
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_by_400() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn test_divisible_by_100_not_400() {
        assert!(!is_leap_year(1900));
    }
}
```

**Characteristics:**
- Uses `#[test]` attribute for test functions
- Organized in `tests` module
- Run with `cargo test` command
- Includes test output and failure details
- Can be built/run separately from main code

## Error Handling

### OCaml Behavior
- **Negative numbers:** `mod` operator works with negative numbers
  - Example: `(-5) mod 3 = (-2)` (follows divisor sign)
- **Type errors:** Caught at compile time (strict type checking)
- **Runtime errors:** Exceptions raised (e.g., Division by zero)

### Rust Behavior
- **Unsigned integers:** `u32` prevents negative input at type level
- **Unsigned mod:** `%` with `u32` always produces non-negative result
- **Type errors:** Caught at compile time (strict type checking)
- **Panics:** Division by zero panics; prevented at type level for `u32`

## Documentation

### OCaml
```ocaml
(** Determines if a year is a leap year.
    @param year the year to check
    @return true if the year is a leap year, false otherwise
*)
let leap_year year = ...
```

**Tool:** OCamldoc or plain comments

### Rust
```rust
/// Determines if a year is a leap year.
///
/// # Arguments
/// * `year` - The year to check
///
/// # Returns
/// `true` if the year is a leap year, `false` otherwise
pub fn is_leap_year(year: u32) -> bool { ... }
```

**Tool:** `rustdoc` (integrated into cargo)

## 5 Key Insights

### 1. **Operator Similarity Masks Type System Differences**

At first glance, the code looks nearly identical:
```
(year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)  // OCaml
(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)      // Rust
```

But beneath the surface:
- **OCaml:** `year` could be any numeric type; `mod` is polymorphic
- **Rust:** `year` must be `u32` (or explicitly annotated); no ambiguity
- **Impact:** Rust catches type errors earlier; OCaml is more flexible but requires inference

### 2. **Integer Type Matters More in Rust**

In Rust, choosing `u32` vs `i32` vs `u64` has real consequences:
- **`u32`** (unsigned 32-bit): Years fit comfortably; prevents negative numbers
- **`i32`** (signed 32-bit): Allows negative years but uses extra bit for sign
- **`u64`** (unsigned 64-bit): Overkill for years, wastes space

In OCaml, `int` is polymorphic and the type checker infers the best fit. Rust forces the programmer to decide, which can feel tedious but is more explicit.

### 3. **Short-Circuit Evaluation Is Critical**

Both languages support short-circuit evaluation:
```
(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
```

This means:
- If `year % 400 == 0` is true, the rest of the expression is **never evaluated**
- This saves 2-3 modulo operations on average
- Both languages implement this identically

Without short-circuit evaluation, we'd always compute 3 modulo operations (wasteful).

### 4. **Guard Clauses Trade Brevity for Clarity**

The guard clause version is longer but potentially clearer:
```rust
if year % 400 == 0 { return true; }      // "First check this"
if year % 100 == 0 { return false; }     // "Then check this"
year % 4 == 0                             // "Finally check this"
```

This reads more like natural language ("if this, return true; else if that, return false").

The expression version is more compact but requires the reader to understand operator precedence and associativity.

### 5. **Rust's Explicit Return Patterns Enable Refactoring**

In the guard clause version:
```rust
if year % 400 == 0 { return true; }
```

If you later want to add logging or side effects:
```rust
if year % 400 == 0 {
    println!("Year {} is leap (divisible by 400)", year);
    return true;
}
```

In the expression version, adding side effects would require restructuring:
```rust
pub fn is_leap_year(year: u32) -> bool {
    let divisible_by_400 = year % 400 == 0;
    if divisible_by_400 {
        println!("Year {} is leap (divisible by 400)", year);
    }
    divisible_by_400 || (year % 4 == 0 && year % 100 != 0)
}
```

This is one reason why guard clauses are sometimes preferred in larger functions: they give you a "handle" to insert imperative code.

## Summary Table

| Metric | OCaml | Rust |
|--------|-------|------|
| **Lines of Code** | 1 | 1 (expression) / 6 (guards) |
| **Type Inference** | Complete | Partial (parameters explicit) |
| **Integer Type** | Inferred | Explicit (`u32`) |
| **Documentation** | OCamldoc-style | Rustdoc (integrated) |
| **Testing** | Ad-hoc assertions | Built-in test framework |
| **Compilation** | Bytecode or native | LLVM IR to native |
| **Execution Speed** | Fast (native compiler) | Very fast (LLVM optimization) |
| **Refactoring Ease** | Easy (simple expression) | Moderate (type constraints) |
| **Maintainability** | High (concise, clear) | High (explicit, safe) |

Both implementations are correct, safe, and efficient. The choice between them depends on your project's priorities: OCaml favors brevity and inference, while Rust favors explicitness and safety guarantees.
