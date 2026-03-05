(* Semigroup Laws in OCaml *)

module type SEMIGROUP = sig
  type t
  val combine : t -> t -> t
end

module SumSemigroup : SEMIGROUP with type t = int = struct
  type t = int
  let combine a b = a + b
end

module StringSemigroup : SEMIGROUP with type t = string = struct
  type t = string
  let combine a b = a ^ b
end

let verify_associativity (type a) (module S : SEMIGROUP with type t = a) a b c =
  S.combine (S.combine a b) c = S.combine a (S.combine b c)

let () =
  let open SumSemigroup in
  let a, b, c = 1, 2, 3 in
  let left = combine (combine a b) c in
  let right = combine a (combine b c) in
  Printf.printf "Sum: %d = %d, associative: %b\n" left right (left = right)
