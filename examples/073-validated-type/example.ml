(* 073: Parse Don't Validate — Validated Types *)

(* Approach 1: NonEmptyString *)
module NonEmptyString : sig
  type t
  val create : string -> t option
  val value : t -> string
  val length : t -> int
end = struct
  type t = string
  let create s = if String.length s = 0 then None else Some s
  let value s = s
  let length = String.length
end

(* Approach 2: PositiveInt *)
module PositiveInt : sig
  type t
  val create : int -> t option
  val value : t -> int
  val add : t -> t -> t
end = struct
  type t = int
  let create n = if n <= 0 then None else Some n
  let value n = n
  let add a b = a + b  (* always positive if inputs are *)
end

(* Approach 3: Email type *)
module Email : sig
  type t
  val create : string -> (t, string) result
  val to_string : t -> string
end = struct
  type t = string
  let create s =
    if not (String.contains s '@') then Error "Missing @"
    else if String.length s < 3 then Error "Too short"
    else Ok s
  let to_string s = s
end

(* Using validated types — no further checks needed *)
let greet name =
  Printf.sprintf "Hello, %s!" (NonEmptyString.value name)

let double_positive n =
  PositiveInt.value n * 2

(* Tests *)
let () =
  assert (NonEmptyString.create "" = None);
  assert (NonEmptyString.create "hello" <> None);
  (match NonEmptyString.create "Alice" with
   | Some name -> assert (greet name = "Hello, Alice!")
   | None -> assert false);
  assert (PositiveInt.create 0 = None);
  assert (PositiveInt.create (-5) = None);
  (match PositiveInt.create 42 with
   | Some n -> assert (double_positive n = 84)
   | None -> assert false);
  assert (Email.create "bad" = Error "Missing @");
  assert (Result.is_ok (Email.create "a@b.com"));
  Printf.printf "✓ All tests passed\n"
