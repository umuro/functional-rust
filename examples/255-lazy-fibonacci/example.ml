(* Example 255: Lazy Fibonacci — Infinite stream using thunks *)

(* Idiomatic OCaml: coinductive stream type *)
type 'a stream = Cons of 'a * (unit -> 'a stream)

(* Build an infinite Fibonacci stream starting from (a, b) *)
let rec fibs a b = Cons (a, fun () -> fibs b (a + b))

(* Take the first n elements from a stream *)
let rec take n (Cons (x, rest)) =
  if n = 0 then []
  else x :: take (n - 1) (rest ())

(* Recursive: peek at the nth element *)
let rec nth n (Cons (x, rest)) =
  if n = 0 then x
  else nth (n - 1) (rest ())

let () =
  let fib_stream = fibs 0 1 in
  let first_ten = take 10 fib_stream in
  assert (first_ten = [0; 1; 1; 2; 3; 5; 8; 13; 21; 34]);
  assert (nth 0 fib_stream = 0);
  assert (nth 9 fib_stream = 34);
  assert (take 5 (fibs 1 1) = [1; 1; 2; 3; 5]);
  List.iter (Printf.printf "%d ") first_ten;
  print_newline ();
  print_endline "ok"
