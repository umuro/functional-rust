(* 291. map(), and_then(), or_else() on Result - OCaml *)

let () =
  (* map transforms Ok value *)
  let ok : (int, string) result = Ok 5 in
  let mapped = Result.map (fun x -> x * 2) ok in
  Printf.printf "map Ok: %s\n"
    (match mapped with Ok n -> string_of_int n | Error e -> e);

  (* and_then chains fallible operations *)
  let parse s = match int_of_string_opt s with
    | Some n -> Ok n | None -> Error ("not a number: " ^ s)
  in
  let divide x y = if y = 0 then Error "division by zero" else Ok (x / y) in

  let result = parse "10" |> Result.bind (fun n -> divide n 2) in
  Printf.printf "chain: %s\n"
    (match result with Ok n -> string_of_int n | Error e -> "Error: " ^ e);

  (* Short-circuit on error *)
  let result2 = parse "abc" |> Result.bind (fun n -> divide n 2) in
  Printf.printf "short-circuit: %s\n"
    (match result2 with Ok n -> string_of_int n | Error e -> "Error: " ^ e);

  (* map_error transforms Err *)
  let rich_error = Result.map_error (fun e -> "Parse failed: " ^ e) (parse "abc") in
  (match rich_error with Error e -> Printf.printf "Error: %s\n" e | Ok _ -> ())
