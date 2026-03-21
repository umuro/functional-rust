📖 **[View on hightechmind.io →](https://hightechmind.io/rust/537-lifetime-coercion)**

---

# Lifetime Coercion and Subtyping
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Lifetime subtyping is the mechanism that makes Rust's borrow checker flexible without requiring programmers to always use exactly matching lifetimes. The rule is simple: a longer lifetime can always be used where a shorter one is expected, because a reference valid for longer is certainly valid for shorter. This is similar to how a subtype can be used where its supertype is expected. Without this coercion, every reference would require perfectly matched lifetime scopes, making APIs rigid and hard to use.

## Learning Outcomes

- Why `'static` references can be passed to functions expecting shorter-lived references
- How `'long: 'short` (outlives) expresses that `'long` is a subtype of `'short`
- How reborrowing creates a shorter-lived reference from a longer-lived one
- Why lifetime coercion makes it possible to store `'static` items in `Vec<&'short str>`
- How `demonstrate_variance<'long: 'short, 'short>` expresses the outlives constraint

## Rust Application

`use_briefly<'short>(s: &'short str) -> usize` accepts any reference. `coercion_demo` shows `&'static str` being passed to `use_briefly` — `'static` coerces to `'short` automatically. `store_with_coercion<'short>(storage: &mut Vec<&'short str>, item: &'static str)` stores a static reference where a shorter-lived one is required. `demonstrate_variance<'long: 'short, 'short>` explicitly names the outlives relationship — `'long: 'short` means `'long` outlives `'short`.

Key patterns:
- `'long: 'short` — "long outlives short" / "long is a subtype of short"
- Implicit coercion: `&'static str` used where `&'short str` expected
- Reborrowing: `let short: &'short str = &*long_ref` creates shorter borrow

## OCaml Approach

OCaml has no lifetime coercion because there are no lifetime annotations. The GC ensures all referenced values are kept alive. Subtyping in OCaml is structural (via polymorphic variants and object types), not lifetime-based.

```ocaml
(* No equivalent concept — all references are GC-managed *)
let use_briefly s = String.length s
let _ = use_briefly "static string"  (* always fine *)
```

## Key Differences

1. **Automatic coercion**: Rust automatically coerces `'long` to `'short` at assignment; OCaml has no lifetime coercion because lifetimes do not exist.
2. **Explicit outlives**: Rust's `'long: 'short` syntax expresses a compile-time constraint; OCaml programs never write or verify such relationships.
3. **Subtype direction**: In Rust, longer lifetimes are subtypes of shorter ones (counterintuitive but correct); OCaml subtyping is based on structural compatibility, not lifetime ordering.
4. **Practical impact**: Lifetime coercion enables functions with short-lived references to accept static data without special handling; OCaml APIs need no such accommodation.

## Exercises

1. **Longest-lifetime function**: Write `fn most_general<'a>(s: &'a str) -> &'a str { s }` and show it can accept both `&'static str` and short-lived references.
2. **Vec of mixed lifetimes**: Build a `Vec<&str>` and insert both `&'static str` literals and references to local `String` values — verify the compiler correctly constrains the vec's lifetime.
3. **Subtyping chain**: Write three functions with lifetimes `'a: 'b: 'c` where the first returns `&'a str`, the second accepts `&'b str`, and the third accepts `&'c str` — show the chain compiles.
