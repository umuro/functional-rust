(* 078: Where Clauses / Complex Constraints
   OCaml expresses complex constraints via module signatures and functors *)

(* --- Approach 1: Polymorphic compare + display via passed functions --- *)

(* Like Rust's where T: Display + PartialEq *)
let print_if_equal show eq a b =
  if eq a b
  then Printf.sprintf "%s == %s" (show a) (show b)
  else Printf.sprintf "%s != %s" (show a) (show b)

(* --- Approach 2: zip_with — multiple type params with constraints --- *)

let zip_with f xs ys =
  List.map2 f xs ys

(* --- Approach 3: Module constraints (= Rust where clauses on assoc. types) --- *)

module type ADDABLE = sig
  type t
  val zero : t
  val add  : t -> t -> t
end

(* sum_items: like Rust's where I::Item: Add + Default *)
module SumItems (A : ADDABLE) = struct
  let sum = List.fold_left A.add A.zero
end

module IntAdd  = SumItems (struct type t = int   let zero = 0   let add = ( + ) end)
module FloatAdd = SumItems (struct type t = float let zero = 0.0 let add = ( +. ) end)

(* dot product — requires both mul and add *)
let dot_product mul add zero xs ys =
  List.fold_left2 (fun acc x y -> add acc (mul x y)) zero xs ys

(* display_collection: like where I::Item: Display *)
let display_collection show xs =
  "[" ^ String.concat "; " (List.map show xs) ^ "]"

let () =
  Printf.printf "%s\n"
    (print_if_equal string_of_int ( = ) 5 5);
  Printf.printf "%s\n"
    (print_if_equal string_of_int ( = ) 3 4);

  Printf.printf "zip_with (+) [1;2;3] [4;5;6] = [%s]\n"
    (String.concat "; " (List.map string_of_int
      (zip_with ( + ) [1;2;3] [4;5;6])));

  Printf.printf "sum ints [1..5] = %d\n" (IntAdd.sum [1;2;3;4;5]);
  Printf.printf "sum floats = %.1f\n" (FloatAdd.sum [1.0;2.0;3.0]);

  Printf.printf "dot [1;2;3] [4;5;6] = %d\n"
    (dot_product ( * ) ( + ) 0 [1;2;3] [4;5;6]);

  Printf.printf "display_collection [1;2;3] = %s\n"
    (display_collection string_of_int [1;2;3])
