# Example 003: Pattern Matching Basics

## Concept

Pattern matching is the heart of functional programming. Both OCaml and Rust provide powerful, exhaustive pattern matching, but with different syntax and guarantees. This example explores the core patterns: literals, constructors, guards, tuples, lists, and or-patterns.

## Key Similarities

Both languages enforce:
- **Exhaustiveness checking** - Compiler ensures all cases are covered
- **Irrefutable patterns** - Some contexts require patterns that always match
- **Nested matching** - Patterns can destructure deeply
- **Guard conditions** - Extra boolean tests on matched values

## Syntax Comparison

### Basic Match

**OCaml:**
```ocaml
match value with
| pattern1 -> result1
| pattern2 -> result2
```

**Rust:**
```rust
match value {
    pattern1 => result1,
    pattern2 => result2,
}
```

### Guards

**OCaml:**
```ocaml
match n with
| x when x < 0 -> "negative"
| 0 -> "zero"
```

**Rust:**
```rust
match n {
    x if x < 0 => "negative",
    0 => "zero",
}
```

### Or Patterns

**OCaml:**
```ocaml
| Red | Green | Blue -> true
```

**Rust:**
```rust
Color::Red | Color::Green | Color::Blue => true
// Or use matches! macro:
matches!(color, Color::Red | Color::Green | Color::Blue)
```

### As Patterns

**OCaml:**
```ocaml
| (x :: _) as lst -> x :: lst  (* Bind both x and entire list *)
```

**Rust:**
```rust
[x, rest @ ..] => {  // Bind x and rest slice
    // ...
}
```

## Key Differences

### Ownership in Patterns

**Rust requires thinking about ownership:**

```rust
match value {
    MyEnum::Variant(x) => ...,  // Moves x out of value
}

// To avoid move, use reference patterns:
match &value {
    MyEnum::Variant(x) => ...,  // x is &Type
}
```

OCaml's GC makes this automatic.

### Exhaustiveness Errors

**OCaml:**
```ocaml
(* Warning: pattern matching not exhaustive *)
let f = function
  | Red -> "red"
  (* Missing Green, Blue, RGB cases *)
```

**Rust:**
```rust
// Compiler error: non-exhaustive patterns
match color {
    Color::Red => "red",
    // Must add _ or remaining variants
}
```

Rust's exhaustiveness checking is stricter and happens at compile time as an error (not warning).

### Refutability

**Irrefutable patterns** (always match):
- OCaml: `let (x, y) = tuple` - runtime error if not a tuple
- Rust: Compile error unless pattern is irrefutable

**Refutable patterns** (might not match):
- OCaml: `match` expressions
- Rust: `match`, `if let`, `while let`

## Advanced Features

### matches! Macro (Rust)

Convenient boolean check without full match:

```rust
if matches!(color, Color::Red | Color::Blue) {
    // ...
}
```

OCaml equivalent requires explicit match returning bool.

### Nested Destructuring

Both languages support deep matching:

**OCaml:**
```ocaml
| Rectangle (w, h) when w = h -> "square"
```

**Rust:**
```rust
Shape::Rectangle(w, h) if w == h => "square"
```

## Functional Patterns Demonstrated

1. **Data-oriented design** - Shape behavior via matching, not methods
2. **Exhaustiveness** - Compiler guarantees all cases handled
3. **Immutability** - Patterns extract data without mutation
4. **Expression-oriented** - Match expressions return values

## Common Pitfalls

### Rust-specific

1. **Moving vs borrowing** - Match on `&value` to avoid moves
2. **Pattern types** - Must be compatible with matched value's type
3. **Wildcard placement** - `_` must come last (catch-all)

### OCaml-specific

1. **Warnings not errors** - Non-exhaustive match is warning (runtime error possible)
2. **No if-let** - Must use full match for single-case checks

## When to Use

**Pattern matching over if-else:**
- Working with ADTs/enums
- Multiple related conditions
- Extracting structured data
- Want exhaustiveness guarantees

**if-else over pattern matching:**
- Simple boolean conditions
- Non-exhaustive checks intentionally

## Next Steps

Example 004 will introduce Option and Result, Rust's answer to OCaml's option and result types, showing how pattern matching handles errors functionally.
