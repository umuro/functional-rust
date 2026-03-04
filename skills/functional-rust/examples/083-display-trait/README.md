# 083: Display Trait

**Difficulty:** 2  **Level:** Intermediate

Implement `fmt::Display` for your type so it works with `println!("{}")`, `format!()`, and `.to_string()` — the standard Rust vocabulary for user-facing string representation.

## The Problem This Solves

Every custom type needs two kinds of string representation: one for developers (debug output, logs) and one for users (UI, reports, messages). Rust separates these cleanly: `Debug` (via `{:?}`) is for developers and can be derived automatically; `Display` (via `{}`) is for users and you implement it yourself.

Without `Display`, you can't use your type in `println!("{}", my_value)`, can't pass it to `format!`, and can't call `.to_string()`. You'd end up writing ad-hoc `to_string` methods that return `String` — scattered, inconsistent, and not interoperable with the rest of the ecosystem.

Implementing `Display` once gives you all of this for free, and any function parameterized on `T: Display` can work with your type automatically.

## The Intuition

In Python, you implement `__str__` for user-facing output and `__repr__` for developer output. In Java, you override `toString()`. In OCaml, you write a `pp` function or use `Format.formatter`. In Rust, `Display` is the standard that the whole ecosystem depends on — implement it and your type integrates everywhere.

The key: `Display` uses a `Formatter` rather than returning a `String`. This is a performance choice — you write directly into the output buffer, no intermediate allocation required. The `write!(f, ...)` macro inside your `fmt` method works just like `println!` but targets the formatter.

## How It Works in Rust

```rust
use std::fmt;

struct Point { x: f64, y: f64 }

// Display: for users — "(3.14, 2.72)"
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// Debug: for developers — derive it when possible
#[derive(Debug)]
struct Point { x: f64, y: f64 }
```

```rust
// Enum Display: match on variant, write different strings
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red   => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue  => write!(f, "Blue"),
        }
    }
}
```

```rust
// Multi-line Display: use writeln! for intermediate lines, write! for the last
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.data.iter().enumerate() {
            write!(f, "| ")?;                          // ? propagates errors
            for val in row { write!(f, "{:6.2} ", val)?; }
            write!(f, "|")?;
            if i < self.data.len() - 1 { writeln!(f)?; }  // newline between rows
        }
        Ok(())
    }
}
```

```rust
// Recursive Display for trees — works because T: Display is a bound
impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Leaf => write!(f, "."),
            Tree::Node(l, v, r) => write!(f, "({} {} {})", l, v, r),  // recursive
        }
    }
}
```

Once `Display` is implemented, `.to_string()` is available for free — it's automatically provided by the `ToString` trait, which is blanket-implemented for all `Display` types.

## What This Unlocks

- **`format!` and `println!` integration**: your type works in any string formatting context without ceremony.
- **Generic display functions**: write `fn show_all<T: Display>(items: &[T])` once — works for your type and every other `Display` type.
- **Error messages and logging**: when your domain types implement `Display`, error messages can embed them directly: `format!("Failed to process {}", my_value)`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| User-facing representation | Custom `to_string` or `pp` function | `impl fmt::Display` |
| Developer/debug output | Custom `show` or `Format.fprintf` | `#[derive(Debug)]` + `{:?}` |
| String conversion | `string_of_*` functions | `.to_string()` (free from `Display`) |
| Format destination | `Format.formatter` or `Buffer.t` | `fmt::Formatter` (write with `write!`) |
| Recursive types | Manual recursive `pp` | Recursive `fmt` call — `write!(f, "{}", child)` |
