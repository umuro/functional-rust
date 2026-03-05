# OCaml vs Rust: Advanced Enum Patterns

## JSON-like Type

### OCaml
```ocaml
type json =
  | Null
  | Bool   of bool
  | Num    of float
  | Str    of string
  | Array  of json list
  | Object of (string * json) list
```

### Rust
```rust
enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}
```

## Recursive Operations

### OCaml
```ocaml
let rec depth = function
  | Array xs   -> 1 + List.fold_left max 0 (List.map depth xs)
  | Object kvs -> 1 + List.fold_left max 0 (List.map (fun (_,v) -> depth v) kvs)
  | _          -> 0
```

### Rust
```rust
fn depth(&self) -> usize {
    match self {
        Json::Array(xs) => 1 + xs.iter().map(|x| x.depth()).max().unwrap_or(0),
        Json::Object(kv) => 1 + kv.iter().map(|(_, v)| v.depth()).max().unwrap_or(0),
        _ => 0,
    }
}
```

## Display/Show

### OCaml
```ocaml
let rec show = function
  | Null -> "null"
  | Bool b -> string_of_bool b
  | Num n -> string_of_float n
  | Str s -> Printf.sprintf "%S" s
  | Array xs -> Printf.sprintf "[%s]" (String.concat "," (List.map show xs))
  | Object kvs -> (* ... *)
```

### Rust
```rust
impl Display for Json {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Json::Null => write!(f, "null"),
            Json::Bool(b) => write!(f, "{}", b),
            // ...
        }
    }
}
```

## Key Patterns

1. **Recursive variants** - `Array(Vec<Json>)` contains more `Json`
2. **Associated data** - Each variant carries different data
3. **Nested matching** - Match into nested structures
4. **Type dispatch** - Different behavior per variant
