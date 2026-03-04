# 526: Pipe Operator Simulation

**Difficulty:** 2  **Level:** Beginner-Intermediate

Simulate OCaml's `|>` operator in Rust — left-to-right function application that reads like a sentence.

## The Problem This Solves

Nested function calls read inside-out: `format_output(normalize(parse(input)))` — you have to read right-to-left to understand the order of operations. With three transformations it's manageable; with five it's a puzzle.

Rust's method chaining solves this for types that own their methods. But what about free functions? `parse(input)` followed by `normalize(...)` followed by `format_output(...)` — there are no methods to chain because these aren't methods.

Also: sometimes you want to apply a function from an external crate, pass through a generic utility, or change the type at each step. None of these fit method chaining. You need a way to write free-function pipelines left-to-right.

## The Intuition

OCaml's `|>` pipe operator is just function application flipped: `x |> f` means `f(x)`. The power comes from chaining: `x |> f |> g |> h` means `h(g(f(x)))` — but reads left-to-right like a recipe: "start with x, apply f, then g, then h."

Elixir, F#, and Haskell all have this operator. Rust's RFC for `|>` was declined (method chaining is preferred). But a `Pipe` extension trait achieves the same result: `x.pipe(f).pipe(g).pipe(h)`.

Think of it as reversing the function call: instead of `f(x)`, you write `x.pipe(f)`. Now the data flows left-to-right, and each transformation step is equally visible.

## How It Works in Rust

```rust
// Extension trait: adds .pipe() to every type
trait Pipe: Sized {
    fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B {
        f(self)    // simply calls f with self — all the magic is in the type
    }
    fn pipe_ref<B, F: FnOnce(&Self) -> B>(&self, f: F) -> B {
        f(self)    // non-consuming: self remains usable after
    }
    fn pipe_mut<B, F: FnOnce(&mut Self) -> B>(&mut self, f: F) -> B {
        f(self)
    }
}
impl<T> Pipe for T {}  // blanket impl: works on all types

fn double(x: i32) -> i32 { x * 2 }
fn add1(x: i32) -> i32 { x + 1 }
fn square(x: i32) -> i32 { x * x }

// Without pipe: right-to-left reading (backward)
let r1 = square(add1(double(3)));   // read: square of (add1 of (double of 3))

// With pipe: left-to-right reading (natural)
let r2 = 3i32.pipe(double).pipe(add1).pipe(square);  // 3 → 6 → 7 → 49

// Type changes work naturally
let result = 42i32
    .pipe(|x| x.to_string())           // i32 → String
    .pipe(|s| format!("value={}", s))  // String → String
    .pipe(|s| s.to_uppercase());       // String → String

// Closures capture context
let offset = 10;
let result2 = 5i32
    .pipe(|x| x + offset)    // captures offset
    .pipe(|x| x * 3)
    .pipe(|x| x.to_string());  // "45"

// Non-consuming pipe_ref — data stays owned
let data = vec![5, 3, 8, 1, 9, 2];
let max = data.pipe_ref(|v| v.iter().max().copied()); // data still owned
println!("{:?}", data); // still usable

// Complex pipeline with type changes at each step
let word_lengths: Vec<usize> = "hello world rust"
    .pipe(|s| s.split_whitespace().collect::<Vec<_>>())  // &str → Vec<&str>
    .pipe(|words| words.iter().map(|w| w.len()).collect()); // → Vec<usize>
```

## What This Unlocks

- **Readable linear pipelines** — multi-step transformations that read in execution order, not reverse order.
- **Free function integration** — chain functions from any crate without needing them to be methods on the type.
- **Type-changing pipelines** — `String → usize → bool` chains work naturally where method chaining would fail on type boundaries.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pipe operator | `x \|> f \|> g` — built-in syntax | `x.pipe(f).pipe(g)` — extension trait |
| Availability | Always available | Import the `Pipe` trait |
| Type change | `x \|> (fun x -> x + 1)` — natural | `x.pipe(\|x\| x + 1)` — same |
| Consume vs borrow | `\|>` always consumes | `.pipe()` consumes; `.pipe_ref()` borrows |
| Point-free style | `let f = g \|> h` | Composable with closures |
