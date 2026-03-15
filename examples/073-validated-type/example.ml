(* 073: Validated Types / Parse Don't Validate
   Encode invariants in the type; validation happens once at the boundary *)

(* --- Approach 1: NonEmptyString — private constructor via module --- *)

module NonEmptyString : sig
  type t
  val make   : string -> t option
  val value  : t -> string
  val length : t -> int
end = struct
  type t = string
  let make s = if String.length s = 0 then None else Some s
  let value s = s
  let length = String.length
end

(* --- Approach 2: PositiveInt — always > 0 by construction --- *)

module PositiveInt : sig
  type t
  val make  : int -> t option
  val value : t -> int
  val add   : t -> t -> t    (* sum of two positives is positive *)
end = struct
  type t = int
  let make n = if n <= 0 then None else Some n
  let value n = n
  let add a b = a + b        (* safe: no validation needed *)
end

(* --- Approach 3: Email — validated at construction --- *)

module Email : sig
  type t
  val make  : string -> (t, string) result
  val value : t -> string
end = struct
  type t = string
  let make s =
    if not (String.contains s '@') then Error "Missing @"
    else if String.length s < 3   then Error "Too short"
    else Ok s
  let value e = e
end

(* Functions that accept validated types never need to re-validate *)
let greet name =
  Printf.sprintf "Hello, %s!" (NonEmptyString.value name)

let double_positive n =
  PositiveInt.value n * 2

let () =
  (* NonEmptyString *)
  Printf.printf "empty string -> %s\n"
    (match NonEmptyString.make "" with None -> "None" | Some _ -> "Some");
  (match NonEmptyString.make "Alice" with
   | None -> ()
   | Some s ->
     Printf.printf "greet Alice: %s\n" (greet s);
     Printf.printf "length: %d\n" (NonEmptyString.length s));

  (* PositiveInt *)
  Printf.printf "PositiveInt 0  -> %s\n"
    (match PositiveInt.make 0 with None -> "None" | Some _ -> "Some");
  (match PositiveInt.make 42 with
   | None -> ()
   | Some n -> Printf.printf "double 42 = %d\n" (double_positive n));

  (* Email *)
  Printf.printf "Email 'bad' -> %s\n"
    (match Email.make "bad" with Ok _ -> "Ok" | Error e -> "Error: " ^ e);
  Printf.printf "Email 'a@b.com' -> %s\n"
    (match Email.make "a@b.com" with Ok e -> Email.value e | Error e -> "Error: " ^ e)
