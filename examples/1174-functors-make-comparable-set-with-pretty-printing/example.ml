(* Functors — Make Comparable Set with Pretty Printing *)
(* Combine functors with custom printing *)

module type PRINTABLE_COMPARABLE = sig
  type t
  val compare : t -> t -> int
  val to_string : t -> string
end

module MakePrintableSet (E : PRINTABLE_COMPARABLE) = struct
  include Set.Make(E)
  let to_string s =
    "{" ^ String.concat ", " (List.map E.to_string (elements s)) ^ "}"
end

module PIntSet = MakePrintableSet(struct
  type t = int
  let compare = compare
  let to_string = string_of_int
end)

let s = PIntSet.of_list [3; 1; 4; 1; 5; 9]
let () = Printf.printf "Set: %s (size: %d)\n"
  (PIntSet.to_string s) (PIntSet.cardinal s)
