(* JSON-like format built from scratch in OCaml *)

(* ── Value type ─────────────────────────────────────────────────────────────── *)
type json =
  | JNull
  | JBool   of bool
  | JInt    of int
  | JFloat  of float
  | JString of string
  | JArray  of json list
  | JObject of (string * json) list

(* ── Serializer ──────────────────────────────────────────────────────────────── *)
let rec to_string = function
  | JNull       -> "null"
  | JBool true  -> "true"
  | JBool false -> "false"
  | JInt n      -> string_of_int n
  | JFloat f    -> Printf.sprintf "%g" f
  | JString s   -> Printf.sprintf "%S" s
  | JArray arr  ->
    let items = String.concat ", " (List.map to_string arr) in
    Printf.sprintf "[%s]" items
  | JObject obj ->
    let pairs =
      List.map (fun (k, v) -> Printf.sprintf "%S: %s" k (to_string v)) obj
      |> String.concat ", "
    in
    Printf.sprintf "{%s}" pairs

(* ── Minimal parser ──────────────────────────────────────────────────────────── *)
let parse_error msg = failwith ("JSON parse error: " ^ msg)

let skip_ws s pos =
  while !pos < String.length s && (s.[!pos] = ' ' || s.[!pos] = '\n' || s.[!pos] = '\t') do
    incr pos
  done

let parse_string s pos =
  (* pos points to opening '"' *)
  incr pos;
  let buf = Buffer.create 16 in
  let continue = ref true in
  while !continue do
    if !pos >= String.length s then parse_error "unterminated string";
    let c = s.[!pos] in
    incr pos;
    if c = '"' then continue := false
    else if c = '\\' then begin
      if !pos >= String.length s then parse_error "escape at EOF";
      let esc = s.[!pos] in incr pos;
      Buffer.add_char buf (match esc with
        | 'n' -> '\n' | 't' -> '\t' | '"' -> '"' | '\\' -> '\\'
        | c -> c)
    end else Buffer.add_char buf c
  done;
  Buffer.contents buf

let rec parse_value s pos =
  skip_ws s pos;
  if !pos >= String.length s then parse_error "unexpected EOF";
  match s.[!pos] with
  | '"' -> JString (parse_string s pos)
  | 't' -> pos := !pos + 4; JBool true
  | 'f' -> pos := !pos + 5; JBool false
  | 'n' -> pos := !pos + 4; JNull
  | '[' ->
    incr pos; skip_ws s pos;
    if s.[!pos] = ']' then (incr pos; JArray [])
    else begin
      let items = ref [] in
      let go = ref true in
      while !go do
        items := parse_value s pos :: !items;
        skip_ws s pos;
        if !pos < String.length s && s.[!pos] = ',' then incr pos
        else go := false
      done;
      skip_ws s pos;
      if s.[!pos] <> ']' then parse_error "expected ']'";
      incr pos;
      JArray (List.rev !items)
    end
  | '{' ->
    incr pos; skip_ws s pos;
    if s.[!pos] = '}' then (incr pos; JObject [])
    else begin
      let pairs = ref [] in
      let go = ref true in
      while !go do
        skip_ws s pos;
        let k = parse_string s pos in
        skip_ws s pos;
        if s.[!pos] <> ':' then parse_error "expected ':'";
        incr pos;
        let v = parse_value s pos in
        pairs := (k, v) :: !pairs;
        skip_ws s pos;
        if !pos < String.length s && s.[!pos] = ',' then incr pos
        else go := false
      done;
      skip_ws s pos;
      if s.[!pos] <> '}' then parse_error "expected '}'";
      incr pos;
      JObject (List.rev !pairs)
    end
  | c when c >= '0' && c <= '9' || c = '-' ->
    let start = !pos in
    while !pos < String.length s &&
          (let ch = s.[!pos] in
           (ch >= '0' && ch <= '9') || ch = '.' || ch = '-' || ch = 'e' || ch = 'E')
    do incr pos done;
    let tok = String.sub s start (!pos - start) in
    (try JInt (int_of_string tok)
     with _ -> try JFloat (float_of_string tok)
     with _ -> parse_error ("bad number: " ^ tok))
  | c -> parse_error (Printf.sprintf "unexpected char '%c'" c)

let parse s =
  let pos = ref 0 in
  parse_value s pos

let () =
  let v = JObject [
    ("name", JString "Alice");
    ("age", JInt 30);
    ("scores", JArray [JInt 95; JInt 87; JInt 100]);
    ("active", JBool true);
    ("address", JNull);
  ] in
  let s = to_string v in
  Printf.printf "Serialized:\n%s\n\n" s;
  let v2 = parse s in
  Printf.printf "Re-serialized:\n%s\n" (to_string v2)
