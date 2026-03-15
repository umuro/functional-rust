(* 1042: Bidirectional Map (BiMap)
   Two OCaml maps (forward and backward) maintained in sync.
   Inserting with a duplicate key or value removes the old mapping. *)

module StringMap = Map.Make(String)
module IntMap    = Map.Make(Int)

(* Concrete BiMap: string <-> int for clarity *)
type bimap = {
  forward  : int StringMap.t;    (* key -> value *)
  backward : string IntMap.t;    (* value -> key *)
}

let empty_bm = { forward = StringMap.empty; backward = IntMap.empty }

let insert key value bm =
  (* Remove any existing forward mapping for this key *)
  let bm =
    match StringMap.find_opt key bm.forward with
    | None -> bm
    | Some old_val ->
      { bm with
        forward  = StringMap.remove key bm.forward;
        backward = IntMap.remove old_val bm.backward }
  in
  (* Remove any existing backward mapping for this value *)
  let bm =
    match IntMap.find_opt value bm.backward with
    | None -> bm
    | Some old_key ->
      { bm with
        forward  = StringMap.remove old_key bm.forward;
        backward = IntMap.remove value bm.backward }
  in
  { forward  = StringMap.add key value bm.forward;
    backward = IntMap.add value key bm.backward }

let get_by_key key bm = StringMap.find_opt key bm.forward
let get_by_value v bm = IntMap.find_opt v bm.backward

let remove_by_key key bm =
  match StringMap.find_opt key bm.forward with
  | None -> (None, bm)
  | Some value ->
    let bm = { forward  = StringMap.remove key bm.forward;
               backward = IntMap.remove value bm.backward } in
    (Some value, bm)

let remove_by_value value bm =
  match IntMap.find_opt value bm.backward with
  | None -> (None, bm)
  | Some key ->
    let bm = { forward  = StringMap.remove key bm.forward;
               backward = IntMap.remove value bm.backward } in
    (Some key, bm)

let len bm = StringMap.cardinal bm.forward
let contains_key k bm = StringMap.mem k bm.forward
let contains_value v bm = IntMap.mem v bm.backward

let () =
  let bm = ref empty_bm in
  bm := insert "one" 1 !bm;
  bm := insert "two" 2 !bm;
  bm := insert "three" 3 !bm;

  assert (get_by_key "two" !bm = Some 2);
  assert (get_by_value 2 !bm = Some "two");
  assert (get_by_key "four" !bm = None);
  assert (len !bm = 3);

  (* Overwrite: "a"->1, then "a"->3 *)
  let bm2 = ref empty_bm in
  bm2 := insert "a" 1 !bm2;
  bm2 := insert "b" 2 !bm2;
  bm2 := insert "a" 3 !bm2;
  assert (get_by_key "a" !bm2 = Some 3);
  assert (get_by_value 1 !bm2 = None);   (* old mapping gone *)
  assert (get_by_value 3 !bm2 = Some "a");

  (* Color codec *)
  let bm3 = ref empty_bm in
  bm3 := insert "red"   0xFF0000 !bm3;
  bm3 := insert "green" 0x00FF00 !bm3;
  bm3 := insert "blue"  0x0000FF !bm3;
  assert (get_by_key "red" !bm3 = Some 0xFF0000);
  assert (get_by_value 0x00FF00 !bm3 = Some "green");
  let (_, bm3') = remove_by_key "red" !bm3 in
  assert (get_by_key "red" bm3' = None);
  assert (get_by_value 0xFF0000 bm3' = None);

  (* Remove by value *)
  let bm4 = ref empty_bm in
  bm4 := insert "x" 10 !bm4;
  bm4 := insert "y" 20 !bm4;
  let (removed, bm4) = remove_by_value 10 !bm4 in
  assert (removed = Some "x");
  assert (not (contains_key "x" bm4));
  assert (not (contains_value 10 bm4));

  Printf.printf "All bimap tests passed.\n"
