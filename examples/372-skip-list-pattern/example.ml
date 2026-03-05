(* OCaml: simplified skip list concept *)

(* We model skip list as sorted list with 'express lanes' *)
type 'a node = {
  value : 'a;
  next  : 'a node option array;  (* one per level *)
}

(* Simplified: just show the concept with a sorted list *)
let insert_sorted lst v =
  let rec go = function
    | [] -> [v]
    | x::xs -> if v <= x then v::x::xs else x :: go xs
  in go lst

let search lst v = List.exists ((=) v) lst

let () =
  let lst = List.fold_left insert_sorted [] [5;3;7;1;9;4;6;2;8] in
  Printf.printf "Sorted: [%s]\n" (String.concat ";" (List.map string_of_int lst));
  Printf.printf "Search 7: %b\n" (search lst 7);
  Printf.printf "Search 10: %b\n" (search lst 10)
