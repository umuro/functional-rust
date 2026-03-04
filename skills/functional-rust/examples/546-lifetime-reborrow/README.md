# 546: Reborrowing Patterns

**Difficulty:** 4  **Level:** Advanced

Reuse a mutable reference across multiple calls without moving or losing it.

## The Problem This Solves

Rust's ownership rules say you can only have one mutable reference at a time. This sounds like it would make it impossible to pass `&mut T` to a function and then use it again — but that's not true. The key is *reborrowing*: the compiler can create a shorter-lived `&mut T` from an existing one, pass that shorter borrow to a function, and then let the original reference resume once the shorter borrow ends.

Without reborrowing, you would have to restructure your code around indices, clone values, or redesign your APIs. With it, you can write natural, ergonomic code: pass a mutable reference to a helper, then keep using the original reference afterward.

This shows up constantly in real code — sorting a `Vec`, then pushing to it, then reading it back. Or passing `&mut String` to a function that appends, then using the string again. Reborrowing is what makes all of that work silently and safely.

## The Intuition

Think of reborrowing like lending someone your pen to sign one document, then getting it back when they're done. You still own the pen. The loan was temporary and bounded. The compiler tracks exactly when the "loan" ends.

An implicit reborrow happens when you pass `&mut T` where a function expects `&T` — the compiler silently reborrows it as a shared reference for the duration of the call. An explicit reborrow (`&*r` or `&mut *r`) creates a new reference at a specific scope, which you can drop early to free the original.

## How It Works in Rust

1. **Implicit shared reborrow** — pass `&mut T` where `&T` is expected; Rust coerces via `Deref`, borrow ends with the call.
2. **Explicit reborrow** — write `&*r` to create a shared borrow, or `&mut *r` for a shorter mutable borrow; use blocks `{}` to control scope.
3. **Reborrow through Deref** — `&mut String` can reborrow as `&str` via `Deref<Target=str>`; lets you call `&str` APIs without consuming the reference.
4. **Sequential reborrows** — each method call on `r` is a new implicit reborrow; `r.sort()` then `r.push()` both work because each call ends before the next begins.
5. **Split reborrows** — borrow `r.0` and `r.1` independently; the compiler tracks field-level borrows (NLL).

## What This Unlocks

- Write helper functions that take `&mut T` without the caller losing access to the original.
- Chain mutable operations fluently without worrying about the borrow being "consumed."
- Understand why so many Rust APIs that take `&mut` can be called repeatedly in sequence.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable aliasing | Multiple refs to same mutable value allowed | Exactly one `&mut T` at a time; reborrow creates sub-borrow |
| Reference passing | Pass by value or `ref`; no lifetime tracking | Borrow checker tracks reborrow scope precisely |
| Implicit coercion | No coercion model | `&mut T` implicitly reborrows as `&T` when needed |
| Field-level borrows | N/A | Borrow checker understands struct field independence |
