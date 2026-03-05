(* Tuple pattern matching in OCaml *)
let fizzbuzz n = match (n mod 3 = 0, n mod 5 = 0) with
  | (true,true)  -> "FizzBuzz"
  | (true,false) -> "Fizz"
  | (false,true) -> "Buzz"
  | (false,false)-> string_of_int n

type light = Red | Yellow | Green
let next (l, emergency) = match (l,emergency) with
  | (_,true)       -> Red
  | (Red,false)    -> Green
  | (Green,false)  -> Yellow
  | (Yellow,false) -> Red

let () =
  List.iter (fun n -> Printf.printf "%s " (fizzbuzz n)) (List.init 15 (fun i->i+1));
  print_newline ()
