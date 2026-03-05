(* Bifunctor in OCaml *)

module type BIFUNCTOR = sig
  type ('a, 'b) t
  val bimap : ('a -> 'c) -> ('b -> 'd) -> ('a, 'b) t -> ('c, 'd) t
end

(* Tuple bifunctor *)
module PairBifunctor : BIFUNCTOR with type ('a, 'b) t = 'a * 'b = struct
  type ('a, 'b) t = 'a * 'b
  let bimap f g (a, b) = (f a, g b)
end

(* Either bifunctor *)
type ('a, 'b) either = Left of 'a | Right of 'b

module EitherBifunctor : BIFUNCTOR with type ('a, 'b) t = ('a, 'b) either = struct
  type ('a, 'b) t = ('a, 'b) either
  let bimap f g = function
    | Left a -> Left (f a)
    | Right b -> Right (g b)
end

let () =
  let pair = (10, "hello") in
  let (x, y) = PairBifunctor.bimap (( * ) 2) String.length pair in
  Printf.printf "bimap tuple: (%d, %d)\n" x y
