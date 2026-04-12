let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds = List.filter (fun x -> x mod 2 <> 0) numbers
let () = Printf.printf "Evens: %s\n"
  (String.concat ", " (List.map string_of_int evens))
let () = Printf.printf "Odds: %s\n"
  (String.concat ", " (List.map string_of_int odds))