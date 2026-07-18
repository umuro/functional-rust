(* Result Basics *)
(* OCaml 99 Problems #45 *)

let parse_positive s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "not an integer: %s" s)
  | Some n when n <= 0 -> Error (Printf.sprintf "expected positive, got %d" n)
  | Some n -> Ok n

(* Tests *)
let () =
  assert (parse_positive "42" = Ok 42);
  assert (parse_positive "abc" = Error "not an integer: abc");
  assert (parse_positive "-5" = Error "expected positive, got -5");
  assert (parse_positive "0" = Error "expected positive, got 0");
  assert (Result.to_option (parse_positive "10") = Some 10);
  assert (Result.to_option (parse_positive "x") = None);
  print_endline "✓ OCaml tests passed"
