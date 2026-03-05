# 547: Polonius Borrow Checker Concepts

**Difficulty:** 5  **Level:** Advanced

The next-generation borrow checker that eliminates false positives in NLL.

## The Problem This Solves

Rust's current borrow checker (NLL — Non-Lexical Lifetimes) is conservative. In some cases it rejects code that is actually safe. The most famous example is the "get-or-insert" pattern: you try to look up a key in a map, and if it's missing, insert a default. The match arm that *found* the value holds a borrow, but the arm that *didn't find* it (and therefore wants to mutate the map) shouldn't need to hold that borrow. NLL can't see the difference. Polonius can.

Polonius is a new analysis algorithm based on a richer model of "where does this value come from?" rather than "when is this borrow live?" It can prove that the borrow from a successful `get()` doesn't flow into the `None` branch, so the map can be mutated there safely.

Until Polonius ships in stable Rust, you work around NLL's conservatism using patterns like the Entry API (`map.entry(key).or_insert(...)`) or splitting the logic into index-based two-pass operations. Knowing *why* NLL rejects certain code — and that Polonius would accept it — is essential for designing APIs and choosing workarounds that preserve intent.

## The Intuition

NLL thinks in terms of time: "this borrow was created here and is live until here." Polonius thinks in terms of data flow: "this returned value *originates from* this input." The `None` branch never touches the value that `Some(v)` returned, so the borrow that produced `v` simply doesn't exist in that branch under Polonius's model.

Two-phase borrows (already in stable Rust) are a partial version of this idea: the outer mutable borrow is "reserved" while an inner shared borrow completes, enabling `v.push(v.len())` to compile.

## How It Works in Rust

1. **NLL limitation** — `match map.get(&key) { Some(v) => v, None => { map.insert(...); ... } }` is rejected even though the borrow only matters in the `Some` arm.
2. **Entry API workaround** — `map.entry(key).or_insert_with(|| ...)` is the idiomatic stable solution; it encapsulates the get-or-insert in a single operation.
3. **Index-based workaround** — compute the index first (`v.iter().position(...)`), then index into `v` with `&mut v[i]`; NLL accepts this because no borrow straddles the mutation.
4. **Two-phase borrows** — `v.push(v.len())` compiles on stable because the inner `v.len()` is a shared borrow inside a "reserved" mutable borrow.
5. **Flow-sensitive** — branching code where one arm borrows and the other mutates can compile when NLL can prove the borrows don't overlap.

## What This Unlocks

- Understand *why* Rust rejects certain obviously-safe patterns and reach for the right workaround.
- Recognize the boundary between NLL limitations and actual unsafety.
- Write forward-compatible code that will compile without changes when Polonius lands in stable.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Borrow model | No borrow checker; GC handles aliasing | NLL: borrow liveness ranges; Polonius: data-flow origins |
| Get-or-insert | Natural expression; GC prevents dangling | NLL rejects; Entry API or index workaround needed |
| Mutation in match arms | Allowed freely | NLL: conservative; Polonius: branch-aware |
| Two-phase borrows | N/A | Stable in NLL: inner shared borrow inside reserved `&mut` |
