(* 293. ? operator and early returns - OCaml *)
(* OCaml: use let* with Result monad or explicit bind *)

let ( let* ) = Result.bind

let parse_and_add s1 s2 =
  let* a = match int_of_string_opt s1 with
    | Some n -> Ok n | None -> Error ("bad: " ^ s1) in
  let* b = match int_of_string_opt s2 with
    | Some n -> Ok n | None -> Error ("bad: " ^ s2) in
  Ok (a + b)

let () =
  (match parse_and_add "10" "20" with
  | Ok n -> Printf.printf "Sum: %d\n" n
  | Error e -> Printf.printf "Error: %s\n" e);

  (match parse_and_add "10" "abc" with
  | Ok n -> Printf.printf "Sum: %d\n" n
  | Error e -> Printf.printf "Error: %s\n" e);

  (* Option version *)
  let ( let* ) = Option.bind in
  let lookup env key =
    let* value = List.assoc_opt key env in
    let* n = int_of_string_opt value in
    Some (n * 2)
  in
  let env = [("x", "5"); ("y", "bad")] in
  (match lookup env "x" with Some n -> Printf.printf "x*2=%d\n" n | None -> print_endline "None");
  (match lookup env "y" with Some n -> Printf.printf "y*2=%d\n" n | None -> print_endline "None")
