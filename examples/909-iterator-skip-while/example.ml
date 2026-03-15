(* 265. Conditional skipping with skip_while() - OCaml *)

let rec skip_while pred = function
  | [] -> []
  | x :: xs as lst ->
    if pred x then skip_while pred xs else lst

let () =
  let nums = [1; 2; 3; 4; 5; 4; 3; 2; 1] in
  let from_4 = skip_while (fun x -> x < 4) nums in
  Printf.printf "Skip <4: %s\n"
    (String.concat ", " (List.map string_of_int from_4));

  let tokens = [' '; ' '; 'h'; 'e'; 'l'; 'l'; 'o'] in
  let stripped = skip_while (fun c -> c = ' ') tokens in
  Printf.printf "Stripped: '%s'\n"
    (String.concat "" (List.map (String.make 1) stripped));

  let with_zeros = [0; 0; 0; 1; 2; 3; 0; 4] in
  let no_leading = skip_while (fun x -> x = 0) with_zeros in
  Printf.printf "No leading zeros: %s\n"
    (String.concat ", " (List.map string_of_int no_leading))
