(* Result chaining in OCaml *)
let (let*) = Result.bind

let parse s = match int_of_string_opt s with
  | Some n -> Ok n | None -> Error (Printf.sprintf "not int: %s" s)

let validate n =
  if n>=1 && n<=100 then Ok n else Error (Printf.sprintf "%d out of range" n)

let process s =
  let* n = parse s in
  let* v = validate n in
  Ok (v*v)

let () =
  List.iter (fun s ->
    match process s with
    | Ok v  -> Printf.printf "%s->%d\n" s v
    | Error e -> Printf.printf "%s->Err:%s\n" s e
  ) ["42";"abc";"0";"100";"101"]
