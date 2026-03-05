(* Traversal via functor mapping in OCaml *)

(* Traverse a list with option effects *)
let traverse_opt f xs =
  let rec go = function
    | []      -> Some []
    | x :: xs ->
      match (f x, go xs) with
      | (Some y, Some ys) -> Some (y :: ys)
      | _                 -> None
  in go xs

(* Traverse a list collecting all results or failing *)
let parse_all_ints strings =
  traverse_opt (fun s -> try Some (int_of_string s) with _ -> None) strings

(* Traverse nested structure *)
type 'a matrix = 'a list list

let traverse_matrix f m =
  traverse_opt (traverse_opt f) m

let () =
  (match parse_all_ints ["1";"2";"3"] with
  | Some xs -> Printf.printf "parsed: %s\n" (String.concat "," (List.map string_of_int xs))
  | None    -> Printf.printf "failed\n");
  (match parse_all_ints ["1";"x";"3"] with
  | Some xs -> ignore xs
  | None    -> Printf.printf "parse failed (expected)\n")
