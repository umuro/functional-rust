📖 **[View on hightechmind.io →](https://hightechmind.io/rust/305-unwrap-or-patterns)**

---

# 305: unwrap_or, unwrap_or_else, unwrap_or_default

## Problem Statement

Extracting a value from `Option<T>` or `Result<T, E>` when a sensible default exists is extremely common: getting a config value or using a default, parsing a number or falling back to 0, looking up a cache entry or computing a fresh value. The `unwrap_or` family provides safe alternatives to `unwrap()` that handle the `None`/`Err` case without panicking. The three variants differ in when the default value is computed — eagerly, lazily, or from the type's `Default` implementation.

## Learning Outcomes

- Understand the three variants: `unwrap_or(val)`, `unwrap_or_else(f)`, `unwrap_or_default()`
- Use `unwrap_or_else` for expensive default computations to avoid computing when not needed
- Use `unwrap_or_default()` for types implementing `Default` (0 for numbers, `""` for strings)
- Recognize that `unwrap_or(val)` always evaluates `val` — use `unwrap_or_else` for lazy computation

## Rust Application

```rust
// unwrap_or: eager default — val is always evaluated
pub fn get_or(opt: Option<i32>, default: i32) -> i32 { opt.unwrap_or(default) }

// unwrap_or_else: lazy default — f is only called when None
pub fn get_or_compute<F: FnOnce() -> i32>(opt: Option<i32>, f: F) -> i32 {
    opt.unwrap_or_else(f)
}

// unwrap_or_default: uses type's Default impl
pub fn get_or_default<T: Default>(opt: Option<T>) -> T {
    opt.unwrap_or_default()
}
// None::<i32>.unwrap_or_default() == 0
// None::<String>.unwrap_or_default() == ""
// None::<Vec<i32>>.unwrap_or_default() == vec![]
```

## OCaml Approach

OCaml uses `Option.value opt ~default:val` (Base library) or `Option.fold ~none:default ~some:Fun.id opt`:

```ocaml
(* Eager default *)
let get_or opt default = Option.value opt ~default

(* Lazy default (Base library) *)
let get_or_lazy opt f = Option.value_or_thunk opt ~f

(* Default via Option.fold *)
let get_or_zero opt = Option.fold ~none:0 ~some:Fun.id opt
```

## Key Differences

1. **Eagerness distinction**: `unwrap_or(val)` is eager (OCaml's `Option.value` is also eager); `unwrap_or_else(f)` is lazy — an important performance distinction for expensive defaults.
2. **Default trait**: `unwrap_or_default()` is unique to Rust — OCaml has no equivalent `Default` typeclass.
3. **Result too**: These methods exist on `Result` too: `result.unwrap_or(0)` extracts the `Ok` value or returns 0 on `Err`.
4. **Panic avoidance**: These methods are always safe; `unwrap()` panics on `None`/`Err` — reserve `unwrap()` for provably non-None situations or tests.

## Exercises

1. Implement a configuration reader that uses `unwrap_or_else(|| read_default_config())` to lazily load defaults only when no user config is found.
2. Use `unwrap_or_default()` to provide empty defaults for a struct with multiple `Option<Vec<String>>` fields.
3. Compare `opt.unwrap_or(expensive_computation())` vs `opt.unwrap_or_else(|| expensive_computation())` — when does the difference matter?
