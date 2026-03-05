📖 **[View on hightechmind.io →](https://hightechmind.io/rust/525-closure-tap-pattern)**

---

# 525: Tap Pattern for Side Effects

**Difficulty:** 2  **Level:** Beginner-Intermediate

Insert logging, debugging, or instrumentation into a pipeline without breaking the data flow.

## The Problem This Solves

You have a clean functional pipeline: `data.iter().filter(pred).map(transform).collect()`. You need to debug it — but adding a `println!` breaks the chain. You'd have to split the pipeline into separate `let` bindings, add the print, then continue. Now your clean one-liner is four lines.

The same problem in production: you want to log intermediate values, emit metrics, or trigger side effects at specific points in a transformation chain without restructuring the chain or touching the data flowing through it.

Without tap, debugging a functional pipeline requires either restructuring it (breaking the flow) or switching to an imperative loop (losing the composition benefits).

## The Intuition

Tap is the functional programmer's `println!` that doesn't interrupt the flow. It's inspired by the Unix `tee` command — pipe data through, copy it somewhere else, let the original flow continue untouched.

`tap(value, |v| println!("{:?}", v))` returns `value` unchanged after running the closure. The value flows through as if tap wasn't there; the side effect happens invisibly.

In Ruby, `.tap` is a built-in method on every object. In JavaScript and Python, you'd write `(x => { console.log(x); return x; })(value)` — a tap lambda. Rust's extension trait approach gives you `.tap(|v| ...)` as a method on any type.

## How It Works in Rust

```rust
// Free function version: run side effect, return value unchanged
fn tap<T, F: FnOnce(&T)>(value: T, f: F) -> T {
    f(&value);   // side effect — observe but don't consume
    value        // return original
}

// Extension trait: adds .tap() to EVERY type
trait Tap: Sized {
    fn tap(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }
    fn tap_mut(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }
    fn tap_if(self, condition: bool, f: impl FnOnce(&Self)) -> Self {
        if condition { f(&self); }
        self
    }
}
impl<T> Tap for T {}   // blanket impl: works on all types

// Usage in a transformation chain
let result = vec![3, 1, 4, 1, 5, 9, 2, 6]
    .tap(|v| println!("input: {:?}", v))      // observe raw input
    .tap_mut(|v| v.sort())                    // sort in place
    .tap(|v| println!("sorted: {:?}", v))     // observe sorted
    .tap_mut(|v| v.dedup())                   // deduplicate
    .tap(|v| println!("deduped: {:?}", v));   // observe deduped

// In an iterator pipeline — map acts as tap for individual elements
let sum: i32 = (1..=5)
    .map(|x| tap(x, |v| print!("in:{} ", v)))     // observe: before filter
    .filter(|&x| x % 2 != 0)                       // keep odds
    .map(|x| tap(x * x, |v| print!("sq:{} ", v))) // observe: after squaring
    .sum();

// Conditional tap — only active in debug builds
let debug = cfg!(debug_assertions);
let x = compute_value()
    .tap_if(debug, |v| log::debug!("value: {:?}", v));
```

The `FnOnce` bound (not `Fn`) reflects that tap is called exactly once per value — this is the minimum required bound, accepting the widest range of closures.

## What This Unlocks

- **Non-destructive debugging** — insert `dbg!` or `println!` anywhere in a pipeline without breaking the chain or restructuring your code.
- **Instrumentation without pollution** — emit metrics or trace spans at pipeline stages; remove them by deleting one line with no structural change.
- **Conditional logging** — `tap_if(debug_mode, log)` or `tap_if(cfg!(test), assert)` adds behavior that compiles away in release mode.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tap | `let () = f x in x` | `x.tap(\|v\| side_effect(v))` |
| Extension method | Module-level function | Blanket `impl<T> Tap for T` |
| Mutable tap | Rebind with `let` | `.tap_mut(\|v\| v.sort())` — in place |
| Conditional | `if debug then f x` | `.tap_if(condition, f)` |
| Iterator tap | Map then ignore result | `.map(\|x\| tap(x, inspect))` |
