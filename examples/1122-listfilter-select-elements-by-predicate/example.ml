(* Idiomatic OCaml — List.filter with a predicate function *)
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds  = List.filter (fun x -> x mod 2 <> 0) numbers

(* Recursive OCaml — explicit structural recursion, mirrors List.filter *)
let rec filter_rec pred = function
  | [] -> []
  | x :: rest -> if pred x then x :: filter_rec pred rest else filter_rec pred rest

let () =
  Printf.printf "Evens: %s\n"
    (String.concat ", " (List.map string_of_int evens));
  Printf.printf "Odds: %s\n"
    (String.concat ", " (List.map string_of_int odds));
  assert (evens = [2; 4; 6; 8]);
  assert (odds  = [1; 3; 5; 7]);
  assert (filter_rec (fun x -> x mod 2 = 0) numbers = [2; 4; 6; 8]);
  assert (filter_rec (fun x -> x > 4) numbers = [5; 6; 7; 8]);
  print_endline "ok"
