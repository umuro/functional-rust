📖 **[View on hightechmind.io →](https://hightechmind.io/rust/571-pattern-dotdot-wildcard)**

---

# 571: `..` and `_` Wildcards

**Difficulty:** 2  **Level:** Beginner

Ignore fields, elements, and bindings you don't need in pattern matches.

## The Problem This Solves

Rust's patterns are exhaustive by design — you must account for every field and variant. But often you only care about one or two fields in a large struct, or the first and last elements of a tuple. Writing out every field name just to ignore most of them is verbose and brittle: adding a field to the struct breaks every match site.

`..` ("dot-dot") and `_` ("underscore") solve this. `..` in a struct pattern skips all remaining fields. `..` in a tuple pattern matches any number of elements in the middle. `_` ignores a single binding (and suppresses unused-variable warnings). Together they make patterns concise without sacrificing exhaustiveness.

## The Intuition

`_` is a throwaway slot: "put something here but I don't want it." `..` is a throwaway range: "skip everything else I didn't name." In struct patterns, `..` means "I only named the fields I care about; let the rest be whatever." In tuples, `(first, .., last)` says "give me the first and last; the middle doesn't matter."

The `_x` convention (leading underscore) binds the variable to suppress warnings, unlike bare `_` which doesn't bind at all — useful when you want the *name* for documentation clarity but the *value* isn't used yet.

## How It Works in Rust

1. **Struct `..`** — `Config { host, port, .. }` in a function parameter destructures only `host` and `port`; all other fields are silently ignored.
2. **Enum variant `_`** — `Response::Ok(v, _, _)` extracts the first field and ignores the others by position.
3. **Tuple `..`** — `let (first, .., last) = (1, 2, 3, 4, 5)` binds first and last; middle elements are discarded.
4. **Function parameter `_`** — `fn always_zero(_: i32) -> i32 { 0 }` accepts the argument but doesn't bind it.
5. **`_x` suppression** — `let _unused = String::from("...");` binds without triggering unused-variable lint.

## What This Unlocks

- Match large structs without listing every field — future fields won't break the pattern.
- Write function signatures that accept parameters for API compatibility without using them.
- Extract head and tail of tuples in a single destructuring without intermediate bindings.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Ignore a field | `_` in pattern; records use `{ field; _ }` | `_` for single field; `..` to skip all remaining struct fields |
| Tuple remainder | `(first, _)` or `(first, _rest)` | `(first, .., last)` — `..` matches any number of middle elements |
| Unused binding | `_x` convention in OCaml too | `_x` or `_` — `_x` binds, `_` doesn't |
| Exhaustiveness | Compiler warns on missing cases | Same; `..` satisfies exhaustiveness for remaining struct fields |
