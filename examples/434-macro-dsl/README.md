📖 **[View on hightechmind.io →](https://hightechmind.io/rust/434-macro-dsl)**

---

# 434: DSL Design with Macros

**Difficulty:** 4  **Level:** Expert

Create a mini-language inside Rust using `macro_rules!` — config literals, assertion DSLs, route declarations — that reads like a domain-specific syntax while compiling to ordinary Rust.

## The Problem This Solves

Boilerplate kills readability. A config that takes 20 lines of `HashMap::insert` calls could be one declarative block. A test suite with bespoke assertion messages could use domain-specific vocabulary (`assert_that!(v, contains 3)`) instead of generic `assert!` calls. A web router could declare routes with HTTP-method keywords (`GET "/users" => handler`) rather than method calls.

Rust's standard API is general-purpose. DSLs built on top of it can be narrow and expressive — they look like the domain, not like programming. The payoff: code that non-experts can read, diffs that show intent rather than mechanics, and custom syntax that's impossible to misuse because the macro validates structure at compile time.

`macro_rules!` is Rust's tool for this. Unlike C macros or Lisp macros, it matches structured token patterns — keywords, operators, brackets — not raw text. This lets you define a grammar fragment and expand it to arbitrary Rust code.

## The Intuition

A `macro_rules!` DSL works by defining patterns that match a custom syntax and produce standard Rust code. The trick is that Rust's token tree is flexible: identifiers, literals, operators, and punctuation can all appear in patterns. You can match `GET` as an identifier, `"/path"` as a string literal, and `=>` as a punctuation sequence — then expand to whatever Rust you need.

The caller writes syntax that feels like the domain. The macro translates it to idiomatic Rust. Zero performance overhead: the translation happens at compile time, and the output is ordinary code.

## How It Works in Rust

```rust
// ── Config DSL ────────────────────────────────────────────────────────────────
// Call site:   config!(host = "localhost", port = 8080i32, debug = true)
// Expansion:   builds a HashMap<String, ConfigValue>

macro_rules! config {
    ($($key:ident = $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert(stringify!($key).to_string(), ConfigValue::from($value));)*
        Config(map)
    }};
}

let cfg = config!(host = "localhost", port = 8080i32, debug = true);
// Reads like a config file; compiles to three HashMap inserts

// ── Assertion DSL ─────────────────────────────────────────────────────────────
// Custom vocabulary for readable test assertions
macro_rules! assert_that {
    ($val:expr, equals $expected:expr) => {
        assert_eq!($val, $expected, "Expected {} to equal {}", stringify!($val), stringify!($expected));
    };
    ($val:expr, is_some) => {
        assert!($val.is_some(), "Expected {} to be Some", stringify!($val));
    };
    ($val:expr, contains $item:expr) => {
        assert!($val.contains(&$item), "Expected {:?} to contain {:?}", $val, $item);
    };
    ($val:expr, has_len $len:expr) => {
        assert_eq!($val.len(), $len);
    };
}

// Tests read like prose:
assert_that!(v, has_len 3);
assert_that!(v, contains 2);
assert_that!(config.get("host"), is_some);

// ── Route DSL (keyword matching) ──────────────────────────────────────────────
// Matching identifiers as "keywords" in the macro pattern
macro_rules! method_str {
    (GET)    => { "GET" };
    (POST)   => { "POST" };
    (DELETE) => { "DELETE" };
}
// Full router! macro would expand to route table entries
```

**Key DSL design principles:**
1. Use `$(,)?` to allow trailing commas — feels natural, prevents user frustration
2. Match identifiers as pseudo-keywords (`GET`, `equals`, `is_some`)
3. Use `stringify!($key)` to turn identifier tokens into string keys
4. Emit `compile_error!` for invalid input — be a good language designer
5. Keep each arm's expansion self-contained and easy to expand by hand

## What This Unlocks

- **Configuration literals** — `config! { host = "...", port = 8080 }` looks like TOML but is compiled Rust with type checking.
- **Test DSLs** — `assert_that!(x, equals y)` reads as English, produces better error messages, and guides reviewers who don't know Rust well.
- **Embedded languages** — `json!({...})` (serde_json), `html!(<div>...</div>)` (yew), `sql!(SELECT ...)` — all real crates built on this pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Embedded DSL | `ppx` for AST transformation; `camlp5` for syntax extension | `macro_rules!` token matching; proc macros for full AST power |
| Config literals | No native syntax; use records or assoc lists | `macro_rules!` config DSL compiles to HashMap construction |
| Pattern vocabulary | ppx matches OCaml AST nodes | `macro_rules!` matches token trees — identifiers, literals, punctuation |
| Compile-time validation | ppx can reject invalid forms | `compile_error!` in catch-all arm |
| Real-world examples | `ppx_sexp_conv`, `ppx_let` | `json!`, `html!`, `sql!`, `vec!`, `format!` |
