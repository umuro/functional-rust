(* 1042: Bidirectional Map — Two Maps for Key↔Value *)
(* Both directions must be unique for bimap to work *)

module StringMap = Map.Make(String)
module IntMap = Map.Make(Int)

type bimap = {
  forward: int StringMap.t;     (* string -> int *)
  backward: string IntMap.t;    (* int -> string *)
}

let empty = { forward = StringMap.empty; backward = IntMap.empty }

let insert key value bm =
  (* Remove old mappings if either key or value already exists *)
  let bm = match StringMap.find_opt key bm.forward with
    | Some old_val -> { bm with backward = IntMap.remove old_val bm.backward }
    | None -> bm
  in
  let bm = match IntMap.find_opt value bm.backward with
    | Some old_key -> { bm with forward = StringMap.remove old_key bm.forward }
    | None -> bm
  in
  { forward = StringMap.add key value bm.forward;
    backward = IntMap.add value key bm.backward }

let get_by_key key bm = StringMap.find_opt key bm.forward
let get_by_value value bm = IntMap.find_opt value bm.backward

let remove_by_key key bm =
  match StringMap.find_opt key bm.forward with
  | None -> bm
  | Some value ->
    { forward = StringMap.remove key bm.forward;
      backward = IntMap.remove value bm.backward }

let size bm = StringMap.cardinal bm.forward

(* Approach 1: Basic bidirectional lookup *)
let basic_ops () =
  let bm = empty
    |> insert "one" 1
    |> insert "two" 2
    |> insert "three" 3
  in
  assert (get_by_key "two" bm = Some 2);
  assert (get_by_value 2 bm = Some "two");
  assert (get_by_key "four" bm = None);
  assert (size bm = 3)

(* Approach 2: Overwrite handling *)
let overwrite_test () =
  let bm = empty
    |> insert "a" 1
    |> insert "b" 2
    |> insert "a" 3  (* overwrites "a"->1, removes 1->"a" *)
  in
  assert (get_by_key "a" bm = Some 3);
  assert (get_by_value 1 bm = None);  (* old mapping gone *)
  assert (get_by_value 3 bm = Some "a")

(* Approach 3: Encoding/decoding table *)
let codec_test () =
  let bm = empty
    |> insert "red" 0xFF0000
    |> insert "green" 0x00FF00
    |> insert "blue" 0x0000FF
  in
  assert (get_by_key "red" bm = Some 0xFF0000);
  assert (get_by_value 0x00FF00 bm = Some "green");
  let bm = remove_by_key "red" bm in
  assert (get_by_key "red" bm = None);
  assert (get_by_value 0xFF0000 bm = None)

let () =
  basic_ops ();
  overwrite_test ();
  codec_test ();
  Printf.printf "✓ All tests passed\n"
