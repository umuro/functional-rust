# OCaml vs Rust: Function Composition

## Side-by-Side Code

### OCaml
```ocaml
let compose f g x = f (g x)

let double x = 2 * x
let square x = x * x

let square_then_double = compose double square

let () =
  Printf.printf "square_then_double 3 = %d\n" (square_then_double 3)  (* 18 *)
  Printf.printf "square_then_double 4 = %d\n" (square_then_double 4)  (* 32 *)
```

### Rust (idiomatic)
```rust
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

pub fn double(x: i32) -> i32 { 2 * x }
pub fn square(x: i32) -> i32 { x * x }

fn main() {
    let square_then_double = compose(double, square);
    println!("square_then_double 3 = {}", square_then_double(3));   // 18
    println!("square_then_double 4 = {}", square_then_double(4));   // 32
}
```

### Rust (with function pointers)
```rust
pub fn compose_fn<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

// Usage:
let square_then_double = compose_fn(double, square);
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Compose type | `('a -> 'b) -> ('c -> 'a) -> ('c -> 'b)` | `F: Fn(B) -> C, G: Fn(A) -> B` |
| Return type | Implicit closure | `impl Fn(A) -> C` |
| Function type | `int -> int` | `fn(i32) -> i32` |
| Closure capture | Automatic (lexical) | Explicit with `move` |

## Key Insights

1. **Generic Type Parameters in Rust:** While OCaml can express composition with a single polymorphic signature, Rust needs three type parameters (`A`, `B`, `C`) to represent the domain, intermediate value, and codomain. This explicitness is a feature—the types are clear to the compiler and all callers.

2. **Higher-Rank Trait Bounds:** Rust's `F: Fn(B) -> C` syntax is equivalent to OCaml's `('a -> 'b)` annotation. The `Fn` trait allows Rust to accept any callable (closure, function pointer, or function item) as long as it has the right signature.

3. **Zero-Cost Abstraction:** The `impl Fn` return type means Rust generates monomorphized code for each specific composition. There's no vtable or dynamic dispatch—the composition is inlined at compile time.

4. **Closure Capture Semantics:** In Rust, the `move` keyword explicitly captures `f` and `g` by ownership. In OCaml, this happens automatically. Rust's explicitness prevents accidental lifetime issues.

5. **Concrete vs. Abstract Return Types:** Rust offers two trade-offs:
   - `impl Fn` (what we use here): maximally flexible, accepts any callable, zero overhead
   - `fn(A) -> B` (function pointers): more restrictive, requires function items, still zero overhead

## When to Use Each Style

**Use idiomatic Rust (`impl Fn + closures`) when:** You need maximum flexibility—accepting any callable (closures, function items, methods) and returning an efficient, inlined composition.

**Use function pointers (`fn(A) -> B`) when:** You specifically need to work with function items (not closures) and want a concrete, simple signature. This is less flexible but clearer in some contexts.

**Use OCaml when:** You want the ultimate simplicity and don't need the explicit type control that Rust provides. OCaml's implicit polymorphism is elegant for mathematical function composition.

## Common Pitfalls (Rust)

| Pitfall | Example | Solution |
|---------|---------|----------|
| Forgetting `move` | `\|x\| f(g(x))` might not compile | Add `move` to capture `f` and `g` by value |
| Wrong trait bound | Using `Fn` instead of `FnOnce` | Use `Fn` for functions called multiple times |
| Concrete function types | `let c: fn(i32)->i32 = compose(double, square)` | Use `impl Fn` return type instead |

## Performance

Both implementations compile to identical machine code:
- **OCaml:** The closure is stack-allocated; the JIT or bytecode interpreter executes it efficiently.
- **Rust:** With `impl Fn`, the composition is monomorphized and inlined—you get the same performance as hand-written `move |x| f(g(x))`.
