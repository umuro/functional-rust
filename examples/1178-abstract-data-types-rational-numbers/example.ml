(* Abstract Data Types — Rational Numbers *)
(* Encapsulate representation with abstract types *)

module Rational : sig
  type t
  val make : int -> int -> t
  val add : t -> t -> t
  val mul : t -> t -> t
  val to_string : t -> string
end = struct
  type t = { num: int; den: int }

  let gcd a b =
    let rec aux a b = if b = 0 then a else aux b (a mod b) in
    aux (abs a) (abs b)

  let make n d =
    if d = 0 then failwith "zero denominator";
    let g = gcd n d in
    let sign = if d < 0 then -1 else 1 in
    { num = sign * n / g; den = sign * d / g }

  let add a b = make (a.num * b.den + b.num * a.den) (a.den * b.den)
  let mul a b = make (a.num * b.num) (a.den * b.den)
  let to_string r = Printf.sprintf "%d/%d" r.num r.den
end

let a = Rational.make 1 3
let b = Rational.make 1 6
let () = Printf.printf "%s + %s = %s\n"
  (Rational.to_string a) (Rational.to_string b)
  (Rational.to_string (Rational.add a b))
