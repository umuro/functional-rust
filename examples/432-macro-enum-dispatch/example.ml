(* enum_dispatch via macros in OCaml *)

(* OCaml algebraic types already achieve this pattern natively *)
type animal =
  | Dog of string  (* name *)
  | Cat of string
  | Bird of string * bool  (* name, can_fly *)

(* This IS "enum dispatch" — pattern match is the dispatch mechanism *)
let speak = function
  | Dog _ -> "Woof!"
  | Cat _ -> "Meow!"
  | Bird _ -> "Tweet!"

let name = function
  | Dog n | Cat n | Bird (n, _) -> n

let can_fly = function
  | Bird (_, f) -> f
  | _ -> false

let describe a =
  Printf.printf "%s says: %s (flies: %b)\n"
    (name a) (speak a) (can_fly a)

let () =
  let animals = [Dog "Rex"; Cat "Whiskers"; Bird ("Tweety", true)] in
  List.iter describe animals
