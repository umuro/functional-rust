# 128: Type-Level Booleans

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Encode `true`/`false` as types instead of values so the compiler can enforce "both conditions must hold" without any runtime checks.

## The Problem This Solves

Suppose you have a `Config` object that must be both validated *and* have logging enabled before you're allowed to call `execute()`. One approach: add two `bool` fields and check them at runtime with `if !self.validated { panic!() }`. But now you're writing runtime guards for things you *always* know at compile time — the call site either did both setup steps or it didn't.

The runtime approach also lets you build a `Config` and call `execute()` in the wrong order, then only discover the mistake when tests run (or worse, in production). The error message is a panic string, not a compiler error pointing you to the missing setup call.

Type-level booleans flip this: validation state becomes part of the *type*. `Config<False, False>` doesn't have an `execute()` method — it literally doesn't exist on that type. You can't call what isn't there. The compiler tells you exactly what's missing before you ever run the program.

## The Intuition

Instead of storing `true` or `false` in a field, you create two empty structs: `struct True;` and `struct False;`. These carry no data — they're just labels. You then make your generic struct `Config<Validated, Logged>` where `Validated` and `Logged` are type parameters that will be filled with either `True` or `False`.

Now you write different `impl` blocks for different combinations. `execute()` only exists in `impl Config<True, True>`. The Rust compiler won't let you call a method that doesn't exist for your particular type combination. The type signature *is* the precondition.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Empty structs — they're just type-level labels, no data stored
struct True;
struct False;

// Optional: a trait to read the bool value at runtime
trait Bool { const VALUE: bool; }
impl Bool for True  { const VALUE: bool = true; }
impl Bool for False { const VALUE: bool = false; }

// Type-level AND: the Output type IS the result
trait And<B: Bool> { type Output: Bool; }
impl<B: Bool> And<B> for True  { type Output = B; }     // true && B = B
impl<B: Bool> And<B> for False { type Output = False; } // false && B = false

// Config holds two phantom type parameters — no extra memory used
struct Config<Validated: Bool, Logged: Bool> {
    data: String,
    _phantom: PhantomData<(Validated, Logged)>,  // tells the compiler these types matter
}

// Start: neither validated nor logged
impl Config<False, False> {
    fn new(data: String) -> Self {
        Config { data, _phantom: PhantomData }
    }
}

// validate() is only available when Logged can be anything (not yet validated)
impl<L: Bool> Config<False, L> {
    fn validate(self) -> Config<True, L> {  // transitions Validated from False → True
        Config { data: self.data, _phantom: PhantomData }
    }
}

// enable_logging() is only available when not yet logged
impl<V: Bool> Config<V, False> {
    fn enable_logging(self) -> Config<V, True> {  // transitions Logged from False → True
        Config { data: self.data, _phantom: PhantomData }
    }
}

// execute() ONLY exists when both are True — no other impl block has it
impl Config<True, True> {
    fn execute(&self) -> String {
        format!("Running: {}", self.data)
    }
}
```

Usage:
```rust
let config = Config::new("task".into())
    .validate()
    .enable_logging()
    .execute();           // compiles ✓

// Config::new("task".into()).execute();  // compile error: no method `execute`
// Config::new("task".into()).validate().execute(); // compile error: same
```

## What This Unlocks

- **Multi-step setup protocols** — database connections that must authenticate before querying; HTTP requests that must have both a URL and method before building.
- **Permission systems** — `Request<Authenticated, Authorized>` where only `impl Request<True, True>` can access protected resources.
- **Compile-time feature flags** — library types that expose different APIs depending on which builder methods were called, with zero runtime overhead from the flags themselves.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bool encoding | Module type `BOOL` with `type t` and `val value : bool` | Empty structs `True`/`False` + trait `Bool` |
| Type-level AND | Functor `And(A)(B)` computing a new module | Trait `And<B>` with `type Output` — resolved by compiler |
| Method gating | GADT or phantom type annotation, no value consumed | Different `impl` blocks per type combination; `self` consumption prevents reuse |
| Runtime access | `module.value` to get the bool | `<True as Bool>::VALUE` — reads the const from the trait |
