(* Example 171: CSV Parser *)
(* Complete CSV parser using combinators (handles quotes, escaping) *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let many0 p : 'a list parser = fun input ->
  let rec go acc r = match p r with Ok (v, r') -> go (v::acc) r' | Error _ -> Ok (List.rev acc, r)
  in go [] input

let tag expected : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" expected)

let chars_to_string chars = String.init (List.length chars) (List.nth chars)

(* Approach 1: Unquoted field — characters until comma or newline *)
let unquoted_field : string parser = fun input ->
  match many0 (satisfy (fun c -> c <> ',' && c <> '\n' && c <> '\r') "field char") input with
  | Ok (chars, rest) -> Ok (String.trim (chars_to_string chars), rest)
  | Error e -> Error e

(* Approach 2: Quoted field — handles escaped quotes "" *)
let quoted_field : string parser = fun input ->
  match satisfy (fun c -> c = '"') "opening quote" input with
  | Error e -> Error e
  | Ok (_, rest) ->
    let buf = Buffer.create 32 in
    let rec go remaining =
      if String.length remaining = 0 then Error "Unterminated quoted field"
      else if remaining.[0] = '"' then
        let after = String.sub remaining 1 (String.length remaining - 1) in
        if String.length after > 0 && after.[0] = '"' then begin
          Buffer.add_char buf '"';
          go (String.sub after 1 (String.length after - 1))
        end else
          Ok (Buffer.contents buf, after)
      else begin
        Buffer.add_char buf remaining.[0];
        go (String.sub remaining 1 (String.length remaining - 1))
      end
    in go rest

(* Approach 3: Full CSV parser *)
let field : string parser = fun input ->
  if String.length input > 0 && input.[0] = '"' then quoted_field input
  else unquoted_field input

let row : string list parser = fun input ->
  match field input with
  | Error e -> Error e
  | Ok (first, rest) ->
    let rec go acc remaining =
      match tag "," remaining with
      | Error _ -> Ok (List.rev acc, remaining)
      | Ok (_, after_comma) ->
        match field after_comma with
        | Ok (f, rest') -> go (f :: acc) rest'
        | Error e -> Error e
    in go [first] rest

let line_ending : unit parser = fun input ->
  if String.length input >= 2 && String.sub input 0 2 = "\r\n" then
    Ok ((), String.sub input 2 (String.length input - 2))
  else if String.length input >= 1 && input.[0] = '\n' then
    Ok ((), String.sub input 1 (String.length input - 1))
  else if String.length input = 0 then Ok ((), "")
  else Error "Expected line ending"

let csv : string list list parser = fun input ->
  let rec go acc remaining =
    if String.length remaining = 0 then Ok (List.rev acc, "")
    else
      match row remaining with
      | Error e -> Error e
      | Ok (r, rest) ->
        match line_ending rest with
        | Ok ((), rest') -> go (r :: acc) rest'
        | Error e -> Error e
  in go [] input

(* Tests *)
let () =
  assert (field "hello,world" = Ok ("hello", ",world"));
  assert (field "\"hello,world\"" = Ok ("hello,world", ""));
  assert (field "\"say \"\"hi\"\"\"" = Ok ("say \"hi\"", ""));

  assert (row "a,b,c" = Ok (["a"; "b"; "c"], ""));
  assert (row "\"x,y\",z" = Ok (["x,y"; "z"], ""));

  (match csv "a,b\n1,2\n3,4" with
   | Ok ([["a";"b"];["1";"2"];["3";"4"]], "") -> ()
   | _ -> failwith "CSV test");

  print_endline "✓ All tests passed"
