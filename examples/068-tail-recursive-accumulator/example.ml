(* 068: Tail-Recursive Accumulator *)
(* Transform naive recursion into tail-recursive form *)

(* Approach 1: Naive vs tail-recursive sum *)
let rec sum_naive = function
  | [] -> 0
  | x :: xs -> x + sum_naive xs

let sum_tail lst =
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (acc + x) xs
  in
  aux 0 lst

(* Approach 2: Factorial *)
let rec fact_naive n =
  if n <= 1 then 1 else n * fact_naive (n - 1)

let fact_tail n =
  let rec aux acc n =
    if n <= 1 then acc
    else aux (acc * n) (n - 1)
  in
  aux 1 n

(* Approach 3: Fibonacci with accumulator *)
let fib_tail n =
  let rec aux a b n =
    if n = 0 then a
    else aux b (a + b) (n - 1)
  in
  aux 0 1 n

let rec fib_naive n =
  if n <= 1 then n
  else fib_naive (n - 1) + fib_naive (n - 2)

(* Tests *)
let () =
  assert (sum_naive [1; 2; 3; 4; 5] = 15);
  assert (sum_tail [1; 2; 3; 4; 5] = 15);
  assert (sum_tail [] = 0);
  assert (fact_naive 5 = 120);
  assert (fact_tail 5 = 120);
  assert (fact_tail 0 = 1);
  assert (fib_tail 0 = 0);
  assert (fib_tail 1 = 1);
  assert (fib_tail 10 = 55);
  assert (fib_naive 10 = 55);
  (* Tail-recursive can handle large inputs *)
  let large = List.init 100000 (fun _ -> 1) in
  assert (sum_tail large = 100000);
  Printf.printf "✓ All tests passed\n"
