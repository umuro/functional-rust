(* Binary strings *)
(* Rosetta Code Binary strings implementation in OCaml *)

# let str = "some text" ;;
val str : string = "some text"

(* modifying a character, OCaml strings are mutable *)
# str.[0] <- 'S' ;;
- : unit = ()
