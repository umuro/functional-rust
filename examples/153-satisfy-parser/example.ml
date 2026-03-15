(* Example 153: Satisfy Parser *)
(* Parse a character matching a predicate *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let advance input =
  if String.length input > 0 then
    Some (input.[0], String.sub input 1 (String.length input - 1))
  else None

(* Approach 1: satisfy with a predicate *)
let satisfy (pred : char -> bool) (desc : string) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) when pred ch -> Ok (ch, rest)
  | Some (ch, _) -> Error (Printf.sprintf "Character '%c' does not satisfy %s" ch desc)
  | None -> Error (Printf.sprintf "Expected %s, got EOF" desc)

(* Approach 2: Build specific parsers from satisfy *)
let is_digit = satisfy (fun c -> c >= '0' && c <= '9') "digit"
let is_letter = satisfy (fun c ->
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) "letter"
let is_alphanumeric = satisfy (fun c ->
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
  (c >= '0' && c <= '9')) "alphanumeric"
let is_whitespace = satisfy (fun c -> c = ' ' || c = '\t' || c = '\n' || c = '\r') "whitespace"

(* Approach 3: Satisfy with custom error message *)
let satisfy_or (pred : char -> bool) (on_fail : char -> string) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) when pred ch -> Ok (ch, rest)
  | Some (ch, _) -> Error (on_fail ch)
  | None -> Error "Unexpected EOF"

let is_uppercase = satisfy_or
  (fun c -> c >= 'A' && c <= 'Z')
  (fun c -> Printf.sprintf "'%c' is not uppercase" c)

(* Tests *)
let () =
  assert (is_digit "42" = Ok ('4', "2"));
  assert (Result.is_error (is_digit "abc"));
  assert (is_letter "hello" = Ok ('h', "ello"));
  assert (Result.is_error (is_letter "123"));
  assert (is_alphanumeric "a1" = Ok ('a', "1"));
  assert (is_alphanumeric "1a" = Ok ('1', "a"));
  assert (is_whitespace " x" = Ok (' ', "x"));
  assert (is_uppercase "Hello" = Ok ('H', "ello"));
  assert (Result.is_error (is_uppercase "hello"));
  assert (Result.is_error (is_digit ""));
  print_endline "✓ All tests passed"
