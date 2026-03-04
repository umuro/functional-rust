Convert this OCaml example to idiomatic Rust.

Directory: examples/268-continuation-passing-style/

## OCaml source
```ocaml
(* Direct style - not tail recursive *)
let rec factorial n =
  if n = 0 then 1 else n * factorial (n - 1)

(* CPS style - tail recursive *)
let factorial_cps n =
  let rec go n k =
    if n = 0 then k 1
    else go (n - 1) (fun result -> k (n * result))
  in
  go n Fun.id

(* CPS tree sum *)
type 'a tree = Leaf of 'a | Node of 'a tree * 'a tree

let sum_cps t =
  let rec go t k = match t with
    | Leaf x -> k x
    | Node (l, r) -> go l (fun sl -> go r (fun sr -> k (sl + sr)))
  in go t Fun.id

let () =
  Printf.printf "%d\n" (factorial_cps 10);
  let t = Node (Node (Leaf 1, Leaf 2), Node (Leaf 3, Leaf 4)) in
  Printf.printf "%d\n" (sum_cps t)
```

## Topic
Converting recursive functions to continuation-passing style for tail recursion. Demonstrates closures as continuations, CPS transform on trees.

Read CLAUDE.md in this directory — it defines all quality standards, file structure, and self-verification steps. Follow it exactly.

When done, report:
DONE — 268-continuation-passing-style — cargo fmt ✓ clippy ✓ test ✓ [N tests passed]
