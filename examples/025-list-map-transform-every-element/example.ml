let numbers = [1; 2; 3; 4; 5]
let doubled = List.map (fun x -> x * 2) numbers
let () = List.iter (fun x -> Printf.printf "%d " x) doubled
(* Output: 2 4 6 8 10 *)