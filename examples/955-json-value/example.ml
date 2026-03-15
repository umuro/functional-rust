(* 955: JSON Value Type *)
(* Approach 1: Algebraic data type definition *)

type json =
  | Null
  | Bool of bool
  | Number of float
  | Str of string
  | Array of json list
  | Object of (string * json) list

(* Approach 2: Constructors and basic operations *)

let to_string_simple j =
  match j with
  | Null -> "null"
  | Bool true -> "true"
  | Bool false -> "false"
  | Number n ->
    if Float.is_integer n then string_of_int (int_of_float n)
    else Printf.sprintf "%g" n
  | Str s -> Printf.sprintf "\"%s\"" s
  | Array _ -> "[...]"
  | Object _ -> "{...}"

let is_null = function Null -> true | _ -> false
let is_bool = function Bool _ -> true | _ -> false
let is_number = function Number _ -> true | _ -> false
let is_string = function Str _ -> true | _ -> false
let is_array = function Array _ -> true | _ -> false
let is_object = function Object _ -> true | _ -> false

(* Approach 3: Pattern matching and equality *)

let rec equal a b = match a, b with
  | Null, Null -> true
  | Bool x, Bool y -> x = y
  | Number x, Number y -> x = y
  | Str x, Str y -> x = y
  | Array xs, Array ys ->
    List.length xs = List.length ys &&
    List.for_all2 equal xs ys
  | Object xs, Object ys ->
    List.length xs = List.length ys &&
    List.for_all2 (fun (k1,v1) (k2,v2) -> k1 = k2 && equal v1 v2) xs ys
  | _ -> false

let () =
  let j_null = Null in
  let j_bool = Bool true in
  let j_num = Number 42.0 in
  let j_str = Str "hello" in
  let j_arr = Array [Number 1.0; Number 2.0; Number 3.0] in
  let j_obj = Object [("name", Str "Alice"); ("age", Number 30.0)] in

  assert (is_null j_null);
  assert (is_bool j_bool);
  assert (is_number j_num);
  assert (is_string j_str);
  assert (is_array j_arr);
  assert (is_object j_obj);

  assert (to_string_simple j_null = "null");
  assert (to_string_simple j_bool = "true");
  assert (to_string_simple j_num = "42");
  assert (to_string_simple j_str = "\"hello\"");

  assert (equal Null Null);
  assert (equal (Bool true) (Bool true));
  assert (not (equal (Bool true) (Bool false)));
  assert (equal (Number 1.0) (Number 1.0));
  assert (equal (Array [Null; Bool true]) (Array [Null; Bool true]));
  assert (not (equal (Array [Null]) (Array [Bool true])));

  Printf.printf "✓ All tests passed\n"
