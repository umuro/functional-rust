# 053: Pipeline Operator

**Difficulty:** 1  **Level:** Beginner

Thread a value through a sequence of functions left-to-right — Rust's answer to OCaml's `|>`.

## The Problem This Solves

Nested function calls read backwards. `exclaim(shout(trim(" hello ")))` means "first trim, then shout, then exclaim" but your eye reads right-to-left. With three functions it's manageable; with six it's a maze.

OCaml's `|>` operator solves this elegantly: `" hello " |> trim |> shout |> exclaim`. Left to right. The data flows in the direction you read.

Rust has no `|>` operator. But Rust has method chaining (`.method()`) which achieves the same left-to-right readability for types that own the right methods. When those methods don't exist, a `Pipe` trait and a `pipe!` macro replicate the `|>` pattern for any type.

## The Intuition

A pipeline is an assembly line. The product starts at one end, each station transforms it, and the final product comes out the other end. `|>` makes that assembly line visible in code: each `|>` is a station.

Rust's `.pipe(f)` does the same thing: `x.pipe(f)` is just `f(x)`. It's reverse function application, nothing more. But the syntax makes the data flow obvious:

```
5.pipe(double).pipe(add1)   →  reads left to right
add1(double(5))             →  reads right to left
```

## How It Works in Rust

```rust
// A trait that adds .pipe(f) to any type
pub trait Pipe: Sized {
    fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B {
        f(self)  // that's it — just call f with self
    }
}
impl<T> Pipe for T {}  // implement for every type

// Macro for explicit visual pipelines
macro_rules! pipe {
    ($val:expr => $($f:expr),+) => {{
        let mut v = $val;
        $(v = $f(v);)+
        v
    }};
}

// Three ways to write the same thing:
let r1 = add1(double(5));           // nested — reads right to left
let r2 = 5.pipe(double).pipe(add1); // trait — reads left to right
let r3 = pipe!(5 => double, add1);  // macro — explicit pipeline syntax
// r1 == r2 == r3 == 11
```

`FnOnce` (not `Fn`) works here because each step consumes the value and produces a new one. Ownership flows through the chain naturally.

## What This Unlocks

- **Readable data transformation chains** — parsing, normalising, and formatting pipelines that read in execution order.
- **Ad-hoc function application** — apply any closure to a value inline: `user.pipe(|u| cache.get(u))`.
- **Zero cost** — `.pipe(f)` inlines to a direct function call at compile time; no overhead.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in operator | `\|>` in stdlib | Not available |
| Equivalent pattern | `x \|> f \|> g` | `x.pipe(f).pipe(g)` or `pipe!(x => f, g)` |
| Method chains | `x \|> String.uppercase_ascii` | `x.to_uppercase()` (if method exists) |
| Ownership | GC handles intermediate values | Each step consumes and produces |
| Macro option | No | `pipe!` macro replicates `\|>` syntax |
