let ( >>= ) r f = match r with
  | Error e -> Error e
  | Ok v    -> f v

let ( >>| ) r f = match r with
  | Error e -> Error e
  | Ok v    -> Ok (f v)

let parse_int s =
  match int_of_string_opt s with
  | None   -> Error (Printf.sprintf "not an integer: %S" s)
  | Some n -> Ok n

let positive x =
  if x > 0 then Ok x
  else Error (Printf.sprintf "%d is not positive" x)

let sqrt_safe x =
  positive x >>| (fun n -> sqrt (float_of_int n))

let process s =
  parse_int s >>= positive >>= sqrt_safe

let () =
  assert (process "16" = Ok 4.0);
  assert (process "25" = Ok 5.0);
  assert (process "-4" = Error "-4 is not positive");
  assert (process "hello" = Error "not an integer: \"hello\"");
  assert (process "0" = Error "0 is not positive");
  print_endline "All assertions passed."
