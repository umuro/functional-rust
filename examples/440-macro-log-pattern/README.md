📖 **[View on hightechmind.io →](https://hightechmind.io/rust/440-macro-log-pattern)**

---

# 440: Logging Macros Pattern

**Difficulty:** 2  **Level:** Intermediate

Build a minimal logging framework using `macro_rules!` that captures `file!()` and `line!()` at the call site, supports log levels with atomic filtering, and costs nothing when the level is disabled.

## The Problem This Solves

`println!` is the first debugging tool everyone reaches for, but it has no levels, no location info, and no way to disable output at runtime. Adding those features by hand means writing `if self.level >= Level::Warn { eprintln!("[WARN] {}:{} — {}", file!(), line!(), msg) }` everywhere — tedious and inconsistent.

External logging crates (`log`, `tracing`) solve this well in production code. But in embedded contexts, educational code, or early-stage projects, pulling in a crate just for logging feels heavy. Understanding how these crates work by building a minimal version is also an effective way to learn macro design: a logging macro family is small enough to fit in 50 lines but exercises every key `macro_rules!` technique.

The key insight is that `file!()` and `line!()` are macros themselves — they expand to the location in the *calling* source file, not the logging implementation file. This means the macro must call them at the call site, which is exactly what a macro delegation chain achieves.

## The Intuition

A logging macro is a delegation chain. `warn!("msg")` calls `log!(Level::Warn, "msg")`, which calls `log_impl(Level::Warn, file!(), line!(), &format!("msg"))`. Each step is a macro, so `file!()` and `line!()` are always expanded at the original call site — wherever `warn!` was written.

The level filter is an `AtomicU8` global. Comparison is a single load with `Relaxed` ordering — effectively free. When the level is disabled, the format string is never evaluated (short-circuit evaluation before calling `format!`). The `is_enabled` check gates the `format!` call so string allocation only happens when the message will actually be printed.

## How It Works in Rust

```rust
use std::sync::atomic::{AtomicU8, Ordering};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Level { Debug = 0, Info = 1, Warn = 2, Error = 3 }

impl Level {
    fn as_str(self) -> &'static str {
        match self { Level::Debug => "DEBUG", Level::Info => "INFO",
                     Level::Warn  => "WARN",  Level::Error => "ERROR" }
    }
}

// Single atomic: one load per log call, no lock
static MIN_LEVEL: AtomicU8 = AtomicU8::new(Level::Info as u8);

pub fn set_level(l: Level) { MIN_LEVEL.store(l as u8, Ordering::Relaxed); }
pub fn is_enabled(l: Level) -> bool { l as u8 >= MIN_LEVEL.load(Ordering::Relaxed) }

pub fn log_impl(level: Level, file: &str, line: u32, msg: &str) {
    if is_enabled(level) {
        // file!() and line!() are already resolved — this fn sees them as strings
        eprintln!("[{}] {}:{} — {}", level.as_str(), file, line, msg);
    }
}

// ── The macro chain — each level delegates upward ────────────────────────────
#[macro_export]
macro_rules! log {
    // $crate:: ensures log_impl refers to THIS crate even from another crate
    ($lv:expr, $($a:tt)*) => {
        $crate::log_impl($lv, file!(), line!(), &format!($($a)*))
    };
}

#[macro_export] macro_rules! debug { ($($a:tt)*) => { log!($crate::Level::Debug, $($a)*) }; }
#[macro_export] macro_rules! info  { ($($a:tt)*) => { log!($crate::Level::Info,  $($a)*) }; }
#[macro_export] macro_rules! warn  { ($($a:tt)*) => { log!($crate::Level::Warn,  $($a)*) }; }
#[macro_export] macro_rules! error { ($($a:tt)*) => { log!($crate::Level::Error, $($a)*) }; }

// ── Usage ─────────────────────────────────────────────────────────────────────
info!("Starting v{}", "1.0");          // [INFO] src/main.rs:42 — Starting v1.0
debug!("Hidden at Info level");        // (no output — Debug < Info)
warn!("Low memory: {} MB", 42);        // [WARN] src/main.rs:44 — Low memory: 42 MB
set_level(Level::Debug);
debug!("Now visible: x={}", 99);       // [DEBUG] src/main.rs:46 — Now visible: x=99
```

## What This Unlocks

- **Zero-cost disabled levels** — when `debug!` is below the minimum level, the `format!` call is still evaluated… unless you add an `is_enabled` check before `format!` in the `log!` macro for true zero-cost.
- **Correct call-site location** — `file!()` and `line!()` always point to where `warn!` was called, not into the logging library — impossible without macros.
- **Foundation for `log` crate compatibility** — the `log` crate uses exactly this pattern; understanding it lets you write logging-framework-agnostic code.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Call-site location | `__FILE__` / `__LINE__` (not standard); `Printexc.get_callstack` (runtime) | `file!()` / `line!()` macros — compile-time, zero cost |
| Log level filtering | Third-party (`Logs` crate); uses runtime dispatch | `AtomicU8` global — single load, no lock |
| Macro delegation chain | ppx transformations; `Printf.printf` variadic | `macro_rules!` delegation: `warn!` → `log!` → `log_impl` |
| Format string | `Printf.sprintf format` | `format!($($a)*)` — same format string syntax as `println!` |
