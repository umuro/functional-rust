# Comparison: Infinite Iterators

## Repeat

**OCaml:**
```ocaml
let repeat x () = Seq.Cons (x, repeat x)
seq_take 5 (repeat 42)  (* [42; 42; 42; 42; 42] *)
```

**Rust:**
```rust
std::iter::repeat(42).take(5).collect::<Vec<_>>()  // [42, 42, 42, 42, 42]
```

## Cycle

**OCaml:**
```ocaml
let rec cycle lst () =
  let rec from = function
    | [] -> cycle lst ()
    | x :: rest -> Seq.Cons (x, fun () -> from rest)
  in from lst
```

**Rust:**
```rust
[1, 2, 3].iter().copied().cycle().take(7).collect::<Vec<_>>()
```

## Iterate (Successors)

**OCaml:**
```ocaml
let iterate f x =
  let rec aux v () = Seq.Cons (v, aux (f v)) in
  aux x

let doubles = iterate (fun x -> x * 2) 1
```

**Rust:**
```rust
std::iter::successors(Some(1u64), |&x| Some(x * 2))
```

## Unfold

**OCaml:**
```ocaml
let unfold f init =
  let rec aux state () =
    match f state with
    | None -> Seq.Nil
    | Some (value, next) -> Seq.Cons (value, aux next)
  in aux init

let fib = unfold (fun (a,b) -> Some (a, (b, a+b))) (0, 1)
```

**Rust:**
```rust
fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0u64, 1u64);
    std::iter::from_fn(move || {
        let val = state.0;
        state = (state.1, state.0 + state.1);
        Some(val)
    })
}
```
