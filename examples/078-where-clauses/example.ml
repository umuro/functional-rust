(* 078: Where Clauses — OCaml functor constraints *)
(* OCaml uses module signatures as "where clause" equivalent *)

module type SUMMABLE = sig
  type t
  val zero : t
  val add : t -> t -> t
  val to_string : t -> string
end

module type MULTIPLIABLE = sig
  type t
  val one : t
  val mul : t -> t -> t
end

(* Functor with multiple constraints *)
module MathOps (S : SUMMABLE) = struct
  let sum lst = List.fold_left S.add S.zero lst
  let sum_to_string lst =
    S.to_string (sum lst)
end

module IntSum = MathOps(struct
  type t = int
  let zero = 0
  let add = ( + )
  let to_string = string_of_int
end)

module FloatSum = MathOps(struct
  type t = float
  let zero = 0.0
  let add = ( +. )
  let to_string = string_of_float
end)

(* Complex constraint: both summable and multipliable *)
module type RING = sig
  include SUMMABLE
  include MULTIPLIABLE with type t := t
end

module RingOps (R : RING) = struct
  let dot_product a b =
    List.fold_left2 (fun acc x y -> R.add acc (R.mul x y)) R.zero a b
end

module IntRing = RingOps(struct
  type t = int
  let zero = 0
  let one = 1
  let add = ( + )
  let mul = ( * )
  let to_string = string_of_int
end)

(* Tests *)
let () =
  assert (IntSum.sum [1; 2; 3; 4; 5] = 15);
  assert (IntSum.sum_to_string [1; 2; 3] = "6");
  assert (abs_float (FloatSum.sum [1.0; 2.0; 3.0] -. 6.0) < 0.001);
  assert (IntRing.dot_product [1; 2; 3] [4; 5; 6] = 32);
  Printf.printf "✓ All tests passed\n"
