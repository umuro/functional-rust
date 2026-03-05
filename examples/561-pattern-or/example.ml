(* Or-patterns in OCaml match expressions *)
type color = Red | Green | Blue | Yellow | Purple

let classify c =
  match c with
  | Red | Yellow -> "warm"
  | Green | Blue -> "cool"
  | Purple       -> "mixed"

let is_primary c =
  match c with
  | Red | Green | Blue -> true
  | _                  -> false

let () =
  List.iter (fun c -> Printf.printf "%s\n" (classify c))
    [Red; Green; Blue; Yellow; Purple];
  Printf.printf "Red primary? %b\n" (is_primary Red);
  Printf.printf "Yellow primary? %b\n" (is_primary Yellow)
