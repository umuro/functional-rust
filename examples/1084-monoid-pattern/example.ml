(* OCaml implementation of the Monoid pattern *)
(* A monoid consists of a type, an associative binary operation, and an identity element. *)

module type MONOID = sig
  type t
  val empty : t
  val combine : t -> t -> t
end

(* Sum Monoid *)
module Sum : MONOID with type t = int = struct
  type t = int
  let empty = 0
  let combine x y = x + y
end

(* Product Monoid *)
module Product : MONOID with type t = int = struct
  type t = int
  let empty = 1
  let combine x y = x * y
end

(* List reduction function using a Monoid *)
let reduce (type a) (module M : MONOID with type t = a) (lst : a list) : a = 
  List.fold_left M.combine M.empty lst

(* Test cases (using standard OCaml interactive top-level syntax for demonstration) *)
let () = 
  print_endline "--- Sum Monoid ---";
  let sum_list = [1; 2; 3; 4; 5] in
  let total_sum = reduce (module Sum) sum_list in
  Printf.printf "Sum of [1;2;3;4;5] is: %d\n" total_sum; (* Expected: 15 *)
  assert (total_sum = 15);

  let empty_sum = reduce (module Sum) [] in
  Printf.printf "Sum of [] is: %d\n" empty_sum; (* Expected: 0 *)
  assert (empty_sum = 0);

  print_endline "\n--- Product Monoid ---";
  let product_list = [1; 2; 3; 4; 5] in
  let total_product = reduce (module Product) product_list in
  Printf.printf "Product of [1;2;3;4;5] is: %d\n" total_product; (* Expected: 120 *)
  assert (total_product = 120);

  let empty_product = reduce (module Product) [] in
  Printf.printf "Product of [] is: %d\n" empty_product; (* Expected: 1 *)
  assert (empty_product = 1);

  print_endline "All Monoid tests passed!"
