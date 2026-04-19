(* Idiomatic OCaml — List.fold_left accumulates left-to-right *)
let numbers = [1; 2; 3; 4; 5]

let sum = List.fold_left (+) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let concat =
  List.fold_left
    (fun acc x -> acc ^ " " ^ string_of_int x)
    "Numbers:"
    numbers

(* Recursive OCaml — explicit fold_left, tail-recursive *)
let rec fold_left_rec f acc = function
  | [] -> acc
  | x :: xs -> fold_left_rec f (f acc x) xs

let () =
  assert (sum = 15);
  assert (product = 120);
  assert (concat = "Numbers: 1 2 3 4 5");
  assert (fold_left_rec (+) 0 numbers = 15);
  assert (List.fold_left (-) 0 numbers = -15);
  Printf.printf "Sum: %d\n" sum;
  Printf.printf "Product: %d\n" product;
  Printf.printf "%s\n" concat;
  print_endline "ok"
