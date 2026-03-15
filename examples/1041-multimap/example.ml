(* 1041: Multimap — HashMap<K, Vec<V>> with Helpers *)
(* A map where each key maps to multiple values *)

module StringMap = Map.Make(String)

type 'a multimap = 'a list StringMap.t

let empty : 'a multimap = StringMap.empty

let add key value (mm : 'a multimap) : 'a multimap =
  let current = match StringMap.find_opt key mm with
    | Some vs -> vs
    | None -> []
  in
  StringMap.add key (current @ [value]) mm

let get key (mm : 'a multimap) : 'a list =
  match StringMap.find_opt key mm with
  | Some vs -> vs
  | None -> []

let remove_key key (mm : 'a multimap) : 'a multimap =
  StringMap.remove key mm

let remove_value key value (mm : 'a multimap) : 'a multimap =
  match StringMap.find_opt key mm with
  | None -> mm
  | Some vs ->
    let filtered = List.filter (fun v -> v <> value) vs in
    if filtered = [] then StringMap.remove key mm
    else StringMap.add key filtered mm

let count_values key (mm : 'a multimap) : int =
  List.length (get key mm)

let total_values (mm : 'a multimap) : int =
  StringMap.fold (fun _ vs acc -> acc + List.length vs) mm 0

(* Approach 1: Basic multimap operations *)
let basic_ops () =
  let mm = empty
    |> add "fruits" "apple"
    |> add "fruits" "banana"
    |> add "fruits" "cherry"
    |> add "vegs" "carrot"
    |> add "vegs" "pea"
  in
  assert (get "fruits" mm = ["apple"; "banana"; "cherry"]);
  assert (get "vegs" mm = ["carrot"; "pea"]);
  assert (count_values "fruits" mm = 3);
  assert (total_values mm = 5)

(* Approach 2: Remove operations *)
let remove_ops () =
  let mm = empty
    |> add "tags" "rust"
    |> add "tags" "ocaml"
    |> add "tags" "haskell"
  in
  let mm = remove_value "tags" "ocaml" mm in
  assert (get "tags" mm = ["rust"; "haskell"]);
  let mm = remove_key "tags" mm in
  assert (get "tags" mm = [])

(* Approach 3: Index building *)
let build_index items =
  List.fold_left (fun mm (key, value) ->
    add key value mm
  ) empty items

let index_test () =
  let data = [
    ("lang", "Rust"); ("lang", "OCaml"); ("lang", "Haskell");
    ("paradigm", "functional"); ("paradigm", "imperative");
  ] in
  let mm = build_index data in
  assert (get "lang" mm = ["Rust"; "OCaml"; "Haskell"]);
  assert (get "paradigm" mm = ["functional"; "imperative"])

let () =
  basic_ops ();
  remove_ops ();
  index_test ();
  Printf.printf "✓ All tests passed\n"
