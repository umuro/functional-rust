(* 259. Flattening with flat_map() - OCaml *)

let () =
  let words = ["hello"; "world"] in
  let chars = List.concat_map (fun w ->
    List.init (String.length w) (fun i -> w.[i])
  ) words in
  List.iter (fun c -> Printf.printf "%c " c) chars;
  print_newline ();

  let nums = [1; 2; 3] in
  let expanded = List.concat_map (fun n -> List.init n Fun.id) nums in
  Printf.printf "%s\n" (String.concat ", " (List.map string_of_int expanded));

  let strs = ["1"; "two"; "3"; "four"; "5"] in
  let parsed = List.concat_map (fun s ->
    match int_of_string_opt s with
    | Some n -> [n * 2]
    | None -> []
  ) strs in
  Printf.printf "%s\n" (String.concat ", " (List.map string_of_int parsed))
