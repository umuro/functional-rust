# 418: stringify! and concat!

**Difficulty:** 2  **Level:** Intermediate

Turn source code into strings and join string literals at compile time — the bridge between code and text.

## The Problem This Solves

Sometimes you need the source text of an expression, not its value. `assert_eq!(a, b)` shows `"left == right: a = 42, b = 43"` — it knows what `a` and `b` are *named* in the source. A regular function can't do this: it only receives values, not the identifiers that produced them.

`stringify!` captures an expression, identifier, type, or any token sequence and produces it as a string literal — the source text, verbatim. `concat!` joins string literals (including those from `stringify!`, `file!`, `line!`) into a single compile-time constant. Together they let you build rich diagnostic messages, generate human-readable names from identifiers, and produce SQL-like strings from Rust code — all without any runtime cost.

## The Intuition

`stringify!` doesn't evaluate its input. It captures the token sequence and turns it into a `&str` literal. `stringify!(1 + 1)` is `"1 + 1"`, not `"2"`. `stringify!(my_var)` is `"my_var"`, not the value of the variable. This is the key distinction from any runtime operation — you're operating on syntax, not values.

`concat!` is similar: it joins literal strings at compile time into a single `&str`. Since all arguments must be compile-time constants (`literal` or other compile-time macros), there's no runtime allocation. The resulting string is baked into the binary.

Together with `file!`, `line!`, `column!`, and `env!`, they provide a toolkit for compile-time string construction — diagnostic infrastructure, version strings, code location tracking.

## How It Works in Rust

```rust
// stringify!: source text as string, not the value
macro_rules! show_expr {
    ($e:expr) => {
        println!("{} = {:?}", stringify!($e), $e)
        //         ^^^^^^^^^^^              ^^
        //         source text of e         evaluated value
    };
}

// Field name → string (for serialization, reflection, debugging)
macro_rules! field_name {
    ($field:ident) => { stringify!($field) };
}

// Generate test functions with descriptive names
macro_rules! test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            assert_eq!($input, $expected,
                concat!("Test '", stringify!($name), "' failed"));
        }
    };
}

// concat!: join literals at compile time — zero allocation
const HELLO: &str = concat!("Hello", ", ", "World", "!");

// Build a version string from multiple sources
const VERSION: &str = concat!("myapp v1.0.0 (", file!(), ")");

// SQL DSL using stringify! for table and column names
macro_rules! select_cols {
    ($f:ident) => { stringify!($f) };
    ($f:ident, $($rest:ident),+) => {
        concat!(stringify!($f), ", ", select_cols!($($rest),+))
    };
}

macro_rules! select {
    ($table:ident . $($col:ident),+) => {
        concat!("SELECT ", select_cols!($($col),+), " FROM ", stringify!($table))
    };
}

// Location info at compile time
macro_rules! here {
    () => { concat!(file!(), ":", line!()) };
}

fn main() {
    // show_expr: source text vs value
    show_expr!(2 + 3 * 4);       // "2 + 3 * 4 = 14"
    show_expr!("hello".len());   // "\"hello\".len() = 5"

    // stringify doesn't evaluate — it captures source text
    let x = 42;
    println!("stringify!(x) = {}", stringify!(x));       // "x" not "42"
    println!("stringify!(1+1) = {}", stringify!(1 + 1)); // "1 + 1" not "2"

    // field names for serialization/display
    println!("Field: {}", field_name!(user_id));      // "user_id"
    println!("Field: {}", field_name!(created_at));   // "created_at"

    // compile-time constants
    println!("{}", HELLO);
    println!("{}", VERSION);

    // SQL DSL
    println!("{}", select!(users.id, name, email));
    // "SELECT id, name, email FROM users"

    println!("Called at: {}", here!());
}

// These generate actual #[test] functions:
test_case!(addition_works, 2 + 2, 4);
test_case!(string_length, "rust".len(), 4);
```

**compile-time string macros quick reference:**

| Macro | Returns | Example |
|-------|---------|---------|
| `stringify!($tokens)` | Source text as `&str` | `stringify!(a + b)` → `"a + b"` |
| `concat!(...)` | Joined literals as `&str` | `concat!("a", "b")` → `"ab"` |
| `file!()` | Current file path | `"src/main.rs"` |
| `line!()` | Current line number | `42usize` |
| `column!()` | Current column | `8usize` |
| `env!("VAR")` | Env var at compile time | `env!("CARGO_PKG_VERSION")` |
| `include_str!("f")` | File contents as `&str` | Contents of file at compile time |

## What This Unlocks

- **Rich diagnostics** — `assert_eq!`, `dbg!`, custom check macros that show both the expression text and its value — impossible with regular functions.
- **Reflection without runtime overhead** — `field_name!(user_id)` → `"user_id"` for JSON serialization, database column names, or debug output — all at compile time.
- **Compile-time string DSLs** — generate SQL, regex patterns, format strings, version identifiers from structured Rust syntax; validates structure at compile time, produces `&str` at zero cost.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Source location | `__FILE__`, `__LINE__`, `__LOC__` — same idea | `file!()`, `line!()`, `column!()` — same concept |
| Identifier → string | Manual string literal `"field_name"` | `stringify!(field_name)` — guaranteed to match the identifier |
| String concatenation | `"a" ^ "b"` — runtime | `concat!("a", "b")` — compile-time, produces a single `&str` constant |
| Reflection | `Obj.field` introspection — runtime | `stringify!` — syntax-level, compile-time only |
