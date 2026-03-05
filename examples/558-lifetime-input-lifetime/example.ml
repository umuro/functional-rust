(* Input lifetimes in OCaml -- always automatic *)
let transform f lst = List.map f lst
let find_by pred lst = List.find_opt pred lst
let fold_refs acc_fn init lst = List.fold_left acc_fn init lst

let () =
  let nums = [1;2;3;4;5] in
  let doubled = transform (fun x -> x * 2) nums in
  Printf.printf "doubled: [%s]\n" (String.concat ";" (List.map string_of_int doubled));
  let found = find_by (fun x -> x > 3) nums in
  Printf.printf "found: %s\n" (match found with Some x -> string_of_int x | None -> "None")
