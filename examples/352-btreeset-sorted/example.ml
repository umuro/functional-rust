(* 352: BTreeSet Sorted
   OCaml's Set functor is a balanced BST — sorted, deduplicated,
   with efficient set operations. Equivalent to Rust's BTreeSet. *)

module IntSet = Set.Make(Int)

let sorted_set items = List.fold_left (fun s x -> IntSet.add x s) IntSet.empty items

(* Standard set operations — all return new sets (persistent/functional) *)
let union        = IntSet.union
let intersection = IntSet.inter
let difference   = IntSet.diff

let () =
  (* Sorted, deduplicated *)
  let s = sorted_set [3;1;4;1;5] in
  let v = IntSet.elements s in
  assert (v = [1;3;4;5]);
  Printf.printf "sorted+dedup: %s\n%!"
    (v |> List.map string_of_int |> String.concat ", ");

  (* Set operations *)
  let a = sorted_set [1;2;3] in
  let b = sorted_set [2;3;4] in

  let inter = intersection a b |> IntSet.elements in
  assert (inter = [2;3]);
  Printf.printf "intersection: %s\n%!"
    (inter |> List.map string_of_int |> String.concat ", ");

  let diff = difference a b |> IntSet.elements in
  assert (diff = [1]);
  Printf.printf "difference a\\b: %s\n%!"
    (diff |> List.map string_of_int |> String.concat ", ");

  let uni = union a b |> IntSet.elements in
  assert (uni = [1;2;3;4]);
  Printf.printf "union: %s\n%!"
    (uni |> List.map string_of_int |> String.concat ", ")
