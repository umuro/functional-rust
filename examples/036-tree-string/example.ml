(* Tree String *)
(* OCaml 99 Problems #36 *)

type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree

let node x l r = Node (x, l, r)
let leaf = Leaf

let rec to_string = function
  | Leaf -> ""
  | Node (c, l, r) ->
    let ls = to_string l and rs = to_string r in
    if ls = "" && rs = "" then String.make 1 c
    else Printf.sprintf "%c(%s,%s)" c ls rs

let from_string s =
  let n = String.length s in
  let rec parse pos =
    if pos >= n || s.[pos] = ',' || s.[pos] = ')' then (Leaf, pos)
    else
      let c = s.[pos] in
      let pos = pos + 1 in
      if pos < n && s.[pos] = '(' then
        let pos = pos + 1 in
        let left, pos = parse pos in
        let pos = pos + 1 in
        let right, pos = parse pos in
        let pos = pos + 1 in
        (Node (c, left, right), pos)
      else (Node (c, Leaf, Leaf), pos)
  in
  fst (parse 0)

(* Tests *)
let () =
  let sample =
    node 'a'
      (node 'b' (node 'd' leaf leaf) (node 'e' leaf leaf))
      (node 'c' leaf (node 'f' leaf leaf))
  in
  assert (to_string sample = "a(b(d,e),c(,f))");
  assert (from_string "a(b(d,e),c(,f))" = sample);
  assert (from_string (to_string sample) = sample);
  let single = node 'x' leaf leaf in
  assert (to_string single = "x");
  assert (from_string "x" = single);
  assert (to_string leaf = "");
  assert (from_string "" = leaf);
  print_endline "✓ OCaml tests passed"
