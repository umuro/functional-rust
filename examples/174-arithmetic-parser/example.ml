(* Example 174: Arithmetic Expression Evaluator *)
(* Full arithmetic evaluator: +,-,*,/ with precedence, parens, unary minus *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

let ws0 input =
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t') then skip (i+1) else i in
  let i = skip 0 in String.sub input i (String.length input - i)

(* Approach 1: Recursive descent evaluator *)
let parse_number input =
  let s = ws0 input in
  let len = String.length s in
  let pos = ref 0 in
  while !pos < len && (s.[!pos] >= '0' && s.[!pos] <= '9' || s.[!pos] = '.') do incr pos done;
  if !pos = 0 then Error "Expected number"
  else Ok (float_of_string (String.sub s 0 !pos), String.sub s !pos (len - !pos))

let rec eval_expr input = eval_additive input

and eval_additive input =
  match eval_multiplicative input with
  | Error e -> Error e
  | Ok (lhs, rest) -> eval_additive_loop lhs rest

and eval_additive_loop lhs input =
  let s = ws0 input in
  if String.length s > 0 && s.[0] = '+' then
    match eval_multiplicative (String.sub s 1 (String.length s - 1)) with
    | Ok (rhs, rest) -> eval_additive_loop (lhs +. rhs) rest
    | Error e -> Error e
  else if String.length s > 0 && s.[0] = '-' then
    match eval_multiplicative (String.sub s 1 (String.length s - 1)) with
    | Ok (rhs, rest) -> eval_additive_loop (lhs -. rhs) rest
    | Error e -> Error e
  else Ok (lhs, s)

and eval_multiplicative input =
  match eval_unary input with
  | Error e -> Error e
  | Ok (lhs, rest) -> eval_multiplicative_loop lhs rest

and eval_multiplicative_loop lhs input =
  let s = ws0 input in
  if String.length s > 0 && s.[0] = '*' then
    match eval_unary (String.sub s 1 (String.length s - 1)) with
    | Ok (rhs, rest) -> eval_multiplicative_loop (lhs *. rhs) rest
    | Error e -> Error e
  else if String.length s > 0 && s.[0] = '/' then
    match eval_unary (String.sub s 1 (String.length s - 1)) with
    | Ok (rhs, rest) ->
      if rhs = 0.0 then Error "Division by zero"
      else eval_multiplicative_loop (lhs /. rhs) rest
    | Error e -> Error e
  else Ok (lhs, s)

and eval_unary input =
  let s = ws0 input in
  if String.length s > 0 && s.[0] = '-' then
    match eval_unary (String.sub s 1 (String.length s - 1)) with
    | Ok (v, rest) -> Ok (-. v, rest)
    | Error e -> Error e
  else eval_primary s

and eval_primary input =
  let s = ws0 input in
  if String.length s > 0 && s.[0] = '(' then
    match eval_expr (String.sub s 1 (String.length s - 1)) with
    | Ok (v, rest) ->
      let r = ws0 rest in
      if String.length r > 0 && r.[0] = ')' then
        Ok (v, String.sub r 1 (String.length r - 1))
      else Error "Expected ')'"
    | Error e -> Error e
  else parse_number s

(* Approach 2: Evaluate string directly *)
let evaluate (expr : string) : (float, string) result =
  match eval_expr expr with
  | Ok (v, rest) ->
    if String.length (ws0 rest) = 0 then Ok v
    else Error (Printf.sprintf "Unexpected trailing: \"%s\"" rest)
  | Error e -> Error e

(* Approach 3: With function support *)
let eval_function name arg =
  match name with
  | "sqrt" -> Ok (sqrt arg)
  | "abs" -> Ok (abs_float arg)
  | "sin" -> Ok (sin arg)
  | "cos" -> Ok (cos arg)
  | _ -> Error (Printf.sprintf "Unknown function: %s" name)

(* Tests *)
let () =
  assert (evaluate "2 + 3" = Ok 5.0);
  assert (evaluate "2 + 3 * 4" = Ok 14.0);
  assert (evaluate "(2 + 3) * 4" = Ok 20.0);
  assert (evaluate "10 / 2 - 3" = Ok 2.0);
  assert (evaluate "-5" = Ok (-5.0));
  assert (evaluate "-(2 + 3)" = Ok (-5.0));
  assert (evaluate "2 * -3" = Ok (-6.0));
  assert (evaluate "1.5 + 2.5" = Ok 4.0);
  assert (Result.is_error (evaluate "1 / 0"));
  assert (Result.is_error (evaluate "2 +"));

  print_endline "✓ All tests passed"
