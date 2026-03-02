# LinkedIn Post: Function Composition

🦀 **Functional Rust #006: Function Composition**

Compose small functions into pipelines. OCaml uses operators. Rust uses iterators.

**OCaml:**
```ocaml
let (>>) f g x = g (f x)
let pipeline = square >> double >> add3
pipeline 4  (* 35 *)
```

**Rust (manual):**
```rust
fn compose<A,B,C,F,G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(B) -> C, G: Fn(A) -> B
{ move |x| f(g(x)) }
```

**Rust (idiomatic):**
```rust
vec![1, 2, 3]
    .into_iter()
    .map(square)
    .map(double)
    .map(add3)
    .collect()
```

**Why iterators win:**

⚡ **Zero-cost** - Optimized away at compile time
🔗 **Chainable** - Readable left-to-right flow
🦥 **Lazy** - Only compute when needed
✅ **Type-safe** - Compiler infers everything

Functional composition is beautiful. Rust's iterators give you the same power with better performance.

#Rust #FunctionalProgramming #Iterators #RustLang #ZeroCost
