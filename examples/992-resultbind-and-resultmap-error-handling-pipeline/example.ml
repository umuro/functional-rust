(* Result.bind and Result.map — Error Handling Pipeline *)
(* Chain computations that may fail *)

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error ("Not a number: " ^ s)

let check_positive n =
  if n > 0 then Ok n else Error "Must be positive"

let check_range n =
  if n <= 100 then Ok n else Error "Must be <= 100"

let validate s =
  parse_int s
  |> Result.bind check_positive
  |> Result.bind check_range
  |> Result.map (fun n -> n * 2)

let () = match validate "42" with
  | Ok v -> Printf.printf "Valid: %d\n" v
  | Error e -> Printf.printf "Error: %s\n" e
