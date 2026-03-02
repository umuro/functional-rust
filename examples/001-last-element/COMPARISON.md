# OCaml vs Rust: Last Element of a List

## Side-by-Side Code

### OCaml — idiomatic recursive
```ocaml
let rec last = function
  | []  -> None
  | [x] -> Some x
  | _ :: t -> last t
```

### OCaml — stdlib (`List.rev`)
```ocaml
let last_stdlib lst =
  match List.rev lst with
  | []    -> None
  | h :: _ -> Some h
```

### OCaml — tail-recursive
```ocaml
let last_tail lst =
  let rec aux acc = function
    | []     -> acc
    | h :: t -> aux (Some h) t
  in
  aux None lst
```

---

### Rust — idiomatic (`slice::last`)
```rust
pub fn last<T>(list: &[T]) -> Option<&T> {
    list.last()   // O(1): reads the final index directly
}
```

### Rust — stdlib via iterator
```rust
pub fn last_stdlib<T>(list: &[T]) -> Option<&T> {
    list.iter().last()   // O(n): visits every element
}
```

### Rust — recursive slice patterns
```rust
pub fn last_recursive<T>(list: &[T]) -> Option<&T> {
    match list {
        []             => None,
        [x]            => Some(x),
        [_, rest @ ..] => last_recursive(rest),
    }
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Sequence type | `'a list` (linked list, cons cells) | `&[T]` (slice — contiguous memory) |
| Recursive style | Idiomatic; TCO guaranteed by the compiler | Supported via slice patterns; **no TCO** |
| Stdlib one-liner | `List.rev lst \| List.hd_opt` — O(n) | `slice.last()` — O(1) |
| Null safety | `'a option` | `Option<T>` |
| Memory management | GC | Borrow checker / lifetimes |
| Return type | Owned value (`'a option`) | Borrowed reference (`Option<&T>`) |

---

## Type Signatures

```ocaml
(* OCaml — polymorphic, returns owned value *)
val last : 'a list -> 'a option
```

```rust
// Rust — generic, returns borrowed reference
fn last<T>(list: &[T]) -> Option<&T>
//          ^^^^  borrows     ^ borrows back from input
```

The key difference: OCaml's `Some x` copies (or shares via GC) the value.
Rust's `Some(&x)` is a reference into the original slice — the caller must keep
`list` alive for as long as the returned `Option<&T>` is used.

---

## Slice Patterns vs Cons Patterns

| Feature | OCaml | Rust |
|---------|-------|------|
| Empty | `[]` | `[]` |
| Single element | `[x]` | `[x]` |
| Head + tail | `h :: t` | `[h, rest @ ..]` |
| Last element direct | *(requires recursion)* | `[.., last]` |

Rust slice patterns can match from **either end**, which OCaml cons patterns
cannot — `[.., last]` directly binds the final element with no recursion.

---

## 5 Takeaways

1. **`slice::last()` is O(1) in Rust; the recursive OCaml version is O(n).**
   Slices know their length, so the stdlib call is a single pointer addition.

2. **Rust has no guaranteed tail-call optimisation.**
   The idiomatic answer to "tail-recursive traversal" in Rust is an iterator,
   not explicit recursion.

3. **Rust returns a borrow, OCaml returns a value.**
   `Option<&T>` ties the returned reference's lifetime to the slice. This is
   the borrow checker at work — memory safety without GC.

4. **Slice patterns mirror cons patterns almost 1-to-1.**
   Translating `_ :: t -> last t` to `[_, rest @ ..] => last_recursive(rest)`
   is mechanical, making OCaml → Rust migration of pattern-heavy code readable.

5. **Prefer iteration over recursion in Rust.**
   `list.iter().last()` and `list.last()` compile to tight loops or single
   instructions with no stack growth — the idiomatic preference in a language
   without TCO.
