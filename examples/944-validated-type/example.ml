(* 944: Validated Type — Smart Constructors and Accumulating Errors

   Smart constructors enforce invariants at the type level.
   The type is abstract — construction goes through validated constructors.

   OCaml uses modules to enforce abstraction: the concrete representation
   is hidden, and only the module interface is visible. *)

(* ── NonEmptyString ──────────────────────────────────────────────────────── *)

module NonEmptyString : sig
  type t
  val create : string -> (t, string) result
  val value  : t -> string
  val length : t -> int
  val concat : t -> t -> t
  val to_string : t -> string
end = struct
  type t = string  (* private: not visible outside module *)

  let create s =
    if s <> "" then Ok s
    else Error "string must be non-empty"

  let value s = s
  let length s = String.length s
  let concat a b = a ^ b
  let to_string s = s
end

(* ── PositiveInt ─────────────────────────────────────────────────────────── *)

module PositiveInt : sig
  type t
  val create : int -> (t, string) result
  val value  : t -> int
  val add    : t -> t -> t
  val mul    : t -> t -> t
  val to_string : t -> string
end = struct
  type t = int

  let create n =
    if n > 0 then Ok n
    else Error (Printf.sprintf "%d is not positive" n)

  let value n = n
  let add a b = a + b
  let mul a b = a * b
  let to_string n = string_of_int n
end

(* ── Validated: applicative accumulating errors ──────────────────────────── *)

(* Unlike Result which short-circuits on first error, Validated collects all *)
type ('a, 'e) validated =
  | Valid   of 'a
  | Invalid of 'e list

let valid x = Valid x
let invalid e = Invalid [e]

let is_valid = function Valid _ -> true | Invalid _ -> false

let map_v f = function
  | Valid x    -> Valid (f x)
  | Invalid es -> Invalid es

(* Apply: combine two validated values, collecting ALL errors *)
let and_v va vb =
  match (va, vb) with
  | (Valid a, Valid b)       -> Valid (a, b)
  | (Invalid e1, Invalid e2) -> Invalid (e1 @ e2)
  | (Invalid e, _) | (_, Invalid e) -> Invalid e

(* Sequence a list of validated values *)
let sequence_v vs =
  List.fold_right
    (fun v acc ->
      match (v, acc) with
      | (Valid x, Valid xs)     -> Valid (x :: xs)
      | (Invalid e1, Invalid e2) -> Invalid (e1 @ e2)
      | (Invalid e, _) | (_, Invalid e) -> Invalid e)
    vs
    (Valid [])

(* ── Validated form parsing ──────────────────────────────────────────────── *)

(* Validate a form with multiple fields simultaneously *)
type user_form = {
  name : NonEmptyString.t;
  age  : PositiveInt.t;
}

let validate_form raw_name raw_age =
  let v_name = match NonEmptyString.create raw_name with
    | Ok n    -> valid n
    | Error e -> invalid e
  in
  let v_age = match PositiveInt.create raw_age with
    | Ok a    -> valid a
    | Error e -> invalid e
  in
  match and_v v_name v_age with
  | Valid (name, age) -> valid { name; age }
  | Invalid es        -> Invalid es

let () =
  (* NonEmptyString *)
  (match NonEmptyString.create "hello" with
   | Ok s ->
     assert (NonEmptyString.value s = "hello");
     assert (NonEmptyString.length s = 5)
   | Error _ -> failwith "expected Ok");

  assert (NonEmptyString.create "" = Error "string must be non-empty");

  (* concat always produces non-empty *)
  (match (NonEmptyString.create "foo", NonEmptyString.create "bar") with
   | (Ok a, Ok b) ->
     let c = NonEmptyString.concat a b in
     assert (NonEmptyString.value c = "foobar")
   | _ -> failwith "expected Ok");

  (* PositiveInt *)
  (match PositiveInt.create 42 with
   | Ok n -> assert (PositiveInt.value n = 42)
   | Error _ -> failwith "expected Ok");

  assert (match PositiveInt.create 0  with Error _ -> true | Ok _ -> false);
  assert (match PositiveInt.create (-5) with Error _ -> true | Ok _ -> false);

  (* add/mul preserve positivity *)
  (match (PositiveInt.create 3, PositiveInt.create 4) with
   | (Ok a, Ok b) ->
     assert (PositiveInt.value (PositiveInt.add a b) = 7);
     assert (PositiveInt.value (PositiveInt.mul a b) = 12)
   | _ -> failwith "expected Ok");

  (* Validated accumulates errors *)
  let v1 : (int, string) validated = invalid "error 1" in
  let v2 : (int, string) validated = invalid "error 2" in
  (match and_v v1 v2 with
   | Invalid es -> assert (List.length es = 2)
   | Valid _    -> failwith "expected Invalid");

  assert (and_v (valid 1) (valid 2) = Valid (1, 2));

  (* validate_form: both valid *)
  (match validate_form "Alice" 30 with
   | Valid u ->
     assert (NonEmptyString.value u.name = "Alice");
     assert (PositiveInt.value u.age = 30)
   | Invalid _ -> failwith "expected Valid");

  (* validate_form: both invalid — all errors collected *)
  (match validate_form "" 0 with
   | Invalid es -> assert (List.length es = 2)
   | Valid _    -> failwith "expected Invalid");

  (* sequence_v *)
  assert (sequence_v [valid 1; valid 2; valid 3] = Valid [1; 2; 3]);
  (match sequence_v [valid 1; invalid "bad"; invalid "worse"] with
   | Invalid es -> assert (List.length es = 2)
   | Valid _    -> failwith "expected Invalid");

  print_endline "944-validated-type: all tests passed"
