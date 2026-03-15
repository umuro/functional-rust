(* Example 175: Complete JSON Parser *)
(* Full JSON parser: null, bool, number, string, array, object *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

type json =
  | Null
  | Bool of bool
  | Number of float
  | String of string
  | Array of json list
  | Object of (string * json) list

let ws0 input =
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t' || input.[i] = '\n' || input.[i] = '\r')
    then skip (i+1) else i in
  let i = skip 0 in String.sub input i (String.length input - i)

(* Parse JSON string *)
let parse_json_string input =
  let s = ws0 input in
  if String.length s = 0 || s.[0] <> '"' then Error "Expected '\"'"
  else
    let buf = Buffer.create 32 in
    let rec go i =
      if i >= String.length s then Error "Unterminated string"
      else if s.[i] = '"' then
        Ok (Buffer.contents buf, String.sub s (i+1) (String.length s - i - 1))
      else if s.[i] = '\\' && i + 1 < String.length s then begin
        (match s.[i+1] with
         | 'n' -> Buffer.add_char buf '\n'
         | 't' -> Buffer.add_char buf '\t'
         | 'r' -> Buffer.add_char buf '\r'
         | '"' -> Buffer.add_char buf '"'
         | '\\' -> Buffer.add_char buf '\\'
         | '/' -> Buffer.add_char buf '/'
         | _ -> Buffer.add_char buf '\\'; Buffer.add_char buf s.[i+1]);
        go (i + 2)
      end else begin
        Buffer.add_char buf s.[i]; go (i + 1)
      end
    in go 1

(* Parse JSON number *)
let parse_json_number input =
  let s = ws0 input in
  let len = String.length s in
  let pos = ref 0 in
  if !pos < len && s.[!pos] = '-' then incr pos;
  if !pos < len && s.[!pos] = '0' then incr pos
  else while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done;
  if !pos < len && s.[!pos] = '.' then begin
    incr pos;
    while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done
  end;
  if !pos < len && (s.[!pos] = 'e' || s.[!pos] = 'E') then begin
    incr pos;
    if !pos < len && (s.[!pos] = '+' || s.[!pos] = '-') then incr pos;
    while !pos < len && s.[!pos] >= '0' && s.[!pos] <= '9' do incr pos done
  end;
  if !pos = 0 then Error "Expected number"
  else
    let num_str = String.sub s 0 !pos in
    Ok (Number (float_of_string num_str), String.sub s !pos (len - !pos))

(* Main JSON parser *)
let rec parse_json input =
  let s = ws0 input in
  if String.length s = 0 then Error "Unexpected EOF"
  else match s.[0] with
  | 'n' -> parse_keyword s "null" Null
  | 't' -> parse_keyword s "true" (Bool true)
  | 'f' -> parse_keyword s "false" (Bool false)
  | '"' -> (match parse_json_string s with
            | Ok (str, rest) -> Ok (String str, rest)
            | Error e -> Error e)
  | '[' -> parse_array s
  | '{' -> parse_object s
  | '-' | '0'..'9' -> parse_json_number s
  | c -> Error (Printf.sprintf "Unexpected character: '%c'" c)

and parse_keyword input kw value =
  let len = String.length kw in
  if String.length input >= len && String.sub input 0 len = kw then
    Ok (value, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" kw)

and parse_array input =
  let rest = ws0 (String.sub input 1 (String.length input - 1)) in
  if String.length rest > 0 && rest.[0] = ']' then
    Ok (Array [], String.sub rest 1 (String.length rest - 1))
  else
    let rec go acc remaining =
      match parse_json remaining with
      | Error e -> Error e
      | Ok (v, rest) ->
        let rest = ws0 rest in
        if String.length rest > 0 && rest.[0] = ',' then
          go (v :: acc) (String.sub rest 1 (String.length rest - 1))
        else if String.length rest > 0 && rest.[0] = ']' then
          Ok (Array (List.rev (v :: acc)), String.sub rest 1 (String.length rest - 1))
        else Error "Expected ',' or ']'"
    in go [] rest

and parse_object input =
  let rest = ws0 (String.sub input 1 (String.length input - 1)) in
  if String.length rest > 0 && rest.[0] = '}' then
    Ok (Object [], String.sub rest 1 (String.length rest - 1))
  else
    let rec go acc remaining =
      match parse_json_string remaining with
      | Error e -> Error e
      | Ok (key, rest) ->
        let rest = ws0 rest in
        if String.length rest = 0 || rest.[0] <> ':' then Error "Expected ':'"
        else
          let rest = String.sub rest 1 (String.length rest - 1) in
          match parse_json rest with
          | Error e -> Error e
          | Ok (value, rest) ->
            let rest = ws0 rest in
            if String.length rest > 0 && rest.[0] = ',' then
              go ((key, value) :: acc) (ws0 (String.sub rest 1 (String.length rest - 1)))
            else if String.length rest > 0 && rest.[0] = '}' then
              Ok (Object (List.rev ((key, value) :: acc)),
                  String.sub rest 1 (String.length rest - 1))
            else Error "Expected ',' or '}'"
    in go [] rest

(* Pretty printer *)
let rec json_to_string indent = function
  | Null -> "null"
  | Bool true -> "true"
  | Bool false -> "false"
  | Number n ->
    if Float.is_integer n then string_of_int (int_of_float n)
    else string_of_float n
  | String s -> Printf.sprintf "\"%s\"" s
  | Array items ->
    let inner = List.map (json_to_string (indent + 2)) items in
    "[" ^ String.concat ", " inner ^ "]"
  | Object entries ->
    let inner = List.map (fun (k, v) ->
      Printf.sprintf "\"%s\": %s" k (json_to_string (indent + 2) v)) entries in
    "{" ^ String.concat ", " inner ^ "}"

(* Tests *)
let () =
  assert (parse_json "null" = Ok (Null, ""));
  assert (parse_json "true" = Ok (Bool true, ""));
  assert (parse_json "false" = Ok (Bool false, ""));
  assert (parse_json "42" = Ok (Number 42., ""));
  assert (parse_json "-3.14" = Ok (Number (-3.14), ""));

  (match parse_json "\"hello\"" with
   | Ok (String "hello", "") -> ()
   | _ -> failwith "String test");

  (match parse_json "\"hello\\nworld\"" with
   | Ok (String s, "") -> assert (s = "hello\nworld")
   | _ -> failwith "Escape test");

  (match parse_json "[1, 2, 3]" with
   | Ok (Array [Number 1.; Number 2.; Number 3.], "") -> ()
   | _ -> failwith "Array test");

  assert (parse_json "[]" = Ok (Array [], ""));

  (match parse_json "{\"a\": 1, \"b\": true}" with
   | Ok (Object [("a", Number 1.); ("b", Bool true)], "") -> ()
   | _ -> failwith "Object test");

  (* Nested *)
  (match parse_json "{\"data\": [1, {\"x\": null}]}" with
   | Ok (Object [("data", Array [Number 1.; Object [("x", Null)]])], "") -> ()
   | _ -> failwith "Nested test");

  print_endline "✓ All tests passed"
