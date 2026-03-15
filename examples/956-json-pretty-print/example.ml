(* 956: JSON Pretty Print *)

type json =
  | Null
  | Bool of bool
  | Number of float
  | Str of string
  | Array of json list
  | Object of (string * json) list

(* Approach 1: Simple recursive pretty-printer with indentation *)

let escape_string s =
  let buf = Buffer.create (String.length s + 2) in
  String.iter (fun c ->
    match c with
    | '"' -> Buffer.add_string buf "\\\""
    | '\\' -> Buffer.add_string buf "\\\\"
    | '\n' -> Buffer.add_string buf "\\n"
    | '\t' -> Buffer.add_string buf "\\t"
    | '\r' -> Buffer.add_string buf "\\r"
    | c -> Buffer.add_char buf c
  ) s;
  Buffer.contents buf

let rec pretty_print ?(indent=0) j =
  let pad = String.make (indent * 2) ' ' in
  let pad2 = String.make ((indent + 1) * 2) ' ' in
  match j with
  | Null -> "null"
  | Bool true -> "true"
  | Bool false -> "false"
  | Number n ->
    if Float.is_integer n then string_of_int (int_of_float n)
    else Printf.sprintf "%g" n
  | Str s -> Printf.sprintf "\"%s\"" (escape_string s)
  | Array [] -> "[]"
  | Array items ->
    let items_str = List.map (fun item ->
      pad2 ^ pretty_print ~indent:(indent+1) item
    ) items in
    "[\n" ^ String.concat ",\n" items_str ^ "\n" ^ pad ^ "]"
  | Object [] -> "{}"
  | Object pairs ->
    let pairs_str = List.map (fun (k, v) ->
      pad2 ^ Printf.sprintf "\"%s\": %s" (escape_string k) (pretty_print ~indent:(indent+1) v)
    ) pairs in
    "{\n" ^ String.concat ",\n" pairs_str ^ "\n" ^ pad ^ "}"

(* Approach 2: Compact (single-line) printer *)

let rec compact j =
  match j with
  | Null -> "null"
  | Bool true -> "true"
  | Bool false -> "false"
  | Number n ->
    if Float.is_integer n then string_of_int (int_of_float n)
    else Printf.sprintf "%g" n
  | Str s -> Printf.sprintf "\"%s\"" (escape_string s)
  | Array items ->
    "[" ^ String.concat "," (List.map compact items) ^ "]"
  | Object pairs ->
    "{" ^ String.concat "," (List.map (fun (k,v) ->
      Printf.sprintf "\"%s\":%s" k (compact v)
    ) pairs) ^ "}"

let () =
  let json = Object [
    ("name", Str "Alice");
    ("age", Number 30.0);
    ("scores", Array [Number 95.0; Number 87.0; Number 92.0]);
    ("address", Object [("city", Str "Amsterdam"); ("zip", Str "1234AB")]);
    ("active", Bool true);
    ("note", Null);
  ] in

  let pretty = pretty_print json in
  assert (String.length pretty > 0);
  assert (String.sub pretty 0 1 = "{");

  let c = compact json in
  assert (String.length c > 0);
  assert (not (String.contains c '\n'));

  (* Test escape *)
  let with_special = Str "hello \"world\"\nnewline" in
  let escaped = pretty_print with_special in
  assert (escaped = "\"hello \\\"world\\\"\\nnewline\"");

  Printf.printf "%s\n" pretty;
  Printf.printf "✓ All tests passed\n"
