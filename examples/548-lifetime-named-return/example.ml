(* Return values connected to inputs in OCaml — GC handles it *)
let max_element lst =
  List.fold_left max (List.hd lst) lst

let find_keyword keywords text =
  List.find_opt (fun kw -> String.length kw > 0 && String.sub text 0 (min (String.length kw) (String.length text)) = kw) keywords

let () =
  let data = [3;1;4;1;5;9;2;6] in
  Printf.printf "max: %d\n" (max_element data);
  let kws = ["fn"; "let"; "if"; "match"] in
  match find_keyword kws "fn main() {" with
  | Some kw -> Printf.printf "found keyword: %s\n" kw
  | None -> Printf.printf "no keyword found\n"
