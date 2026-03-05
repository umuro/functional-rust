# OCaml vs Rust: Option.value and Option.is_some — Default Values

## Side-by-Side Code

### OCaml

```ocaml
let config_port = None
let config_host = Some "localhost"

let port = Option.value ~default:8080 config_port
let host = Option.value ~default:"0.0.0.0" config_host

let () = Printf.printf "Server: %s:%d\n" host port
let () = Printf.printf "Port set: %b, Host set: %b\n"
  (Option.is_some config_port) (Option.is_some config_host)
```

### Rust (idiomatic)

```rust
pub fn option_value<T>(opt: Option<T>, default: T) -> T {
    opt.unwrap_or(default)
}

pub fn option_is_some<T>(opt: &Option<T>) -> bool {
    opt.is_some()
}

let port = option_value(config_port, 8080);
let host = option_value(config_host, "0.0.0.0");
```

### Rust (functional / lazy default)

```rust
pub fn option_value_lazy<T, F: FnOnce() -> T>(opt: Option<T>, default_fn: F) -> T {
    opt.unwrap_or_else(default_fn)
}

// Closure is only called when opt is None
let port = option_value_lazy(config_port, || compute_default_port());
```

### Rust (map + unwrap_or_else chain)

```rust
pub fn describe_port(port: Option<u16>) -> String {
    port.map(|p| format!("port {p}"))
        .unwrap_or_else(|| "default port".to_string())
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Extract with default | `val value : default:'a -> 'a option -> 'a` | `fn unwrap_or(self, default: T) -> T` |
| Lazy default | *(no built-in)* | `fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T` |
| Test presence | `val is_some : 'a option -> bool` | `fn is_some(&self) -> bool` |
| Option type | `'a option` | `Option<T>` |

## Key Insights

1. **Method vs module function:** OCaml places `Option.value` in a module;
   Rust places `unwrap_or` directly on the `Option<T>` type as a method.
   Both express the same semantics — the call site reads differently but
   the meaning is identical.

2. **Labeled vs positional arguments:** OCaml's `~default:8080` is a named
   parameter that can appear in any order. Rust uses a positional argument,
   but the intent is just as clear because the method name (`unwrap_or`)
   already implies "or use this value".

3. **Eager vs lazy evaluation:** `unwrap_or(expr)` evaluates `expr`
   unconditionally — just like OCaml's `Option.value`. Use `unwrap_or_else(||
   expr)` when the default is expensive or has side effects; the closure is
   only called when the option is `None`. OCaml has no built-in lazy equivalent.

4. **Borrowing for inspection:** OCaml's `Option.is_some` does not consume its
   argument (OCaml values are immutable by default). Rust's `.is_some()` takes
   `&self`, so it also does not consume the `Option` — you can test and then
   still use the value.

5. **Combinator chains:** The `.map().unwrap_or_else()` pattern is idiomatic
   Rust for "transform if present, fall back to a default". OCaml would express
   this as `match opt with Some x -> f x | None -> default` or using
   `Option.map` followed by `Option.value`.

## When to Use Each Style

**Use `unwrap_or`** when the default value is cheap to compute (a literal, a
`Copy` value, or something already in scope).

**Use `unwrap_or_else`** when the default involves allocation or computation —
e.g., `unwrap_or_else(|| Vec::new())` — to avoid the cost when the option is
`Some`.

**Use `.map().unwrap_or_else()`** when you want to transform the inner value and
provide a structurally different fallback string or object.
