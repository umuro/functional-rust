# OCaml vs Rust: Pipeline Operator

The pipeline operator is a simple but powerful tool for making function composition readable. In OCaml, it's a lightweight operator that enables left-to-right function application. In Rust, we achieve the same result through different idiomatic patterns.

## Side-by-Side Code

### OCaml
```ocaml
(* The pipeline operator is just a higher-order function *)
let ( |> ) x f = f x

let double x = 2 * x
let add1 x = x + 1

(* Read: start with 5, double it, add 1 *)
let result = 5 |> double |> add1   (* 11 *)

(* Chaining string operations *)
let shout s = String.uppercase_ascii s
let exclaim s = s ^ "!"

let greeting = "hello" |> shout |> exclaim   (* "HELLO!" *)
```

### Rust (idiomatic)
```rust
pub fn double(x: i32) -> i32 {
    2 * x
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Idiomatic Rust: direct function composition (nested calls)
pub fn compute_result_idiomatic() -> i32 {
    add_one(double(5))  // 11
}

pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn add_exclaim(s: &str) -> String {
    format!("{}!", s)
}

pub fn compute_greeting_idiomatic() -> String {
    add_exclaim(&shout("hello"))  // "HELLO!"
}
```

### Rust (functional/pipe)
```rust
// Functional Rust: explicit pipe function (mimics OCaml's |>)
pub fn pipe<T, U>(value: T, f: impl FnOnce(T) -> U) -> U {
    f(value)
}

pub fn compute_result_with_pipe() -> i32 {
    pipe(pipe(5, double), add_one)  // 11
}

pub fn compute_greeting_with_pipe() -> String {
    let shouted = pipe("hello", shout);
    pipe(&shouted, |s| add_exclaim(s))  // "HELLO!"
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Pipeline operator | `val (|>) : 'a -> ('a -> 'b) -> 'b` | `fn pipe<T, U>(value: T, f: impl FnOnce(T) -> U) -> U` |
| Integer transformation | `int -> int` | `fn(i32) -> i32` |
| String transformation | `string -> string` | `fn(&str) -> String` |
| Composed function | Implicit through `\|>` | Explicit via `compose` trait or nested calls |

## Key Insights

1. **Function Application vs Syntax:** OCaml's `|>` is syntactic sugar for simple function application. Rust achieves the same through nested function calls or an explicit `pipe` function. This shows that operators are just functions in different syntax.

2. **Ownership and Borrowing:** The Rust `pipe` function uses `FnOnce` to accept a function that consumes its input. This is Rust's way of enforcing that each transformation takes ownership of the value. OCaml handles this implicitly without explicit ownership semantics.

3. **Method Chaining vs Operators:** In idiomatic Rust, the preferred approach for many pipelines is method chaining (`.map()`, `.filter()`, etc.), which reads left-to-right like `|>`. OCaml doesn't have methods on built-in types, so `|>` is the idiomatic solution there.

4. **Closures for Conversion:** When piping a `String` to a function expecting `&str`, Rust requires an explicit closure (`|s| add_exclaim(s)`) to handle type conversion. OCaml's implicit coercion would handle this automatically, showing a difference in type system strictness.

5. **Generics and Trait Bounds:** Rust's `pipe` function is generic over both input and output types with `impl FnOnce(T) -> U`, enabling type-safe composition without runtime overhead. OCaml's polymorphic `'a -> ('a -> 'b) -> 'b` achieves the same with implicit polymorphism.

## When to Use Each Style

**Use idiomatic Rust nested calls when:**
- Composing two or three functions (e.g., `f(g(h(x)))`).
- Working with iterator chains (`.map()`, `.filter()`, etc.), which are left-to-right and naturally readable.
- The composition fits naturally on one line or is part of a larger expression.

**Use the pipe function in Rust when:**
- You want to explicitly show function application order similar to OCaml's `|>`.
- Demonstrating functional programming concepts or translating OCaml code directly.
- Chaining many custom functions where the pipe notation improves readability over nested calls.

**Use function composition in Rust when:**
- Creating reusable composed functions (e.g., a function that doubles then adds one).
- Higher-order programming where the composition itself becomes a parameter.
- You want to name intermediate transformations for clarity.

## Syntactic Observations

OCaml's `5 |> double |> add_one` reads perfectly left-to-right. Rust's equivalent nested form `add_one(double(5))` reads right-to-left (inside-out), which is why method chaining and the pipe function are often preferred for readability. This is a key difference in expressiveness: OCaml's operator syntax makes the data flow obvious, while Rust requires more deliberate structuring to achieve the same clarity.
