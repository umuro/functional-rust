(* 257. Pairing elements with zip() - OCaml *)

let () =
  let names = ["Alice"; "Bob"; "Carol"] in
  let scores = [95; 87; 92] in
  let paired = List.combine names scores in
  List.iter (fun (name, score) ->
    Printf.printf "%s: %d\n" name score
  ) paired;

  let (ns, ss) = List.split paired in
  Printf.printf "Names: %s\n" (String.concat ", " ns);
  Printf.printf "Scores: %s\n" (String.concat ", " (List.map string_of_int ss));

  let indexed = List.mapi (fun i x -> (i, x)) ["a"; "b"; "c"] in
  List.iter (fun (i, x) -> Printf.printf "%d: %s\n" i x) indexed
