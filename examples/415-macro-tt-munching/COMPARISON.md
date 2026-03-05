# OCaml vs Rust: Token Tree Munching

## What is TT Munching?

Token Tree Munching (TTM) is a macro technique that:
1. Consumes input tokens one at a time
2. Accumulates results in an internal state
3. Recurses until input is exhausted

It's the macro equivalent of recursive descent parsing.

---

## Example: Parsing a DSL

```rust
// Parse: struct Name { field: Type = default, ... }
macro_rules! define_config {
    // Done: emit the struct
    (@fields $name:ident {} -> { $($fields:tt)* }) => {
        struct $name { $($fields)* }
    };

    // Munch one field
    (@fields $name:ident {
        $field:ident : $ty:ty = $default:expr,
        $($rest:tt)*
    } -> { $($fields:tt)* }) => {
        define_config!(@fields $name { $($rest)* } -> {
            $($fields)*
            $field: $ty,
        });
    };

    // Entry point
    (struct $name:ident { $($body:tt)* }) => {
        define_config!(@fields $name { $($body)* } -> {});
    };
}

define_config!(struct Config {
    port: u16 = 8080,
    debug: bool = false,
});
```

---

## The Pattern

```rust
macro_rules! muncher {
    // Base case: input exhausted
    (@internal $acc:tt) => { /* emit result */ };

    // Recursive case: consume one token, accumulate, recurse
    (@internal $acc:tt $head:tt $($tail:tt)*) => {
        muncher!(@internal (/* new acc with $head */) $($tail)*)
    };

    // Entry point
    ($($input:tt)*) => {
        muncher!(@internal () $($input)*)
    };
}
```

---

## OCaml Equivalent

OCaml doesn't have macros, but parser combinators achieve similar goals:

```ocaml
(* Using parser combinators *)
let rec parse_fields = function
  | [] -> []
  | Field(name, ty, default) :: rest ->
      (name, ty, default) :: parse_fields rest
```

---

## When to Use TTM

- **DSL parsing**: Config files, query languages
- **Code generation**: Structs with defaults
- **Complex syntax**: When simple repetition isn't enough

---

## 5 Takeaways

1. **TTM is recursive descent for macros.**
   Consume tokens left-to-right, accumulate results.

2. **Use `@internal` prefix for helper rules.**
   Keeps the public API clean.

3. **`$($rest:tt)*` captures remaining tokens.**
   The "rest of input" to process recursively.

4. **Accumulator pattern collects results.**
   Build up output as you consume input.

5. **Complex parsing needs TTM.**
   Simple `$(...)*` repetition can't handle context-sensitive syntax.
