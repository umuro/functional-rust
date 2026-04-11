📖 **[View on hightechmind.io →](https://hightechmind.io/rust/072-result-railway)**

---

# 072 — Railway-Oriented Programming

## Problem Statement

Railway-oriented programming (ROP), coined by Scott Wlaschin in his 2014 talk and blog series, is a visual metaphor for monadic error handling. Picture a two-track railway: the "happy track" carries `Ok` values forward; the "error track" carries `Err` values that bypass all remaining steps. Each processing function is a switch: if the input arrives on the happy track and the function succeeds, the output stays on the happy track; if the function fails, the output switches to the error track and stays there for all subsequent steps.

This pattern, implemented via `and_then` chaining, produces code that reads top-to-bottom as a linear sequence of steps — exactly like procedural code with exceptions, but without hidden control flow. The `?` operator in Rust is syntactic sugar for `and_then` applied to the current function's result type. Used in F#, Rust, Scala, Haskell, and any language where `Result`/`Either` is the primary error-handling mechanism, this pattern scales from simple validation pipelines to multi-step business logic with complex failure modes.

## Learning Outcomes

- Chain validation functions with `and_then` to form a railway
- Understand that each `and_then` step runs only on the happy track
- Use `validate_item`, `validate_quantity`, `validate_price` as railway switches
- Implement railway-style pipelines using both explicit `and_then` and `?`
- Recognize that the railway metaphor explains `Result` monad behavior

## Rust Application

Each validation step takes an `Order` and returns `Result<Order, String>`:
- `validate_quantity` checks that quantity > 0
- `validate_price` checks that price > 0.0
- `validate_item` checks that the item name is non-empty

`validate_order_chain` composes them with `.and_then(validate_quantity).and_then(validate_price).and_then(validate_item)` — a left-to-right railway. `validate_order_question` achieves the same with `?`, which is more readable for deep pipelines. Both are semantically identical: the first failure short-circuits everything after it.

## OCaml Approach

OCaml's railway uses `Result.bind` or custom operators:

```ocaml
let validate_order order =
  let ( >>= ) = Result.bind in
  validate_item order
  >>= validate_quantity
  >>= validate_price
```

With OCaml 4.08+ binding operators, you can also write `let* o = validate_item order in let* o = validate_quantity o in validate_price o`. Each step receives the validated value from the previous step. The symmetry with Rust's `.and_then` chain is exact.

## Key Differences

1. **Visual metaphor**: The railway metaphor makes `and_then` chains intuitive — you are either on the happy track or the error track. `?` makes the track switch implicit.
2. **Pass-through pattern**: Each validator takes and returns the full order (pass-through). This allows validators to be composed in any order without changing signatures.
3. **Error accumulation vs railway**: The railway pattern stops at the first error. For all-errors-at-once, use the `Validation` pattern (example 072-error-accumulation). Knowing when to use which is key.
4. **`>>=` in OCaml**: Defining `let (>>=) = Result.bind` locally in OCaml makes railway chains very readable: `a >>= f >>= g >>= h`. Rust's method syntax `a.and_then(f).and_then(g)` is equivalent.

## Exercises

1. **Enriched errors**: Change the error type to `struct ValidationError { field: String, message: String }`. Update the validators to include the field name. This enables structured error reporting.
2. **Parallel validation**: Combine the railway approach (sequential) with applicative validation (parallel): validate all fields simultaneously and accumulate errors, then construct the Order only if all pass.
3. **Undo railway**: Write a `rollback` function for a sequence of operations: if step 3 fails, undo steps 1 and 2. Model this with a stack of undo actions.
