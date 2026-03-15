(* OCaml: entry-like patterns with Hashtbl *)

let upsert tbl k f default =
  let v = try Hashtbl.find tbl k with Not_found -> default in
  Hashtbl.replace tbl k (f v)

let () =
  let tbl = Hashtbl.create 8 in
  List.iter (fun w -> upsert tbl w (fun n -> n+1) 0) ["a";"b";"a";"c";"a";"b"];
  Hashtbl.iter (fun k v -> Printf.printf "%s: %d\n" k v) tbl
