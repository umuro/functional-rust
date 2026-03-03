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
