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
  assert (factorial 10 = 3628800);
  assert (factorial_cps 10 = 3628800);
  assert (factorial_cps 0 = 1);
  assert (factorial_cps 5 = 120);
  let t = Node (Node (Leaf 1, Leaf 2), Node (Leaf 3, Leaf 4)) in
  assert (sum_cps t = 10);
  assert (sum_cps (Leaf 42) = 42);
  Printf.printf "%d\n" (factorial_cps 10);
  Printf.printf "%d\n" (sum_cps t);
  print_endline "ok"
