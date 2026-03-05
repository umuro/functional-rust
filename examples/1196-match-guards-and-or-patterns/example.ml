(* Match Guards and Or-Patterns *)
(* Pattern matching with when guards and combined patterns *)

let classify_char = function
  | 'a' | 'e' | 'i' | 'o' | 'u'
  | 'A' | 'E' | 'I' | 'O' | 'U' -> "vowel"
  | c when c >= 'a' && c <= 'z' -> "consonant"
  | c when c >= 'A' && c <= 'Z' -> "consonant"
  | c when c >= '0' && c <= '9' -> "digit"
  | _ -> "other"

let fizzbuzz n = match (n mod 3, n mod 5) with
  | (0, 0) -> "FizzBuzz"
  | (0, _) -> "Fizz"
  | (_, 0) -> "Buzz"
  | _ -> string_of_int n

let () = List.init 20 (fun i -> i+1)
  |> List.iter (fun n -> Printf.printf "%s " (fizzbuzz n))
