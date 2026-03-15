(* 266. Striding with step_by() - OCaml *)

let step_by n lst = List.filteri (fun i _ -> i mod n = 0) lst

let () =
  let nums = List.init 10 Fun.id in
  let thirds = step_by 3 nums in
  Printf.printf "Every 3rd: %s\n"
    (String.concat ", " (List.map string_of_int thirds));

  let mult_5 = step_by 5 (List.init 51 Fun.id) in
  Printf.printf "Multiples of 5: %s\n"
    (String.concat ", " (List.map string_of_int mult_5));

  let s = "abcdefgh" in
  let chars = List.init (String.length s) (fun i -> s.[i]) in
  let every_other = step_by 2 chars in
  Printf.printf "Every other char: %s\n"
    (String.concat "" (List.map (String.make 1) every_other))
