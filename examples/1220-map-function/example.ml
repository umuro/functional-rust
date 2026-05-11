let rec map f lst =
  match lst with
  | [] -> []
  | hd :: tl -> f hd :: map f tl

let double x = 2 * x

let () =
  let nums = [1; 2; 3; 4] in
  let doubled = map double nums in
  List.iter (Printf.printf "%d ") doubled;  (* 2 4 6 8 *)
  print_newline ()