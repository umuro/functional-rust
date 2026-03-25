# OCaml vs Rust: Applying a Function Twice

## Side-by-Side Code

### OCaml
```ocaml
let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double   (* partial application — type: int -> int *)
let fourth = twice square   (* partial application — type: int -> int *)

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);    (* 12 *)
  Printf.printf "fourth 2 = %d\n" (fourth 2)   (* 16 *)
```

### Rust (idiomatic — direct application)
```rust
pub fn twice<T, F>(f: F, x: T) -> T
where
    F: Fn(T) -> T,
{
    f(f(x))
}

twice(double, 3)   // 12
twice(square, 2)   // 16
```

### Rust (curried — partial application mirroring OCaml)
```rust
pub fn twice_curried<T, F>(f: F) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
{
    move |x| f(f(x))
}

let quad   = twice_curried(double);
let fourth = twice_curried(square);
quad(3)    // 12
fourth(2)  // 16
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| `twice` type | `('a -> 'a) -> 'a -> 'a` | `fn twice<T, F: Fn(T) -> T>(f: F, x: T) -> T` |
| Curried `twice` | same (automatic) | `fn twice_curried<T, F: Fn(T) -> T>(f: F) -> impl Fn(T) -> T` |
| Function argument | `'a -> 'a` | `F: Fn(T) -> T` |
| Partial result | `'a -> 'a` | `impl Fn(T) -> T` |
| `double` | `val double : int -> int` | `fn double(x: i64) -> i64` |

## Key Insights

1. **Automatic vs explicit currying:** OCaml curries every multi-argument function for free. `let quad = twice double` just works because OCaml sees `twice` as `('a->'a) -> ('a -> 'a)`. In Rust, partial application requires a separate function that explicitly returns a closure.

2. **`move` closure and ownership:** The `twice_curried` implementation uses `move |x| f(f(x))` to transfer ownership of `f` into the returned closure. Without `move`, the closure would hold a reference to `f` that outlives the enclosing stack frame — a compile error. OCaml's GC makes this invisible.

3. **Endomorphism constraint (`Fn(T) -> T`):** Both languages enforce that `f` maps a type to itself. OCaml does this via type unification at compile time (the type variable `'a` must unify in both uses). Rust expresses this explicitly with the bound `Fn(T) -> T` — the same `T` appears as input and output, which the compiler verifies.

4. **`impl Trait` in return position:** Returning `impl Fn(T) -> T` lets Rust avoid naming the concrete closure type (which is unnameable). This is the idiomatic way to return closures from functions in Rust and corresponds directly to the OCaml `'a -> 'a` return type of a partially applied `twice`.

5. **Generic vs monomorphic:** OCaml's `twice` is polymorphic over any type `'a`. Rust's version is also generic over `T`, but each monomorphized instance is a separate compiled function. When you call `twice(double, 3)`, Rust instantiates `T = i64` and `F = fn(i64) -> i64` — zero runtime cost from generics.

## When to Use Each Style

**Use direct `twice(f, x)`** when the function and value are both available at the call site and you just want the result inline.

**Use `twice_curried(f)`** when you want to create a reusable named function from partial application — the OCaml idiom of `let quad = twice double` — or when passing a "function applied twice" as a value to another higher-order function.
