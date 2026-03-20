📖 **[View on hightechmind.io →](https://hightechmind.io/rust/574-pattern-if-let)**

---

# if-let and while-let
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

`match` requires handling all cases, which is verbose when you only care about one. `if let` provides single-arm matching: execute a block only when the pattern matches, with an optional `else` for the non-matching case. `while let` loops while a pattern continues to match, perfect for draining queues, popping stacks, and processing iterators. These constructs are ubiquitous in real Rust code — understanding them and knowing when to prefer them over `match` is essential.

## Learning Outcomes

- How `if let Some(n) = opt { ... }` binds and branches in one expression
- How `if let` chains work and when to prefer them over `match`
- How `while let Some(x) = queue.pop() { ... }` processes collections until empty
- How `if let` handles enum variants, `Result`, and custom patterns
- When to use `matches!` vs `if let` vs `match` for different use cases

## Rust Application

`describe_option(opt: Option<i32>)` uses `if let Some(n) = opt { ... } else { ... }`. `categorize` uses nested `if let` with further conditions. `while let` examples drain a `VecDeque` or pop a `Vec`: `while let Some(item) = queue.pop_front() { process(item); }`. `if let` also works with `Result`: `if let Ok(value) = parse_result { ... }`.

Key patterns:
- `if let Some(x) = opt { ... } else { ... }` — conditional binding
- `while let Some(x) = v.pop() { ... }` — drain-while loop
- `if let Variant(data) = val { ... }` — enum variant check with extraction
- Chaining: `if let Some(n) = opt { if n > 0 { ... } }` — nested conditions

## OCaml Approach

OCaml uses `match` with a single relevant arm:

```ocaml
(* if let equivalent *)
let describe_option opt =
  match opt with
  | Some n -> Printf.sprintf "Got: %d" n
  | None -> "Nothing"

(* while let equivalent: functional recursion *)
let rec drain queue =
  match Queue.pop_opt queue with
  | None -> ()
  | Some item -> process item; drain queue
```

## Key Differences

1. **Conciseness**: `if let` is more concise than `match` for single-arm cases; OCaml's `match` requires both arms but is equally concise.
2. **while let**: Rust's `while let` is idiomatic for draining collections; OCaml uses recursive functions or `while` loops with mutable state.
3. **let chains**: Rust (nightly) supports `if let A = x && let B = y { ... }` for chaining pattern checks; OCaml uses `match` nesting or `Option.bind`.
4. **`matches!` macro**: Rust's `matches!(val, Pattern)` is the most concise boolean check; OCaml uses a helper function or `match val with Pat -> true | _ -> false`.

## Exercises

1. **Stack drain**: Write `fn collect_stack(v: &mut Vec<i32>) -> Vec<i32>` using `while let Some(x) = v.pop()` to collect elements in reverse order.
2. **Chained if-let**: Write a validation function using three chained `if let` checks that extract values from nested `Option<Option<i32>>` — compare with a `let-else` version.
3. **Enum filter**: Write `fn keep_moves(events: Vec<Event>) -> Vec<(i32, i32)>` using `if let Event::Move { x, y } = e { ... }` in a loop to collect only move events.
