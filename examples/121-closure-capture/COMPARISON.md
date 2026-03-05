# OCaml vs Rust: Closure Capture Modes

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml closures always capture by reference — the GC keeps the environment alive *)

(* Capture by value (immutable) *)
let add_one_to x = fun () -> x + 1

(* Mutable capture via ref cell *)
let sum_with_closure values =
  let total = ref 0 in
  let add x = total := !total + x in
  List.iter add values;
  !total

(* Closure outliving its creation scope — GC handles lifetime *)
let make_multiplier factor = fun x -> x * factor

(* Non-Copy capture — GC keeps string alive *)
let make_greeter name = fun () -> "Hello, " ^ name ^ "!"
```

### Rust (idiomatic — move closures for owned captures)
```rust
// Closure returned from a function must own its captures — use `move`
pub fn add_one_to(x: i32) -> impl Fn() -> i32 {
    move || x + 1   // `i32` is Copy, so this copies x into the closure
}

pub fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

pub fn make_greeter(name: String) -> impl Fn() -> String {
    move || format!("Hello, {}!", name)  // String is moved into the closure
}
```

### Rust (functional/recursive — mutable capture in local scope)
```rust
pub fn sum_with_closure(values: &[i32]) -> i32 {
    let mut total = 0;
    // Compiler infers &mut total because the closure mutates it.
    // Borrow ends when the closure is dropped.
    let mut add = |x: i32| total += x;
    for &v in values { add(v); }
    drop(add);
    total
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Returned closure | `val make_multiplier : int -> int -> int` | `fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32` |
| Mutable state | `let r = ref 0` | `let mut total = 0` |
| String ownership | GC reference (transparent) | `String` moved with `move \|\|` |
| Callable type | `'a -> 'b` (uniform) | `Fn`, `FnMut`, or `FnOnce` (distinct traits) |

## Key Insights

1. **GC vs ownership:** OCaml's GC keeps captured environments alive automatically. Rust requires the programmer to decide — borrow or move — and enforces the decision at compile time.

2. **Three capture modes:** Rust closures capture by `&T` (read-only), `&mut T` (mutation in scope), or `T` (ownership via `move`). The compiler chooses the least restrictive mode; `move` overrides that choice.

3. **`move` for `Copy` types is free:** For types like `i32` that implement `Copy`, `move` copies the value into the closure — the original variable is still accessible in the same scope. The effect is purely about letting the closure outlive its creation scope.

4. **`move` for non-`Copy` types transfers ownership:** For `String` or `Vec`, `move` actually moves the value into the closure. The original binding becomes inaccessible, matching what OCaml achieves transparently through the GC.

5. **`Fn` / `FnMut` / `FnOnce`:** Rust expresses capture semantics in the type system. A closure that only reads is `Fn`; one that mutates captures is `FnMut`; one that consumes captures is `FnOnce`. OCaml has no such distinction — all closures are uniform functions.

## When to Use Each Style

**Use default (borrow) capture when:** the closure lives entirely within the enclosing scope and only needs to read or mutate a local variable — the compiler handles it correctly and no annotation is needed.

**Use `move` when:** the closure must outlive its creation scope (returned from a function, passed to a thread, stored in a struct). For `Copy` types this is a cheap copy; for heap types it transfers ownership into the closure.
