(* Contravariant Functor in OCaml *)

module type CONTRAVARIANT = sig
  type 'a t
  val contramap : ('b -> 'a) -> 'a t -> 'b t
end

(* Predicate as contravariant *)
module Predicate : sig
  type 'a t
  val create : ('a -> bool) -> 'a t
  val test : 'a t -> 'a -> bool
  val contramap : ('b -> 'a) -> 'a t -> 'b t
end = struct
  type 'a t = 'a -> bool
  let create f = f
  let test p x = p x
  let contramap f p = fun b -> p (f b)
end

(* Comparator as contravariant *)
let contramap_compare f cmp =
  fun b1 b2 -> cmp (f b1) (f b2)

let () =
  let is_positive = Predicate.create (fun x -> x > 0) in
  let len_positive = Predicate.contramap String.length is_positive in
  Printf.printf "\"hello\" has positive length: %b\n" 
    (Predicate.test len_positive "hello")
