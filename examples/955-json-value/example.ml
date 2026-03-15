(* 955: JSON Value Type

   OCaml: algebraic data type — one variant per JSON kind.
   This is the most natural encoding in OCaml and directly mirrors
   the Rust enum translation. *)

(* ── JSON ADT ────────────────────────────────────────────────────────────── *)

type json =
  | Null
  | Bool   of bool
  | Number of float
  | Str    of string
  | Array  of json list
  | Object of (string * json) list

(* ── Type checks ─────────────────────────────────────────────────────────── *)

let is_null   = function Null     -> true | _ -> false
let is_bool   = function Bool _   -> true | _ -> false
let is_number = function Number _ -> true | _ -> false
let is_string = function Str _    -> true | _ -> false
let is_array  = function Array _  -> true | _ -> false
let is_object = function Object _ -> true | _ -> false

(* ── Simple single-line representation ──────────────────────────────────── *)

let to_string_simple = function
  | Null       -> "null"
  | Bool true  -> "true"
  | Bool false -> "false"
  | Number n ->
    if Float.is_finite n && Float.rem n 1.0 = 0.0
    then string_of_int (int_of_float n)
    else string_of_float n
  | Str s      -> Printf.sprintf "%S" s
  | Array _    -> "[...]"
  | Object _   -> "{...}"

(* ── Structural equality (built-in via = for ADTs) ──────────────────────── *)
(* OCaml's polymorphic = handles structural equality on ADTs automatically *)

(* ── Builder helpers ─────────────────────────────────────────────────────── *)

let json_object pairs = Object pairs
let json_array  items = Array items
let json_string s     = Str s
let json_number n     = Number n
let json_bool b       = Bool b
let json_null         = Null

(* ── Extraction helpers ──────────────────────────────────────────────────── *)

let to_bool   = function Bool b   -> Some b | _ -> None
let to_float  = function Number n -> Some n | _ -> None
let to_string = function Str s    -> Some s | _ -> None
let to_array  = function Array a  -> Some a | _ -> None
let to_object = function Object o -> Some o | _ -> None

let () =
  (* type checks *)
  assert (is_null   Null);
  assert (is_bool   (Bool true));
  assert (is_number (Number 1.0));
  assert (is_string (Str "x"));
  assert (is_array  (Array []));
  assert (is_object (Object []));

  (* to_string_simple *)
  assert (to_string_simple Null         = "null");
  assert (to_string_simple (Bool true)  = "true");
  assert (to_string_simple (Bool false) = "false");
  assert (to_string_simple (Number 42.0) = "42");
  assert (to_string_simple (Array [])   = "[...]");
  assert (to_string_simple (Object [])  = "{...}");

  (* structural equality — OCaml's = works on ADTs *)
  assert (Null = Null);
  assert (Bool true = Bool true);
  assert (Bool true <> Bool false);
  assert (Number 1.0 = Number 1.0);

  let arr1 = Array [Null; Bool true] in
  let arr2 = Array [Null; Bool true] in
  assert (arr1 = arr2);

  (* nested object *)
  let obj = json_object [
    ("name",   json_string "Alice");
    ("age",    json_number 30.0);
    ("active", json_bool true);
  ] in
  assert (is_object obj);
  (match obj with
   | Object pairs ->
     assert (List.length pairs = 3);
     assert (fst (List.hd pairs) = "name")
   | _ -> failwith "expected Object");

  (* extraction *)
  assert (to_bool (Bool false) = Some false);
  assert (to_float (Number 3.14) = Some 3.14);
  assert (to_array (Array [Null]) = Some [Null]);
  assert (to_string (Str "hi") = Some "hi");
  assert (to_bool Null = None);

  print_endline "955-json-value: all tests passed"
