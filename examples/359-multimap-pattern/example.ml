(* OCaml: multimap via Hashtbl with list values *)

let mm_add tbl k v =
  let lst = try Hashtbl.find tbl k with Not_found -> [] in
  Hashtbl.replace tbl k (v::lst)

let () =
  let mm = Hashtbl.create 8 in
  List.iter (fun (k,v) -> mm_add mm k v)
    [("fruit","apple");("veg","carrot");("fruit","banana");("veg","broccoli");("fruit","cherry")];
  Hashtbl.iter (fun k vs ->
    Printf.printf "%s: [%s]\n" k (String.concat ", " vs)
  ) mm
