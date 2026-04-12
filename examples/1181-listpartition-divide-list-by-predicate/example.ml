(* Idiomatic OCaml — List.partition splits a list by a predicate *)
let numbers = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let (small, big) = List.partition (fun x -> x <= 5) numbers

let () = Printf.printf "Small: %s\n"
  (String.concat " " (List.map string_of_int small))
let () = Printf.printf "Big: %s\n"
  (String.concat " " (List.map string_of_int big))

(* Recursive OCaml — shows the explicit structural recursion *)
let rec partition_rec pred = function
  | [] -> ([], [])
  | x :: rest ->
    let (yes, no) = partition_rec pred rest in
    if pred x then (x :: yes, no)
    else (yes, x :: no)

let () =
  let (s, b) = partition_rec (fun x -> x <= 5) numbers in
  assert (s = [1; 2; 3; 4; 5]);
  assert (b = [6; 7; 8; 9; 10]);
  let (evens, odds) = List.partition (fun x -> x mod 2 = 0) numbers in
  assert (evens = [2; 4; 6; 8; 10]);
  assert (odds = [1; 3; 5; 7; 9]);
  assert (List.partition (fun _ -> true) [] = ([], []));
  print_endline "ok"
