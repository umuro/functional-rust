(* Catamorphism — Generalized Fold on ADTs *)

type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree

(* The catamorphism replaces constructors with functions *)
let rec cata ~leaf ~node = function
  | Leaf -> leaf
  | Node (l, v, r) -> node (cata ~leaf ~node l) v (cata ~leaf ~node r)

let size = cata ~leaf:0 ~node:(fun l _ r -> 1 + l + r)
let sum = cata ~leaf:0 ~node:(fun l v r -> l + v + r)
let height = cata ~leaf:0 ~node:(fun l _ r -> 1 + max l r)
let mirror = cata ~leaf:Leaf ~node:(fun l v r -> Node (r, v, l))
let to_list = cata ~leaf:[] ~node:(fun l v r -> l @ [v] @ r)

let () =
  let t = Node (Node (Leaf, 1, Leaf), 2, Node (Leaf, 3, Leaf)) in
  assert (size t = 3);
  assert (sum t = 6);
  assert (height t = 2);
  assert (to_list (mirror t) = [3; 2; 1])
