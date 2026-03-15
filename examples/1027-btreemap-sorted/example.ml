(* 1027: BTreeMap — Sorted Key Iteration *)
(* OCaml's Map module is tree-based and always sorted *)

module IntMap = Map.Make(Int)

(* Approach 1: Build and iterate in sorted order *)
let sorted_iteration () =
  let m = IntMap.empty
    |> IntMap.add 5 "five"
    |> IntMap.add 1 "one"
    |> IntMap.add 3 "three"
    |> IntMap.add 7 "seven"
    |> IntMap.add 2 "two"
  in
  let keys = IntMap.fold (fun k _v acc -> k :: acc) m [] |> List.rev in
  assert (keys = [1; 2; 3; 5; 7]);
  let values = IntMap.fold (fun _k v acc -> v :: acc) m [] |> List.rev in
  assert (values = ["one"; "two"; "three"; "five"; "seven"])

(* Approach 2: Range queries using split *)
let range_query () =
  let m = IntMap.empty
    |> IntMap.add 1 "a"
    |> IntMap.add 2 "b"
    |> IntMap.add 3 "c"
    |> IntMap.add 4 "d"
    |> IntMap.add 5 "e"
  in
  (* Get elements with keys in [2, 4] *)
  let (_, _, right) = IntMap.split 1 m in
  let (left, _, _) = IntMap.split 5 right in
  let range_keys = IntMap.fold (fun k _v acc -> k :: acc) left [] |> List.rev in
  assert (range_keys = [2; 3; 4])

(* Approach 3: Min/max binding *)
let min_max () =
  let m = IntMap.empty
    |> IntMap.add 10 "ten"
    |> IntMap.add 3 "three"
    |> IntMap.add 7 "seven"
  in
  let (min_k, _) = IntMap.min_binding m in
  let (max_k, _) = IntMap.max_binding m in
  assert (min_k = 3);
  assert (max_k = 10)

let () =
  sorted_iteration ();
  range_query ();
  min_max ();
  Printf.printf "✓ All tests passed\n"
