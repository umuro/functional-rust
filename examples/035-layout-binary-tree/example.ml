(* Layout Binary Tree *)
(* OCaml 99 Problems #35 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let layout tree =
  let x = ref 0 in
  let rec lay depth = function
    | Leaf -> Leaf
    | Node (v, l, r) ->
      let left = lay (depth + 1) l in
      incr x;
      let pos = (!x, depth) in
      let right = lay (depth + 1) r in
      Node ((v, pos), left, right)
  in
  lay 1 tree

(* Tests *)
let () =
  let t = node 'a' (node 'b' leaf leaf) (node 'c' leaf leaf) in
  let expected =
    node ('a', (2, 1)) (node ('b', (1, 2)) leaf leaf) (node ('c', (3, 2)) leaf leaf)
  in
  assert (layout t = expected);
  assert (layout leaf = Leaf);
  let single = node 'a' leaf leaf in
  assert (layout single = node ('a', (1, 1)) leaf leaf);
  print_endline "✓ OCaml tests passed"
