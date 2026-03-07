(* The pipeline operator is just a higher-order function *)
let ( |> ) x f = f x

let double x = 2 * x
let add1   x = x + 1

(* Read: start with 5, double it, add 1 *)
let result = 5 |> double |> add1   (* 11 *)

(* Chaining string operations *)
let shout s = String.uppercase_ascii s
let exclaim s = s ^ "!"

let greeting = "hello" |> shout |> exclaim   (* "HELLO!" *)

let () =
  Printf.printf "%d\n" result;
  Printf.printf "%s\n" greeting