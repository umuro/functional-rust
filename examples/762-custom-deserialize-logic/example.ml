(* Custom deserialization with visitor pattern in OCaml
   We implement a visitor-style callback interface *)

(* The "deserializer" calls into a visitor *)
type 'a visit_result = Ok of 'a | Err of string

(* Visitor module type — the type being built provides this *)
module type VISITOR = sig
  type output
  val visit_string : string -> output visit_result
  val visit_int    : int    -> output visit_result
  val visit_float  : float  -> output visit_result
  val visit_bool   : bool   -> output visit_result
  val visit_seq    : string list -> output visit_result
  val visit_map    : (string * string) list -> output visit_result
  val expecting    : string  (* human-readable description *)
end

(* Deserializer drives the process *)
module Deserializer = struct
  type token =
    | TString of string
    | TInt    of int
    | TFloat  of float
    | TBool   of bool
    | TMap    of (string * string) list

  (* Parse a simple wire format into a token *)
  let parse s =
    if String.length s > 4 && String.sub s 0 4 = "str:" then
      TString (String.sub s 4 (String.length s - 4))
    else if String.length s > 4 && String.sub s 0 4 = "int:" then
      TInt (int_of_string (String.sub s 4 (String.length s - 4)))
    else if String.length s > 6 && String.sub s 0 6 = "float:" then
      TFloat (float_of_string (String.sub s 6 (String.length s - 6)))
    else if s = "true" then TBool true
    else if s = "false" then TBool false
    else if String.length s > 4 && String.sub s 0 4 = "map:" then
      let rest = String.sub s 4 (String.length s - 4) in
      let pairs =
        String.split_on_char ',' rest
        |> List.filter_map (fun p ->
          match String.split_on_char '=' p with
          | [k; v] -> Some (k, v)
          | _ -> None)
      in
      TMap pairs
    else TString s   (* fallback *)

  let drive (type a) (module V : VISITOR with type output = a) token =
    match token with
    | TString s -> V.visit_string s
    | TInt    i -> V.visit_int i
    | TFloat  f -> V.visit_float f
    | TBool   b -> V.visit_bool b
    | TMap    m -> V.visit_map m
end

(* ---------- Domain type ---------- *)
type person = { name: string; age: int }

module PersonVisitor : VISITOR with type output = person = struct
  type output = person
  let expecting = "a map with name and age"
  let visit_string _ = Err "expected map, got string"
  let visit_int    _ = Err "expected map, got int"
  let visit_float  _ = Err "expected map, got float"
  let visit_bool   _ = Err "expected map, got bool"
  let visit_seq    _ = Err "expected map, got seq"
  let visit_map pairs =
    match List.assoc_opt "name" pairs, List.assoc_opt "age" pairs with
    | Some name, Some age_s ->
      (try Ok { name; age = int_of_string age_s }
       with Failure _ -> Err "age is not an int")
    | _ -> Err ("missing field; expecting: " ^ expecting)
end

let () =
  let wire = "map:name=Alice,age=30" in
  let token = Deserializer.parse wire in
  match Deserializer.drive (module PersonVisitor) token with
  | Ok p  -> Printf.printf "Got person: %s, age %d\n" p.name p.age
  | Err e -> Printf.printf "Error: %s\n" e
