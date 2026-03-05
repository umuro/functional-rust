# OCaml vs Rust: Shared References (&T)

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: immutable by default — all bindings are effectively "shared reads" *)
let string_info s =
  let len = String.length s in
  let upper = String.uppercase_ascii s in
  Printf.printf "String '%s' has length %d, upper: %s\n" s len upper;
  len

let () =
  let msg = "hello world" in
  let len1 = string_info msg in
  let len2 = string_info msg in   (* still available — OCaml never moves values *)
  assert (len1 = len2)

(* Multiple readers of the same list *)
let sum_list  lst = List.fold_left ( + ) 0 lst
let max_list  lst = List.fold_left max min_int lst
let min_list  lst = List.fold_left min max_int lst

let stats lst = (sum_list lst, max_list lst, min_list lst)
```

### Rust (idiomatic — shared borrows, `&T`)
```rust
// &str borrows a String without taking ownership
pub fn string_info(s: &str) -> usize {
    s.len()
}

pub fn sum_slice(data: &[i32]) -> i32  { data.iter().sum() }
pub fn max_slice(data: &[i32]) -> Option<i32> { data.iter().copied().reduce(i32::max) }
pub fn min_slice(data: &[i32]) -> Option<i32> { data.iter().copied().reduce(i32::min) }

// Three simultaneous shared borrows — all legal because none mutate
pub fn stats(data: &[i32]) -> (i32, Option<i32>, Option<i32>) {
    (sum_slice(data), max_slice(data), min_slice(data))
}
```

### Rust (lifetime-annotated — explicit shared lifetimes)
```rust
// 'a ties both inputs and the output to the same lifetime
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Read-only string param | `string -> int` (value copy / GC-managed) | `fn f(s: &str) -> usize` |
| Read-only list/slice | `int list -> int` | `fn f(data: &[i32]) -> i32` |
| Shared lifetime | implicit (GC) | `fn f<'a>(a: &'a str, b: &'a str) -> &'a str` |
| Optional result | `'a option` | `Option<T>` |

## Key Insights

1. **Immutability is the default in OCaml; borrowing is the mechanism in Rust.** OCaml values are immutable by default, so multiple readers coexist naturally under GC. Rust achieves the same safety without GC by tracking ownership and borrows at compile time.

2. **`&T` is a zero-cost compile-time read-lock.** While any `&T` exists, the compiler prevents any `&mut T` from existing. Iterator invalidation, data races, and use-after-free are ruled out structurally — not by runtime checks.

3. **Slices (`&[T]`) are the Rust equivalent of OCaml lists for read-only access.** OCaml's `list` is a persistent, GC-managed structure. Rust's `&[T]` is a fat pointer (data + length) that borrows any contiguous sequence without allocation.

4. **Lifetime annotations make sharing contracts explicit.** OCaml's GC hides lifetimes. Rust's `'a` annotations express "this output lives at least as long as these inputs" — turning implicit GC guarantees into verifiable compiler contracts.

5. **Multiple simultaneous `&T` borrows are always safe.** `sum_slice`, `max_slice`, and `min_slice` can all hold a shared borrow of the same slice at once. The compiler knows none of them can mutate it, so no synchronization or copying is needed.

## When to Use Each Style

**Use `&T` (shared borrow) when:** you need read-only access to data that someone else owns — function parameters, iterator consumers, analysis functions, display/formatting logic.

**Use `&mut T` (exclusive borrow) when:** you need to modify in place — sorting, filling a buffer, updating fields. The compiler ensures no `&T` borrows overlap with `&mut T`.

**Use owned `T` when:** the function logically takes responsibility for the value — constructors, thread spawning, returning data to a new owner.
