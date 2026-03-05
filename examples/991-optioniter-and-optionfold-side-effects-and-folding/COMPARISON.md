# OCaml vs Rust: Option.iter and Option.fold — Side Effects and Folding

## Side-by-Side Code

### OCaml

```ocaml
let maybe_name = Some "Alice"
let no_name : string option = None

(* Option.iter — side effect only when Some *)
let () = Option.iter (fun name -> Printf.printf "Hello, %s!\n" name) maybe_name
let () = Option.iter (fun name -> Printf.printf "Hello, %s!\n" name) no_name

(* Option.fold — collapse to a value with a default *)
let greeting =
  Option.fold ~none:"Hello, stranger!" ~some:(fun n -> "Hello, " ^ n ^ "!") maybe_name
let () = print_endline greeting
```

### Rust (idiomatic — iterator + map_or_else)

```rust
pub fn greet_if_present(name: Option<&str>) -> Vec<String> {
    let mut log = Vec::new();
    // Option::iter() yields 0 or 1 items — mirrors OCaml's Option.iter
    name.iter().for_each(|n| log.push(format!("Hello, {}!", n)));
    log
}

pub fn greeting(name: Option<&str>) -> String {
    // map_or_else: if None → closure(); if Some(v) → f(v)
    name.map_or_else(|| "Hello, stranger!".to_owned(), |n| format!("Hello, {}!", n))
}
```

### Rust (functional/recursive — explicit match)

```rust
pub fn greet_if_present_iflet(name: Option<&str>) -> Vec<String> {
    let mut log = Vec::new();
    if let Some(n) = name {
        log.push(format!("Hello, {}!", n));
    }
    log
}

pub fn greeting_match(name: Option<&str>) -> String {
    match name {
        None => "Hello, stranger!".to_owned(),
        Some(n) => format!("Hello, {}!", n),
    }
}
```

### Rust (generic option_fold)

```rust
/// Direct encoding of OCaml's `Option.fold ~none ~some` signature.
pub fn option_fold<T, U>(opt: Option<T>, none: U, some: impl FnOnce(T) -> U) -> U {
    opt.map_or_else(|| none, some)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Option type | `'a option` | `Option<T>` |
| iter / side effect | `Option.iter : ('a -> unit) -> 'a option -> unit` | `opt.iter().for_each(\|x\| …)` |
| fold / collapse | `Option.fold : none:'b -> some:('a -> 'b) -> 'a option -> 'b` | `opt.map_or_else(\|\| none, some)` |
| Conditional run | `Option.iter f opt` | `if let Some(x) = opt { f(x) }` |
| Eager default | `Option.value opt ~default:d` | `opt.unwrap_or(d)` |
| Lazy default | `Option.fold ~none:(expensive()) ~some:f opt` | `opt.map_or_else(\|\| expensive(), f)` |

## Key Insights

1. **Option as iterator:** Rust's `Option<T>` implements `IntoIterator`, so it
   integrates seamlessly with the full iterator API.  `opt.iter()` borrows;
   `opt.into_iter()` consumes.  This is a capability OCaml's `Option.iter` only
   partially covers.

2. **Lazy vs eager evaluation:** OCaml's `Option.fold` evaluates both `~none` and
   `~some` arguments eagerly at the call site (they are values/closures).  Rust's
   `map_or_else` accepts *closures* for both branches, so the unused branch is
   never called — important when the default is expensive to produce.

3. **`map_or` vs `map_or_else`:** Use `map_or(default, f)` when `default` is a
   cheap `Copy` value.  Use `map_or_else(|| default, f)` when `default` requires
   allocation (e.g. `String`) to avoid paying the allocation on the `Some` path.

4. **`if let` is zero-cost:** `if let Some(x) = opt { … }` compiles to the same
   machine code as a raw `match`.  It is the idiomatic choice for a single side
   effect; `iter().for_each` is preferred when composing with other iterators.

5. **Ownership discipline:** When the value inside `Option` must be consumed (not
   just borrowed), use `opt.into_iter()` or a consuming `match`.  Borrowing via
   `opt.iter()` is correct when the `Option` must survive the side-effect call.

## When to Use Each Style

**Use `map_or_else` when:** you are producing a new value from an `Option` and the
default branch involves allocation or side-effects that should be skipped on the
`Some` path.

**Use `if let` when:** you need a simple, readable conditional side effect and are
not composing with other iterator combinators.

**Use `iter().for_each` when:** you want to treat the `Option` uniformly as a
container in a larger iterator pipeline (e.g. after `flat_map` or inside an
adapter).

**Use an explicit `match` when:** both branches have non-trivial logic or you need
to bind the value in a way that `map_or_else` would obscure.
