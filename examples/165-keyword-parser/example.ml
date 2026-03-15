(* Example 165: Keyword Parser *)
(* Keywords with word boundary checking *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let tag expected : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" expected)

let is_ident_char c =
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
  (c >= '0' && c <= '9') || c = '_'

(* Approach 1: keyword — match string + check word boundary *)
let keyword (kw : string) : string parser = fun input ->
  match tag kw input with
  | Error e -> Error e
  | Ok (matched, rest) ->
    if String.length rest > 0 && is_ident_char rest.[0] then
      Error (Printf.sprintf "\"%s\" is not a complete keyword (followed by '%c')" kw rest.[0])
    else
      Ok (matched, rest)

(* Approach 2: keyword mapping to enum-like value *)
type token = If | Then | Else | Let | In | Fun

let keyword_token (kw : string) (tok : token) : token parser = fun input ->
  match keyword kw input with
  | Ok (_, rest) -> Ok (tok, rest)
  | Error e -> Error e

(* Approach 3: any_keyword — try multiple keywords *)
let any_keyword (keywords : (string * 'a) list) : 'a parser = fun input ->
  let rec try_kws = function
    | [] -> Error "Expected keyword"
    | (kw, tok) :: rest ->
      match keyword kw input with
      | Ok (_, rem) -> Ok (tok, rem)
      | Error _ -> try_kws rest
  in
  (* Sort longest first to avoid prefix issues *)
  let sorted = List.sort (fun (a, _) (b, _) ->
    compare (String.length b) (String.length a)) keywords in
  try_kws sorted

(* Tests *)
let () =
  assert (keyword "if" "if x" = Ok ("if", " x"));
  assert (Result.is_error (keyword "if" "iffy"));
  assert (keyword "if" "if(" = Ok ("if", "("));
  assert (keyword "else" "else{" = Ok ("else", "{"));

  assert (keyword_token "if" If "if x" = Ok (If, " x"));

  let kw_parser = any_keyword [
    ("if", If); ("then", Then); ("else", Else);
    ("let", Let); ("in", In); ("fun", Fun)] in
  assert (kw_parser "let x" = Ok (Let, " x"));
  assert (kw_parser "fun x" = Ok (Fun, " x"));
  assert (Result.is_error (kw_parser "hello"));

  print_endline "✓ All tests passed"
