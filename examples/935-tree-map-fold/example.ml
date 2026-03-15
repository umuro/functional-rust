(* 935: Map and Fold on Trees

   Lifting map and fold from lists to binary trees.
   Once fold_tree is defined, size / depth / sum / traversals
   require no explicit recursion — fold does it all. *)

(* ── Tree type ───────────────────────────────────────────────────────────── *)

type 'a tree =
  | Leaf
  | Node of 'a tree * 'a * 'a tree

let node l v r = Node (l, v, r)

(* ── map_tree ────────────────────────────────────────────────────────────── *)

(* Map a function over every node value, producing a new tree *)
let rec map_tree f = function
  | Leaf -> Leaf
  | Node (l, v, r) -> Node (map_tree f l, f v, map_tree f r)

(* ── fold_tree (catamorphism on binary trees) ─────────────────────────────── *)

(* The fold replaces Leaf with `z` and Node with `f left_result v right_result` *)
let rec fold_tree z f = function
  | Leaf -> z
  | Node (l, v, r) ->
    let lv = fold_tree z f l in
    let rv = fold_tree z f r in
    f lv v rv

(* ── All derived via fold — no explicit recursion ────────────────────────── *)

let size   t = fold_tree 0 (fun l _ r -> 1 + l + r) t
let depth  t = fold_tree 0 (fun l _ r -> 1 + max l r) t
let sum    t = fold_tree 0 (fun l v r -> l + v + r) t

let preorder t =
  fold_tree [] (fun l v r -> [v] @ l @ r) t

let inorder t =
  fold_tree [] (fun l v r -> l @ [v] @ r) t

let postorder t =
  fold_tree [] (fun l v r -> l @ r @ [v]) t

(* to_sorted_list for BST: same as inorder *)
let to_sorted_list = inorder

let () =
  (*      4
         / \
        2   6
       / \
      1   3  *)
  let t = node (node (node Leaf 1 Leaf) 2 (node Leaf 3 Leaf)) 4 (node Leaf 6 Leaf) in

  assert (size t = 5);
  assert (size Leaf = 0);

  assert (depth t = 3);
  assert (depth Leaf = 0);

  assert (sum t = 16);
  assert (sum Leaf = 0);

  assert (preorder t = [4; 2; 1; 3; 6]);
  assert (inorder  t = [1; 2; 3; 4; 6]);
  assert (postorder t = [1; 3; 2; 6; 4]);

  (* map_tree *)
  let doubled = map_tree (fun v -> v * 2) t in
  assert (sum doubled = 32);
  assert (preorder doubled = [8; 4; 2; 6; 12]);

  (* Single node *)
  let single = node Leaf 42 Leaf in
  assert (size single = 1);
  assert (sum single = 42);
  assert (preorder single = [42]);

  (* fold can express any tree computation *)
  let product = fold_tree 1 (fun l v r -> l * v * r) t in
  (* Node(Node(Node(1,1,1), 2, Node(1,3,1)), 4, Node(1,6,1))
     = (1 * 1 * 1) * 2 * (1 * 3 * 1) * 4 * (1 * 6 * 1) = ... *)
  assert (product > 0);  (* just verify it runs without error *)

  (* A balanced BST: inorder gives sorted output *)
  let bst = node (node Leaf 1 Leaf) 2 (node Leaf 3 Leaf) in
  assert (to_sorted_list bst = [1; 2; 3]);

  print_endline "935-tree-map-fold: all tests passed"
