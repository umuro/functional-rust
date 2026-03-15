(* 1023: Safe Integer Parsing *)
(* OCaml int_of_string_opt vs exception-based parsing *)

(* Approach 1: Exception-based (old style) *)
let parse_exn s =
  try int_of_string s
  with Failure _ -> 0  (* silent default — BAD *)

(* Approach 2: Option-based (safe) *)
let parse_opt s = int_of_string_opt s

let parse_or_default default s =
  match int_of_string_opt s with
  | Some n -> n
  | None -> default

(* Approach 3: Result-based with error message *)
let parse_result s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "cannot parse '%s' as integer" s)

let parse_positive s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "not a number: %s" s)
  | Some n when n < 0 -> Error (Printf.sprintf "negative: %d" n)
  | Some n -> Ok n

(* Parse with range validation *)
let parse_in_range ~min ~max s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "not a number: %s" s)
  | Some n when n < min -> Error (Printf.sprintf "%d < min(%d)" n min)
  | Some n when n > max -> Error (Printf.sprintf "%d > max(%d)" n max)
  | Some n -> Ok n

let test_exception () =
  assert (parse_exn "42" = 42);
  assert (parse_exn "abc" = 0);  (* silent failure! *)
  Printf.printf "  Approach 1 (exception, unsafe): passed\n"

let test_option () =
  assert (parse_opt "42" = Some 42);
  assert (parse_opt "abc" = None);
  assert (parse_opt "" = None);
  assert (parse_or_default 0 "abc" = 0);
  assert (parse_or_default 0 "42" = 42);
  Printf.printf "  Approach 2 (option): passed\n"

let test_result () =
  assert (parse_result "42" = Ok 42);
  (match parse_result "abc" with Error _ -> () | Ok _ -> assert false);
  assert (parse_positive "42" = Ok 42);
  assert (parse_positive "-5" = Error "negative: -5");
  assert (parse_in_range ~min:1 ~max:100 "50" = Ok 50);
  assert (parse_in_range ~min:1 ~max:100 "0" |> Result.is_error);
  assert (parse_in_range ~min:1 ~max:100 "101" |> Result.is_error);
  Printf.printf "  Approach 3 (result): passed\n"

let () =
  Printf.printf "Testing safe integer parsing:\n";
  test_exception ();
  test_option ();
  test_result ();
  Printf.printf "✓ All tests passed\n"
