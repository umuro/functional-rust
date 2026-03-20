📖 **[View on hightechmind.io →](https://hightechmind.io/rust/548-lifetime-named-return)**

---

# Named Return Lifetimes

## Problem Statement

When a function returns a reference, the lifetime of that reference must be tied to one of its inputs. In simple cases, elision handles this automatically. But when functions have multiple reference parameters and it is important to document or enforce which input the output borrows from, explicit named lifetimes in the return type make the relationship unambiguous. Named return lifetimes are especially valuable in parser structs, view adapters, and any API where the relationship between input source and output view matters for correctness.

## Learning Outcomes

- How named output lifetimes like `'out` and `'input` clarify which input a reference comes from
- How `prefer_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str` documents the output source
- How `Parser<'input>` uses a named lifetime to tie parsed output to the input buffer
- When naming lifetimes with semantically meaningful names (`'input`, `'source`, `'out`) helps readability
- The difference between elided lifetimes and explicitly named ones in terms of semantics

## Rust Application

`first<'out>(items: &'out [i32]) -> Option<&'out i32>` names the output lifetime `'out` explicitly — functionally identical to elision but more self-documenting. `prefer_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str` names two input lifetimes and shows the output is tied to the first. `Parser<'input>` stores `data: &'input str` and `parse(&self) -> &'input str` returns data with the stored `'input` lifetime — longer-lived than `&self` itself.

Key patterns:
- Semantically named lifetimes: `'input`, `'source`, `'out` vs generic `'a`, `'b`
- `fn parse(&self) -> &'input str` — output outlives the method borrow of `self`
- Two-input functions explicitly documenting which input the output borrows from

## OCaml Approach

OCaml function return types carry no lifetime annotation. The relationship between input and output references is a convention expressed through documentation:

```ocaml
type 'input parser = { data: 'input }
let parse p = String.trim p.data  (* always valid — GC-managed *)
```

## Key Differences

1. **Documentation vs enforcement**: Rust named return lifetimes both document and enforce which input is borrowed; OCaml documentation comments describe the relationship with no enforcement.
2. **`'input` vs `'a`**: Using `'input` as a lifetime name is purely cosmetic in Rust — it carries no additional meaning beyond `'a` to the type checker, but significantly improves human readability.
3. **Parser struct lifetimes**: Rust `Parser<'input>` enforces that parsed results cannot outlive the input buffer; OCaml parsers hold GC-managed strings with no lifetime constraint.
4. **Compiler verification**: Rust verifies that `-> &'out i32` actually comes from the `'out`-annotated input; OCaml relies on programmer discipline and testing.

## Exercises

1. **Named lifetime parser**: Implement `struct Tokenizer<'src> { source: &'src str, pos: usize }` with a method `fn next_token(&mut self) -> Option<&'src str>` that returns a slice of `source`.
2. **Two-input choose**: Write `fn choose<'long: 'short, 'short>(cond: bool, a: &'long str, b: &'short str) -> &'short str` — explain why `'long: 'short` is needed here.
3. **Lifetime documentation**: Take any three functions from earlier examples and rewrite them with semantically named lifetimes (`'input`, `'key`, `'value`) instead of `'a`/`'b` — assess whether readability improves.
