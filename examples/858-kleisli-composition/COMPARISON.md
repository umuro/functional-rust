# Comparison: Kleisli Composition

## Kleisli Operator

**OCaml:**
```ocaml
let ( >=> ) f g x = f x >>= g

let validate = parse_int >=> check_positive >=> safe_half
let result = validate "42"  (* Some 21 *)
```

**Rust:**
```rust
fn kleisli<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
}

let validate = kleisli(kleisli(parse_int, check_positive), safe_half);
validate("42") // Some(21)
```

## Dynamic Pipeline

**OCaml:**
```ocaml
let pipeline steps x =
  List.fold_left (fun acc step -> acc >>= step) (Some x) steps

let steps = [check_positive; safe_half]
let result = pipeline steps 50  (* Some 25 *)
```

**Rust:**
```rust
fn pipeline(steps: &[fn(i32) -> Option<i32>], x: i32) -> Option<i32> {
    steps.iter().fold(Some(x), |acc, step| acc.and_then(step))
}

let steps: Vec<fn(i32) -> Option<i32>> = vec![check_positive, safe_half];
pipeline(&steps, 50) // Some(25)
```
