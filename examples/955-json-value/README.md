**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[json-value on hightechmind.io](https://hightechmind.io/posts/functional-rust/json-value)

---

## Problem Statement

Represent JSON values as a recursive Rust enum, mirroring OCaml's algebraic type `type json = Null | Bool of bool | Number of float | Str of string | Array of json list | Object of (string * json) list`. Implement type predicates, a simple string serializer, and builder helpers. This exercise demonstrates how ADTs model recursive data structures and how Rust's `enum` corresponds exactly to OCaml's variant types.

## Learning Outcomes

- Define a recursive enum `JsonValue` with six variants: Null, Bool, Number, Str, Array, Object
- Implement `is_null`, `is_bool`, `is_number`, etc. using the `matches!` macro
- Implement `to_string_simple` with pattern matching that handles each variant including integer/float formatting for `Number`
- Build builder helpers: `JsonValue::object(&[(&str, JsonValue)])`, `JsonValue::array(&[JsonValue])`
- Understand the Rust/OCaml parallel: `Vec<JsonValue>` ↔ `json list`, `Vec<(String, JsonValue)>` ↔ `(string * json) list`

## Rust Application

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    pub fn is_null(&self)   -> bool { matches!(self, JsonValue::Null) }
    pub fn is_number(&self) -> bool { matches!(self, JsonValue::Number(_)) }
    pub fn is_array(&self)  -> bool { matches!(self, JsonValue::Array(_)) }

    pub fn to_string_simple(&self) -> String {
        match self {
            JsonValue::Null       => "null".to_string(),
            JsonValue::Bool(true) => "true".to_string(),
            JsonValue::Bool(false) => "false".to_string(),
            JsonValue::Number(n) =>
                if n.fract() == 0.0 && n.is_finite() { format!("{}", *n as i64) }
                else { format!("{}", n) },
            JsonValue::Str(s)   => format!("\"{}\"", s),
            JsonValue::Array(_) => "[...]".to_string(),
            JsonValue::Object(_) => "{...}".to_string(),
        }
    }

    pub fn object(pairs: &[(&str, JsonValue)]) -> Self {
        JsonValue::Object(
            pairs.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
        )
    }
}
```

The `matches!` macro expands to a boolean pattern match — more concise than writing `if let JsonValue::Null = self { true } else { false }`. 

`Number` formatting distinguishes integers (`fract() == 0.0`) from floats to produce `42` rather than `42.0`. The `is_finite()` guard prevents `format!("{}", f64::INFINITY as i64)` from panicking.

`Object` uses `Vec<(String, JsonValue)>` rather than `HashMap` to preserve insertion order — matching JSON specification.

## OCaml Approach

```ocaml
type json =
  | Null
  | Bool of bool
  | Number of float
  | Str of string
  | Array of json list
  | Object of (string * json) list

let rec to_string = function
  | Null -> "null"
  | Bool true -> "true"
  | Bool false -> "false"
  | Number n ->
    if Float.is_integer n then string_of_int (int_of_float n)
    else string_of_float n
  | Str s -> Printf.sprintf "%S" s  (* %S: OCaml string with escaping *)
  | Array _ -> "[...]"
  | Object _ -> "{...}"

let is_null = function Null -> true | _ -> false
let is_bool = function Bool _ -> true | _ -> false
```

OCaml's `type json` and Rust's `enum JsonValue` are structurally identical. The `function` keyword in OCaml is shorthand for `fun x -> match x with`. OCaml's pattern matching and Rust's `match` have the same power and exhaustiveness checking.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Recursive enum | `Vec<JsonValue>` in variant | `json list` in variant |
| `#[derive(Clone)]` | Explicit; `Vec` requires `Clone` | Automatic structural sharing via GC |
| `matches!` macro | Pattern predicate shorthand | `function` shorthand |
| String vs str | `String` (owned) for keys | `string` always owned |
| Ordered object | `Vec<(String, JsonValue)>` | `(string * json) list` |

This type is the foundation for examples 956 (pretty-print), 957 (query), and related JSON manipulation exercises. The ADT approach makes impossible states unrepresentable: a `Number` always contains a valid `f64`, an `Array` always contains valid `JsonValue` elements.

## Exercises

1. Implement `get(key: &str)` on `Object` variants that returns `Option<&JsonValue>`.
2. Implement `index(i: usize)` on `Array` variants that returns `Option<&JsonValue>`.
3. Add `From<bool>`, `From<f64>`, `From<&str>`, `From<i64>` trait implementations for ergonomic construction.
4. Implement `deep_equal(a: &JsonValue, b: &JsonValue) -> bool` without using `PartialEq`.
5. Implement `flatten_arrays` that replaces nested arrays with flat ones recursively.
