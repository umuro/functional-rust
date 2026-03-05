(* Modules — Private Types *)
(* Hide constructors with private types in signatures *)

module PositiveInt : sig
  type t = private int
  val of_int : int -> t option
  val to_int : t -> int
  val add : t -> t -> t
end = struct
  type t = int
  let of_int n = if n > 0 then Some n else None
  let to_int n = n
  let add a b = a + b
end

let () = match PositiveInt.of_int 42 with
  | Some n ->
    Printf.printf "Positive: %d\n" (PositiveInt.to_int n);
    (* Can read as int: *)
    Printf.printf "As int: %d\n" (n :> int)
  | None -> print_endline "Not positive"
