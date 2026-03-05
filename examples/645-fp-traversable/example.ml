(* Traversable in OCaml *)

let traverse_option f xs =
  let rec go acc = function
    | [] -> Some (List.rev acc)
    | x :: rest ->
      match f x with
      | None -> None
      | Some y -> go (y :: acc) rest
  in
  go [] xs

let sequence_option xs = traverse_option Fun.id xs

let parse_int s =
  try Some (int_of_string s)
  with Failure _ -> None

let () =
  let strings = ["1"; "2"; "3"] in
  match traverse_option parse_int strings with
  | Some nums -> Printf.printf "Parsed: [%s]\n" 
      (String.concat "; " (List.map string_of_int nums))
  | None -> print_endline "Failed"
