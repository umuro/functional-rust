(* 351: BTreeMap Ordered
   OCaml's Map functor gives a balanced BST with sorted keys,
   equivalent to Rust's BTreeMap. *)

module IntMap = Map.Make(Int)
module StringMap = Map.Make(String)

(* Build a sorted map from a list of (key, value) pairs *)
let sorted_map pairs =
  List.fold_left (fun m (k, v) -> IntMap.add k v m) IntMap.empty pairs

(* Range query: keys in [lo, hi] inclusive *)
let range_query map lo hi =
  IntMap.to_seq map
  |> Seq.filter (fun (k, _) -> k >= lo && k <= hi)
  |> List.of_seq

(* Minimum and maximum keys *)
let min_key map =
  match IntMap.min_binding_opt map with
  | Some (k, _) -> Some k
  | None        -> None

let max_key map =
  match IntMap.max_binding_opt map with
  | Some (k, _) -> Some k
  | None        -> None

let () =
  (* Sorted iteration *)
  let m = sorted_map [(3,'c');(1,'a');(2,'b')] in
  let keys = IntMap.bindings m |> List.map fst in
  assert (keys = [1;2;3]);
  Printf.printf "sorted keys: %s\n%!"
    (keys |> List.map string_of_int |> String.concat ", ");

  (* Range query *)
  let m2 = List.init 10 (fun i -> (i, i*i)) |> sorted_map in
  let r = range_query m2 3 5 in
  assert (r = [(3,9);(4,16);(5,25)]);
  Printf.printf "range [3,5]: %s\n%!"
    (r |> List.map (fun (k,v) -> Printf.sprintf "%d->%d" k v) |> String.concat ", ");

  (* Min / max *)
  let m3 = sorted_map [(5,'e');(1,'a');(9,'i')] in
  assert (min_key m3 = Some 1);
  assert (max_key m3 = Some 9);
  Printf.printf "min=%d max=%d\n%!"
    (Option.get (min_key m3))
    (Option.get (max_key m3))
