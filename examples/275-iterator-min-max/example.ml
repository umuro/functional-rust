(* 275: min and max over a sequence — returns Option, None for empty.
   OCaml: List.fold_left with comparison, or explicit pattern match.
   Also: List.find_map with min_by_key idiom. *)

(* min_list: minimum element or None *)
let min_list = function
  | []      -> None
  | x :: xs -> Some (List.fold_left min x xs)

(* max_list: maximum element or None *)
let max_list = function
  | []      -> None
  | x :: xs -> Some (List.fold_left max x xs)

(* min/max by key function *)
let min_by_key key = function
  | []      -> None
  | x :: xs ->
    Some (List.fold_left (fun acc el ->
      if key el < key acc then el else acc) x xs)

let max_by_key key = function
  | []      -> None
  | x :: xs ->
    Some (List.fold_left (fun acc el ->
      if key el > key acc then el else acc) x xs)

let () =
  let v = [5; 3; 8; 1; 9; 2] in

  Printf.printf "min [5;3;8;1;9;2] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (min_list v));
  Printf.printf "max [5;3;8;1;9;2] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (max_list v));

  (* Empty *)
  Printf.printf "min [] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (min_list []));
  Printf.printf "max [] = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int (max_list []));

  (* min_by_key / max_by_key using string length *)
  let words = ["hello"; "hi"; "world"] in
  Printf.printf "shortest word = %s\n"
    (Option.value ~default:"?" (min_by_key String.length words));
  Printf.printf "longest  word = %s\n"
    (Option.value ~default:"?" (max_by_key String.length words));

  (* OCaml built-in: List.sort + List.hd for min *)
  let sorted = List.sort compare v in
  Printf.printf "min via sort: %d  max via sort: %d\n"
    (List.hd sorted) (List.hd (List.rev sorted))
