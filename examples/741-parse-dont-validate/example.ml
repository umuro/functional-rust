(* 741: Parse Don't Validate — OCaml with module privacy *)

(* Email: can only be constructed via Email.parse *)
module Email : sig
  type t
  val parse  : string -> (t, string) result
  val to_string : t -> string
end = struct
  type t = string  (* private representation *)

  let parse s =
    (* Simple validation: must contain @ and a dot after @ *)
    match String.split_on_char '@' s with
    | [local; domain] when String.length local > 0
                       && String.contains domain '.' ->
        Ok s
    | _ -> Error (Printf.sprintf "'%s' is not a valid email" s)

  let to_string t = t
end

(* NonEmptyString: guaranteed non-empty *)
module NonEmpty : sig
  type t
  val parse     : string -> (t, string) result
  val to_string : t -> string
  val length    : t -> int
end = struct
  type t = string
  let parse s =
    if String.length s = 0 then Error "string is empty"
    else Ok s
  let to_string t = t
  let length t = String.length t
end

(* BoundedInt: integer in range [lo, hi] *)
module BoundedInt : sig
  type t
  val make      : lo:int -> hi:int -> int -> (t, string) result
  val value     : t -> int
end = struct
  type t = int
  let make ~lo ~hi n =
    if n >= lo && n <= hi then Ok n
    else Error (Printf.sprintf "%d not in [%d, %d]" n lo hi)
  let value t = t
end

let () =
  (* Email *)
  (match Email.parse "user@example.com" with
  | Ok e -> Printf.printf "Valid email: %s\n" (Email.to_string e)
  | Error e -> Printf.printf "Error: %s\n" e);
  (match Email.parse "notanemail" with
  | Ok _ -> ()
  | Error e -> Printf.printf "Error: %s\n" e);

  (* NonEmpty *)
  (match NonEmpty.parse "" with
  | Ok _ -> ()
  | Error e -> Printf.printf "Error: %s\n" e);
  (match NonEmpty.parse "hello" with
  | Ok s -> Printf.printf "NonEmpty: %s (len=%d)\n" (NonEmpty.to_string s) (NonEmpty.length s)
  | Error _ -> ());

  (* BoundedInt *)
  (match BoundedInt.make ~lo:1 ~hi:100 42 with
  | Ok n -> Printf.printf "Bounded: %d\n" (BoundedInt.value n)
  | Error e -> Printf.printf "Error: %s\n" e);
  (match BoundedInt.make ~lo:1 ~hi:100 999 with
  | Ok _ -> ()
  | Error e -> Printf.printf "Error: %s\n" e)
