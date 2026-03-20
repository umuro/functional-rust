(* Idiomatic OCaml: List.flatten flattens a list of lists *)
let nested = [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]
let flat = List.flatten nested

(* Recursive OCaml: append head list to flattened tail *)
let rec flatten_rec = function
  | [] -> []
  | head :: rest -> head @ flatten_rec rest

(* concat_map: flat_map each element to a derived list *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]

let () =
  assert (flat = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]);
  assert (flatten_rec nested = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]);
  assert (pairs = [1; 10; 2; 20; 3; 30]);
  Printf.printf "Flat: %s\n"
    (String.concat " " (List.map string_of_int flat));
  print_endline "ok"
