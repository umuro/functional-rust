(* 272. One-level flattening with flatten() - OCaml *)

let () =
  let nested = [[1; 2]; [3; 4]; [5; 6]] in
  let flat = List.concat nested in
  Printf.printf "Flattened: %s\n"
    (String.concat ", " (List.map string_of_int flat));

  let words = ["hello"; "world"] in
  let all_chars = List.concat_map
    (fun w -> List.init (String.length w) (fun i -> w.[i]))
    words in
  Printf.printf "Chars: %s\n"
    (String.concat " " (List.map (String.make 1) all_chars));

  let opts = [Some 1; None; Some 3; None; Some 5] in
  let values = List.filter_map Fun.id opts in
  Printf.printf "Option values: %s\n"
    (String.concat ", " (List.map string_of_int values))
