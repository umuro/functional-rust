(* 1027: BTreeMap — Sorted Key Iteration
   OCaml's Map functor (Map.Make) is a balanced BST — keys are always sorted.
   It is immutable and purely functional, unlike Rust's BTreeMap.
   Iteration naturally produces keys in ascending order. *)

module IntMap = Map.Make(Int)

(* Build and iterate — keys come out sorted *)
let sorted_iteration () =
  let m =
    IntMap.empty
    |> IntMap.add 5 "five"
    |> IntMap.add 1 "one"
    |> IntMap.add 3 "three"
    |> IntMap.add 7 "seven"
    |> IntMap.add 2 "two"
  in
  (* bindings() returns sorted (key, value) pairs *)
  let keys = List.map fst (IntMap.bindings m) in
  assert (keys = [1; 2; 3; 5; 7]);

  let values = List.map snd (IntMap.bindings m) in
  assert (values = ["one"; "two"; "three"; "five"; "seven"])

(* Range query — find elements with keys in [lo, hi] *)
let range_query () =
  let m =
    [(1,"a"); (2,"b"); (3,"c"); (4,"d"); (5,"e")]
    |> List.fold_left (fun acc (k, v) -> IntMap.add k v acc) IntMap.empty
  in
  (* Filter bindings with keys in [2, 4] *)
  let range_keys =
    IntMap.bindings m
    |> List.filter (fun (k, _) -> k >= 2 && k <= 4)
    |> List.map fst
  in
  assert (range_keys = [2; 3; 4]);

  (* Tail: keys >= 3 *)
  let tail =
    IntMap.bindings m
    |> List.filter (fun (k, _) -> k >= 3)
    |> List.map fst
  in
  assert (tail = [3; 4; 5])

(* Min and max keys *)
let min_max () =
  let m =
    [(10, "ten"); (3, "three"); (7, "seven")]
    |> List.fold_left (fun acc (k, v) -> IntMap.add k v acc) IntMap.empty
  in
  let (min_k, _) = IntMap.min_binding m in
  let (max_k, _) = IntMap.max_binding m in
  assert (min_k = 3);
  assert (max_k = 10)

let () =
  sorted_iteration ();
  range_query ();
  min_max ();

  (* from list of pairs — collect in sorted order *)
  let pairs = [(3, "c"); (1, "a"); (2, "b")] in
  let m = List.fold_left (fun acc (k, v) -> IntMap.add k v acc) IntMap.empty pairs in
  let keys = List.map fst (IntMap.bindings m) in
  assert (keys = [1; 2; 3]);

  Printf.printf "BTreeMap (Map) tests passed — keys always sorted\n"
