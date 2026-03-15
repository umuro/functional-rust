(* Example 162: Identifier Parser *)
(* Parse identifiers: letter followed by alphanumeric/underscore *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let many0 p : 'a list parser = fun input ->
  let rec go acc r = match p r with Ok (v, r') -> go (v::acc) r' | Error _ -> Ok (List.rev acc, r)
  in go [] input

let map f p : 'b parser = fun input ->
  match p input with Ok (v, r) -> Ok (f v, r) | Error e -> Error e

let is_letter c = (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
let is_digit c = c >= '0' && c <= '9'
let is_ident_start c = is_letter c || c = '_'
let is_ident_char c = is_letter c || is_digit c || c = '_'

(* Approach 1: Direct implementation *)
let identifier : string parser = fun input ->
  match satisfy is_ident_start "letter or _" input with
  | Error e -> Error e
  | Ok (first, rest) ->
    match many0 (satisfy is_ident_char "alphanumeric or _") rest with
    | Ok (chars, rem) ->
      let s = String.make 1 first ^ String.init (List.length chars) (List.nth chars) in
      Ok (s, rem)
    | Error e -> Error e

(* Approach 2: Using combinators *)
let identifier2 : string parser =
  let start = satisfy is_ident_start "letter or _" in
  let cont = many0 (satisfy is_ident_char "alphanumeric or _") in
  fun input ->
    match start input with
    | Error e -> Error e
    | Ok (first, rest) ->
      match cont rest with
      | Ok (chars, rem) ->
        let s = String.make 1 first ^ String.init (List.length chars) (List.nth chars) in
        Ok (s, rem)
      | Error e -> Error e

(* Approach 3: With reserved word checking *)
let reserved_words = ["let"; "in"; "if"; "then"; "else"; "match"; "with"; "fun"]

let identifier_not_reserved : string parser = fun input ->
  match identifier input with
  | Ok (name, rest) when not (List.mem name reserved_words) -> Ok (name, rest)
  | Ok (name, _) -> Error (Printf.sprintf "'%s' is a reserved word" name)
  | Error e -> Error e

(* Tests *)
let () =
  assert (identifier "hello world" = Ok ("hello", " world"));
  assert (identifier "_foo bar" = Ok ("_foo", " bar"));
  assert (identifier "x1y2z3!" = Ok ("x1y2z3", "!"));
  assert (identifier "_" = Ok ("_", ""));
  assert (Result.is_error (identifier "123"));
  assert (Result.is_error (identifier ""));

  assert (identifier_not_reserved "myVar" = Ok ("myVar", ""));
  assert (Result.is_error (identifier_not_reserved "let"));
  assert (Result.is_error (identifier_not_reserved "if"));

  print_endline "✓ All tests passed"
