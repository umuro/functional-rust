(* 294. Defining custom Error types - OCaml *)

type parse_error =
  | InvalidNumber of string
  | OutOfRange of int * int * int
  | EmptyInput

let pp_parse_error = function
  | InvalidNumber s -> Printf.sprintf "invalid number: '%s'" s
  | OutOfRange (n, lo, hi) ->
    Printf.sprintf "value %d out of range [%d, %d]" n lo hi
  | EmptyInput -> "empty input"

let parse_bounded s lo hi =
  if String.length s = 0 then Error EmptyInput
  else match int_of_string_opt s with
    | None -> Error (InvalidNumber s)
    | Some n ->
      if n < lo || n > hi then Error (OutOfRange (n, lo, hi))
      else Ok n

let () =
  let tests = [("42", 0, 100); ("abc", 0, 100); ("", 0, 100); ("200", 0, 100)] in
  List.iter (fun (s, lo, hi) ->
    match parse_bounded s lo hi with
    | Ok n -> Printf.printf "Ok: %d\n" n
    | Error e -> Printf.printf "Error: %s\n" (pp_parse_error e)
  ) tests
