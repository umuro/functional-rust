let numbers = [1; 2; 3; 4; 5]

(* Idiomatic OCaml — use List.fold_left with operator sections *)
let sum     = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers

(* Recursive fold_left — shows the explicit recursion *)
let rec fold_left_rec f acc = function
  | []      -> acc
  | x :: xs -> fold_left_rec f (f acc x) xs

let sum2 = fold_left_rec ( + ) 0 numbers

let () =
  assert (sum     = 15);
  assert (product = 120);
  assert (max_val = 5);
  assert (sum2    = 15);
  Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum product max_val;
  print_endline "ok"
