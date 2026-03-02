let rec filter p = function
  | [] -> []
  | h :: t -> if p h then h :: filter p t else filter p t

let evens = filter (fun n -> n mod 2 = 0)
let odds = filter (fun n -> n mod 2 <> 0)
let pos = filter (fun n -> n > 0)

let () =
  let nums = [-3; -1; 0; 2; 4; 5; 7] in
  List.iter (Printf.printf "%d ") (evens nums); print_newline ();
  List.iter (Printf.printf "%d ") (odds nums); print_newline ();
  List.iter (Printf.printf "%d ") (pos nums); print_newline ()
