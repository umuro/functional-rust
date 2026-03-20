📖 **[View on hightechmind.io →](https://hightechmind.io/rust/306-ok-or-patterns)**

---

# 306: ok_or and ok_or_else
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Code often needs to convert an `Option<T>` to a `Result<T, E>` — treating absence as an error with a specific message. A missing configuration key is an `Option<&str>`, but the caller expects a `Result` with a descriptive error. The `ok_or()` and `ok_or_else()` methods convert `Option<T>` to `Result<T, E>`, supplying an error value for the `None` case. This is the bridge between optionality and error handling.

## Learning Outcomes

- Use `ok_or(err)` to convert `None` to `Err(err)` — eager error value
- Use `ok_or_else(|| err)` to convert `None` to `Err(f())` — lazy error computation
- Chain `ok_or_else` with `?` to propagate missing values as typed errors
- Recognize the pattern: HashMap lookup → `ok_or_else` → `?` for "required key" semantics

## Rust Application

`ok_or` is the primary tool for "required lookup" patterns:

```rust
pub fn lookup<'a>(map: &'a HashMap<&str, &str>, key: &str) -> Result<&'a str, String> {
    map.get(key)
       .copied()
       .ok_or_else(|| format!("key '{}' not found", key))
}

pub fn get_port(config: &HashMap<&str, &str>) -> Result<u16, String> {
    let s = config.get("port").copied().ok_or("port not set")?;
    s.parse::<u16>().map_err(|e| format!("invalid port: {}", e))
}
```

## OCaml Approach

OCaml uses `Option.to_result ~none:err` (Base library) or `Option.fold`:

```ocaml
let lookup map key =
  Hashtbl.find_opt map key
  |> Option.to_result ~none:(Printf.sprintf "key '%s' not found" key)

(* Or manually: *)
let require_some msg = function
  | None -> Error msg
  | Some v -> Ok v
```

## Key Differences

1. **Naming**: `ok_or` maps `None -> Err(e)`, `Some(v) -> Ok(v)` — the name reads as "convert to Ok, or use this error".
2. **Eagerness**: `ok_or(err)` always evaluates `err` (even for `Some`); `ok_or_else(|| err)` is lazy — prefer `ok_or_else` for format strings or allocations.
3. **Standard library**: `ok_or` is a standard method on `Option` in Rust; OCaml's `Option.to_result` is in the `Base` library or requires manual wrapping.
4. **Inverse**: `Result::ok()` converts `Result<T, E>` to `Option<T>` — `ok_or` and `ok()` are inverses (with loss of error detail in the `ok()` direction).

## Exercises

1. Write a function that validates a `HashMap<String, String>` config, using `ok_or_else` to produce descriptive errors for each missing required key.
2. Implement a `required<T: Clone>(map: &HashMap<&str, T>, key: &str) -> Result<T, String>` helper that extracts required map values.
3. Chain three `ok_or_else` calls with `?` to look up `host`, `port`, and `path` from a config map, failing with a descriptive error if any is missing.
