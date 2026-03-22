(* Idiomatic OCaml: fold_left reduces a list with an accumulator *)
let numbers = [1; 2; 3; 4; 5]
let sum = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers

(* Recursive OCaml: explicit tail-recursive fold *)
let rec fold_left_rec f acc = function
  | [] -> acc
  | x :: rest -> fold_left_rec f (f acc x) rest

let () =
  assert (sum = 15);
  assert (product = 120);
  assert (max_val = 5);
  assert (List.fold_left ( + ) 0 [] = 0);
  assert (fold_left_rec ( + ) 0 [1;2;3;4;5] = 15);
  assert (fold_left_rec ( * ) 1 [1;2;3;4;5] = 120);
  print_endline "ok"
