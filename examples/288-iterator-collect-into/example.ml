(* 288. Collecting into various collections - OCaml *)

module StringSet = Set.Make(String)
module StringMap = Map.Make(String)

let () =
  (* List (default collection) *)
  let nums = List.init 5 (fun i -> i * i) in
  Printf.printf "List: %s\n"
    (String.concat ", " (List.map string_of_int nums));

  (* Set (unique elements) *)
  let words = ["apple"; "banana"; "apple"; "cherry"; "banana"] in
  let set = List.fold_left (fun s w -> StringSet.add w s) StringSet.empty words in
  Printf.printf "Set: %s\n" (String.concat ", " (StringSet.elements set));

  (* Map from key-value pairs *)
  let pairs = [("a", 1); ("b", 2); ("c", 3)] in
  let map = List.fold_left (fun m (k, v) -> StringMap.add k v m) StringMap.empty pairs in
  Printf.printf "Map a=%d, b=%d\n" (StringMap.find "a" map) (StringMap.find "b" map);

  (* String from chars *)
  let chars = ['h'; 'e'; 'l'; 'l'; 'o'] in
  let s = String.concat "" (List.map (String.make 1) chars) in
  Printf.printf "String: %s\n" s
