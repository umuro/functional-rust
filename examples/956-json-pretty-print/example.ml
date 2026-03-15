(* 956: JSON Pretty Print

   Recursive pretty-printer: OCaml uses Buffer for efficient string building.
   Two modes: pretty (indented multi-line) and compact (single-line). *)

type json =
  | Null
  | Bool   of bool
  | Number of float
  | Str    of string
  | Array  of json list
  | Object of (string * json) list

(* ── String escaping ─────────────────────────────────────────────────────── *)

let escape_string s =
  let buf = Buffer.create (String.length s) in
  String.iter (fun c ->
    match c with
    | '"'  -> Buffer.add_string buf "\\\""
    | '\\' -> Buffer.add_string buf "\\\\"
    | '\n' -> Buffer.add_string buf "\\n"
    | '\t' -> Buffer.add_string buf "\\t"
    | '\r' -> Buffer.add_string buf "\\r"
    | c    -> Buffer.add_char buf c
  ) s;
  Buffer.contents buf

(* ── Number formatting ───────────────────────────────────────────────────── *)

let format_number n =
  if Float.is_finite n && Float.rem n 1.0 = 0.0
  then string_of_int (int_of_float n)
  else string_of_float n

(* ── Pretty printer (indented) ───────────────────────────────────────────── *)

let rec pretty_print indent j =
  let pad  = String.make (indent * 2) ' ' in
  let pad2 = String.make ((indent + 1) * 2) ' ' in
  match j with
  | Null       -> "null"
  | Bool true  -> "true"
  | Bool false -> "false"
  | Number n   -> format_number n
  | Str s      -> Printf.sprintf "\"%s\"" (escape_string s)
  | Array []   -> "[]"
  | Object []  -> "{}"
  | Array items ->
    let inner = List.map (fun item ->
      Printf.sprintf "%s%s" pad2 (pretty_print (indent + 1) item)
    ) items in
    Printf.sprintf "[\n%s\n%s]" (String.concat ",\n" inner) pad
  | Object pairs ->
    let inner = List.map (fun (k, v) ->
      Printf.sprintf "%s\"%s\": %s" pad2 (escape_string k)
        (pretty_print (indent + 1) v)
    ) pairs in
    Printf.sprintf "{\n%s\n%s}" (String.concat ",\n" inner) pad

(* ── Compact (single-line) printer ──────────────────────────────────────── *)

let rec compact = function
  | Null       -> "null"
  | Bool true  -> "true"
  | Bool false -> "false"
  | Number n   -> format_number n
  | Str s      -> Printf.sprintf "\"%s\"" (escape_string s)
  | Array items ->
    Printf.sprintf "[%s]" (String.concat "," (List.map compact items))
  | Object pairs ->
    let kvs = List.map (fun (k, v) ->
      Printf.sprintf "\"%s\":%s" (escape_string k) (compact v)
    ) pairs in
    Printf.sprintf "{%s}" (String.concat "," kvs)

(* ── Buffer-based pretty printer (more efficient for large JSON) ─────────── *)

let rec pretty_buf buf indent j =
  let pad  = String.make (indent * 2) ' ' in
  let pad2 = String.make ((indent + 1) * 2) ' ' in
  match j with
  | Null         -> Buffer.add_string buf "null"
  | Bool true    -> Buffer.add_string buf "true"
  | Bool false   -> Buffer.add_string buf "false"
  | Number n     -> Buffer.add_string buf (format_number n)
  | Str s        ->
    Buffer.add_char buf '"';
    Buffer.add_string buf (escape_string s);
    Buffer.add_char buf '"'
  | Array []     -> Buffer.add_string buf "[]"
  | Object []    -> Buffer.add_string buf "{}"
  | Array items  ->
    Buffer.add_string buf "[\n";
    List.iteri (fun i item ->
      Buffer.add_string buf pad2;
      pretty_buf buf (indent + 1) item;
      if i < List.length items - 1 then Buffer.add_string buf ","
      else ();
      Buffer.add_char buf '\n'
    ) items;
    Buffer.add_string buf pad;
    Buffer.add_char buf ']'
  | Object pairs ->
    Buffer.add_string buf "{\n";
    List.iteri (fun i (k, v) ->
      Buffer.add_string buf pad2;
      Buffer.add_char buf '"';
      Buffer.add_string buf (escape_string k);
      Buffer.add_string buf "\": ";
      pretty_buf buf (indent + 1) v;
      if i < List.length pairs - 1 then Buffer.add_string buf ","
      else ();
      Buffer.add_char buf '\n'
    ) pairs;
    Buffer.add_string buf pad;
    Buffer.add_char buf '}'

let pretty_to_string j =
  let buf = Buffer.create 256 in
  pretty_buf buf 0 j;
  Buffer.contents buf

let () =
  (* primitives *)
  assert (pretty_print 0 Null = "null");
  assert (pretty_print 0 (Bool true)  = "true");
  assert (pretty_print 0 (Bool false) = "false");
  assert (pretty_print 0 (Number 42.0) = "42");
  assert (pretty_print 0 (Str "hi") = "\"hi\"");

  (* escape *)
  let s = Str "hello \"world\"\nnewline" in
  assert (pretty_print 0 s = "\"hello \\\"world\\\"\\nnewline\"");

  (* empty array / object *)
  assert (pretty_print 0 (Array []) = "[]");
  assert (pretty_print 0 (Object []) = "{}");

  (* compact: no newlines *)
  let json = Object [
    ("a", Number 1.0);
    ("b", Bool false);
  ] in
  let c = compact json in
  assert (not (String.contains c '\n'));
  assert (let pos = try ignore (Str.search_forward (Str.regexp_string "\"a\":1") c 0); true
                    with Not_found -> false in pos);

  (* nested pretty: starts with [ ends with ] *)
  let json2 = Array [
    Number 1.0;
    Array [Number 2.0; Number 3.0];
  ] in
  let p = pretty_print 0 json2 in
  assert (String.contains p '\n');
  assert (p.[0] = '[');
  assert (p.[String.length p - 1] = ']');

  print_endline "956-json-pretty-print: all tests passed"
