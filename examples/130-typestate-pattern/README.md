📖 **[View on hightechmind.io →](https://hightechmind.io/rust/130-typestate-pattern)**

---

# Typestate Pattern — State Machines in Types
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

State machines govern everything from network connections to file handles to UI workflows. Typically, invalid transitions (locking an open door, reading from a closed file, sending on a disconnected socket) fail at runtime with errors or panics. The typestate pattern encodes each valid state as a distinct type, so invalid transitions become compile errors. This eliminates entire classes of runtime bugs with zero overhead — the state information disappears at compile time.

## Learning Outcomes

- Understand how phantom type parameters encode state without runtime cost
- Learn to implement consuming state transitions that prevent reuse of the old state
- See how methods are selectively available only in the correct state
- Recognize real-world applications: connection lifecycle, file handles, builder patterns, protocol sequences

## Rust Application

`Door<Open>`, `Door<Closed>`, and `Door<Locked>` are three different types — the compiler tracks which state a door is in. Each `impl Door<State>` block defines only the methods valid in that state: `walk_through` exists only on `Door<Open>`, `lock` only on `Door<Closed>`. State transitions like `close(self) -> Door<Closed>` consume the old state — after calling `close`, the `Door<Open>` value is gone, preventing any access through a closed door.

## OCaml Approach

OCaml can encode the typestate pattern using phantom types and GADTs:
```ocaml
type open_ = Open
type closed = Closed
type 'state door = { material: string }
let close : open_ door -> closed door = fun d -> d
let open_ : closed door -> open_ door = fun d -> d
```
The transition functions change the phantom parameter. OCaml's approach is syntactically lighter, but state transitions do not consume the old value — the programmer must not use the old binding after transitioning (enforced by convention, not the type system).

## Key Differences

1. **Affine types**: Rust's move semantics ensure the old state value cannot be used after a transition; OCaml retains the old binding (GC-managed), relying on programmer discipline.
2. **Method availability**: Rust's `impl` blocks per state restrict which methods exist on each type; OCaml typically uses a single module with runtime guards or phantom-typed functions.
3. **Zero-sized states**: Both `Open`, `Closed`, `Locked` in Rust and their OCaml equivalents are zero-sized; `PhantomData` in Rust explicitly marks the field as carrying no data.
4. **Composability**: Rust typestate works for any number of state dimensions (multiple phantom parameters); OCaml's phantom approach scales similarly but with more boilerplate.

## Exercises

1. Add a `Broken` state to the door and define `break_open(self) -> Door<Broken>` callable only from `Door<Locked>`.
2. Implement a `TcpConnection` typestate with `Unconnected`, `Connected`, and `Closed` states and appropriate methods on each.
3. Build a three-stage builder using typestate: `Builder<NoName>` → `Builder<Named>` → `Builder<Complete>`, where `build()` is only available on `Builder<Complete>`.
