(* Example 164: Number Parser *)
(* Parse floating point numbers with optional sign and decimal *)

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
  match p input with Error e -> Error e
  | Ok (v, r) -> match many0 p r with Ok (vs, r') -> Ok (v::vs, r') | Error e -> Error e

let opt p : 'a option parser = fun input ->
  match p input with Ok (v, r) -> Ok (Some v, r) | Error _ -> Ok (None, input)

let is_digit c = c >= '0' && c <= '9'
let digit = satisfy is_digit "digit"

(* Approach 1: Float as string collection *)
let float_string : string parser = fun input ->
  let buf = Buffer.create 16 in
  let pos = ref 0 in
  let len = String.length input in
  (* optional sign *)
  if !pos < len && (input.[!pos] = '+' || input.[!pos] = '-') then begin
    Buffer.add_char buf input.[!pos]; incr pos end;
  (* integer part *)
  let start = !pos in
  while !pos < len && is_digit input.[!pos] do
    Buffer.add_char buf input.[!pos]; incr pos done;
  (* decimal part *)
  if !pos < len && input.[!pos] = '.' then begin
    Buffer.add_char buf '.'; incr pos;
    while !pos < len && is_digit input.[!pos] do
      Buffer.add_char buf input.[!pos]; incr pos done end;
  (* exponent *)
  if !pos < len && (input.[!pos] = 'e' || input.[!pos] = 'E') then begin
    Buffer.add_char buf input.[!pos]; incr pos;
    if !pos < len && (input.[!pos] = '+' || input.[!pos] = '-') then begin
      Buffer.add_char buf input.[!pos]; incr pos end;
    while !pos < len && is_digit input.[!pos] do
      Buffer.add_char buf input.[!pos]; incr pos done end;
  if !pos = start then Error "Expected number"
  else Ok (Buffer.contents buf, String.sub input !pos (len - !pos))

(* Approach 2: Combinator-based *)
let chars_to_string chars = String.init (List.length chars) (List.nth chars)

let number_combinator : float parser = fun input ->
  match opt (satisfy (fun c -> c = '+' || c = '-') "sign") input with
  | Ok (sign, r1) ->
    (match many1 digit r1 with
     | Ok (int_part, r2) ->
       (match opt (satisfy (fun c -> c = '.') "dot") r2 with
        | Ok (Some _, r3) ->
          (match many0 digit r3 with
           | Ok (frac_part, r4) ->
             let s = (match sign with Some c -> String.make 1 c | None -> "") ^
                     chars_to_string int_part ^ "." ^ chars_to_string frac_part in
             Ok (float_of_string s, r4)
           | Error e -> Error e)
        | Ok (None, r3) ->
          let s = (match sign with Some c -> String.make 1 c | None -> "") ^
                  chars_to_string int_part in
          Ok (float_of_string s, r3)
        | Error e -> Error e)
     | Error _ ->
       (match satisfy (fun c -> c = '.') "dot" r1 with
        | Ok (_, r2) ->
          (match many1 digit r2 with
           | Ok (frac_part, r3) ->
             let s = (match sign with Some c -> String.make 1 c | None -> "") ^
                     "0." ^ chars_to_string frac_part in
             Ok (float_of_string s, r3)
           | Error e -> Error e)
        | Error _ -> Error "Expected number"))
  | Error e -> Error e

(* Tests *)
let () =
  assert (float_string "42rest" = Ok ("42", "rest"));
  assert (float_string "3.14!" = Ok ("3.14", "!"));
  assert (float_string "-2.5x" = Ok ("-2.5", "x"));
  assert (float_string "1e10" = Ok ("1e10", ""));
  assert (float_string "1.5e-3" = Ok ("1.5e-3", ""));

  assert (number_combinator "42" = Ok (42.0, ""));
  assert (number_combinator "3.14" = Ok (3.14, ""));
  assert (number_combinator "-2.5" = Ok (-2.5, ""));
  assert (number_combinator ".5" = Ok (0.5, ""));

  print_endline "✓ All tests passed"
