(* Continuation Passing Style (CPS) *)
(* Transform functions to continuation-passing style *)

(* Direct style *)
let rec factorial n =
  if n <= 1 then 1 else n * factorial (n - 1)

(* CPS style — always tail-recursive *)
let factorial_cps n =
  let rec aux n k =
    if n <= 1 then k 1
    else aux (n - 1) (fun result -> k (n * result))
  in aux n Fun.id

(* CPS tree traversal *)
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec sum_cps t k = match t with
  | Leaf -> k 0
  | Node (l, v, r) ->
    sum_cps l (fun sl -> sum_cps r (fun sr -> k (sl + v + sr)))

let () = Printf.printf "5! = %d\n" (factorial_cps 5)
