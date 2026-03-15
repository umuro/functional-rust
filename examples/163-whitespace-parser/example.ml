(* Example 163: Whitespace Parser *)
(* Parse and skip whitespace: ws, ws0, ws1 *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let many0 p : 'a list parser = fun input ->
  let rec go acc r = match p r with Ok (v, r') -> go (v::acc) r' | Error _ -> Ok (List.rev acc, r)
  in go [] input

let many1 p : 'a list parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (v, r) -> match many0 p r with Ok (vs, r') -> Ok (v::vs, r') | Error e -> Error e

let is_ws c = c = ' ' || c = '\t' || c = '\n' || c = '\r'

(* Approach 1: ws0 — skip zero or more whitespace *)
let ws0 : unit parser = fun input ->
  match many0 (satisfy is_ws "whitespace") input with
  | Ok (_, rest) -> Ok ((), rest)
  | Error e -> Error e

(* Approach 2: ws1 — require at least one whitespace *)
let ws1 : unit parser = fun input ->
  match many1 (satisfy is_ws "whitespace") input with
  | Ok (_, rest) -> Ok ((), rest)
  | Error e -> Error e

(* Approach 3: ws_wrap — parse p surrounded by optional whitespace *)
let ws_wrap (p : 'a parser) : 'a parser = fun input ->
  match ws0 input with
  | Ok ((), r1) ->
    (match p r1 with
     | Ok (v, r2) ->
       (match ws0 r2 with
        | Ok ((), r3) -> Ok (v, r3)
        | Error e -> Error e)
     | Error e -> Error e)
  | Error e -> Error e

(* line comment: skip from # to newline *)
let line_comment : unit parser = fun input ->
  if String.length input > 0 && input.[0] = '#' then
    let rec skip i =
      if i >= String.length input || input.[i] = '\n' then i
      else skip (i + 1) in
    let end_pos = skip 1 in
    Ok ((), String.sub input end_pos (String.length input - end_pos))
  else Error "Expected '#'"

(* Tests *)
let () =
  assert (ws0 "  hello" = Ok ((), "hello"));
  assert (ws0 "hello" = Ok ((), "hello"));
  assert (ws0 "" = Ok ((), ""));

  assert (ws1 "  hello" = Ok ((), "hello"));
  assert (Result.is_error (ws1 "hello"));

  let tag s : string parser = fun input ->
    let len = String.length s in
    if String.length input >= len && String.sub input 0 len = s then
      Ok (s, String.sub input len (String.length input - len))
    else Error (Printf.sprintf "Expected \"%s\"" s) in

  assert (ws_wrap (tag "hello") "  hello  rest" = Ok ("hello", "rest"));
  assert (ws_wrap (tag "hello") "hello" = Ok ("hello", ""));

  assert (line_comment "# comment\ncode" = Ok ((), "code"));

  print_endline "✓ All tests passed"
