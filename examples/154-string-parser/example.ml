(* Example 154: String Parser *)
(* Parse exact string literals *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

(* Approach 1: Parse exact string *)
let tag (expected : string) : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else
    Error (Printf.sprintf "Expected \"%s\"" expected)

(* Approach 2: Case-insensitive string match *)
let tag_no_case (expected : string) : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len &&
     String.lowercase_ascii (String.sub input 0 len) = String.lowercase_ascii expected then
    Ok (String.sub input 0 len, String.sub input len (String.length input - len))
  else
    Error (Printf.sprintf "Expected \"%s\" (case insensitive)" expected)

(* Approach 3: Build string parser from char parsers *)
let char_parser (c : char) : char parser = fun input ->
  if String.length input > 0 && input.[0] = c then
    Ok (c, String.sub input 1 (String.length input - 1))
  else
    Error (Printf.sprintf "Expected '%c'" c)

let string_from_chars (s : string) : string parser = fun input ->
  let len = String.length s in
  let rec go i remaining =
    if i >= len then Ok (s, remaining)
    else
      match char_parser s.[i] remaining with
      | Ok (_, rest) -> go (i + 1) rest
      | Error e -> Error e
  in
  go 0 input

(* Tests *)
let () =
  assert (tag "hello" "hello world" = Ok ("hello", " world"));
  assert (tag "hello" "hello" = Ok ("hello", ""));
  assert (Result.is_error (tag "hello" "world"));
  assert (Result.is_error (tag "hello" "hel"));

  assert (tag_no_case "Hello" "HELLO world" = Ok ("HELLO", " world"));
  assert (tag_no_case "hello" "HeLLo!" = Ok ("HeLLo", "!"));

  assert (string_from_chars "abc" "abcdef" = Ok ("abc", "def"));
  assert (Result.is_error (string_from_chars "abc" "axc"));

  print_endline "✓ All tests passed"
