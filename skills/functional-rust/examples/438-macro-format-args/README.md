# 438: format_args! for Zero-Alloc Formatting

**Difficulty:** 3  **Level:** Advanced

Capture format arguments lazily without allocating a `String` — the low-level primitive that `println!`, `format!`, and logging frameworks build on.

## The Problem This Solves

`format!("{} = {}", key, value)` always allocates a `String`, even if you're just going to write it to a buffer immediately. In hot loops, logging infrastructure, or embedded systems where the heap is precious, this allocation is waste. You want to describe *what* to format without paying the allocation cost until the bytes are actually written somewhere.

`format_args!` is the compile-time macro that captures format arguments as a `fmt::Arguments` struct — a lazy description of the formatting operation. No allocation happens. You pass `fmt::Arguments` to any `Write` implementor (`File`, `TcpStream`, your custom buffer) and it formats directly into the destination. `println!`, `format!`, `write!`, and `log!` all use `format_args!` internally.

This matters when building logging, tracing, or I/O infrastructure. Accept `fmt::Arguments` in your API and callers get zero-alloc formatting for free.

## The Intuition

`format_args!` captures a format expression as a value that describes the formatting to do, without doing it — allocation happens only when you write it to a concrete output.

## How It Works in Rust

```rust
use std::fmt;
use std::fmt::Write;

// format_args! returns fmt::Arguments — no allocation
let args = format_args!("{} + {} = {}", 1, 2, 3);
// Nothing allocated yet — args is a stack value

// Write to a String (allocates only here)
let mut s = String::new();
write!(s, "{}", args).unwrap();  // "1 + 2 = 3"

// Write to stderr directly — no intermediate String
eprintln!("{}", args);  // format_args! inside

// Accept fmt::Arguments in your own API — zero-alloc logging
fn log(level: &str, args: fmt::Arguments<'_>) {
    // Could write to file, buffer, network — whatever
    println!("[{}] {}", level, args);
}

// Macro wrapper so callers use familiar syntax
macro_rules! my_log {
    ($level:expr, $($arg:tt)*) => {
        log($level, format_args!($($arg)*))
    };
}

my_log!("INFO", "user {} logged in from {}", user_id, ip);
// ↑ No String allocation — format_args captures the args, log() writes them

// format_args! in a struct for deferred formatting
struct Lazy<'a>(fmt::Arguments<'a>);

impl<'a> fmt::Display for Lazy<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(self.0)
    }
}
```

1. `format_args!(...)` — same syntax as `format!` but returns `fmt::Arguments` instead of `String`.
2. `fmt::Arguments` is a stack value referencing the original arguments — no heap allocation.
3. Pass to `write!`, `writeln!`, `print!`, or any `fmt::Write`/`io::Write` implementor.
4. Build macro wrappers: accept `$($arg:tt)*` and forward to `format_args!($($arg)*)`.

## What This Unlocks

- **Zero-allocation logging**: Build log macros that format directly to a sink without intermediate `String`.
- **Deferred formatting**: Capture what to format, decide where to write it later.
- **Efficient I/O**: Write formatted output directly to `TcpStream`, `File`, or custom buffers without intermediate allocation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Format without allocation | `Format.fprintf` with `formatter` | `format_args!` + `fmt::Write` |
| Deferred formatting | `Format.kdprintf` continuation-passing | `fmt::Arguments` value — pass around |
| Format to sink | `Format.fprintf out_channel` | `write!(sink, "{}", args)` |
| Building format macros | `Format.kasprintf` | `macro_rules!` + `format_args!($($arg)*)` |
| String allocation | `Format.asprintf` | `format!` (allocates); `format_args!` (doesn't) |
