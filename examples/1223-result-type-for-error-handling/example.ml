type error = string

let safe_int_of_string s =
  try Ok (int_of_string s)
  with Failure _ -> Error ("Cannot parse: " ^ s)

let pipeline result =
  result >>= fun x ->
  if x > 0 then Ok x
  else Error "Not positive"

let () =
  match pipeline (safe_int_of_string "42") with
  | Ok x -> Printf.printf "Success: %d\n" x
  | Error e -> Printf.printf "Error: %s\n" e