(* Map.Make — Group By Key *)
(* Group list elements by a key function using Map *)

module SMap = Map.Make(String)

let group_by key_fn lst =
  List.fold_left (fun m x ->
    let k = key_fn x in
    let existing = try SMap.find k m with Not_found -> [] in
    SMap.add k (x :: existing) m
  ) SMap.empty lst

let words = ["apple"; "banana"; "avocado"; "blueberry"; "cherry"; "apricot"]
let grouped = group_by (fun s -> String.make 1 s.[0]) words
let () = SMap.iter (fun k vs ->
  Printf.printf "%s: %s\n" k (String.concat ", " vs)
) grouped
