(* 068: Tail-Recursive Accumulator
   OCaml DOES guarantee tail-call optimisation — tail-rec is safe for large inputs *)

(* --- Approach 1: Naive vs tail-recursive sum --- *)

(* NOT tail-recursive: + waits after recursive call *)
let rec sum_naive = function
  | [] -> 0
  | x :: xs -> x + sum_naive xs

(* Tail-recursive with explicit accumulator *)
let sum_acc xs =
  let rec aux acc = function
    | [] -> acc
    | x :: rest -> aux (acc + x) rest
  in
  aux 0 xs

(* Stdlib List.fold_left is also tail-recursive *)
let sum_fold xs = List.fold_left ( + ) 0 xs

(* --- Approach 2: Factorial --- *)

let rec fact_naive n =
  if n <= 1 then 1 else n * fact_naive (n - 1)

let fact_tail n =
  let rec aux acc i =
    if i <= 1 then acc else aux (acc * i) (i - 1)
  in
  aux 1 n

(* --- Approach 3: Fibonacci with accumulator (O(n) time, O(1) space) --- *)

let fib n =
  (* carry two consecutive values forward *)
  let rec aux a b = function
    | 0 -> a
    | k -> aux b (a + b) (k - 1)
  in
  aux 0 1 n

let () =
  let xs = List.init 5 (fun i -> i + 1) in   (* [1;2;3;4;5] *)
  Printf.printf "sum_naive [1..5]  = %d\n" (sum_naive xs);
  Printf.printf "sum_acc   [1..5]  = %d\n" (sum_acc xs);
  Printf.printf "sum_fold  [1..5]  = %d\n" (sum_fold xs);
  Printf.printf "fact_naive 5      = %d\n" (fact_naive 5);
  Printf.printf "fact_tail  5      = %d\n" (fact_tail 5);
  Printf.printf "fib 0 = %d\n" (fib 0);
  Printf.printf "fib 1 = %d\n" (fib 1);
  Printf.printf "fib 10 = %d\n" (fib 10);
  (* large input — safe because tail-recursive *)
  let large = List.init 100_000 (fun _ -> 1) in
  Printf.printf "sum of 100k ones = %d\n" (sum_acc large)
