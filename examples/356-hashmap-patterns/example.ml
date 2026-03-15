(* OCaml: Hashtbl patterns *)

let word_count text =
  let tbl = Hashtbl.create 16 in
  String.split_on_char ' ' text |> List.filter ((<>) "") |> List.iter (fun w ->
    let n = try Hashtbl.find tbl w with Not_found -> 0 in
    Hashtbl.replace tbl w (n+1)
  );
  tbl

let group_by f lst =
  let tbl = Hashtbl.create 8 in
  List.iter (fun x ->
    let k = f x in
    let group = try Hashtbl.find tbl k with Not_found -> [] in
    Hashtbl.replace tbl k (x::group)
  ) lst; tbl

let () =
  let wc = word_count "the cat sat on the mat the cat" in
  Hashtbl.iter (fun w n -> Printf.printf "%s: %d\n" w n) wc
