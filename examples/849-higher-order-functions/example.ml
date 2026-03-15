(* Higher-order functions: functions that take or return other functions *)
let apply f x = f x
let twice f x = f (f x)
let compose f g x = f (g x)
let ( |> ) x f = f x

let () =
  Printf.printf "%d\n" (apply (fun x -> x * 2) 21);
  Printf.printf "%d\n" (twice (fun x -> x + 1) 5);
  Printf.printf "%d\n" ((compose (fun x -> x * 2) (fun x -> x + 1)) 5);
  Printf.printf "%d\n" (5 |> (fun x -> x + 1) |> (fun x -> x * 2))
