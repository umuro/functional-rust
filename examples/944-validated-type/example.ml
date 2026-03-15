(* Smart constructors: enforce invariants at the type level.
   The type is opaque — you can only create values through validated constructors. *)

(* Non-empty string *)
module NonEmptyString : sig
  type t
  val create : string -> (t, string) result
  val value  : t -> string
  val length : t -> int
end = struct
  type t = string
  let create s =
    if String.length s > 0 then Ok s
    else Error "string must be non-empty"
  let value s = s
  let length s = String.length s
end

(* Positive integer *)
module PositiveInt : sig
  type t
  val create : int -> (t, string) result
  val value  : t -> int
  val add    : t -> t -> t
end = struct
  type t = int
  let create n =
    if n > 0 then Ok n
    else Error (Printf.sprintf "%d is not positive" n)
  let value n = n
  let add a b = a + b
end

let () =
  (match NonEmptyString.create "hello" with
   | Ok s  -> Printf.printf "NonEmpty: %s (len %d)\n" (NonEmptyString.value s) (NonEmptyString.length s)
   | Error e -> Printf.printf "Error: %s\n" e);

  (match NonEmptyString.create "" with
   | Ok _  -> assert false
   | Error e -> Printf.printf "Rejected: %s\n" e);

  (match PositiveInt.create 42, PositiveInt.create (-1) with
   | Ok p, Error e ->
     Printf.printf "PositiveInt: %d; rejected: %s\n" (PositiveInt.value p) e
   | _ -> assert false)
