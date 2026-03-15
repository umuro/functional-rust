(* Example 155: Many Parser *)
(* many0 and many1: parse zero or more / one or more *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let satisfy pred desc : char parser = fun input ->
  if String.length input > 0 && pred input.[0] then
    Ok (input.[0], String.sub input 1 (String.length input - 1))
  else Error (Printf.sprintf "Expected %s" desc)

(* Approach 1: many0 — zero or more, always succeeds *)
let many0 (p : 'a parser) : 'a list parser = fun input ->
  let rec go acc remaining =
    match p remaining with
    | Ok (v, rest) -> go (v :: acc) rest
    | Error _ -> Ok (List.rev acc, remaining)
  in
  go [] input

(* Approach 2: many1 — one or more, fails if zero matches *)
let many1 (p : 'a parser) : 'a list parser = fun input ->
  match p input with
  | Error e -> Error e
  | Ok (first, rest) ->
    match many0 p rest with
    | Ok (others, remaining) -> Ok (first :: others, remaining)
    | Error e -> Error e

(* Approach 3: many_till — parse until a terminator succeeds *)
let many_till (p : 'a parser) (stop : 'b parser) : ('a list * 'b) parser = fun input ->
  let rec go acc remaining =
    match stop remaining with
    | Ok (term, rest) -> Ok ((List.rev acc, term), rest)
    | Error _ ->
      match p remaining with
      | Ok (v, rest) -> go (v :: acc) rest
      | Error e -> Error e
  in
  go [] input

(* Collect chars into string *)
let many0_str (p : char parser) : string parser = fun input ->
  match many0 p input with
  | Ok (chars, rest) -> Ok (String.init (List.length chars) (List.nth chars), rest)
  | Error e -> Error e

let is_digit = satisfy (fun c -> c >= '0' && c <= '9') "digit"
let is_letter = satisfy (fun c -> (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) "letter"

(* Tests *)
let () =
  (* many0 *)
  (match many0 is_digit "123abc" with
   | Ok (digits, "abc") -> assert (digits = ['1'; '2'; '3'])
   | _ -> failwith "Test 1 failed");

  (match many0 is_digit "abc" with
   | Ok ([], "abc") -> ()
   | _ -> failwith "Test 2 failed");

  (* many1 *)
  (match many1 is_digit "123abc" with
   | Ok (digits, "abc") -> assert (digits = ['1'; '2'; '3'])
   | _ -> failwith "Test 3 failed");

  assert (Result.is_error (many1 is_digit "abc"));

  (* many_till *)
  let stop = satisfy (fun c -> c = '.') "dot" in
  (match many_till is_letter stop "abc.rest" with
   | Ok ((letters, '.'), "rest") -> assert (letters = ['a'; 'b'; 'c'])
   | _ -> failwith "Test 5 failed");

  print_endline "✓ All tests passed"
