(* 1007: Result Combinators *)
(* Functional transforms on Result values *)

(* OCaml Result module combinators *)
let map f = function Ok v -> Ok (f v) | Error e -> Error e
let map_error f = function Ok v -> Ok v | Error e -> Error (f e)
let bind f = function Ok v -> f v | Error e -> Error e  (* and_then *)

(* Approach 1: Manual pattern matching *)
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "not an int: %s" s)

let double_if_positive n =
  if n > 0 then Ok (n * 2)
  else Error "must be positive"

let process_manual s =
  match parse_int s with
  | Error e -> Error e
  | Ok n ->
    match double_if_positive n with
    | Error e -> Error e
    | Ok v -> Ok (string_of_int v)

(* Approach 2: Using combinators *)
let process_combinators s =
  parse_int s
  |> bind double_if_positive
  |> map string_of_int

(* or_else equivalent *)
let or_else f = function
  | Ok v -> Ok v
  | Error e -> f e

let with_default default = function
  | Ok v -> v
  | Error _ -> default

let test_manual () =
  assert (process_manual "5" = Ok "10");
  assert (process_manual "-3" = Error "must be positive");
  assert (process_manual "abc" = Error "not an int: abc");
  Printf.printf "  Approach 1 (manual matching): passed\n"

let test_combinators () =
  assert (process_combinators "5" = Ok "10");
  assert (process_combinators "-3" = Error "must be positive");
  (* map_error *)
  let r = map_error String.uppercase_ascii (Error "low") in
  assert (r = Error "LOW");
  (* or_else *)
  let r = or_else (fun _ -> Ok 0) (Error "fail") in
  assert (r = Ok 0);
  (* unwrap_or / with_default *)
  assert (with_default 99 (Error "fail") = 99);
  assert (with_default 99 (Ok 42) = 42);
  Printf.printf "  Approach 2 (combinators): passed\n"

let () =
  Printf.printf "Testing result combinators:\n";
  test_manual ();
  test_combinators ();
  Printf.printf "✓ All tests passed\n"
