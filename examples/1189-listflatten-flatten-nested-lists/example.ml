let nested = [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]
let flat = List.flatten nested
let () = Printf.printf "Flat: %s\n"
  (String.concat " " (List.map string_of_int flat))
(* Also useful: List.concat_map *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]
