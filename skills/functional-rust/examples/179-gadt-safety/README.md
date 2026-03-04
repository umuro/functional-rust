# 179: GADT Preventing Runtime Errors

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Encode emptiness/non-emptiness in a list's type so that `head` on an empty list is a compile-time error instead of a runtime panic.

## The Problem This Solves

Every Rust developer has written `vec.first().unwrap()` with a mental note "this can't be empty here." Sometimes you're right. Sometimes a refactor later breaks the invariant, and you get a panic in production at 2 AM. The type system never helped you — `Vec<T>` has no memory of whether it was just created empty or has been filled.

The real problem is that the *guarantee* "this list is non-empty" lives in your head, in comments, maybe in a test — but not in the type. When code changes, the guarantee drifts silently. Phantom types let you bake the guarantee into the type itself: `SafeList<T, NonEmpty>` vs `SafeList<T, Empty>`. The `head` method only exists on the `NonEmpty` variant — the compiler simply won't let you call it on an empty list.

This is a specific application of the type-state pattern (see example 180 for the general form). Here the "state" being tracked is a structural property of the data itself: does it have at least one element?

## The Intuition

A coffee machine has two states: "has coffee" and "empty." The "dispense" button only works in the "has coffee" state — it's physically locked out otherwise. You don't check at runtime whether there's coffee; the machine's state tells you. If you want to dispense, you must first be holding a `Machine<HasCoffee>`, not a `Machine<Empty>`.

A `SafeList<T, NonEmpty>` works the same way. `head` is only defined on `SafeList<T, NonEmpty>`. To get a `NonEmpty` list, you must either start with a `cons` operation (which is always non-empty) or branch based on what you have. The emptiness state is tracked at every step.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Type-level state tags — no data, just markers
struct Empty;
struct NonEmpty;

// The list itself — S is the phantom state
struct SafeList<T, S> {
    data: Vec<T>,
    _state: PhantomData<S>,
}

// Empty constructor — only produces Empty
impl<T> SafeList<T, Empty> {
    fn new() -> Self {
        SafeList { data: Vec::new(), _state: PhantomData }
    }
}

// Pushing an element always produces NonEmpty, regardless of prior state
impl<T, S> SafeList<T, S> {
    fn push(mut self, val: T) -> SafeList<T, NonEmpty> {
        self.data.push(val);
        SafeList { data: self.data, _state: PhantomData }
    }
}

// head only exists for NonEmpty lists — Empty lists literally have no head method
impl<T> SafeList<T, NonEmpty> {
    fn head(&self) -> &T {
        &self.data[0]   // safe — we know it's non-empty by type
    }
}

// Correct usage:
let empty: SafeList<i32, Empty> = SafeList::new();
let nonempty: SafeList<i32, NonEmpty> = empty.push(42);
println!("{}", nonempty.head()); // ✓ compiles

// This fails to compile:
// let empty: SafeList<i32, Empty> = SafeList::new();
// empty.head(); // error: method not found in `SafeList<i32, Empty>`
```

The `head` method doesn't exist on `SafeList<T, Empty>` — not "it exists but checks at runtime," but literally doesn't exist in that `impl` block. The compiler catches the mistake before the program runs.

## What This Unlocks

- **Queue/deque operations** — `dequeue` only available on a `Queue<NonEmpty>`, eliminating a whole class of defensive checks.
- **Parsing combinators** — a match result that carries whether it consumed any input, used to detect infinite loops in grammars at compile time.
- **Resource protocols** — file handles, database cursors, anything that must be populated before reading; the "has data" state is tracked in the type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mechanism | GADT constructors: `SNil : ('a, empty) safe_list` vs `SCons : 'a * ... -> ('a, nonempty) safe_list` | `PhantomData<Empty>` vs `PhantomData<NonEmpty>` with separate `impl` blocks |
| Pattern matching | Compiler knows `SCons` branch is `nonempty`; `SNil` branch in `safe_head` is exhaustively impossible | No match-level type refinement; safety enforced by only having methods on one impl block |
| State transition on push | Implicit via constructor choice | Explicit: `push` consumes `SafeList<T, S>` and returns `SafeList<T, NonEmpty>` |
| Ergonomics | Natural in GADT pattern matching | Method-based API; transitions clear from signatures |
| Zero-cost | Yes | Yes — PhantomData has no runtime representation |
