📖 **[View on hightechmind.io →](https://hightechmind.io/rust/564-pattern-enum-variants)**

---

# Enum Variant Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Algebraic data types (ADTs) with sum-type variants are the backbone of functional programming. Enums in Rust are full ADTs: variants can carry no data (unit), named fields (struct-like), or positional fields (tuple-like). Pattern matching on enum variants is exhaustive — the compiler verifies every variant is handled. This makes enums with match the preferred tool for command dispatch, protocol parsing, event handling, and any domain with a finite set of cases. It is the functional alternative to class hierarchies with virtual dispatch.

## Learning Outcomes

- How to match all four kinds of enum variants: unit, struct-like, tuple, and tuple-struct
- How `if let` provides non-exhaustive matching for a single variant of interest
- How `matches!` tests variant membership without extracting data
- How exhaustiveness checking catches missing variants at compile time
- Where enum patterns replace virtual dispatch: command handling, protocol state machines

## Rust Application

`Message` has `Quit` (unit), `Move { x, y }` (struct-like), `Write(String)` (tuple), and `ChangeColor(u8, u8, u8)` (multi-tuple). `process_message` matches all four with different destructuring syntax per variant. `get_write_text` uses `if let Message::Write(text) = msg { Some(text) } else { None }` for single-variant extraction. `is_quit` uses `matches!(msg, Message::Quit)` for a boolean check.

Key patterns:
- `Message::Move { x, y } => ...` — struct variant destructuring
- `Message::Write(text) => ...` — tuple variant destructuring
- `if let Variant(data) = val { ... }` — single-variant binding
- `matches!(val, Variant)` — boolean variant test

## OCaml Approach

OCaml's variant types are the direct equivalent:

```ocaml
type message = Quit | Move of {x: int; y: int} | Write of string | ChangeColor of int * int * int
let process = function
  | Quit -> "Quit"
  | Move {x; y} -> Printf.sprintf "Move to (%d, %d)" x y
  | Write text -> "Write: " ^ text
  | ChangeColor (r, g, b) -> Printf.sprintf "Color: rgb(%d, %d, %d)" r g b
```

## Key Differences

1. **Unit variants**: Rust `Message::Quit` requires the enum name; OCaml `Quit` can be used unqualified in scope.
2. **Inline record variants**: Rust struct-like variants `Move { x, y }` and OCaml inline records `Move of {x: int; y: int}` are equivalent.
3. **`matches!` vs boolean match**: Rust's `matches!` macro; OCaml achieves the same with `(= Quit)` or a `function Quit -> true | _ -> false`.
4. **Exhaustiveness**: Both compilers warn on missing variants; adding a new variant to the enum causes compile warnings in both.

## Exercises

1. **Command parser**: Implement a `Command` enum with `Move(Direction)`, `Look`, `Take(String)`, `Drop(String)`, `Quit` and write `fn execute(cmd: Command, world: &mut World) -> String`.
2. **JSON value**: Build a `JsonValue` enum covering `Null`, `Bool(bool)`, `Number(f64)`, `Str(String)`, `Array(Vec<JsonValue>)`, `Object(Vec<(String, JsonValue)>)` with a `display` function.
3. **From string**: Write `fn parse_message(s: &str) -> Option<Message>` that parses "quit", "move X Y", "write TEXT", "color R G B" into the `Message` enum.
