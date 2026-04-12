# OCaml vs Rust: Applying a Function Twice

## Side-by-Side Code

### OCaml
```ocaml
let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double   (* partial application *)
let fourth = twice square

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);    (* 12 *)
  Printf.printf "fourth 2 = %d\n" (fourth 2)   (* 16 *)
```

### Rust (idiomatic — function + value together)
```rust
pub fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T {
    f(f(x))
}

fn double(x: i32) -> i32 { 2 * x }
fn square(x: i32) -> i32 { x * x }

println!("{}", twice(double, 3));  // 12
println!("{}", twice(square, 2));  // 16
```

### Rust (higher-order — mirrors OCaml partial application)
```rust
pub fn twice_compose<T, F: Fn(T) -> T>(f: F) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

let quad   = twice_compose(double);   // mirrors: let quad = twice double
let fourth = twice_compose(square);   // mirrors: let fourth = twice square

println!("{}", quad(3));    // 12
println!("{}", fourth(2));  // 16
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| `twice` | `val twice : ('a -> 'a) -> 'a -> 'a` | `fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T` |
| Higher-order variant | same (curried) | `fn twice_compose<T, F: Fn(T) -> T>(f: F) -> impl Fn(T) -> T` |
| `quad` | `val quad : int -> int` | `impl Fn(i32) -> i32` |
| Function argument | `('a -> 'a)` | `F: Fn(T) -> T` |

## Key Insights

1. **Automatic currying vs explicit closures:** In OCaml, `twice double` is free — currying is built in. In Rust, you must explicitly return a closure (`move |x| f(f(x))`) to get the same effect.

2. **`Fn` vs `fn`:** Rust's `Fn(T) -> T` trait accepts closures that capture environment; `fn(T) -> T` is a bare function pointer (Copy, no captured state). OCaml has no such distinction — all functions are closures.

3. **`move` semantics:** The `move` keyword in `move |x| f(f(x))` transfers ownership of `f` into the closure. OCaml's garbage collector handles this automatically without explicit syntax.

4. **Polymorphism mechanism:** OCaml uses parametric polymorphism (`'a`) resolved at compile time via HM inference. Rust uses monomorphization: the compiler generates a concrete copy of `twice` for each type `T` and `F`.

5. **Calling `f` twice:** In both languages, `f(f(x))` works because the function is not consumed. OCaml relies on garbage collection; Rust uses `Fn` (shared borrow), allowing multiple calls on the same closure.

## When to Use Each Style

**Use `twice` (direct style) when:** you have the function and value available together and want the most readable, idiomatic Rust call site.

**Use `twice_compose` (higher-order style) when:** you want to create a reusable derived function (like `quad` or `fourth`) that will be called multiple times, or when you need to pass the composed function as an argument.
