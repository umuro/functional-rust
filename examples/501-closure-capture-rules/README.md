📖 **[View on hightechmind.io →](https://hightechmind.io/rust/501-closure-capture-rules)**

---

# Closure Capture Rules
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


Rust closures capture their environment by the least restrictive mechanism that the closure body requires: immutable borrow, mutable borrow, or move (ownership transfer), chosen automatically by the compiler unless `move` is specified explicitly.

## Problem Statement

In languages with garbage collection, closures capture variables freely — the GC prevents dangling references. Rust has no GC, so the compiler must prove that closure captures are safe. It enforces that: (1) if the closure only reads, it borrows immutably; (2) if it mutates, it borrows mutably (preventing other borrows); (3) if it needs to outlive the captured variable (e.g., return from a function or sent to a thread), it must take ownership via `move`. These rules are the closure-specific application of the borrow checker.

## Learning Outcomes

- Understand that closures capture by immutable borrow, mutable borrow, or move automatically
- Use `move` to force ownership transfer into the closure
- Know that `move` closures over `Copy` types copy the value (not truly "move")
- Return a closure from a function using `impl Fn() -> T` with `move`
- Understand why mutable borrow closures must be declared `mut` at the call site

## Rust Application

Immutable borrow — `s` remains accessible after the closure:

```rust
let s = String::from("hello");
let f = || println!("{}", s);  // borrows s
f(); f();
println!("{}", s);  // s still valid
```

Mutable borrow — `v` is exclusively borrowed while `f` is alive:

```rust
let mut v = vec![1,2,3];
let mut f = || v.push(4);  // mutably borrows v
f();
drop(f);
println!("{:?}", v);  // v accessible again
```

`move` closure — takes ownership, enabling return across function boundaries:

```rust
pub fn return_closure_capturing_param(x: i32) -> impl Fn() -> i32 {
    move || x * 2
}
```

## OCaml Approach

OCaml closures always capture by reference to the GC-managed heap — ownership is irrelevant:

```ocaml
let s = "hello"
let f () = Printf.printf "%s\n" s  (* captures reference *)
let () = f (); f (); Printf.printf "%s\n" s  (* all valid *)

(* Mutable state — use ref *)
let v = ref [1;2;3]
let push x () = v := x :: !v
let () = push 4 (); Printf.printf "%d\n" (List.length !v)
```

## Key Differences

1. **Automatic capture mode**: Rust infers the minimum capture mode; OCaml always captures by reference (GC handles lifetimes).
2. **`mut` requirement**: Rust requires `mut f` at the declaration site for mutable-capturing closures; OCaml has no such requirement.
3. **`move` keyword**: Rust's `move` forces ownership transfer for cross-function or cross-thread use; OCaml has no equivalent because GC manages all lifetimes.
4. **`Copy` types with `move`**: Rust copies `Copy` types into `move` closures rather than moving; OCaml always copies primitive values into closures by value.

## Exercises

1. **Thread closure**: Write a function that takes a `Vec<String>` and returns a `thread::JoinHandle<usize>` that sums the string lengths — requiring `move` to transfer ownership into the thread closure.
2. **Borrow conflict**: Write code that demonstrates the compiler error when trying to use a mutable-borrow closure alongside another borrow of the same variable, then fix it with `drop(f)`.
3. **Closure upgrade**: Write a closure that starts as `Fn` (immutable borrow), then modify it to `FnMut` (add mutation), then `FnOnce` (consume the captured value) — observe how each change affects call-site requirements.
