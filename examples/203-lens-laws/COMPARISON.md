# Comparison: Example 203 — Lens Laws

## Law Definitions

### OCaml
```ocaml
(* GetSet: set (get s) s = s *)
let check_get_set lens s =
  lens.set (lens.get s) s = s

(* SetGet: get (set a s) = a *)
let check_set_get lens a s =
  lens.get (lens.set a s) = a

(* SetSet: set b (set a s) = set b s *)
let check_set_set lens a b s =
  lens.set b (lens.set a s) = lens.set b s
```

### Rust
```rust
fn check_get_set<S: PartialEq + Clone, A: Clone>(lens: &Lens<S, A>, s: &S) -> bool {
    let a = (lens.get)(s);
    (lens.set)(a, s) == *s
}

fn check_set_get<S: Clone, A: PartialEq + Clone>(lens: &Lens<S, A>, a: A, s: &S) -> bool {
    (lens.get)(&(lens.set)(a.clone(), s)) == a
}

fn check_set_set<S: PartialEq + Clone, A: Clone>(lens: &Lens<S, A>, a: A, b: A, s: &S) -> bool {
    (lens.set)(b.clone(), &(lens.set)(a, s)) == (lens.set)(b, s)
}
```

## Unlawful Lens

### OCaml
```ocaml
let bad_lens = {
  get = (fun p -> p.x);
  set = (fun x p -> { x; y = p.y +. 1.0 }); (* side effect! *)
}
```

### Rust
```rust
fn bad_lens() -> Lens<Point, f64> {
    Lens::new(|p| p.x, |x, p| Point { x, y: p.y + 1.0 }) // side effect!
}
```
