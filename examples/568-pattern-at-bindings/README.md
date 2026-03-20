📖 **[View on hightechmind.io →](https://hightechmind.io/rust/568-pattern-at-bindings)**

---

# @ Bindings
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Sometimes you want both to test a value against a pattern and to bind it to a name for use in the arm body. Without `@` bindings, you would either have to test the pattern and reconstruct the value, or bind the name and recheck the condition inside the arm. The `@` operator (at-binding) solves this: `a @ 0..=12` both tests that the value is in range AND binds it to `a`, so you can use `a` in the expression without repeating the match.

## Learning Outcomes

- How `name @ pattern` binds the value to `name` while also testing `pattern`
- How `a @ 0..=12` combines range testing with binding
- How `e @ Event::Click(..)` binds the whole enum value while matching a variant
- How `@` can be used in nested patterns and with guards
- Where `@` is common: error reporting (bind the bad value), range-based classification

## Rust Application

`describe_age(age: u32)` uses `a @ 0..=12 => format!("child ({})", a)` — matches the range AND uses `a` in the format string. `process_event(e: &Event)` uses `e @ Event::Click(_, _) => format!("click: {:?}", e)` — binds the whole event value for display. The `@` binding can be nested: `outer @ OuterStruct { inner @ InnerStruct { value } }` binds at multiple levels simultaneously.

Key patterns:
- `name @ literal` — bind and test exact value
- `name @ lo..=hi` — bind and test range
- `name @ Variant(..)` — bind whole value, test variant
- Nested: `outer @ Type { inner @ Field { ... } }`

## OCaml Approach

OCaml uses `as` for the equivalent:

```ocaml
let describe_age age = match age with
  | a when a <= 12 -> Printf.sprintf "child (%d)" a
  | a when a <= 19 -> Printf.sprintf "teen (%d)" a
  | a -> Printf.sprintf "adult (%d)" a

(* Or with constructor binding: *)
let f = function
  | (Some _ as x) -> Printf.printf "got %s\n" (match x with Some s -> s | None -> "")
  | None -> ()
```

OCaml's `as` in patterns corresponds to Rust's `@`.

## Key Differences

1. **Syntax**: Rust uses `@` (before the pattern); OCaml uses `as` (after the pattern).
2. **Range testing**: Rust `a @ 0..=12` is common with range patterns; OCaml uses `when` guards for numeric ranges.
3. **Nested bindings**: Both support nested `@`/`as` bindings at multiple levels of a pattern.
4. **Use case**: Both languages most commonly use this when you need to both classify and display/log the matched value.

## Exercises

1. **Error with context**: Write `fn check_age(age: u32) -> Result<(), String>` using `a @ 130..=u32::MAX => Err(format!("age {} is unrealistic", a))` to include the invalid value in the error.
2. **Nested @ binding**: Match on `Option<Option<i32>>` using `outer @ Some(inner @ Some(v))` and format all three bindings in the result string.
3. **Struct field binding**: Write `match point { p @ Point { x, y } if x > 0 && y > 0 => format!("{:?} is in Q1", p), ... }` — bind the whole struct while also checking individual fields.
