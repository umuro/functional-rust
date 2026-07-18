(* Error Conversion *)
(* OCaml 99 Problems #49 *)

(* OCaml has no From-style automatic conversion; wrapping is always explicit. *)
type app_error = Parse of string | DivisionByZero

let wrap_parse s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Parse (Printf.sprintf "invalid integer: %s" s))

let safe_div a b = if b = 0 then Error DivisionByZero else Ok (a / b)

let compute s divisor =
  match wrap_parse s with
  | Error e -> Error e
  | Ok n -> safe_div n divisor

(* Tests *)
let () =
  assert (compute "10" 2 = Ok 5);
  (match compute "abc" 2 with
   | Error (Parse _) -> ()
   | _ -> assert false);
  assert (compute "10" 0 = Error DivisionByZero);
  print_endline "✓ OCaml tests passed"
