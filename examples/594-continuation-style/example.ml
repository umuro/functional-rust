(* Continuation-passing style in OCaml *)

(* Direct style *)
let rec fact n = if n <= 1 then 1 else n * fact (n-1)

(* CPS style *)
let rec fact_k n k =
  if n <= 1 then k 1
  else fact_k (n-1) (fun result -> k (n * result))

(* CPS fibonacci *)
let rec fib_k n k =
  if n <= 1 then k n
  else
    fib_k (n-1) (fun r1 ->
    fib_k (n-2) (fun r2 ->
    k (r1 + r2)))

(* CPS list map *)
let rec map_k f lst k =
  match lst with
  | []      -> k []
  | x :: xs -> map_k f xs (fun rest -> k (f x :: rest))

let () =
  fact_k 10 (Printf.printf "10! = %d\n");
  fib_k 10  (Printf.printf "fib(10) = %d\n");
  map_k (fun x -> x*2) [1;2;3;4;5] (fun result ->
    List.iter (Printf.printf "%d ") result; print_newline ())
