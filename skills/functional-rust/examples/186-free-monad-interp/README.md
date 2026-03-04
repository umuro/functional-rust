# 186: Free Monad Interpreter — Separate DSL from Execution

**Difficulty:** Expert  **Level:** 4

Define a key-value store as a pure description of operations, then run that same description against multiple backends — in-memory HashMap, pure list, or anything else.

## The Problem This Solves

You have code that reads and writes a key-value store. In production you use a HashMap (or Redis, or a database). In tests you want a clean slate every time, easy to inspect, no teardown needed. In some contexts you might want an immutable "pure" store for determinism or replay.

The naive approach: parameterize everything with a `&mut HashMap`. Tests write to a real HashMap (fine), production writes to the same HashMap (fine) — but what if you later want a different backend? What if you want to run the same sequence of operations twice, once against each backend, and compare results? What if you want to serialize a sequence of operations and replay it later?

You end up with one of these bad outcomes:
1. **Concrete coupling**: functions take `&mut HashMap` directly — swapping backends means changing every function signature
2. **Trait injection**: functions take `&mut dyn Store` — better, but now side effects are baked into function calls, not separable from logic
3. **Test doubles**: separate mock implementations — diverge from real code over time

The Free Monad approach separates _what operations to perform_ (the program, as data) from _how to perform them_ (the interpreter, as a function). The program is built once. You run it with any interpreter. The same description of "put name=Alice, put age=30, get name" gets executed by the HashMap interpreter in production and by the pure list interpreter in tests. No coupling. This is exactly that pain solved.

## The Intuition

Think of a cooking recipe again — but this time it's a shopping + cooking script:

```
1. Put "flour" in pantry
2. Put "eggs" in fridge
3. Get "flour" from pantry
```

That script is just data. You could "run" it in your actual kitchen (HashMap interpreter: physical storage). Or you could "run" it as a paper simulation (pure list interpreter: just track what's in a list). The script doesn't care. It says _what_, not _where_ or _how_.

In code, the "script" is a `Free<A>` value — a tree of `StoreF` instructions linked by continuations. Here's what the instructions look like:

```rust
enum StoreF<A> {
    Get(String, Box<dyn FnOnce(Option<String>) -> A>),  // "fetch key, pass result to A"
    Put(String, String, A),                              // "store key=value, continue with A"
    Delete(String, A),                                   // "remove key, continue with A"
}
```

And the free monad wraps them:

```rust
enum Free<A> {
    Pure(A),                    // "I'm done, here's the value"
    Free(Box<StoreF<Free<A>>>), // "do this instruction, then continue"
}
```

`bind` threads the result of one instruction into the next. Smart constructors (`get`, `put`, `delete`) build individual instruction nodes. You chain them with `bind` to build the full program tree.

## How It Works in Rust

**Step 1: Smart constructors**

```rust
fn get(key: impl Into<String>) -> Free<Option<String>> {
    Free::Free(Box::new(StoreF::Get(
        key.into(),
        Box::new(|v| Free::Pure(v)),  // continuation: wrap result in Pure
    )))
}

fn put(key: impl Into<String>, val: impl Into<String>, next: Free<()>) -> Free<()> {
    Free::Free(Box::new(StoreF::Put(key.into(), val.into(), next)))
}
```

**Step 2: Build a program (no execution yet)**

```rust
fn build_program() -> Free<Option<String>> {
    // put "name" = "Alice", put "age" = "30", then get "name"
    bind(put("name", "Alice", pure_val(())), |_| {
        bind(put("age", "30", pure_val(())), |_| {
            get("name")  // result: Some("Alice")
        })
    })
}
```

Nothing happens here. You get back a `Free<Option<String>>` — a data structure.

**Step 3: Interpreter 1 — real HashMap**

```rust
fn run_memory<A>(tbl: &mut HashMap<String, String>, program: Free<A>) -> A {
    match program {
        Free::Pure(x) => x,  // done
        Free::Free(instr) => match *instr {
            StoreF::Get(k, cont) => {
                let val = tbl.get(&k).cloned();  // real HashMap lookup
                run_memory(tbl, cont(val))
            }
            StoreF::Put(k, v, next) => {
                tbl.insert(k, v);  // real HashMap insert
                run_memory(tbl, next)
            }
            StoreF::Delete(k, next) => {
                tbl.remove(&k);    // real HashMap remove
                run_memory(tbl, next)
            }
        },
    }
}
```

**Step 4: Interpreter 2 — pure association list**

```rust
fn run_pure<A>(store: Vec<(String, String)>, program: Free<A>) -> (A, Vec<(String, String)>) {
    match program {
        Free::Pure(x) => (x, store),  // done — return value AND final state
        Free::Free(instr) => match *instr {
            StoreF::Get(k, cont) => {
                // Look up in the Vec — no mutation of external state
                let val = store.iter().find(|(ck, _)| ck == &k).map(|(_, v)| v.clone());
                run_pure(store, cont(val))
            }
            StoreF::Put(k, v, next) => {
                // Build a new Vec with the value added — immutable style
                let mut new_store: Vec<_> = store.into_iter().filter(|(ck,_)| ck != &k).collect();
                new_store.push((k, v));
                run_pure(new_store, next)
            }
            StoreF::Delete(k, next) => {
                let new_store: Vec<_> = store.into_iter().filter(|(ck,_)| ck != &k).collect();
                run_pure(new_store, next)
            }
        },
    }
}
```

**Step 5: Same program, both interpreters**

```rust
// Both produce Some("Alice") — same program, different execution
let mut tbl = HashMap::new();
let r1 = run_memory(&mut tbl, build_program());

let (r2, _store) = run_pure(vec![], build_program());

assert_eq!(r1, r2);  // same logical result
```

## What This Unlocks

- **Backend independence**: The same program description runs against any interpreter — HashMap, SQLite, a remote API, a mock — without changing the program itself. Adding a new backend is adding a new interpreter function, nothing else.
- **Deterministic testing**: The pure list interpreter produces the same result every time, returns the final store state for inspection, and needs no teardown. Perfect for property-based testing.
- **Replay and audit**: Because the program is data, you could serialize it, store it, and replay it later — building an audit log or a time-travel debugger from the same structure.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Generic free monad | `type 'a free` works over any functor | Must specialize: `Free<A>` with `StoreF` baked in |
| Higher-kinded abstraction | `Free(F(Free t))` for any `F` | Encode `F` as a concrete enum |
| Continuations in instructions | Closures, garbage collected | `Box<dyn FnOnce>` — heap, explicit lifetime |
| Interpreter | Recursive `match` | Recursive `match` — same structure |
| Pure interpreter return | `(A, store)` tuple | Same: returns `(A, Vec<...>)` |
