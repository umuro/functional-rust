(* Example 159: Map Parser *)
(* map: transform parser output functorially *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

let many0 (p : 'a parser) : 'a list parser = fun input ->
  let rec go acc rem =
    match p rem with
    | Ok (v, rest) -> go (v :: acc) rest
    | Error _ -> Ok (List.rev acc, rem)
  in go [] input

let many1 (p : 'a parser) : 'a list parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (v, rest) ->
    match many0 p rest with
    | Ok (vs, rem) -> Ok (v :: vs, rem)
    | Error e -> Error e

(* Approach 1: map — transform the parsed value *)
let map (f : 'a -> 'b) (p : 'a parser) : 'b parser = fun input ->
  match p input with
  | Ok (v, rest) -> Ok (f v, rest)
  | Error e -> Error e

(* Approach 2: map2 — combine two parser results *)
let map2 (f : 'a -> 'b -> 'c) (p1 : 'a parser) (p2 : 'b parser) : 'c parser = fun input ->
  match p1 input with
  | Error e -> Error e
  | Ok (v1, rest) ->
    match p2 rest with
    | Error e -> Error e
    | Ok (v2, rem) -> Ok (f v1 v2, rem)

(* Approach 3: map with const — ignore result, return fixed value *)
let map_const (value : 'b) (p : 'a parser) : 'b parser = fun input ->
  match p input with
  | Ok (_, rest) -> Ok (value, rest)
  | Error e -> Error e

(* Practical: parse digits and convert to int *)
let is_digit = satisfy (fun c -> c >= '0' && c <= '9') "digit"

let parse_nat : int parser =
  map (fun chars ->
    List.fold_left (fun acc c -> acc * 10 + (Char.code c - Char.code '0')) 0 chars
  ) (many1 is_digit)

(* Tests *)
let () =
  (* map: char to uppercase *)
  let upper_letter = map Char.uppercase_ascii
    (satisfy (fun c -> c >= 'a' && c <= 'z') "lowercase") in
  assert (upper_letter "abc" = Ok ('A', "bc"));

  (* parse_nat: digits to int *)
  assert (parse_nat "42rest" = Ok (42, "rest"));
  assert (parse_nat "0" = Ok (0, ""));
  assert (Result.is_error (parse_nat "abc"));

  (* map2 *)
  let digit = satisfy (fun c -> c >= '0' && c <= '9') "digit" in
  let pair_str = map2 (fun a b -> Printf.sprintf "%c%c" a b) digit digit in
  assert (pair_str "12x" = Ok ("12", "x"));

  (* map_const *)
  let tag s : string parser = fun input ->
    let len = String.length s in
    if String.length input >= len && String.sub input 0 len = s then
      Ok (s, String.sub input len (String.length input - len))
    else Error (Printf.sprintf "Expected \"%s\"" s) in
  let true_parser = map_const true (tag "true") in
  assert (true_parser "true!" = Ok (true, "!"));

  print_endline "✓ All tests passed"
