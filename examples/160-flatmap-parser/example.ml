(* Example 160: FlatMap Parser *)
(* flat_map / and_then: monadic chaining of parsers *)

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

let many1 (p : 'a parser) : 'a list parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (v, rest) ->
    let rec go acc rem = match p rem with
      | Ok (v, r) -> go (v :: acc) r
      | Error _ -> Ok (List.rev acc, rem)
    in match go [v] rest with
    | Ok (vs, r) -> Ok (vs, r)
    | Error e -> Error e

let map f p : 'b parser = fun input ->
  match p input with Ok (v, r) -> Ok (f v, r) | Error e -> Error e

(* Approach 1: and_then / bind — monadic chaining *)
let and_then (p : 'a parser) (f : 'a -> 'b parser) : 'b parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (v, rest) -> (f v) rest

(* Infix operator for bind *)
let ( >>= ) = and_then

(* Approach 2: flat_map with context-sensitive parsing *)
(* Parse a length-prefixed string: "3:abc" *)
let parse_nat : int parser =
  map (fun chars ->
    List.fold_left (fun acc c -> acc * 10 + (Char.code c - Char.code '0')) 0 chars
  ) (many1 (satisfy (fun c -> c >= '0' && c <= '9') "digit"))

let length_prefixed : string parser =
  parse_nat >>= fun n ->
  (satisfy (fun c -> c = ':') "colon") >>= fun _ ->
  (fun input ->
    if String.length input >= n then
      Ok (String.sub input 0 n, String.sub input n (String.length input - n))
    else Error "Not enough characters")

(* Approach 3: Conditional parsing based on first result *)
let conditional_parser : string parser = fun input ->
  let type_parser = satisfy (fun c -> c = 'i' || c = 's') "type tag" in
  (type_parser >>= fun tag ->
    if tag = 'i' then
      map (fun ds -> String.init (List.length ds) (List.nth ds))
        (many1 (satisfy (fun c -> c >= '0' && c <= '9') "digit"))
    else
      map (fun ls -> String.init (List.length ls) (List.nth ls))
        (many1 (satisfy (fun c -> c >= 'a' && c <= 'z') "letter"))
  ) input

(* Tests *)
let () =
  (* and_then basic *)
  let p = and_then
    (satisfy (fun c -> c >= '0' && c <= '9') "digit")
    (fun d -> tag (String.make 1 d)) in
  assert (p "11rest" = Ok ("1", "rest"));

  (* length_prefixed *)
  assert (length_prefixed "3:abcrest" = Ok ("abc", "rest"));
  assert (length_prefixed "5:helloworld" = Ok ("hello", "world"));

  (* conditional *)
  assert (conditional_parser "i42rest" = Ok ("42", "rest"));
  assert (conditional_parser "sabcREST" = Ok ("abc", "REST"));

  print_endline "✓ All tests passed"
