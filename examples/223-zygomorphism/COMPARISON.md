# Comparison: Example 223 — Zygomorphism

## zygo Definition

### OCaml
```ocaml
let rec zygo_both helper main (Fix f) =
  let paired = map_f (fun child -> zygo_both helper main child) f in
  let b_layer = map_f snd paired in
  (main paired, helper b_layer)
```

### Rust
```rust
fn zygo_both<A: Clone, B: Clone>(
    helper: &dyn Fn(ExprF<B>) -> B,
    main: &dyn Fn(ExprF<(A, B)>) -> A,
    fix: &Fix,
) -> (A, B) {
    let paired = fix.0.map_ref(|child| zygo_both(helper, main, child));
    let b_layer = paired.map_ref(|(_, b)| b.clone());
    (main(paired.clone()), helper(b_layer))
}
```

## Pretty Print with Precedence

### OCaml
```ocaml
let prec_helper = function LitF _ -> 100 | AddF _ -> 1 | MulF _ -> 2 | NegF _ -> 3

let show_main = function
  | MulF ((a, pa), (b, pb)) ->
    let la = if pa < 2 then "(" ^ a ^ ")" else a in
    la ^ " * " ^ (if pb < 2 then "(" ^ b ^ ")" else b)
```

### Rust
```rust
fn prec_helper(e: ExprF<u32>) -> u32 {
    match e { ExprF::LitF(_) => 100, ExprF::AddF(..) => 1, ExprF::MulF(..) => 2, ExprF::NegF(_) => 3 }
}

fn show_main(e: ExprF<(String, u32)>) -> String {
    match e {
        ExprF::MulF((a, pa), (b, pb)) => {
            let la = if pa < 2 { format!("({a})") } else { a };
            format!("{la} * {}", if pb < 2 { format!("({b})") } else { b })
        }
        ...
    }
}
```
