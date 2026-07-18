(* Result Bind *)
(* OCaml 99 Problems #47 *)

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "invalid integer: %s" s)

let safe_div a b = if b = 0 then Error "division by zero" else Ok (a / b)

let pipeline s divisor =
  Result.bind (parse_int s) (fun n -> safe_div n divisor) |> Result.map string_of_int

(* Tests *)
let () =
  assert (pipeline "100" 5 = Ok "20");
  assert (pipeline "abc" 5 = Error "invalid integer: abc");
  assert (pipeline "100" 0 = Error "division by zero");
  print_endline "✓ OCaml tests passed"
