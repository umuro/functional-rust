(* Example 056: Result Monad *)
(* Result monad: chain computations that may fail with error info *)

let bind r f = match r with Error e -> Error e | Ok x -> f x
let ( >>= ) = bind
let return_ x = Ok x

(* Approach 1: Parsing pipeline *)
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "Not an integer: %s" s)

let check_positive n =
  if n > 0 then Ok n
  else Error (Printf.sprintf "Not positive: %d" n)

let check_even n =
  if n mod 2 = 0 then Ok n
  else Error (Printf.sprintf "Not even: %d" n)

let validate_input s =
  parse_int s >>= check_positive >>= check_even

(* Approach 2: Using Result.bind from stdlib *)
let validate_input_stdlib s =
  Result.bind (parse_int s) (fun n ->
  Result.bind (check_positive n) (fun n ->
  check_even n))

(* Approach 3: Map and bind combined *)
let double_validated s =
  validate_input s |> Result.map (fun n -> n * 2)

let () =
  assert (validate_input "42" = Ok 42);
  assert (validate_input "hello" = Error "Not an integer: hello");
  assert (validate_input "-4" = Error "Not positive: -4");
  assert (validate_input "7" = Error "Not even: 7");
  assert (double_validated "42" = Ok 84);

  (* Stdlib version *)
  assert (validate_input_stdlib "42" = Ok 42);
  assert (validate_input_stdlib "bad" = Error "Not an integer: bad");

  Printf.printf "✓ All tests passed\n"
