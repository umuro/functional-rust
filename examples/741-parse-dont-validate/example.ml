(* 741: Parse-Don't-Validate — types that can only be constructed via parsing *)
(* Rust uses private fields and associated parse() constructors to ensure
   values are always valid once created. OCaml achieves the same by hiding
   the constructor behind a module signature. *)

(* ── Error type ─────────────────────────────────────────────────────────── *)

type parse_error =
  | EmptyString
  | InvalidEmail of string
  | OutOfRange of { value : int; lo : int; hi : int }
  | InvalidChar of char

let pp_parse_error = function
  | EmptyString -> "string is empty"
  | InvalidEmail s -> Printf.sprintf "'%s' is not a valid email" s
  | OutOfRange { value; lo; hi } ->
    Printf.sprintf "%d not in range [%d, %d]" value lo hi
  | InvalidChar c -> Printf.sprintf "invalid character '%c'" c

(* ── NonEmptyString ─────────────────────────────────────────────────────── *)
(* The module signature hides the constructor — you cannot build a
   NonEmptyString without going through parse. *)

module NonEmptyString : sig
  type t
  val parse  : string -> (t, parse_error) result
  val as_str : t -> string
  val length : t -> int
end = struct
  type t = string
  let parse s =
    if String.length s = 0 then Error EmptyString
    else Ok s
  let as_str s = s
  let length s = String.length s
end

(* ── Email ──────────────────────────────────────────────────────────────── *)

module Email : sig
  type t
  val parse      : string -> (t, parse_error) result
  val as_str     : t -> string
  val local_part : t -> string
  val domain     : t -> string
end = struct
  type t = string

  let parse s =
    match String.index_opt s '@' with
    | None -> Error (InvalidEmail s)
    | Some at ->
      let local  = String.sub s 0 at in
      let domain = String.sub s (at + 1) (String.length s - at - 1) in
      if String.length local = 0
      || not (String.contains domain '.')
      || domain.[0] = '.'
      then Error (InvalidEmail s)
      else Ok (String.lowercase_ascii s)

  let as_str e = e

  let local_part e =
    match String.index_opt e '@' with
    | Some at -> String.sub e 0 at
    | None    -> e

  let domain e =
    match String.index_opt e '@' with
    | Some at -> String.sub e (at + 1) (String.length e - at - 1)
    | None    -> ""
end

(* ── BoundedInt — integer constrained to [lo, hi] ───────────────────────── *)

module type BOUNDS = sig val lo : int val hi : int end

module BoundedInt (B : BOUNDS) : sig
  type t
  val parse : int -> (t, parse_error) result
  val value : t -> int
end = struct
  type t = int
  let parse n =
    if n < B.lo || n > B.hi
    then Error (OutOfRange { value = n; lo = B.lo; hi = B.hi })
    else Ok n
  let value n = n
end

(* ── Functions that REQUIRE parsed types ───────────────────────────────── *)

let send_welcome email =
  Printf.sprintf "Welcome email sent to %s" (Email.as_str email)

let create_account username email =
  Printf.sprintf "Account '%s' created with email %s"
    (NonEmptyString.as_str username) (Email.as_str email)

let () =
  (* valid email *)
  let e = Result.get_ok (Email.parse "user@example.com") in
  assert (Email.domain e = "example.com");
  assert (Email.local_part e = "user");
  print_endline "valid email: ok";

  (* email normalized to lowercase *)
  let e2 = Result.get_ok (Email.parse "USER@EXAMPLE.COM") in
  assert (Email.as_str e2 = "user@example.com");
  print_endline "email lowercase: ok";

  (* invalid emails rejected *)
  assert (Result.is_error (Email.parse ""));
  assert (Result.is_error (Email.parse "noatsign"));
  assert (Result.is_error (Email.parse "@nodomain"));
  assert (Result.is_error (Email.parse "user@nodot"));
  print_endline "invalid emails: ok";

  (* non-empty string *)
  let s = Result.get_ok (NonEmptyString.parse "hello") in
  assert (NonEmptyString.length s = 5);
  assert (NonEmptyString.parse "" = Error EmptyString);
  print_endline "non_empty_string: ok";

  (* bounded int *)
  let module Score = BoundedInt(struct let lo = 0 let hi = 10 end) in
  assert (Score.value (Result.get_ok (Score.parse 5)) = 5);
  assert (Result.is_error (Score.parse (-1)));
  assert (Result.is_error (Score.parse 11));
  print_endline "bounded_int: ok";

  (* functions that require valid types *)
  let msg = send_welcome e in
  assert (String.length msg > 0);
  let ne = Result.get_ok (NonEmptyString.parse "alice") in
  let account = create_account ne e in
  assert (String.length account > 0);
  print_endline "domain functions: ok";

  (* show error messages *)
  print_endline (pp_parse_error EmptyString);
  print_endline (pp_parse_error (InvalidEmail "bad"));

  print_endline "All assertions passed."
