(* OCaml: higher-order functions are first-class; no trampoline needed.
   Closures capture freely. *)

(** Simulated C for-each: applies f to every element. *)
let c_for_each (arr : int array) (f : int -> unit) : unit =
  Array.iter f arr

(** Simulated C reduce: folds arr with f starting from init. *)
let c_reduce (arr : int array) (init : int) (f : int -> int -> int) : int =
  Array.fold_left f init arr

let () =
  let data = [| 1; 2; 3; 4; 5 |] in
  print_string "Elements: ";
  c_for_each data (fun x -> Printf.printf "%d " x);
  print_newline ();
  let sum = c_reduce data 0 ( + ) in
  Printf.printf "Sum: %d\n" sum;
  let product = c_reduce data 1 ( * ) in
  Printf.printf "Product: %d\n" product
