(* OCaml >>= for Result: short-circuit on Error, apply f to Ok *)
let ( >>= ) r f = match r with
  | Error _ as e -> e
  | Ok x -> f x

(* Idiomatic: parse string to int, returning Result *)
let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not an integer: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_even n =
  if n mod 2 = 0 then Ok n else Error "Must be even"

(* Railway-oriented: chain validations with >>= *)
let validate s =
  parse_int s >>= check_positive >>= check_even

(* Recursive-style: explicit function composition via bind *)
let rec bind r f = match r with
  | Error _ as e -> e
  | Ok x -> f x

let validate_explicit s =
  bind (bind (parse_int s) check_positive) check_even

let () =
  assert (validate "42" = Ok 42);
  assert (validate "-3" = Error "Must be positive");
  assert (validate "abc" = Error "Not an integer: abc");
  assert (validate "7" = Error "Must be even");
  assert (validate "0" = Error "Must be positive");
  assert (validate_explicit "42" = Ok 42);
  assert (validate_explicit "-3" = Error "Must be positive");
  List.iter (fun s ->
    match validate s with
    | Ok n -> Printf.printf "%s -> Ok %d\n" s n
    | Error e -> Printf.printf "%s -> Error: %s\n" s e
  ) ["42"; "-3"; "abc"; "7"];
  print_endline "ok"
