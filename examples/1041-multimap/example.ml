(* 1041: Multimap — Map<K, V list>
   OCaml's Map functor provides a persistent multimap naturally.
   Each key maps to a list of values. *)

module StringMap = Map.Make(String)

type ('k, 'v) multimap = 'v list StringMap.t

let empty_mm : (string, 'v) multimap = StringMap.empty

let insert key value mm =
  let current = try StringMap.find key mm with Not_found -> [] in
  StringMap.add key (current @ [value]) mm

let get key mm =
  try StringMap.find key mm with Not_found -> []

let remove_key key mm =
  StringMap.remove key mm

let remove_value key value mm =
  match StringMap.find_opt key mm with
  | None -> (false, mm)
  | Some values ->
    let new_values = List.filter (fun v -> v <> value) values in
    if List.length new_values = List.length values then (false, mm)
    else
      let new_mm =
        if new_values = [] then StringMap.remove key mm
        else StringMap.add key new_values mm
      in
      (true, new_mm)

let count_values key mm =
  List.length (get key mm)

let total_values mm =
  StringMap.fold (fun _ vs acc -> acc + List.length vs) mm 0

let contains_value key value mm =
  List.mem value (get key mm)

let () =
  let mm = empty_mm in
  let mm = insert "fruits" "apple" mm in
  let mm = insert "fruits" "banana" mm in
  let mm = insert "fruits" "cherry" mm in
  let mm = insert "vegs" "carrot" mm in
  let mm = insert "vegs" "pea" mm in

  assert (get "fruits" mm = ["apple"; "banana"; "cherry"]);
  assert (get "vegs" mm = ["carrot"; "pea"]);
  assert (count_values "fruits" mm = 3);
  assert (total_values mm = 5);

  (* Remove value *)
  let mm2 = empty_mm in
  let mm2 = insert "tags" "rust" mm2 in
  let mm2 = insert "tags" "ocaml" mm2 in
  let mm2 = insert "tags" "haskell" mm2 in
  let (removed, mm2) = remove_value "tags" "ocaml" mm2 in
  assert removed;
  assert (get "tags" mm2 = ["rust"; "haskell"]);

  let mm2 = remove_key "tags" mm2 in
  assert (get "tags" mm2 = []);

  (* Build index *)
  let data = [
    ("lang", "Rust"); ("lang", "OCaml"); ("lang", "Haskell");
    ("paradigm", "functional"); ("paradigm", "imperative");
  ] in
  let mm3 = List.fold_left (fun acc (k, v) -> insert k v acc) empty_mm data in
  assert (get "lang" mm3 = ["Rust"; "OCaml"; "Haskell"]);
  assert (get "paradigm" mm3 = ["functional"; "imperative"]);
  assert (contains_value "lang" "Rust" mm3);
  assert (not (contains_value "lang" "Python" mm3));

  (* Empty get *)
  assert (get "missing" mm = []);
  assert (count_values "missing" mm = 0);

  Printf.printf "All multimap tests passed.\n"
