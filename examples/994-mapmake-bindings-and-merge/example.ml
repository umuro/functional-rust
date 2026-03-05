(* Map.Make — Bindings and Merge *)
(* Extract bindings and merge two maps *)

module SMap = Map.Make(String)

let m1 = SMap.of_list [("a", 1); ("b", 2); ("c", 3)]
let m2 = SMap.of_list [("b", 20); ("c", 30); ("d", 40)]

let merged = SMap.union (fun _k v1 v2 -> Some (v1 + v2)) m1 m2

let pairs = SMap.bindings merged
let () = List.iter (fun (k,v) -> Printf.printf "%s: %d\n" k v) pairs
