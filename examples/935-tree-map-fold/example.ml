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
  assert (size t = 5);
  assert (depth t = 3);
  assert (sum t = 16);
  assert (preorder t = [4; 2; 1; 3; 6]);
  assert (inorder t = [1; 2; 3; 4; 6]);
  let t2 = map_tree (fun v -> v * 2) t in
  assert (sum t2 = 32);
  print_endline "All assertions passed."
