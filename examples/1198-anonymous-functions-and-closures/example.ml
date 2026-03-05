(* Anonymous Functions and Closures *)
(* Lambda expressions (fun keyword) *)

(* Anonymous functions with fun *)
let apply f x = f x
let apply2 f x y = f x y

let () =
  Printf.printf "%d\n" (apply (fun x -> x * x) 5);
  Printf.printf "%d\n" (apply2 (fun x y -> x + y) 3 4);

  (* Multi-argument anonymous function *)
  let result = List.map (fun x -> x * x + 1) [1;2;3;4;5] in
  List.iter (fun x -> Printf.printf "%d " x) result;
  print_newline ();

  (* Nested anonymous functions *)
  let make_pair = fun x -> fun y -> (x, y) in
  let (a, b) = make_pair 1 2 in
  Printf.printf "(%d, %d)\n" a b
