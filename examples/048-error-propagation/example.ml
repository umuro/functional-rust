(* Error Propagation *)
(* OCaml 99 Problems #48 *)

(* OCaml has no ? operator; explicit match at each step is the direct equivalent
   of what "?" desugars to in Rust. *)
type app_error = ParseError of string | DivisionByZero

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (ParseError s)

let safe_div a b = if b = 0 then Error DivisionByZero else Ok (a / b)

let compute s divisor =
  match parse_int s with
  | Error e -> Error e
  | Ok n -> (
      match safe_div n divisor with
      | Error e -> Error e
      | Ok d -> Ok (d * 2))

(* Tests *)
let () =
  assert (compute "10" 2 = Ok 10);
  assert (compute "abc" 2 = Error (ParseError "abc"));
  assert (compute "10" 0 = Error DivisionByZero);
  print_endline "✓ OCaml tests passed"
