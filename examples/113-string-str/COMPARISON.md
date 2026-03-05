# OCaml vs Rust: String vs &str

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has one string type — immutable, GC-managed *)
let first_word s =
  match String.index_opt s ',' with
  | Some i -> String.sub s 0 i |> String.trim
  | None   -> String.trim s

let greet name = "Hello, " ^ name ^ "!"

let char_count s = String.length s   (* byte count in OCaml *)

let () =
  assert (first_word "hello, world!" = "hello");
  assert (greet "Alice" = "Hello, Alice!");
  print_endline "ok"
```

### Rust (idiomatic — &str parameters)
```rust
// &str in function parameters: callers can pass literals or &String
pub fn first_word(s: &str) -> &str {
    s.split(',').next().unwrap_or(s).trim()
}

pub fn greet(name: &str) -> String {
    let mut g = String::from("Hello, ");
    g.push_str(name);
    g.push('!');
    g
}

pub fn char_count(s: &str) -> usize {
    s.chars().count()   // Unicode scalar values, not bytes
}
```

### Rust (functional / builder style)
```rust
// Build strings with iterators — no mutation
pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

pub fn append(base: &str, suffix: &str) -> String {
    [base, suffix].concat()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| String type | `string` (single type) | `String` (owned) / `&str` (borrowed) |
| Function parameter | `val f : string -> string` | `fn f(s: &str) -> String` |
| Literal type | `string` | `&'static str` |
| Substring | `String.sub s start len` → `string` (copy) | `&s[start..end]` → `&str` (zero-copy) |
| Mutation | Not allowed (immutable) | `String` is mutable; `&str` is not |
| Char count | `String.length` (bytes) | `s.chars().count()` (Unicode scalars) |

## Key Insights

1. **Two types for two purposes.** Rust's `String` owns heap-allocated text you can mutate and grow. `&str` is a borrowed view — a fat pointer (pointer + length) into any existing string data, requiring no allocation.

2. **Use `&str` in function signatures.** Writing `fn f(s: &str)` lets callers pass a string literal (`"hello"`), a `&String` (via auto-deref through `Deref<Target = str>`), or a slice of a larger string — all without forcing an allocation.

3. **OCaml's single `string` is Rust's `String`.** Both are heap-allocated and managed (GC in OCaml, ownership in Rust). Rust adds `&str` as a zero-cost abstraction that OCaml doesn't have — every OCaml `String.sub` copies; Rust `&s[a..b]` does not.

4. **`String::length` in OCaml counts bytes; `str::chars().count()` counts Unicode scalar values.** The distinction matters for multibyte characters: `"café"` has 4 chars but 5 UTF-8 bytes.

5. **Ownership is visible in the type.** `String` in a return type tells the caller they own new heap memory. `&str` in a return type (borrowing from input) is zero-copy. This contract is enforced by the borrow checker — no runtime surprises.

## When to Use Each Style

**Use `&str` in function parameters when:** you only need to read the string. This is the idiomatic Rust default — it accepts literals, `String` borrows, and subslices without allocation.

**Use `String` (owned) when:** the function needs to build, grow, or return new string data, or when you need the string to outlive the input (e.g., storing in a struct field).

**Use subslice `&str` returns when:** you can return a view into the input string (e.g., `first_word`) — zero allocation, maximum efficiency. The lifetime ties the returned slice to the input.
