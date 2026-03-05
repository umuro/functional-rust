# OCaml vs Rust: Option.map and Option.bind — Safe Value Transformation

## Side-by-Side Code

### OCaml
```ocaml
let parse_int s = match int_of_string_opt s with Some n -> Some n | None -> None
let safe_div x y = if y = 0 then None else Some (x / y)

let result =
  parse_int "42"
  |> Option.map (fun x -> x * 2)
  |> Option.bind (fun x -> safe_div x 7)
(* result = Some 12 *)
```

### Rust (idiomatic — method chaining)
```rust
pub fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

pub fn parse_double_divide(s: &str, divisor: i32) -> Option<i32> {
    parse_int(s)
        .map(|x| x * 2)
        .and_then(|x| safe_div(x, divisor))
}
```

### Rust (? operator — idiomatic for multi-step fallible logic)
```rust
pub fn parse_double_divide_question(s: &str, divisor: i32) -> Option<i32> {
    let n: i32 = s.parse().ok()?;
    let doubled = n * 2;
    safe_div(doubled, divisor)
}
```

### Rust (explicit match — closest to OCaml pattern matching style)
```rust
pub fn parse_double_divide_explicit(s: &str, divisor: i32) -> Option<i32> {
    let n = match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    let doubled = n * 2;
    match divisor {
        0 => None,
        d => Some(doubled / d),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| map | `val map : ('a -> 'b) -> 'a option -> 'b option` | `fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U>` |
| bind / and_then | `val bind : 'a option -> ('a -> 'b option) -> 'b option` | `fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U>` |
| Pipe operator | `\|>` | `.` (method chaining) |
| Short-circuit | `Option.bind` / explicit match | `?` operator |
| String parse | `int_of_string_opt : string -> int option` | `str::parse::<T>() -> Result<T, _>`, then `.ok()` |

## Key Insights

1. **`Option.bind` = `and_then`:** The names differ but the semantics are identical — apply a function that itself returns `Option`, flattening the result so you never get `Some(Some(x))`.
2. **`?` is syntactic sugar for `and_then`:** In a function returning `Option<T>`, writing `value?` is equivalent to `value.ok_or(()).and_then(...)` — it returns `None` immediately if the value is `None`, otherwise unwraps it.
3. **OCaml uses `.ok()`, Rust uses `.ok()`:** OCaml's `int_of_string_opt` already returns `option`; Rust's `str::parse` returns `Result`, so `.ok()` converts it to `Option` by discarding the error.
4. **Pipeline style translates directly:** OCaml's `v |> Option.map f |> Option.bind g` becomes `v.map(f).and_then(g)` in Rust — the same left-to-right readable chain.
5. **None propagation is zero-cost:** Both OCaml and Rust represent `None` as a tag-plus-no-value, and the combinator chain is inlined by the compiler — no heap allocation, no exception overhead.

## When to Use Each Style

**Use `.map().and_then()` chaining when:** you have a clear linear pipeline of transformations and the closures are short — it reads like a data-flow diagram.

**Use the `?` operator when:** the function has multiple fallible steps mixed with other logic, and you want to name intermediate results clearly without nesting closures.

**Use explicit `match` with early `return` when:** the failure branches need different handling or logging, or when the logic is complex enough that a closure would obscure intent.
