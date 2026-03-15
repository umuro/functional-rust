let numbers = [1; 2; 3; 4; 5; 6; 7; 8; 9; 10]
let (small, big) = List.partition (fun x -> x <= 5) numbers
let () = Printf.printf "Small: %s\n"
  (String.concat " " (List.map string_of_int small))
let () = Printf.printf "Big: %s\n"
  (String.concat " " (List.map string_of_int big))
