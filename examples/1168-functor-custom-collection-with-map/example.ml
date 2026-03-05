(* Functor — Custom Collection with Map *)
(* Build a generic interval module using functors *)

module type BOUNDED = sig
  type t
  val compare : t -> t -> int
  val to_string : t -> string
end

module MakeInterval (B : BOUNDED) = struct
  type t = Empty | Range of B.t * B.t

  let create lo hi =
    if B.compare lo hi > 0 then Empty else Range (lo, hi)

  let contains iv x = match iv with
    | Empty -> false
    | Range (lo, hi) -> B.compare x lo >= 0 && B.compare x hi <= 0

  let to_string = function
    | Empty -> "empty"
    | Range (lo, hi) -> Printf.sprintf "[%s, %s]" (B.to_string lo) (B.to_string hi)
end

module IntInterval = MakeInterval(struct
  type t = int
  let compare = compare
  let to_string = string_of_int
end)

let iv = IntInterval.create 1 10
let () = Printf.printf "%s contains 5: %b\n"
  (IntInterval.to_string iv) (IntInterval.contains iv 5)
