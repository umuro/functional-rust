(* Example 152: Character Parsers *)
(* Parse single characters: char_parser, any_char, none_of *)

type 'a parse_result = ('a * string, string) result

type 'a parser = string -> 'a parse_result

(* Helper to advance input by one character *)
let advance input =
  if String.length input > 0 then
    Some (input.[0], String.sub input 1 (String.length input - 1))
  else
    None

(* Approach 1: Parse a specific character *)
let char_parser (c : char) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) when ch = c -> Ok (ch, rest)
  | Some (ch, _) -> Error (Printf.sprintf "Expected '%c', got '%c'" c ch)
  | None -> Error (Printf.sprintf "Expected '%c', got EOF" c)

(* Approach 2: Parse any character *)
let any_char : char parser = fun input ->
  match advance input with
  | Some (ch, rest) -> Ok (ch, rest)
  | None -> Error "Expected any character, got EOF"

(* Approach 3: Parse any character NOT in the given set *)
let none_of (chars : char list) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) ->
    if List.mem ch chars then
      Error (Printf.sprintf "Unexpected character '%c'" ch)
    else
      Ok (ch, rest)
  | None -> Error "Expected a character, got EOF"

(* one_of: parse any character IN the given set *)
let one_of (chars : char list) : char parser = fun input ->
  match advance input with
  | Some (ch, rest) when List.mem ch chars -> Ok (ch, rest)
  | Some (ch, _) -> Error (Printf.sprintf "Character '%c' not in allowed set" ch)
  | None -> Error "Expected a character, got EOF"

(* Tests *)
let () =
  (* char_parser tests *)
  assert (char_parser 'a' "abc" = Ok ('a', "bc"));
  assert (Result.is_error (char_parser 'a' "xyz"));
  assert (Result.is_error (char_parser 'a' ""));

  (* any_char tests *)
  assert (any_char "hello" = Ok ('h', "ello"));
  assert (any_char "x" = Ok ('x', ""));
  assert (Result.is_error (any_char ""));

  (* none_of tests *)
  assert (none_of ['x'; 'y'; 'z'] "abc" = Ok ('a', "bc"));
  assert (Result.is_error (none_of ['a'; 'b'] "abc"));

  (* one_of tests *)
  assert (one_of ['a'; 'b'; 'c'] "beta" = Ok ('b', "eta"));
  assert (Result.is_error (one_of ['x'; 'y'] "abc"));

  print_endline "✓ All tests passed"
