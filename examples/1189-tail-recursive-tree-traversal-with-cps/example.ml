(* Tail-Recursive Tree Traversal with CPS *)
(* Avoid stack overflow on deep trees with CPS *)

type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

let rec insert x = function
  | Leaf -> Node (Leaf, x, Leaf)
  | Node (l, v, r) ->
    if x < v then Node (insert x l, v, r)
    else Node (l, v, insert x r)

(* CPS inorder - tail recursive *)
let inorder t =
  let rec aux t k = match t with
    | Leaf -> k []
    | Node (l, v, r) ->
      aux r (fun right ->
        aux l (fun left ->
          k (left @ [v] @ right)))
  in aux t Fun.id

let t = List.fold_left (fun t x -> insert x t) Leaf [5;2;8;1;3;7;9]
let () = List.iter (fun x -> Printf.printf "%d " x) (inorder t)
