(* Example 161: Digit Parser *)
(* Parse digits: single digit, multi-digit integer, positive/negative *)

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

let map f p : 'b parser = fun input ->
  match p input with Ok (v, r) -> Ok (f v, r) | Error e -> Error e

let opt p : 'a option parser = fun input ->
  match p input with Ok (v, r) -> Ok (Some v, r) | Error _ -> Ok (None, input)

(* Approach 1: Single digit *)
let digit : int parser =
  map (fun c -> Char.code c - Char.code '0')
    (satisfy (fun c -> c >= '0' && c <= '9') "digit")

(* Approach 2: Natural number (unsigned) *)
let natural : int parser =
  map (fun digits -> List.fold_left (fun acc d -> acc * 10 + d) 0 digits)
    (many1 digit)

(* Approach 3: Signed integer *)
let integer : int parser = fun input ->
  match opt (satisfy (fun c -> c = '+' || c = '-') "sign") input with
  | Ok (sign, rest) ->
    (match natural rest with
     | Ok (n, rem) ->
       let value = match sign with Some '-' -> -n | _ -> n in
       Ok (value, rem)
     | Error e -> Error e)
  | Error e -> Error e

(* Tests *)
let () =
  assert (digit "5rest" = Ok (5, "rest"));
  assert (Result.is_error (digit "abc"));

  assert (natural "42rest" = Ok (42, "rest"));
  assert (natural "0" = Ok (0, ""));
  assert (natural "100" = Ok (100, ""));

  assert (integer "42" = Ok (42, ""));
  assert (integer "-42" = Ok (-42, ""));
  assert (integer "+42" = Ok (42, ""));
  assert (integer "0" = Ok (0, ""));
  assert (Result.is_error (integer "abc"));

  print_endline "✓ All tests passed"
