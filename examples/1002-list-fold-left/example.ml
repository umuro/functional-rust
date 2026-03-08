(* List fold_left in OCaml
 * Demonstrates the standard library's List.fold_left and a custom recursive implementation
 *)

(* Standard library approach using List.fold_left *)
let fold_left_stdlib numbers =
  let sum = List.fold_left ( + ) 0 numbers in
  let product = List.fold_left ( * ) 1 numbers in
  let max_val = List.fold_left max min_int numbers in
  (sum, product, max_val)

(* Custom recursive implementation (like fold_left does internally) *)
let rec fold_left_recursive f acc = function
  | [] -> acc
  | head :: tail -> fold_left_recursive f (f acc head) tail

let fold_left_custom numbers =
  let sum = fold_left_recursive ( + ) 0 numbers in
  let product = fold_left_recursive ( * ) 1 numbers in
  let max_val = fold_left_recursive max min_int numbers in
  (sum, product, max_val)

(* Convenience functions *)
let sum numbers = List.fold_left ( + ) 0 numbers
let product numbers = List.fold_left ( * ) 1 numbers
let max_value numbers = List.fold_left max min_int numbers
let min_value numbers = List.fold_left min max_int numbers

(* Test with assertions *)
let () =
  let numbers = [1; 2; 3; 4; 5] in

  (* Test standard library *)
  let (sum_std, prod_std, max_std) = fold_left_stdlib numbers in
  Printf.printf "=== Standard library fold_left ===\n";
  Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum_std prod_std max_std;
  assert (sum_std = 15);
  assert (prod_std = 120);
  assert (max_std = 5);

  (* Test custom recursive *)
  let (sum_rec, prod_rec, max_rec) = fold_left_custom numbers in
  Printf.printf "\n=== Custom recursive fold_left ===\n";
  Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum_rec prod_rec max_rec;
  assert (sum_rec = 15);
  assert (prod_rec = 120);
  assert (max_rec = 5);

  (* Test convenience functions *)
  Printf.printf "\n=== Convenience functions ===\n";
  Printf.printf "Sum: %d, Product: %d, Max: %d\n" (sum numbers) (product numbers) (max_value numbers);
  assert (sum numbers = 15);
  assert (product numbers = 120);
  assert (max_value numbers = 5);

  (* Test edge cases *)
  Printf.printf "\n=== Edge cases ===\n";
  let empty = [] in
  Printf.printf "Empty list - sum: %d, product: %d, max: %d\n" (sum empty) (product empty) (max_value empty);
  assert (sum empty = 0);
  assert (product empty = 1);
  assert (max_value empty = min_int);

  let single = [42] in
  Printf.printf "Single element [42] - sum: %d, product: %d, max: %d\n" (sum single) (product single) (max_value single);
  assert (sum single = 42);
  assert (product single = 42);
  assert (max_value single = 42);

  let negative = [-5; 10; -3] in
  Printf.printf "Negative values [-5; 10; -3] - sum: %d, product: %d, max: %d\n" (sum negative) (product negative) (max_value negative);
  assert (sum negative = 2);
  assert (product negative = 150);
  assert (max_value negative = 10);

  Printf.printf "\n✓ All assertions passed!\n"
