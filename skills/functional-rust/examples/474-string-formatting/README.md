# 474: format!, write!, writeln!

**Difficulty:** 1  **Level:** Beginner

Rust's compile-time-checked formatting macros — Python's f-strings, but verified at compile time.

## The Problem This Solves

In Python, f-strings and `.format()` are evaluated at runtime. A typo in a format string (`f"{naem}"`) only fails when that line executes. In JavaScript, template literals offer no type checking. You can pass anything and get a surprise.

Rust's format macros are checked at compile time. If you write `format!("{} {}", x)` but only provide one argument, it's a **compile error** — not a runtime crash. If you use `{:?}` on a type that doesn't implement `Debug`, it won't compile. This catches entire classes of bugs before your code ships.

Beyond correctness, Rust's format specifiers are comprehensive: alignment, padding, precision, radix (hex, binary, octal), scientific notation, named arguments, and debug printing. And with `write!`/`writeln!`, you can format directly into any `Write` target — a `String`, a file, a network socket — without intermediate allocations.

## The Intuition

`format!()` is Python's f-strings. The `{}` placeholder calls `Display` (the user-facing representation). `{:?}` calls `Debug` (the developer representation — roughly `repr()` in Python). `{:#?}` is pretty-printed debug.

Think of format specifiers as a mini-language inside the braces:
- `{:>10}` — right-align in 10 chars
- `{:.2}` — 2 decimal places  
- `{:08b}` — 8 digits, zero-padded, binary
- `{:x}` — lowercase hex
- `{name}` — named argument (Rust 1.58+)

`write!(buf, ...)` writes into a `String` or `File` — same syntax, no allocation overhead.

## How It Works in Rust

```rust
use std::fmt::Write as FmtWrite;  // needed for write! on String

// Basic substitution
let s = format!("Hello, {}! Age: {}", "Alice", 30);

// Alignment: < left, > right, ^ center
println!("|{:<10}|{:>10}|", "left", "right"); // |left      |     right|
println!("|{:^10}|", "center");               // |  center  |

// Zero-padding: fill char before >
println!("{:0>8}", 42);   // 00000042
println!("{:08}", 42);    // 00000042 (shorthand for numbers)

// Number radix
println!("{:x}", 255);    // ff
println!("{:X}", 255);    // FF
println!("{:b}", 42);     // 101010
println!("{:o}", 8);      // 10

// Floating point precision
println!("{:.2}", 3.14159);    // 3.14
println!("{:e}", 1_234_567.0); // 1.234567e6

// Debug vs Display
println!("{:?}", vec![1,2,3]);  // [1, 2, 3]  ← Debug
println!("{:#?}", vec![1,2,3]); // pretty-printed

// Write into a String buffer (no allocation for the format itself)
let mut buf = String::new();
write!(buf, "x={} ", 42).unwrap();
writeln!(buf, "y={}", 99).unwrap();

// Named arguments (Rust 1.58+)
let name = "Bob";
let age = 25u32;
println!("{name} is {age}");  // same as "{} is {}", name, age
```

## What This Unlocks

- **Building dynamic strings** — config values, error messages, log lines — with full type safety.
- **Writing to files and buffers** — `write!(file, ...)` works exactly like `write!(string, ...)`.
- **Custom Display/Debug** — implement `fmt::Display` on your types to control how they print everywhere.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String interpolation | `Printf.sprintf "%s! %d" str int` | `format!("{} {}", str, int)` |
| Left-align | `%-10s` | `{:<10}` |
| Right-align | `%10s` | `{:>10}` |
| Zero-pad number | `%08d` | `{:08}` |
| Float precision | `%.2f` | `{:.2}` |
| Hex | `%x` | `{:x}` |
| Write to buffer | `Printf.bprintf buf` | `write!(buf, ...)` |
| Debug print | No built-in | `{:?}` via `Debug` trait |
| Format checked at | Runtime | **Compile time** |
