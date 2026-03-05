(* Association List to Map Conversion *)
(* Convert between association lists and maps *)

module SMap = Map.Make(String)

let alist_to_map lst =
  List.fold_left (fun m (k, v) -> SMap.add k v m) SMap.empty lst

let map_to_alist m = SMap.bindings m

let data = [("name", "Alice"); ("city", "Amsterdam"); ("lang", "OCaml")]
let m = alist_to_map data
let () = SMap.iter (fun k v -> Printf.printf "%s: %s\n" k v) m

(* Update and convert back *)
let m2 = SMap.add "year" "2024" m |> SMap.remove "city"
let pairs = map_to_alist m2
let () = List.iter (fun (k,v) -> Printf.printf "%s=%s " k v) pairs
