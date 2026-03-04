# 542: Higher-Ranked Trait Bounds (for<'a>)

**Difficulty:** 5  **Level:** Advanced

`for<'a>` means "for any possible lifetime 'a." It's universal quantification over lifetimes. Use it when you need a callback that works with references of any duration — not just one specific lifetime.

## The Problem This Solves

Without `for<'a>`, a callback's lifetime is fixed at the call site. This causes problems when you want to store a closure that will process references with different lifetimes at different times:

```rust
// Fixed 'a: the closure only works for one specific lifetime
fn apply_fixed<'a, F>(f: F, s: &'a str) -> &'a str
where
    F: Fn(&'a str) -> &'a str,  // F is tied to THIS specific 'a
{ f(s) }
```

Try to store that closure in a struct and use it with different strings over time — you can't. The `'a` was fixed when you wrote the function. You need a closure that promises to work for *any* `'a`:

```rust
// HRTB: F works for any lifetime, not just one specific one
struct Processor {
    transform: Box<dyn for<'a> Fn(&'a str) -> &'a str>,
}
// Now transform can be called with a string of any duration
```

## The Intuition

A regular generic `<'a, F: Fn(&'a str) -> &'a str>` says: "pick a specific `'a` at the call site, and F must work for that `'a`."

`for<'a> Fn(&'a str) -> &'a str` says: "F must work for *every* possible `'a`." It's a stronger requirement on F — F can't cheat by only working for long-lived references. It has to handle references that live for one nanosecond or one year.

This is why closures like `|s: &str| s.trim()` satisfy `for<'a>` bounds — they're genuinely lifetime-agnostic. Closures that *capture* a reference of a specific lifetime do not — they're tied to that capture's scope.

## How It Works in Rust

**The syntax:**

```rust
// Read as: "F implements Fn for any lifetime 'a"
fn apply_hrtb<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,  // works for ANY 'a
{
    f(s).to_string()
}

// Equivalent shorthand (compiler infers HRTB for simple cases):
fn apply_hrtb<F: Fn(&str) -> &str>(f: F, s: &str) -> String {
    f(s).to_string()
}
```

**Stored in a struct:**

```rust
struct Processor {
    transform: Box<dyn for<'a> Fn(&'a str) -> &'a str>,
}

impl Processor {
    fn new(f: impl for<'a> Fn(&'a str) -> &'a str + 'static) -> Self {
        Processor { transform: Box::new(f) }
    }

    fn process<'a>(&self, input: &'a str) -> &'a str {
        (self.transform)(input)  // works regardless of input's lifetime
    }
}

// A closure that's lifetime-agnostic works:
let p = Processor::new(|s: &str| s.trim());

// Call with any lifetime — all work:
let owned = String::from("  hello  ");
println!("{}", p.process(&owned));     // 'a = owned's lifetime
println!("{}", p.process("literal"));  // 'a = 'static
```

**Used in generic iterators:**

```rust
fn map_all<T, F>(items: &[T], f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a T) -> String,
{
    items.iter().map(|x| f(x)).collect()
}

// Works because for<'a> Fn(&'a T) means the closure handles elements of any lifetime
let nums = vec![1, 2, 3];
let s = map_all(&nums, |n| format!("{}", n));
```

**When the compiler infers HRTB automatically:**

```rust
// You usually don't need to write for<'a> explicitly
// The compiler infers it from usage:
fn apply<F: Fn(&str) -> usize>(f: F, s: &str) -> usize { f(s) }
// This is equivalent to: F: for<'a> Fn(&'a str) -> usize
```

## What This Unlocks

- **Generic callbacks stored in structs** — a `Processor`, `Visitor`, or `Transformer` that holds a closure can call it with inputs of any duration without being parameterized over a specific lifetime.
- **Trait objects with borrowed arguments** — `Box<dyn for<'a> Fn(&'a T) -> &'a U>` lets you erase the concrete callback type while keeping it usable with any lifetime.
- **Zero-copy iteration adapters** — a combinator library that maps `&T → &U` over a slice needs HRTB to work with any slice's lifetime.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Higher-order functions | Parametric polymorphism handles this naturally | Lifetime polymorphism requires explicit `for<'a>` when closure arguments involve references |
| Rank-2 polymorphism | `let f : (type a. a -> a) = fun x -> x` | `for<'a> Fn(&'a T) -> &'a U` — rank-2 quantification over lifetimes |
| Storing generic callbacks | Functors and modules handle this | HRTB enables erasing a lifetime-polymorphic closure into a trait object |
| Type inference | Full HM inference handles rank-1 types | HRTB needs explicit `for<'a>` in complex cases; inferred for simple `Fn(&T)` |
| Functor pattern | Type class / module-based | HRTB `for<'a> Fn(&'a T) -> &'a U` is the Rust equivalent for reference-in, reference-out |
