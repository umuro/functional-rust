# OCaml vs Rust: Mutable References (&mut T)

## Side-by-Side Code

### OCaml
```ocaml
(* Mutable record fields — mutation is opt-in per field *)
type counter = { mutable count : int }

let increment c = c.count <- c.count + 1

(* Ref cells — first-class mutable references *)
let sum_into total lst =
  List.iter (fun x -> total := !total + x) lst

(* Mutable arrays — in-place mutation *)
let reverse_in_place arr =
  let n = Array.length arr in
  for i = 0 to n / 2 - 1 do
    let tmp = arr.(i) in
    arr.(i) <- arr.(n - 1 - i);
    arr.(n - 1 - i) <- tmp
  done
```

### Rust (idiomatic)
```rust
pub fn increment(c: &mut Counter) {
    c.count += 1;
}

pub fn sum_into(data: &[i32], total: &mut i32) {
    for &x in data {
        *total += x;
    }
}

pub fn reverse_in_place(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n / 2 {
        arr.swap(i, n - 1 - i);
    }
}
```

### Rust (iterator style)
```rust
pub fn double_all(values: &mut Vec<i32>) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable struct field | `type t = { mutable x : int }` | `struct T { x: i32 }` + `&mut T` |
| Ref cell | `int ref` (`ref 0`, `!r`, `:=`) | `&mut i32` (explicit, checked) |
| Mutable slice | `int array` (always mutable) | `&mut [i32]` (exclusive borrow) |
| Mutation operator | `<-` (fields), `:=` (refs) | `*r = ...` or `field = ...` |

## Key Insights

1. **Exclusivity is enforced at compile time.** Rust's borrow checker statically guarantees that `&mut T` is unique — no two mutable aliases can coexist. OCaml's `ref` cells and mutable fields allow aliasing freely; the programmer bears responsibility for avoiding races.

2. **OCaml mutation is opt-in per field; Rust mutation is opt-in per binding.** In OCaml you mark individual record fields `mutable`. In Rust you declare a binding `let mut x` and pass `&mut x` — the mutability travels with the reference, not the type declaration.

3. **Rust `&mut` replaces OCaml `ref` without allocation.** An OCaml `ref` is a heap-allocated box. Rust `&mut i32` is a stack reference — zero overhead. The borrow checker makes this safe where OCaml needs garbage collection to manage ref-cell lifetimes.

4. **`iter_mut()` is the Rust analogue of `List.iter` with mutation.** OCaml's `List.iter (fun x -> total := !total + x)` reads `x` but writes through the captured `ref`. Rust's `iter_mut()` hands out `&mut` to each element directly, keeping all mutation explicit and checked.

5. **No data races by construction.** While `&mut T` exists, the borrow checker forbids any shared `&T` references. This rule is the compile-time equivalent of a mutex — enforced without runtime cost and impossible to forget.

## When to Use Each Style

**Use `&mut T` (Rust) when:** you need to mutate a value in place without transferring ownership — counters, accumulators, in-place sorting, filling buffers. The exclusivity guarantee means you never need a lock for single-threaded code.

**Use `ref` / mutable fields (OCaml) when:** you need shared mutable state across closures or callbacks within a single-threaded context. OCaml's GC manages lifetimes so aliasing is safe (single-threaded), but you lose the static race-freedom guarantee Rust provides.
