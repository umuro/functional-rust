**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[json-query on hightechmind.io](https://hightechmind.io/posts/functional-rust/json-query)

---

## Problem Statement

Implement path-based JSON querying: given a path like `["users", "0", "name"]`, traverse a `JsonValue` tree and return a borrowed reference to the value at that path. Array indices are specified as stringified integers. Implement typed extractors (`get_string`, `get_number`, `get_bool`) that further pattern-match the result.

## Learning Outcomes

- Implement `get<'a>(path: &[&str], json: &'a JsonValue) -> Option<&'a JsonValue>` with lifetime annotations
- Use slice patterns `[]` (empty) and `[key, rest @ ..]` (head and tail) for recursive path traversal
- Handle both `Object` lookup (`pairs.iter().find(|(k, _)| k == key)`) and `Array` index access (`key.parse::<usize>().ok()`)
- Implement typed extractors that combine `get` with a pattern match on the result variant
- Understand why Rust's lifetime `'a` ties the returned reference to the input `json`

## Rust Application

```rust
pub fn get<'a>(path: &[&str], json: &'a JsonValue) -> Option<&'a JsonValue> {
    match path {
        [] => Some(json),
        [key, rest @ ..] => match json {
            JsonValue::Object(pairs) => {
                let found = pairs.iter().find(|(k, _)| k == key);
                found.and_then(|(_, v)| get(rest, v))
            }
            JsonValue::Array(items) => {
                let idx: usize = key.parse().ok()?;
                items.get(idx).and_then(|v| get(rest, v))
            }
            _ => None,
        },
    }
}

pub fn get_string<'a>(path: &[&str], json: &'a JsonValue) -> Option<&'a str> {
    match get(path, json) {
        Some(JsonValue::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

pub fn get_number(path: &[&str], json: &JsonValue) -> Option<f64> {
    match get(path, json) {
        Some(JsonValue::Number(n)) => Some(*n),
        _ => None,
    }
}
```

The lifetime `'a` connects the input `json: &'a JsonValue` to the output `Option<&'a JsonValue>`. The compiler guarantees that the returned reference cannot outlive the JSON tree it was borrowed from.

Slice pattern `[key, rest @ ..]` destructures the path: `key` is the first element, `rest` is a slice reference to the remainder. This is the Rust equivalent of OCaml's `key :: rest` list pattern.

`key.parse::<usize>().ok()?` converts a string index to `usize`, returning `None` on parse failure. The `?` operator propagates `None` out of the function.

## OCaml Approach

```ocaml
let rec get path json =
  match path with
  | [] -> Some json
  | key :: rest ->
    match json with
    | Object pairs ->
      (match List.assoc_opt key pairs with
       | Some v -> get rest v
       | None -> None)
    | Array items ->
      (match int_of_string_opt key with
       | Some idx when idx >= 0 && idx < List.length items ->
         get rest (List.nth items idx)
       | _ -> None)
    | _ -> None

let get_string path json =
  match get path json with
  | Some (Str s) -> Some s
  | _ -> None
```

OCaml's `List.assoc_opt key pairs` looks up a key in an association list — directly replacing the `find` + `and_then` chain. OCaml does not need explicit lifetime annotations; the GC manages value lifetimes transparently.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Return type | `Option<&'a JsonValue>` — borrowed reference with lifetime | `json option` — GC-managed value |
| Path destructuring | `[key, rest @ ..]` slice pattern | `key :: rest` list pattern |
| Object lookup | `iter().find()` | `List.assoc_opt` |
| Array index | `key.parse::<usize>().ok()?` | `int_of_string_opt` + bounds check |
| Lifetime annotation | Required to express borrow through recursion | Not needed |

The lifetime annotation is not extra complexity — it is the compiler making an implicit contract explicit. The returned reference is "borrowed from `json` for `'a`", so the caller cannot mutate or drop `json` while holding the reference.

## Exercises

1. Implement `get_mut<'a>(path: &[&str], json: &'a mut JsonValue) -> Option<&'a mut JsonValue>` to allow mutation at a path.
2. Implement `set(path: &[&str], json: &mut JsonValue, value: JsonValue)` that inserts/replaces a value at a path.
3. Implement `delete(path: &[&str], json: &mut JsonValue) -> bool` that removes a key or array element.
4. Implement `query_all(key: &str, json: &JsonValue) -> Vec<&JsonValue>` that finds all values with matching key at any depth.
5. Parse a JSON path string like `"users[0].name"` into a `Vec<&str>` slice and use it with `get`.
