(* Memoization with Lazy Values *)
(* Use Lazy for one-shot expensive computations *)

(* Lazy Fibonacci stream *)
type 'a stream = Cons of 'a * 'a stream Lazy.t

let rec fibs_from a b = Cons (a, lazy (fibs_from b (a + b)))
let fibs = fibs_from 0 1

let rec take n (Cons (x, rest)) =
  if n <= 0 then []
  else x :: take (n - 1) (Lazy.force rest)

let rec nth n (Cons (x, rest)) =
  if n = 0 then x else nth (n - 1) (Lazy.force rest)

let () =
  Printf.printf "First 10 fibs: %s\n"
    (String.concat " " (List.map string_of_int (take 10 fibs)));
  Printf.printf "Fib(20) = %d\n" (nth 20 fibs)
