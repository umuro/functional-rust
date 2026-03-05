# OCaml vs Rust: Fn, FnMut, FnOnce

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has a single closure type — captures are implicit references *)
let make_greeter prefix =
  fun name -> prefix ^ ", " ^ name ^ "!"

let make_counter () =
  let count = ref 0 in
  fun () -> incr count; !count

(* OCaml cannot express "callable exactly once" in its type system *)
let one_shot msg = fun () -> msg
```

### Rust (idiomatic — Fn, read-only capture)
```rust
// Fn: closure captures `prefix` by move but only reads it
pub fn make_greeter(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{prefix}, {name}!")
}

// Can be called any number of times — no mutation, no consumption
let greet = make_greeter("Hello".into());
assert_eq!(greet("Alice"), "Hello, Alice!");
assert_eq!(greet("Bob"),   "Hello, Bob!");
```

### Rust (FnMut — mutable capture)
```rust
// FnMut: closure mutates `count` each time it is called
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0u32;
    move || { count += 1; count }
}

let mut next = make_counter();
assert_eq!(next(), 1);
assert_eq!(next(), 2);
```

### Rust (FnOnce — consuming capture)
```rust
// FnOnce: closure moves `message` out on the single call
pub fn make_one_shot(message: String) -> impl FnOnce() -> String {
    move || message   // `message` is consumed here — cannot call again
}

let shot = make_one_shot("boom".into());
assert_eq!(shot(), "boom");
// shot() again → compile error: value used after move
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Read-only closure | `'a -> 'b` (implicit) | `impl Fn(A) -> B` |
| Mutable-state closure | `unit -> 'a` with `ref` inside | `impl FnMut() -> A` |
| One-shot closure | not expressible | `impl FnOnce() -> A` |
| Higher-order read | `('a -> 'b) -> 'a -> 'b` | `fn apply<F: Fn(A) -> B>(f: F, x: A) -> B` |
| Higher-order mutable | `(unit -> 'a) -> 'a list` | `fn call_n<F: FnMut() -> A>(f: F, n: usize) -> Vec<A>` |

## Key Insights

1. **Trait hierarchy mirrors capability**: `Fn ⊆ FnMut ⊆ FnOnce`. Every `Fn` is automatically `FnMut` and `FnOnce`; the compiler picks the tightest trait automatically.

2. **OCaml has no equivalent of `FnOnce`**: In OCaml every closure can be called any number of times. Rust makes single-use semantics explicit and compiler-enforced, which matters for consuming resources (file handles, channels, owned data).

3. **Ownership drives the inference**: The compiler inspects what the closure *does* with its captures. Mere reads → `Fn`. Mutation via `&mut` → at least `FnMut`. Moving a value out of the body → only `FnOnce`. No annotation required — the traits are inferred automatically.

4. **Higher-order function contracts become precise**: Writing `fn apply(f: impl Fn())` tells callers the function will call `f` repeatedly and safely. Writing `fn run(f: impl FnOnce())` guarantees `f` is called at most once — an API-level promise enforced by the type system.

5. **`move` keyword transfers ownership into the closure**: OCaml closures always close over references. Rust's `move` keyword forces all captured variables to be *owned* by the closure, which enables returning closures from functions and sending them across threads (`move || …` is `Send` if the captured values are `Send`).

## When to Use Each Style

**Use `Fn`** when you need to call the closure many times without state change — e.g., `Iterator::map`, callbacks, pure transformations.

**Use `FnMut`** when the closure accumulates state between calls — e.g., counters, folds with side effects, event handlers.

**Use `FnOnce`** when the closure must consume its captures — e.g., spawning a thread with owned data, one-time initialization, RAII teardown callbacks.

**Use `move` closures** whenever the closure outlives the current stack frame — returning closures from functions, spawning tasks, or storing closures in data structures.
