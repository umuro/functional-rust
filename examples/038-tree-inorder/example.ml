(* Tree Inorder *)
(* OCaml 99 Problems, extension of #29-40 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec inorder = function
  | Leaf -> []
  | Node (x, l, r) -> inorder l @ [x] @ inorder r

(* Tests *)
let () =
  let t = node 'a' (node 'b' leaf leaf) (node 'c' leaf leaf) in
  assert (inorder t = ['b'; 'a'; 'c']);

  let bst = node 5 (node 3 (node 1 leaf leaf) (node 4 leaf leaf)) (node 8 leaf leaf) in
  assert (inorder bst = [1; 3; 4; 5; 8]);

  assert (inorder leaf = []);
  print_endline "✓ OCaml tests passed"
