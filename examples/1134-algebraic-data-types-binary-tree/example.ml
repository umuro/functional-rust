(* Algebraic Data Types — Binary Tree *)
(* Define and traverse a binary tree with variants *)

type 'a tree =
  | Leaf
  | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else if x > v then Node (l, v, insert x r)
    else Node (l, v, r)

let rec inorder = function
  | Leaf -> []
  | Node (l, v, r) -> inorder l @ [v] @ inorder r

let tree = List.fold_left (fun t x -> insert x t) Leaf [5;3;7;1;4;6;8]
let () = List.iter (fun x -> Printf.printf "%d " x) (inorder tree)
