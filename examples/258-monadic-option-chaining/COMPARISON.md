# OCaml vs Rust: Monadic Option Chaining

## Side-by-Side Code

### OCaml
```ocaml
let ( >>= ) opt f = match opt with
  | None -> None
  | Some x -> f x

let ( >>| ) opt f = match opt with
  | None -> None
  | Some x -> Some (f x)

let safe_div x y = if y = 0 then None else Some (x / y)
let safe_head = function [] -> None | h :: _ -> Some h

let compute lst =
  safe_head lst >>= fun x ->
  safe_div 100 x >>| fun r ->
  r * 2
```

### Rust (idiomatic)
```rust
pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

pub fn safe_head(list: &[i32]) -> Option<i32> {
    list.first().copied()
}

pub fn compute_idiomatic(lst: &[i32]) -> Option<i32> {
    safe_head(lst)
        .and_then(|x| safe_div(100, x))
        .map(|r| r * 2)
}
```

### Rust (explicit bind — shows the desugaring)
```rust
fn bind<T, U>(opt: Option<T>, f: impl FnOnce(T) -> Option<U>) -> Option<U> {
    match opt {
        None => None,
        Some(x) => f(x),
    }
}

pub fn compute_explicit(lst: &[i32]) -> Option<i32> {
    let divided = bind(safe_head(lst), |x| safe_div(100, x));
    divided.map(|r| r * 2)
}
```

### Rust (question-mark operator)
```rust
pub fn compute_question_mark(lst: &[i32]) -> Option<i32> {
    let x = safe_head(lst)?;
    let r = safe_div(100, x)?;
    Some(r * 2)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | `val (>>=) : 'a option -> ('a -> 'b option) -> 'b option` | `fn and_then<U>(self, f: impl FnOnce(T) -> Option<U>) -> Option<U>` |
| Map operator | `val (>>|) : 'a option -> ('a -> 'b) -> 'b option` | `fn map<U>(self, f: impl FnOnce(T) -> U) -> Option<U>` |
| Safe division | `val safe_div : int -> int -> int option` | `fn safe_div(x: i32, y: i32) -> Option<i32>` |
| Safe head | `val safe_head : 'a list -> 'a option` | `fn safe_head(list: &[i32]) -> Option<i32>` |

## Key Insights

1. **`>>=` is `and_then`:** OCaml's custom bind operator and Rust's `Option::and_then` are identical in semantics — both propagate `None` and apply `f` to the inner value when `Some`. Rust provides this in the standard library; OCaml developers historically defined it themselves.

2. **`>>|` is `Option::map`:** The functor-map operator in OCaml is exactly `Option::map` in Rust. Clippy will even reject a manual Rust implementation of `fmap` with `match`, telling you to use `.map()` instead — confirming this identity.

3. **The `?` operator desugars to bind:** Rust's `?` on an `Option` is syntactic sugar for "return `None` early if `None`, otherwise unwrap". This is monadic short-circuit with imperative-style syntax, unique to Rust and without a direct OCaml equivalent.

4. **Value ownership vs. reference semantics:** Rust's `and_then` and `map` consume the `Option` by value, and the closures receive owned `T`. OCaml also passes values, but without explicit ownership tracking. In Rust, using `.copied()` on `Option<&T>` to produce `Option<T>` is the idiomatic way to decouple borrowing from the chain.

5. **No user-defined infix operators in stable Rust:** OCaml makes it natural to define `>>=` as an infix operator. Rust does not support custom infix operators in stable code, so the chaining reads as method calls. This is a deliberate Rust design decision for readability and tooling.

## When to Use Each Style

**Use `and_then` + `map` when:** building a pipeline with multiple fallible steps that reads cleanly left-to-right — the method chain style makes the data flow obvious and composes well with iterator chains.

**Use `?` when:** writing code that resembles sequential imperative steps, or when intermediate values need to be named and reused. The `?` style is easier for developers unfamiliar with monadic thinking to read and debug.

**Use explicit `bind` (match) when:** teaching or documenting what monadic chaining means under the hood, or when porting OCaml code directly for comparison purposes.
