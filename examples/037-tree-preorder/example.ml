(* Tree Preorder *)
(* OCaml 99 Problems, extension of #29-40 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec preorder = function
  | Leaf -> "."
  | Node (c, l, r) -> String.make 1 c ^ preorder l ^ preorder r

let from_preorder s =
  let pos = ref 0 in
  let rec parse () =
    let c = s.[!pos] in
    incr pos;
    if c = '.' then Leaf
    else
      let l = parse () in
      let r = parse () in
      Node (c, l, r)
  in
  parse ()

(* Tests *)
let () =
  let sample = node 'a' (node 'b' leaf leaf) (node 'c' leaf leaf) in
  assert (preorder sample = "ab..c..");
  assert (from_preorder "ab..c.." = sample);
  assert (from_preorder (preorder sample) = sample);
  assert (preorder leaf = ".");
  assert (from_preorder "." = leaf);
  print_endline "✓ OCaml tests passed"
