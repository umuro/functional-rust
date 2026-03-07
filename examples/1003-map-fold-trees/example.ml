type 'a tree =
  | Leaf
  | Node of 'a * 'a tree * 'a tree

let rec map_tree f = function
  | Leaf           -> Leaf
  | Node (v, l, r) -> Node (f v, map_tree f l, map_tree f r)

let rec fold_tree f acc = function
  | Leaf           -> acc
  | Node (v, l, r) -> f v (fold_tree f acc l) (fold_tree f acc r)

let size     t = fold_tree (fun _ l r -> 1 + l + r)    0  t
let depth    t = fold_tree (fun _ l r -> 1 + max l r)  0  t
let sum      t = fold_tree (fun v l r -> v + l + r)    0  t
let preorder t = fold_tree (fun v l r -> [v] @ l @ r) [] t
let inorder  t = fold_tree (fun v l r -> l @ [v] @ r) [] t

let t =
  Node (4, Node (2, Node (1, Leaf, Leaf), Node (3, Leaf, Leaf)),
           Node (6, Leaf, Leaf))

let () =
  Printf.printf "size     = %d\n" (size t);
  Printf.printf "depth    = %d\n" (depth t);
  Printf.printf "sum      = %d\n" (sum t);
  Printf.printf "preorder = %s\n"
    (String.concat " " (List.map string_of_int (preorder t)));
  Printf.printf "inorder  = %s\n"
    (String.concat " " (List.map string_of_int (inorder t)));
  let t2 = map_tree (fun v -> v * 2) t in
  Printf.printf "doubled sum = %d\n" (sum t2)
