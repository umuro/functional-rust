let ( |> ) x f = f x
let double x = 2 * x
let add1 x = x + 1
let result = 5 |> double |> add1
let shout s = String.uppercase_ascii s
let exclaim s = s ^ "!"
let greeting = "hello" |> shout |> exclaim
let () =
  Printf.printf "%d\n" result;
  Printf.printf "%s\n" greeting
