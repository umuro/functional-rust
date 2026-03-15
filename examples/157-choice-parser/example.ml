(* Example 157: Choice Parser *)
(* alt / choice: try parsers in order, return first success *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let tag expected : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" expected)

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

(* Approach 1: alt — try two parsers *)
let alt (p1 : 'a parser) (p2 : 'a parser) : 'a parser = fun input ->
  match p1 input with
  | Ok _ as result -> result
  | Error _ -> p2 input

(* Approach 2: choice — try a list of parsers *)
let choice (parsers : 'a parser list) : 'a parser = fun input ->
  let rec try_parsers = function
    | [] -> Error "No parser matched"
    | p :: rest ->
      match p input with
      | Ok _ as result -> result
      | Error _ -> try_parsers rest
  in
  try_parsers parsers

(* Approach 3: alt with error accumulation *)
let alt_err (p1 : 'a parser) (p2 : 'a parser) : 'a parser = fun input ->
  match p1 input with
  | Ok _ as result -> result
  | Error e1 ->
    match p2 input with
    | Ok _ as result -> result
    | Error e2 -> Error (Printf.sprintf "%s or %s" e1 e2)

(* Tests *)
let () =
  let true_or_false = alt (tag "true") (tag "false") in
  assert (true_or_false "true!" = Ok ("true", "!"));
  assert (true_or_false "false!" = Ok ("false", "!"));
  assert (Result.is_error (true_or_false "maybe"));

  let bool_or_null = choice [tag "true"; tag "false"; tag "null"] in
  assert (bool_or_null "null!" = Ok ("null", "!"));
  assert (bool_or_null "true" = Ok ("true", ""));
  assert (Result.is_error (bool_or_null "xyz"));

  let digit_or_letter = alt_err
    (satisfy (fun c -> c >= '0' && c <= '9') "digit")
    (satisfy (fun c -> (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) "letter") in
  assert (digit_or_letter "5x" = Ok ('5', "x"));
  assert (digit_or_letter "a1" = Ok ('a', "1"));
  assert (Result.is_error (digit_or_letter "!x"));

  print_endline "✓ All tests passed"
