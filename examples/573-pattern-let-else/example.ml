(* OCaml option-chaining to avoid nesting *)
let (let*) = Option.bind

let parse_port s =
  let* n = (try Some(int_of_string s) with _ -> None) in
  if n > 0 && n < 65536 then Some n else None

let parse_config line =
  match String.split_on_char ':' line with
  | [host; port_str] ->
    let* port = parse_port port_str in
    Some (host, port)
  | _ -> None

let () =
  List.iter (fun line ->
    match parse_config line with
    | Some(h,p) -> Printf.printf "-> %s:%d\n" h p
    | None      -> Printf.printf "invalid: %s\n" line
  ) ["localhost:8080";"bad";"host:notaport";"example.com:443"]
