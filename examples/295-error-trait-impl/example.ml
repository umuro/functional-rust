(* 295. Implementing std::error::Error - OCaml *)
(* OCaml: errors are values, no standard trait *)

exception ParseError of string
exception ChainedError of string * exn

let () =
  let safe_parse s =
    try Ok (int_of_string s)
    with Failure _ -> Error (ParseError ("not a number: " ^ s))
  in
  let with_context msg result =
    match result with
    | Ok _ as r -> r
    | Error e -> Error (ChainedError (msg, e))
  in
  let result = safe_parse "abc" |> with_context "in user input" in
  (match result with
  | Ok n -> Printf.printf "Ok: %d\n" n
  | Error (ChainedError (msg, ParseError cause)) ->
    Printf.printf "Error [%s]: %s\n" msg cause
  | Error e -> Printf.printf "Error: %s\n" (Printexc.to_string e))
