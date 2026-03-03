# Stack Module with Signature: OCaml vs Rust

## The Core Insight
Both OCaml and Rust enforce abstraction boundaries, but through different mechanisms. OCaml uses module types (signatures) to hide implementation details; Rust uses traits. The interesting tension: OCaml's stack is naturally persistent (immutable), while Rust's ownership model makes mutable stacks more idiomatic.

## OCaml Approach
OCaml defines a `module type STACK` with abstract type `'a t`, then implements it as `ListStack` backed by a plain list. The signature hides the fact that `'a t = 'a list` — callers can only use the interface functions. `push` prepends to the list (O(1)), `pop` returns the tail — both operations are naturally persistent because lists are immutable. Exceptions (`raise Empty`) handle error cases, which is the traditional OCaml style before `Option`/`Result` became preferred.

## Rust Approach
Rust uses a `trait Stack` with an associated type `Item` to define the interface. The persistent implementation (`ListStack`) clones the internal Vec on each operation — more expensive than OCaml's list cons, but maintains immutability. The mutable variant (`MutStack`) is what idiomatic Rust code would actually use: `&mut self` methods that modify in place, leveraging ownership to guarantee exclusive access. Rust returns `Option` instead of raising exceptions.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Interface | `module type STACK` | `trait Stack` |
| Abstract type | `type 'a t` | `type Item` (associated type) |
| Error handling | `exception Empty` / `raise` | `Option<T>` / `None` |
| Persistence | Natural (list = immutable) | Requires cloning Vec |
| Mutation | Not idiomatic | `&mut self` methods |
| Encapsulation | Signature hides `'a t = 'a list` | Private fields |

## What Rust Learners Should Notice
- OCaml's persistent stack is O(1) for push/pop because list cons shares the tail — Rust's Vec-based persistent stack is O(n) due to cloning
- Rust's `&mut self` mutable stack is the idiomatic choice when you don't need persistence — ownership guarantees no one else holds a reference
- `Option<T>` in Rust replaces OCaml's exception-based error handling — it forces callers to handle the empty case at compile time
- Traits can have associated types (`type Item`) which are more flexible than OCaml's abstract types in some ways (they participate in type inference)
- The persistent vs mutable tradeoff is a core design decision in Rust that OCaml programmers rarely face

## Further Reading
- [The Rust Book — Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [OCaml Modules](https://cs3110.github.io/textbook/chapters/modules/modules.html)
