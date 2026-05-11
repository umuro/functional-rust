type 'a tree =
  | Leaf
  | Node of 'a * 'a tree * 'a tree

let rec depth t =
  match t with
  | Leaf -> 0
  | Node (_, l, r) -> 1 + max (depth l) (depth r)

let rec preorder t =
  match t with
  | Leaf -> []
  | Node (v, l, r) -> v :: (preorder l @ preorder r)

let sample_tree =
  Node (1,
    Node (2, Leaf, Leaf),
    Node (3, Leaf, Leaf))

let () =
  Printf.printf "depth = %d\n" (depth sample_tree);  (* 3 *)
  List.iter (Printf.printf "%d ") (preorder sample_tree)  (* 1 2 3 *)