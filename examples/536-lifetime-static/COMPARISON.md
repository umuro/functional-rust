# OCaml vs Rust: 'static Lifetime

## OCaml
```ocaml
(* No concept of 'static — all values managed by GC *)
let app_name = "MyRustApp"  (* string literal, GC-managed *)

let error_messages = [
  (404, "Not Found");
  (500, "Internal Server Error")
]
```

## Rust
```rust
// 'static means "valid for entire program duration"
pub static APP_NAME: &'static str = "MyRustApp";

// String literals are automatically &'static str
pub fn get_greeting() -> &'static str {
    "Hello, World!"  // embedded in binary
}

// Thread spawn requires 'static (outlive caller)
std::thread::spawn(move || { /* 'static data only */ });
```

## Key Differences

1. **OCaml**: GC handles all lifetimes uniformly
2. **Rust**: 'static means "lives forever" (program duration)
3. **Rust**: String literals are &'static str (in binary)
4. **Rust**: Owned types satisfy 'static bound
5. **Rust**: Thread safety often requires 'static
