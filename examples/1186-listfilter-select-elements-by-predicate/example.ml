(* Idiomatic OCaml — using List.filter with a predicate closure *)
let filter pred lst = List.filter pred lst

(* Recursive OCaml — shows the explicit structural recursion *)
let rec filter_rec pred = function
  | [] -> []
  | head :: tail ->
    if pred head then head :: filter_rec pred tail
    else filter_rec pred tail

let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = filter (fun x -> x mod 2 = 0) numbers
let odds  = filter (fun x -> x mod 2 <> 0) numbers

let () =
  assert (evens = [2; 4; 6; 8]);
  assert (odds  = [1; 3; 5; 7]);
  assert (filter (fun x -> x > 10) numbers = []);
  assert (filter_rec (fun x -> x mod 2 = 0) numbers = [2; 4; 6; 8]);
  Printf.printf "Evens: %s\n" (String.concat ", " (List.map string_of_int evens));
  Printf.printf "Odds: %s\n"  (String.concat ", " (List.map string_of_int odds));
  print_endline "ok"
