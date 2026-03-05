(* OCaml trees — GC is transparent in patterns *)
type tree = Leaf | Node of int * tree * tree

let rec depth = function
  | Leaf -> 0
  | Node(_,l,r) -> 1 + max (depth l) (depth r)

let rec insert v = function
  | Leaf -> Node(v,Leaf,Leaf)
  | Node(x,l,r) when v<x -> Node(x, insert v l, r)
  | Node(x,l,r) when v>x -> Node(x, l, insert v r)
  | t -> t

let rec contains v = function
  | Leaf -> false
  | Node(x,_,_) when v=x  -> true
  | Node(x,l,_) when v<x  -> contains v l
  | Node(_,_,r)            -> contains v r

let () =
  let t = List.fold_left (fun a x->insert x a) Leaf [5;3;7;1;4] in
  Printf.printf "depth=%d\n" (depth t);
  Printf.printf "has 3=%b has 6=%b\n" (contains 3 t) (contains 6 t)
