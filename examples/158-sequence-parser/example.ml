(* Example 158: Sequence Parser *)
(* pair, preceded, terminated, delimited: sequence combinators *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let tag expected : string parser = fun input ->
  let len = String.length expected in
  if String.length input >= len && String.sub input 0 len = expected then
    Ok (expected, String.sub input len (String.length input - len))
  else Error (Printf.sprintf "Expected \"%s\"" expected)

(* Approach 1: pair — run two parsers in sequence, return both results *)
let pair (p1 : 'a parser) (p2 : 'b parser) : ('a * 'b) parser = fun input ->
  match p1 input with
  | Error e -> Error e
  | Ok (v1, rest) ->
    match p2 rest with
    | Error e -> Error e
    | Ok (v2, remaining) -> Ok ((v1, v2), remaining)

(* Approach 2: preceded, terminated — discard one side *)
let preceded (prefix : 'a parser) (p : 'b parser) : 'b parser = fun input ->
  match prefix input with
  | Error e -> Error e
  | Ok (_, rest) -> p rest

let terminated (p : 'a parser) (suffix : 'b parser) : 'a parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (v, rest) ->
    match suffix rest with
    | Error e -> Error e
    | Ok (_, remaining) -> Ok (v, remaining)

(* Approach 3: delimited — discard both sides *)
let delimited (open_p : 'a parser) (p : 'b parser) (close_p : 'c parser) : 'b parser =
  fun input ->
    preceded open_p (terminated p close_p) input

(* triple — three in sequence *)
let triple (p1 : 'a parser) (p2 : 'b parser) (p3 : 'c parser) : ('a * 'b * 'c) parser =
  fun input ->
    match p1 input with
    | Error e -> Error e
    | Ok (v1, r1) ->
      match p2 r1 with
      | Error e -> Error e
      | Ok (v2, r2) ->
        match p3 r2 with
        | Error e -> Error e
        | Ok (v3, r3) -> Ok ((v1, v2, v3), r3)

(* Tests *)
let () =
  let letter = satisfy (fun c -> (c >= 'a' && c <= 'z')) "letter" in
  let digit = satisfy (fun c -> c >= '0' && c <= '9') "digit" in

  (* pair *)
  assert (pair letter digit "a1" = Ok (('a', '1'), ""));
  assert (Result.is_error (pair letter digit "1a"));

  (* preceded: skip prefix *)
  assert (preceded (tag "(") letter "(a)" = Ok ('a', ")"));

  (* terminated: skip suffix *)
  assert (terminated letter (tag ";") "a;rest" = Ok ('a', "rest"));

  (* delimited: extract middle *)
  assert (delimited (tag "(") letter (tag ")") "(x)rest" = Ok ('x', "rest"));

  (* triple *)
  assert (triple letter digit letter "a1b" = Ok (('a', '1', 'b'), ""));

  print_endline "✓ All tests passed"
