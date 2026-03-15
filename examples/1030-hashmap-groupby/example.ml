(* 1030: Group Elements by Key *)
(* Build a map from key to list of values *)

module StringMap = Map.Make(String)

(* Approach 1: Group words by first letter *)
let group_by_first_letter () =
  let words = ["apple"; "avocado"; "banana"; "blueberry"; "cherry"] in
  let groups = List.fold_left (fun acc w ->
    let key = String.make 1 w.[0] in
    let current = match StringMap.find_opt key acc with
      | Some lst -> lst
      | None -> []
    in
    StringMap.add key (current @ [w]) acc
  ) StringMap.empty words in
  assert (StringMap.find "a" groups = ["apple"; "avocado"]);
  assert (StringMap.find "b" groups = ["banana"; "blueberry"]);
  assert (StringMap.find "c" groups = ["cherry"])

(* Approach 2: Group numbers by property *)
let group_by_parity () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8] in
  let groups = List.fold_left (fun acc n ->
    let key = if n mod 2 = 0 then "even" else "odd" in
    let current = match StringMap.find_opt key acc with
      | Some lst -> lst
      | None -> []
    in
    StringMap.add key (current @ [n]) acc
  ) StringMap.empty nums in
  assert (StringMap.find "even" groups = [2; 4; 6; 8]);
  assert (StringMap.find "odd" groups = [1; 3; 5; 7])

(* Approach 3: Generic group_by function *)
let group_by (type a) key_fn (items : a list) =
  List.fold_left (fun acc item ->
    let key = key_fn item in
    let current = match StringMap.find_opt key acc with
      | Some lst -> lst
      | None -> []
    in
    StringMap.add key (current @ [item]) acc
  ) StringMap.empty items

let test_generic_group_by () =
  let data = [("Alice", 90); ("Bob", 85); ("Alice", 92); ("Bob", 88)] in
  let groups = group_by fst data in
  assert (StringMap.find "Alice" groups = [("Alice", 90); ("Alice", 92)]);
  assert (StringMap.find "Bob" groups = [("Bob", 85); ("Bob", 88)])

let () =
  group_by_first_letter ();
  group_by_parity ();
  test_generic_group_by ();
  Printf.printf "✓ All tests passed\n"
