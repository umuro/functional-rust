(* 1030: Group Elements by Key — HashMap<K, Vec<V>>
   OCaml: use Hashtbl for mutable group accumulation, or
   List.sort + custom group_by for a pure functional approach. *)

(* Group words by first character using Hashtbl *)
let group_by_first_letter () =
  let words = ["apple"; "avocado"; "banana"; "blueberry"; "cherry"] in
  let groups : (char, string list) Hashtbl.t = Hashtbl.create 8 in
  List.iter (fun word ->
    let key = word.[0] in
    let existing = match Hashtbl.find_opt groups key with Some l -> l | None -> [] in
    Hashtbl.replace groups key (existing @ [word])
  ) words;
  assert (Hashtbl.find groups 'a' = ["apple"; "avocado"]);
  assert (Hashtbl.find groups 'b' = ["banana"; "blueberry"]);
  assert (Hashtbl.find groups 'c' = ["cherry"])

(* Group numbers by parity *)
let group_by_parity () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8] in
  let groups : (string, int list) Hashtbl.t = Hashtbl.create 4 in
  List.iter (fun n ->
    let key = if n mod 2 = 0 then "even" else "odd" in
    let existing = match Hashtbl.find_opt groups key with Some l -> l | None -> [] in
    Hashtbl.replace groups key (existing @ [n])
  ) nums;
  assert (Hashtbl.find groups "even" = [2; 4; 6; 8]);
  assert (Hashtbl.find groups "odd"  = [1; 3; 5; 7])

(* Pure functional group_by — returns an association list sorted by key *)
let group_by key_fn items =
  List.fold_left (fun acc item ->
    let k = key_fn item in
    let existing = match List.assoc_opt k acc with Some l -> l | None -> [] in
    let updated = List.map (fun (k2, v) -> if k2 = k then (k2, v @ [item]) else (k2, v)) acc in
    if List.assoc_opt k acc = None then acc @ [(k, [item])]
    else updated
  ) [] items

let () =
  group_by_first_letter ();
  group_by_parity ();

  (* Generic group_by *)
  let data = [("Alice", 90); ("Bob", 85); ("Alice", 92); ("Bob", 88)] in
  let groups = group_by (fun (name, _) -> name) data in
  let alice = match List.assoc_opt "Alice" groups with Some l -> l | None -> [] in
  let bob   = match List.assoc_opt "Bob"   groups with Some l -> l | None -> [] in
  assert (List.length alice = 2);
  assert (List.length bob   = 2);
  assert (snd (List.nth alice 0) = 90);
  assert (snd (List.nth alice 1) = 92);

  (* group by length *)
  let words = ["hi"; "hey"; "hello"; "yo"; "yes"] in
  let by_len = group_by (fun w -> String.length w) words in
  let len2 = match List.assoc_opt 2 by_len with Some l -> l | None -> [] in
  let len3 = match List.assoc_opt 3 by_len with Some l -> l | None -> [] in
  let len5 = match List.assoc_opt 5 by_len with Some l -> l | None -> [] in
  assert (List.length len2 = 2);
  assert (List.length len3 = 2);
  assert (List.length len5 = 1);

  Printf.printf "Group-by tests passed\n"
