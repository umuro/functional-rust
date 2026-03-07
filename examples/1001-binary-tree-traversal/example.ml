type 'a tree =
  | Leaf
  | Node of 'a * 'a tree * 'a tree

let rec size = function
  | Leaf           -> 0
  | Node (_, l, r) -> 1 + size l + size r

let rec depth = function
  | Leaf           -> 0
  | Node (_, l, r) -> 1 + max (depth l) (depth r)

let rec mem x = function
  | Leaf           -> false
  | Node (v, l, r) -> v = x || mem x l || mem x r

(* Linear-time preorder using accumulator *)
let preorder t =
  let rec go acc = function
    | Leaf           -> acc
    | Node (v, l, r) -> v :: go (go acc r) l
  in go [] t

(*      4
       / \
      2   5
     / \
    1   3   *)
let t = Node (4, Node (2, Node (1, Leaf, Leaf), Node (3, Leaf, Leaf)), Node (5, Leaf, Leaf))

let () =
  Printf.printf "size     = %d\n" (size t);
  Printf.printf "depth    = %d\n" (depth t);
  Printf.printf "mem 3    = %b\n" (mem 3 t);
  Printf.printf "mem 99   = %b\n" (mem 99 t);
  Printf.printf "preorder = ";
  List.iter (Printf.printf "%d ") (preorder t);
  print_newline ()
