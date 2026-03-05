(* OCaml: simplified B-tree concepts via sorted array *)

type 'a node = {
  mutable keys   : 'a array;
  mutable nkeys  : int;
  mutable children : 'a node option array;
  is_leaf : bool;
}

(* Simplified: show B-tree order properties *)
let validate_btree_order t node =
  (* Each non-root node has at least t-1 keys *)
  node.nkeys >= t - 1 && node.nkeys <= 2*t - 1

let () =
  (* Demo: B-tree of degree 2 (2-3-4 tree) allows 1-3 keys per node *)
  let t = 2 in
  Printf.printf "Min keys per node: %d\n" (t-1);
  Printf.printf "Max keys per node: %d\n" (2*t-1);
  Printf.printf "Max children per node: %d\n" (2*t)
