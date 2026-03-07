# OCaml vs Rust: Applying a Function Twice

## Side-by-Side Code

### OCaml

```ocaml
let twice f x = f (f x)

let double x = 2 * x
let square x = x * x

let quad   = twice double   (* applies double twice *)
let fourth = twice square   (* applies square twice *)

let () =
  Printf.printf "quad 3   = %d\n" (quad 3);    (* 12 *)
  Printf.printf "fourth 2 = %d\n" (fourth 2)   (* 16 *)
```

### Rust (idiomatic — using closures and trait bounds)

```rust
pub fn twice<T>(f: impl Fn(T) -> T, x: T) -> T {
    f(f(x))
}

pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let quad = |x| twice(double, x);
    let fourth = |x| twice(square, x);

    println!("quad 3   = {}", quad(3));     // 12
    println!("fourth 2 = {}", fourth(2));   // 16
}
```

### Rust (functional — composition-based)

```rust
pub fn twice_compose<T: 'static>(f: impl Fn(T) -> T + 'static) -> impl Fn(T) -> T {
    move |x| f(f(x))
}

fn main() {
    let quad_composed = twice_compose(double);
    let fourth_composed = twice_compose(square);

    println!("quad_composed 3   = {}", quad_composed(3));     // 12
    println!("fourth_composed 2 = {}", fourth_composed(2));   // 16
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function type | `('a -> 'a) -> 'a -> 'a` | `fn twice<T>(f: impl Fn(T) -> T, x: T) -> T` |
| Partial application | `let quad = twice double` (automatic) | `\|x\| twice(double, x)` (explicit closure) |
| Closure type | `'a -> 'a` (implicit) | `impl Fn(T) -> T` (trait bound) or `fn(T) -> T` (function pointer) |
| Returned closure | Implicit via type inference | Explicit via `impl Fn(T) -> T` or named `Box<dyn Fn>` |

## Key Insights

1. **Currying is explicit in Rust:** OCaml's automatic currying treats `twice double` as a partial application. In Rust, we must explicitly create a closure `|x| twice(double, x)` to achieve the same effect. This reflects Rust's principle of making ownership and lifetimes explicit.

2. **Function types are more flexible in Rust:** The trait bound `impl Fn(T) -> T` accepts any callable (function pointer, closure with captured variables, method reference). OCaml lumps everything into function types but forces you to think about captures differently.

3. **Lifetime and Move Semantics Matter:** The `twice_compose` variant requires `T: 'static` when returning a closure because the closure needs to own the captured function. In OCaml, this complexity is hidden by the garbage collector.

4. **Three approaches for different needs:**
   - `twice` with `impl Fn` — most flexible, works with any callable
   - `twice_fn` with function pointers — explicit, predictable, no allocations
   - `twice_compose` returning a closure — enables function composition chains

5. **Ownership vs. Garbage Collection:** OCaml lets you casually construct and return functions without thinking about lifetimes. Rust forces you to decide whether the closure owns or borrows the function, leading to safer code but more boilerplate.

## When to Use Each Style

**Use idiomatic Rust when:** You're building APIs that need to accept various callables (functions, closures, method references). The `impl Fn` bound is the Swiss Army knife of function passing.

**Use function pointers when:** You need maximum performance and predictability (no allocations, no closures on the heap). Function pointers are zero-cost abstractions.

**Use closure composition (`twice_compose`) when:** You're building functional programming libraries where composing functions is a core operation. This style mirrors OCaml idioms most closely.

## Performance Characteristics

- **OCaml `twice`:** Zero-cost abstraction; inlining works naturally.
- **Rust `twice` with `impl Fn`:** Zero-cost when the function is a concrete type (monomorphized), but the generic specialization happens at compile time.
- **Rust `twice_fn`:** Guaranteed zero-cost; function pointers are thin pointers with no captures.
- **Rust `twice_compose` returning closure:** May allocate if the closure is boxed or captured across function boundaries. If stack-allocated, still zero-cost.

The bottom line: **Rust's generics are as efficient as OCaml's polymorphism**, but require explicit attention to allocation and ownership.
