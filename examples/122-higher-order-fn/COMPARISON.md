# OCaml vs Rust: Higher-Order Functions with Lifetime Constraints

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml HOFs need no lifetime annotations — GC handles memory *)
let find_first pred lst =
  try Some (List.find pred lst)
  with Not_found -> None

let compose f g x = f (g x)

let apply_n f n init =
  let rec loop acc k = if k = 0 then acc else loop (f acc) (k - 1)
  in loop init n

let () =
  let data = ["apple"; "banana"; "cherry"; "date"] in
  assert (find_first (fun s -> String.length s > 5) data = Some "banana");
  let double_then_add = compose (( + ) 1) (( * ) 2) in
  assert (double_then_add 5 = 11);
  assert (apply_n (( * ) 2) 3 1 = 8);
  print_endline "ok"
```

### Rust (idiomatic — owned types, no lifetimes)
```rust
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))
}

pub fn apply_n<T, F>(f: F, n: usize, init: T) -> T
where
    F: Fn(T) -> T,
{
    (0..n).fold(init, |acc, _| f(acc))
}
```

### Rust (with lifetime annotations — reference-returning HOFs)
```rust
// 'a ties the output reference lifetime to the input slice lifetime
pub fn find_first<'a, F>(items: &'a [&'a str], pred: F) -> Option<&'a str>
where
    F: Fn(&str) -> bool,
{
    items.iter().copied().find(|&s| pred(s))
}

pub fn apply_to_str<'a, F>(s: &'a str, f: F) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(s)
}

pub fn filter_refs<'a, T, F>(items: &'a [T], pred: F) -> Vec<&'a T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| pred(x)).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| HOF with predicate | `('a -> bool) -> 'a list -> 'a option` | `fn find_first<'a, F>(items: &'a [&'a str], pred: F) -> Option<&'a str>` |
| Function composition | `('b -> 'c) -> ('a -> 'b) -> 'a -> 'c` | `fn compose<A,B,C,F,G>(f: F, g: G) -> impl Fn(A) -> C` |
| Repeated application | `('a -> 'a) -> int -> 'a -> 'a` | `fn apply_n<T, F>(f: F, n: usize, init: T) -> T` |
| Reference in output | implicit (GC) | `'a` lifetime annotation required |

## Key Insights

1. **Lifetime annotations are only required when returning references derived from inputs.** When HOFs return owned values (`String`, `Vec<T>`, `i32`), no lifetime annotation is needed — `compose` and `apply_n` are annotation-free.

2. **OCaml's GC makes lifetimes implicit.** The garbage collector tracks reference liveness automatically. Rust requires the programmer to encode the same information as lifetime parameters so the borrow checker can verify safety at compile time.

3. **`'a` is a promise, not a restriction.** Writing `fn find_first<'a>(items: &'a [&'a str]) -> Option<&'a str>` doesn't impose a fixed lifetime — it tells the compiler "whatever lifetime the caller provides, the output won't outlive it." The caller determines the actual lifetime.

4. **Elision hides common cases.** In `filter_refs`, a single input reference determines the output lifetime, so Rust could elide the annotation. Explicit `'a` is used here for clarity and teaching purposes.

5. **Iterator combinators replace recursive OCaml patterns.** OCaml's `List.find` is a recursive HOF. In Rust, `.iter().find()` is an iterator adapter — same concept, different mechanism, zero-cost abstraction with no allocation.

## When to Use Each Style

**Use annotation-free HOFs when:** your higher-order functions work on owned data or produce owned output. `compose`, `apply_n`, and closures over `String`/`Vec` need no lifetime syntax.

**Use explicit lifetime annotations when:** a HOF accepts a reference and returns a reference derived from it — `find_first`, `apply_to_str`, `filter_refs`. The annotation is the compiler's proof that the output won't outlive its source.
