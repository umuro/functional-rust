(* Fold — Universal Iterator *)
(* Implement many list functions using fold *)

(* Everything is a fold *)
let my_length lst = List.fold_left (fun acc _ -> acc + 1) 0 lst
let my_rev lst = List.fold_left (fun acc x -> x :: acc) [] lst
let my_map f lst = List.fold_right (fun x acc -> f x :: acc) lst []
let my_filter p lst = List.fold_right (fun x acc -> if p x then x :: acc else acc) lst []
let my_exists p lst = List.fold_left (fun acc x -> acc || p x) false lst
let my_for_all p lst = List.fold_left (fun acc x -> acc && p x) true lst
let my_flatten lst = List.fold_right ( @ ) lst []

let data = [1; 2; 3; 4; 5]
let () =
  Printf.printf "length: %d\n" (my_length data);
  Printf.printf "rev: %s\n" (String.concat " " (List.map string_of_int (my_rev data)));
  Printf.printf "evens: %s\n" (String.concat " " (List.map string_of_int (my_filter (fun x -> x mod 2 = 0) data)))
