(* Dotstring Parse *)
(* OCaml 99 Problems, complement to #39 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let parse_dotstring s =
  let n = String.length s in
  let rec parse pos =
    if pos >= n then failwith "unexpected end of input"
    else
      let c = s.[pos] in
      if c = '.' then (Leaf, pos + 1)
      else
        let l, p1 = parse (pos + 1) in
        let r, p2 = parse p1 in
        (Node (c, l, r), p2)
  in
  parse 0

(* Tests *)
let () =
  let tree, pos = parse_dotstring "ab..c.." in
  assert (tree = node 'a' (node 'b' leaf leaf) (node 'c' leaf leaf));
  assert (pos = String.length "ab..c..");

  let leaf_tree, _ = parse_dotstring "." in
  assert (leaf_tree = Leaf);

  (try
     let _ = parse_dotstring "ab." in
     assert false
   with Failure _ -> ());

  (try
     let _ = parse_dotstring "" in
     assert false
   with Failure _ -> ());

  print_endline "✓ OCaml tests passed"
