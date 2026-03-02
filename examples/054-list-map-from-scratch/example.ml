let rec map f = function
  | [] -> []
  | h :: t -> let h' = f h in h' :: map f t

let add1 = map (fun x -> x + 1)
let to_string = map string_of_int
let double = map (fun x -> x * 2)

let () =
  let nums = [1; 2; 3; 4; 5] in
  List.iter (Printf.printf "%d ") (add1 nums); print_newline ();
  List.iter (Printf.printf "%s ") (to_string nums); print_newline ();
  List.iter (Printf.printf "%d ") (double nums); print_newline ()
