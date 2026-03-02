# LinkedIn Post: Option and Result Types

🦀 **Functional Rust #004: Option and Result Types**

No null pointers. No exceptions. Just exhaustive pattern matching on success and failure.

**OCaml:**
```ocaml
let safe_divide x y =
  if y = 0 then None
  else Some (x / y)

let parse_int s =
  try Ok (int_of_string s)
  with Failure _ -> Error "Not a valid integer"
```

**Rust:**
```rust
fn safe_divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 { None } else { Some(x / y) }
}

fn parse_int(s: &str) -> Result<i32, String> {
    s.parse().map_err(|_| "Not a valid integer".to_string())
}
```

**Monadic chaining:**

OCaml:
```ocaml
safe_divide 100 5 >>= fun x ->
safe_divide x 2 >>| fun x -> x * 2
```

Rust:
```rust
safe_divide(100, 5)
    .and_then(|x| safe_divide(x, 2))
    .map(|x| x * 2)
```

**Why this matters:**

✅ Compiler forces you to handle errors
🚫 No null pointer exceptions
🔗 Composable error pipelines
📦 Built-in methods (Rust) vs custom operators (OCaml)

Rust's `?` operator makes this even cleaner in functions returning Result.

Next: Currying and partial application 🍛

#Rust #FunctionalProgramming #ErrorHandling #RustLang #TypeSafety
