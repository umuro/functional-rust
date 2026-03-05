(* Tail Recursion *)
(* Convert recursive functions to tail-recursive form *)

(* Non-tail-recursive: stack overflow on large lists *)
let rec sum_naive = function
  | [] -> 0
  | x :: xs -> x + sum_naive xs

(* Tail-recursive with accumulator *)
let sum lst =
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (acc + x) xs
  in aux 0 lst

(* Tail-recursive map using rev *)
let map f lst =
  let rec aux acc = function
    | [] -> List.rev acc
    | x :: xs -> aux (f x :: acc) xs
  in aux [] lst

let () = Printf.printf "Sum: %d\n" (sum (List.init 1000000 Fun.id))
