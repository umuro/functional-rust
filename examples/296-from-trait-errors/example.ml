(* 296. From trait for error conversion - OCaml *)
(* OCaml: manual Result.map_error at each call site *)

type app_error =
  | ParseError of string
  | LogicError of string

let parse_number s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (ParseError s)

let validate_positive n =
  if n > 0 then Ok n
  else Error (LogicError (Printf.sprintf "%d is not positive" n))

(* No automatic conversion - must map_error manually *)
let process s =
  let ( let* ) = Result.bind in
  let* n = parse_number s in
  let* v = validate_positive n in
  Ok (v * 2)

let () =
  match process "21" with
  | Ok n -> Printf.printf "Result: %d\n" n
  | Error (ParseError s) -> Printf.printf "Parse error: %s\n" s
  | Error (LogicError s) -> Printf.printf "Logic error: %s\n" s
