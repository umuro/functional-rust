(* 279. Random access with nth() - OCaml *)

let safe_nth lst n =
  if n < 0 || n >= List.length lst then None
  else Some (List.nth lst n)

let () =
  let nums = [10; 20; 30; 40; 50] in
  Printf.printf "0th: %s\n" (match safe_nth nums 0 with Some n -> string_of_int n | None -> "None");
  Printf.printf "2nd: %s\n" (match safe_nth nums 2 with Some n -> string_of_int n | None -> "None");
  Printf.printf "10th: %s\n" (match safe_nth nums 10 with Some n -> string_of_int n | None -> "None");

  (* Nth after filtering *)
  let evens = List.filter (fun x -> x mod 2 = 0) nums in
  Printf.printf "2nd even: %s\n"
    (match safe_nth evens 1 with Some n -> string_of_int n | None -> "None")
