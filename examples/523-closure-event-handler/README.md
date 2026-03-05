📖 **[View on hightechmind.io →](https://hightechmind.io/rust/523-closure-event-handler)**

---

# 523: Event Handler Pattern

**Difficulty:** 3  **Level:** Intermediate

A priority-ordered event bus with multiple `FnMut` handlers and propagation control.

## The Problem This Solves

UI frameworks, game engines, and plugin systems all need a way to register multiple callbacks for the same event, run them in a defined order, and let high-priority handlers cancel further processing. A flat `Vec<Box<dyn FnMut(&Event) -> bool>>` gets you halfway there, but you also need priority ordering, named handlers for debugging, and the ability to short-circuit propagation.

This is the classic "event bus" or "signal-slot" pattern. In Rust, the key challenge is that handlers must be `FnMut` (they carry mutable state like `click_count`), must be owned by the bus (so the bus controls their lifetime), and must be type-erased so different handlers can coexist in the same `Vec`. `Box<dyn FnMut>` handles all of this.

## The Intuition

Each handler is a closure stored in a `Box`. The bus sorts them by priority on registration and iterates in order on each event. If a handler returns `true`, the bus stops — that handler "consumed" the event. This mirrors how browser DOM event propagation and GTK signal handling work.

The `move` capture in `click-counter` is the key: the closure owns `click_count`, increments it on each call, and the bus never sees the counter directly. State is private to the closure.

## How It Works in Rust

1. **`Box<dyn FnMut(&Event) -> bool>`** — type-erased, heap-allocated, mutable closure; the bus owns it via `Vec<Handler>`.
2. **Priority enum** — `enum Priority { High, Normal, Low }` with `Ord` derived; `sort_by_key` on registration ensures correct dispatch order.
3. **Propagation control** — return `true` from a handler to stop the bus from calling subsequent handlers; `false` to continue.
4. **Stateful handler** — `move |event| { click_count += 1; ... }` captures the counter by value; the `FnMut` trait allows mutation on each call.
5. **Dispatch loop** — iterate `&mut self.handlers`, call each with `(handler.handle)(event)`, break on first `true`.

## What This Unlocks

- Build plugin and middleware systems where independent components register handlers without knowing about each other.
- Implement priority-based input routing: high-priority handlers (e.g., bounds checking) run before low-priority ones (e.g., logging).
- Encapsulate per-handler state (counters, buffers) privately inside closures — no shared struct needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Heterogeneous callback list | `(event -> bool) list`; GC-managed | `Vec<Box<dyn FnMut(&Event) -> bool>>`; heap-allocated, owned |
| Stateful handlers | Mutable closures via `ref` capture | `FnMut` + `move` capture; state is private to closure |
| Priority ordering | Manual sort or priority queue | `sort_by_key` on `Priority` enum |
| Propagation stop | Return value checked in fold/loop | Return `bool`; bus `break`s on `true` |
