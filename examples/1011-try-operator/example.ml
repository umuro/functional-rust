(* 1011: The Try Operator (?) *)
(* OCaml has no ? — we simulate with bind/let* *)

type error = NotFound | ParseFailed of string | TooLarge of int

(* Approach 1: Nested match — what Rust's ? replaces *)
let read_data key =
  if key = "missing" then Error NotFound
  else Ok "42"

let parse_data s =
  match int_of_string_opt s with
  | None -> Error (ParseFailed s)
  | Some n -> Ok n

let validate n =
  if n > 100 then Error (TooLarge n)
  else Ok n

let process_nested key =
  match read_data key with
  | Error e -> Error e
  | Ok s ->
    match parse_data s with
    | Error e -> Error e
    | Ok n ->
      match validate n with
      | Error e -> Error e
      | Ok v -> Ok v

(* Approach 2: bind operator — monadic chaining *)
let ( >>= ) r f = match r with Ok v -> f v | Error e -> Error e

let process_bind key =
  read_data key >>= parse_data >>= validate

(* Approach 3: let* syntax (OCaml 4.08+ binding operators) *)
let ( let* ) = Result.bind

let process_let_star key =
  let* s = read_data key in
  let* n = parse_data s in
  let* v = validate n in
  Ok v

let test_nested () =
  assert (process_nested "ok" = Ok 42);
  assert (process_nested "missing" = Error NotFound);
  Printf.printf "  Approach 1 (nested match): passed\n"

let test_bind () =
  assert (process_bind "ok" = Ok 42);
  assert (process_bind "missing" = Error NotFound);
  Printf.printf "  Approach 2 (bind operator): passed\n"

let test_let_star () =
  assert (process_let_star "ok" = Ok 42);
  assert (process_let_star "missing" = Error NotFound);
  Printf.printf "  Approach 3 (let* syntax): passed\n"

let () =
  Printf.printf "Testing try operator equivalents:\n";
  test_nested ();
  test_bind ();
  test_let_star ();
  Printf.printf "✓ All tests passed\n"
