(* Example 183: Heterogeneous Vector with Safe Downcast *)

(* Approach 1: GADT-based heterogeneous list *)
type _ ty =
  | TInt    : int ty
  | TString : string ty
  | TBool   : bool ty
  | TFloat  : float ty

type entry = Entry : 'a ty * 'a -> entry

let get_int (Entry (ty, v)) : int option =
  match ty with TInt -> Some v | _ -> None

let get_string (Entry (ty, v)) : string option =
  match ty with TString -> Some v | _ -> None

let get_bool (Entry (ty, v)) : bool option =
  match ty with TBool -> Some v | _ -> None

let to_string_entry (Entry (ty, v)) =
  match ty with
  | TInt -> string_of_int v
  | TString -> v
  | TBool -> string_of_bool v
  | TFloat -> string_of_float v

(* Approach 2: Using polymorphic variants + objects *)
type value =
  | VInt of int
  | VStr of string
  | VBool of bool
  | VFloat of float

let value_to_string = function
  | VInt n -> string_of_int n
  | VStr s -> s
  | VBool b -> string_of_bool b
  | VFloat f -> string_of_float f

let as_int = function VInt n -> Some n | _ -> None
let as_string = function VStr s -> Some s | _ -> None

(* Approach 3: Dynamic type with type witness *)
type (_, _) eq = Refl : ('a, 'a) eq

module type DYNAMIC = sig
  type t
  val inject : 'a ty -> 'a -> t
  val project : 'a ty -> t -> 'a option
end

module Dynamic : DYNAMIC = struct
  type t = entry
  let inject ty v = Entry (ty, v)
  let project : type a. a ty -> t -> a option = fun ty (Entry (ty', v)) ->
    match ty, ty' with
    | TInt, TInt -> Some v
    | TString, TString -> Some v
    | TBool, TBool -> Some v
    | TFloat, TFloat -> Some v
    | _ -> None
end

let () =
  (* Test Approach 1 *)
  let entries = [
    Entry (TInt, 42);
    Entry (TString, "hello");
    Entry (TBool, true);
    Entry (TFloat, 3.14);
  ] in
  assert (get_int (List.nth entries 0) = Some 42);
  assert (get_int (List.nth entries 1) = None);
  assert (get_string (List.nth entries 1) = Some "hello");
  let strs = List.map to_string_entry entries in
  assert (List.nth strs 0 = "42");
  assert (List.nth strs 1 = "hello");

  (* Test Approach 2 *)
  let vals = [VInt 1; VStr "x"; VBool false] in
  assert (as_int (List.nth vals 0) = Some 1);
  assert (as_int (List.nth vals 1) = None);
  assert (as_string (List.nth vals 1) = Some "x");

  (* Test Approach 3 *)
  let d = Dynamic.inject TInt 99 in
  assert (Dynamic.project TInt d = Some 99);
  assert (Dynamic.project TString d = None);

  print_endline "✓ All tests passed"
