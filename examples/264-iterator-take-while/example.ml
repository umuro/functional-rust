(* 264. Conditional stopping with take_while() - OCaml *)

let rec take_while pred = function
  | [] -> []
  | x :: xs -> if pred x then x :: take_while pred xs else []

let () =
  let nums = [1; 2; 3; 4; 5; 6; 7; 8; 9] in
  let small = take_while (fun x -> x < 5) nums in
  Printf.printf "Less than 5: %s\n"
    (String.concat ", " (List.map string_of_int small));

  let data = [3; 1; 4; 1; -5; 9; -2; 6] in
  let positives = take_while (fun x -> x > 0) data in
  Printf.printf "Leading positives: %s\n"
    (String.concat ", " (List.map string_of_int positives));

  let naturals = List.init 100 (fun i -> i + 1) in
  let triangular = take_while (fun n -> n * (n+1) / 2 < 30) naturals in
  Printf.printf "n where triangular(n) < 30: %s\n"
    (String.concat ", " (List.map string_of_int triangular))
