# OCaml vs Rust: Tail-Recursive Map with CPS

## Side-by-Side Code

### OCaml
```ocaml
(* Naive — not tail-recursive *)
let rec map_naive f = function
  | [] -> []
  | h :: t -> f h :: map_naive f t

(* Tail-recursive with accumulator + reverse *)
let map_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

(* CPS — tail-recursive, preserves order *)
let map_cps f lst =
  let rec go k = function
    | [] -> k []
    | h :: t -> go (fun rest -> k (f h :: rest)) t
  in go Fun.id lst
```

### Rust (idiomatic)
```rust
pub fn map_idiomatic<T, U>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    list.iter().map(f).collect()
}
```

### Rust (tail-recursive / iterative)
```rust
pub fn map_tr<T, U>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    let mut acc = Vec::with_capacity(list.len());
    for item in list {
        acc.push(f(item));
    }
    acc
}
```

### Rust (CPS)
```rust
pub fn map_cps<T, U: 'static>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    type Cont<U> = Box<dyn FnOnce(Vec<U>) -> Vec<U>>;

    fn go<T, U: 'static>(slice: &[T], f: &dyn Fn(&T) -> U, k: Cont<U>) -> Vec<U> {
        match slice {
            [] => k(Vec::new()),
            [head, tail @ ..] => {
                let mapped = f(head);
                go(tail, f, Box::new(move |mut rest| {
                    rest.insert(0, mapped);
                    k(rest)
                }))
            }
        }
    }

    go(list, &f, Box::new(|v| v))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Naive map | `val map_naive : ('a -> 'b) -> 'a list -> 'b list` | `fn map_naive<T, U>(list: &[T], f: &dyn Fn(&T) -> U) -> Vec<U>` |
| Tail-recursive map | `val map_tr : ('a -> 'b) -> 'a list -> 'b list` | `fn map_tr<T, U>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U>` |
| CPS map | `val map_cps : ('a -> 'b) -> 'a list -> 'b list` | `fn map_cps<T, U: 'static>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U>` |
| Continuation type | `'b list -> 'b list` (implicit closure) | `Box<dyn FnOnce(Vec<U>) -> Vec<U>>` |
| Identity continuation | `Fun.id` | `Box::new(\|v\| v)` |

## Key Insights

1. **CPS is a technique, not a language feature.** OCaml makes it natural because closures are GC-managed and tail calls are optimized. Rust can express CPS but it requires explicit heap allocation (`Box<dyn FnOnce>`) and `'static` lifetime bounds on the output type.

2. **Rust's iterator model replaces the need for CPS in practice.** The entire motivation for CPS map in OCaml — stack safety while preserving order — is solved by `iter().map().collect()` in Rust, which is lazy, stack-safe, and allocation-efficient.

3. **The `'static` bound reveals a real cost.** In OCaml, `map_cps` works with any type. In Rust, the continuation chain requires `U: 'static` because `Box<dyn FnOnce>` captures owned values that must outlive any particular stack frame. This is a genuine expressiveness limitation of CPS in Rust.

4. **Accumulator-and-reverse becomes accumulator-and-push.** OCaml's `map_tr` prepends to a list then reverses because prepend is O(1) on linked lists. Rust's `Vec::push` is amortized O(1), so the accumulator pattern naturally preserves order — no reverse needed.

5. **Performance hierarchy differs.** In OCaml: `map_tr` > `map_cps` > `map_naive` for large lists. In Rust: `map_idiomatic` ≈ `map_tr` >> `map_cps` >> `map_naive`, because CPS heap-allocates a closure per element while iterators compile down to a tight loop.

## When to Use Each Style

**Use idiomatic Rust (`iter().map().collect()`) when:** you want correct, fast, readable code — which is virtually always. This is the default choice for transforming collections.

**Use the iterative/accumulator pattern when:** you need fine-grained control over allocation (e.g., `Vec::with_capacity`) or when the mapping logic has side effects that make iterator chains awkward.

**Use CPS in Rust when:** you're teaching CPS concepts, translating from OCaml/Haskell, or working with tree traversals where CPS genuinely simplifies the control flow (e.g., converting recursive tree walks to iterative ones).
