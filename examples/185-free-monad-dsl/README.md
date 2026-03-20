📖 **[View on hightechmind.io →](https://hightechmind.io/rust/185-free-monad-dsl)**

---

# Console DSL with Free Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Building on the free monad introduction, this example defines a richer Console DSL with `Print`, `ReadLine`, and `Exit` operations. The key insight is that the DSL operations are pure data — `print_line("Hello")` does not print anything, it constructs a value describing the print operation. This makes programs inspectable, serializable, and interpretable in multiple contexts: unit tests simulate I/O without touching the console.

## Learning Outcomes

- Define a domain-specific language as a free monad over Console operations
- Implement `bind` (sequencing) for the free monad DSL
- See how DSL programs are constructed compositionally and interpreted separately
- Understand the "program as data" metaphor: a free monad program is a tree you can traverse

## Rust Application

`Console<A>` encodes three operations. `print_line(msg)` returns `Console::Print(msg, Box::new(|| Console::Pure(())))`. `read_line()` returns `Console::ReadLine(Box::new(|input| Console::Pure(input)))`. Programs are built by nesting: `print_line("?").bind(|_| read_line().bind(|name| print_line(&format!("Hello, {}", name))))`. The `bind` method connects the continuation, building the program tree. The interpreter `run` pattern-matches and executes each step.

## OCaml Approach

OCaml's `let*` desugaring makes DSL programs natural:
```ocaml
let program =
  let* () = print_line "Enter name:" in
  let* name = read_line () in
  print_line (Printf.sprintf "Hello, %s!" name)
```
This is syntactically cleaner than Rust's closure-based chaining. OCaml's type inference handles the `Console<unit>` / `Console<string>` types without annotation.

## Key Differences

1. **Bind implementation**: Both implement monadic bind by substituting `Pure(a)` with `f(a)` recursively; this is O(n) for each bind — tree re-traversal.
2. **`Exit` handling**: The `Exit(i32)` operation terminates the interpreter loop; both languages handle this as a special case that skips the continuation.
3. **Continuation boxing**: Rust uses `Box<dyn FnOnce>` for continuations; OCaml uses plain function values (heap-allocated, GC-managed).
4. **Syntactic sugar**: OCaml's `let*` requires a `binding_op` declaration; Rust's equivalent would require proc macros or manual `bind` chaining.

## Exercises

1. Add a `Prompt(question: String, continuation: Box<dyn FnOnce(String) -> Console<A>>)` operation that prints the question and reads the answer.
2. Implement a pure test runner that takes `Vec<String>` as simulated input and returns `Vec<String>` as captured output.
3. Write a `dry_run` interpreter that lists all operations the program would perform without executing any I/O.
