(* List.flatten — Flatten Nested Lists *)
(* Concatenate a list of lists into a single list *)

let nested = [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]
let flat = List.flatten nested
let () = Printf.printf "Flat: %s\n"
  (String.concat " " (List.map string_of_int flat))
(* Also useful: List.concat_map *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]

let () =
  assert (List.flatten [[1; 2]; [3; 4; 5]; [6]] = [1; 2; 3; 4; 5; 6]);
  assert (List.flatten [] = []);
  assert (List.flatten [[]; [1]; []] = [1]);
  assert (pairs = [1; 10; 2; 20; 3; 30]);
  print_endline "ok"
