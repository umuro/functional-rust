(* 933: Map.Make Functor — String→Int Dictionary

   OCaml's `Map.Make(String)` creates a specialized balanced BST map.
   This is the idiomatic OCaml functor pattern — Rust achieves the same
   with generic BTreeMap<String, usize>. The OCaml approach is more
   expressive: module-level parameterisation via functors. *)

(* ── Create a String-keyed map module via Map.Make ──────────────────────── *)

module StringMap = Map.Make(String)

(* Build a word-length map *)
let word_lengths words =
  List.fold_left (fun m w -> StringMap.add w (String.length w) m)
    StringMap.empty words

(* Filter entries by a predicate on values *)
let filter_by_value pred m =
  StringMap.filter (fun _k v -> pred v) m

(* Map over values, producing a new map *)
let map_values f m =
  StringMap.map f m

(* ── Generic map utilities using the Map.S signature ────────────────────── *)

(* Convert map to sorted association list — Map.bindings gives sorted pairs *)
let to_sorted_list m = StringMap.bindings m

(* Merge two maps, combining values with f when keys collide *)
let merge_with f m1 m2 =
  StringMap.union (fun _key v1 v2 -> Some (f v1 v2)) m1 m2

(* ── Hashtbl-based unordered map (like Rust's HashMap) ─────────────────── *)

let word_lengths_hash words =
  let tbl = Hashtbl.create (List.length words) in
  List.iter (fun w -> Hashtbl.replace tbl w (String.length w)) words;
  tbl

let () =
  let words = ["ocaml"; "rust"; "haskell"; "erlang"; "go"] in
  let m = word_lengths words in

  assert (StringMap.find "rust" m = 4);
  assert (StringMap.find "haskell" m = 7);
  assert (StringMap.find_opt "missing" m = None);

  (* filter: keep words longer than 4 chars *)
  let long = filter_by_value (fun v -> v > 4) m in
  assert (StringMap.cardinal long = 3);  (* ocaml(5), haskell(7), erlang(6) *)
  assert (StringMap.mem "haskell" long);
  assert (not (StringMap.mem "go" long));

  (* map_values: double lengths *)
  let doubled = map_values (fun v -> v * 2) (word_lengths ["rust"; "go"]) in
  assert (StringMap.find "rust" doubled = 8);
  assert (StringMap.find "go" doubled = 4);

  (* empty map *)
  let empty_m = word_lengths [] in
  assert (StringMap.is_empty empty_m);

  (* BTreeMap ordering — Map.Make gives sorted keys *)
  let m2 = word_lengths ["zebra"; "apple"; "mango"] in
  let keys = List.map fst (StringMap.bindings m2) in
  assert (keys = ["apple"; "mango"; "zebra"]);  (* sorted *)

  (* merge_with: combine two maps, summing values on collision *)
  let ma = StringMap.of_seq (List.to_seq [("a", 1); ("b", 2)]) in
  let mb = StringMap.of_seq (List.to_seq [("b", 10); ("c", 3)]) in
  let mc = merge_with ( + ) ma mb in
  assert (StringMap.find "a" mc = 1);
  assert (StringMap.find "b" mc = 12);
  assert (StringMap.find "c" mc = 3);

  (* Hashtbl (unordered) *)
  let h = word_lengths_hash ["rust"; "go"] in
  assert (Hashtbl.find h "rust" = 4);

  print_endline "933-map-functor: all tests passed"
