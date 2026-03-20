📖 **[View on hightechmind.io →](https://hightechmind.io/rust/854-monad-option)**

---

# Option Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A chain of operations that might fail — look up a user, find their settings, get their preferred language — requires either nested `if let Some` blocks (deeply indented "pyramid of doom") or the `?` operator shorthand. The Option monad formalizes this: `and_then` sequences computations where each step might return `None`, automatically propagating absence without explicit checking. This is Rust's `Option::and_then`, Haskell's `Maybe` monad, and OCaml's `Option.bind`. The power: code that looks like a straight pipeline reads cleanly, yet automatically handles every possible absence at every step.

## Learning Outcomes

- Understand `and_then` (monadic bind): if `Some(x)`, apply f to x and return `f(x)`; if `None`, return `None`
- Chain multiple `and_then` calls to build pipelines of fallible lookups
- Recognize Rust's `?` operator as syntactic sugar for `and_then` (or `return Err()`)
- Understand the difference from `map`: `map` wraps the result; `and_then` expects f to return `Option<U>`
- Apply to: multi-step dictionary lookups, configuration path traversal, safe arithmetic chains

## Rust Application

```rust
fn find_user_docs(env: &HashMap<&str, &str>, paths: &HashMap<&str, Vec<&str>>) -> Option<String> {
    env.get("HOME")
        .and_then(|home| paths.get(home))
        .and_then(|docs| docs.first())
        .map(|doc| doc.to_string())
}
```

The chain reads left-to-right: get HOME from env (might be None), look it up in paths (might be None), get first path (might be empty Vec), convert to String. Each `and_then` automatically propagates `None` — if any step returns `None`, the entire chain returns `None`. The final `.map` uses `map` because `to_string()` never fails — it always returns a `String`, not `Option<String>`. This distinction between `map` (infallible transform) and `and_then` (fallible transform) is the key intuition.

## OCaml Approach

OCaml's `Option.bind` is the `and_then` equivalent: `Option.bind (Hashtbl.find_opt env "HOME") (fun home -> Hashtbl.find_opt paths home)`. The `let ( >>= ) = Option.bind` infix operator enables pipeline syntax: `Hashtbl.find_opt env "HOME" >>= Hashtbl.find_opt paths >>= List.nth_opt`. OCaml's `ppx_let` syntax extension allows `let%bind home = ...` for do-notation style. The `Option.map` at the end for the infallible transform mirrors the Rust pattern.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Bind function | `Option::and_then` | `Option.bind` |
| Infix operator | `?` in `fn -> Option<T>` | `>>=` via `let ( >>= )` |
| Do notation | `?` operator | `let%bind` with ppx_let |
| Map vs bind | `map` for `T -> U`, `and_then` for `T -> Option<U>` | `Option.map` vs `Option.bind` |
| None propagation | Automatic via `and_then` | Same |
| Chain length | Unlimited `and_then` chain | Same |

## Exercises

1. Rewrite `find_user_docs` using the `?` operator inside a function returning `Option<String>` and verify same behavior.
2. Implement a safe arithmetic chain: parse a string to int, divide by another parsed int, take square root — all with Option.
3. Implement `Option::and_then` from scratch using only `match` and show the equivalence.
4. Demonstrate the failure mode: for each step in the chain, show which input causes `None` to propagate.
5. Write a configuration file traverser using `and_then`: navigate nested `HashMap<String, Value>` (where Value is an enum) without panicking.
