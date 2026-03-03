(* Continuation-passing style (CPS): instead of returning a value,
   pass it to a continuation function 'k'. Enables:
   - Tail-call optimization for any recursion
   - First-class control flow
   - Building coroutines, generators *)

(* Direct style *)
let rec factorial_direct n =
  if n <= 1 then 1 else n * factorial_direct (n - 1)

(* CPS style: always tail recursive *)
let factorial_cps n k =
  let rec go n k =
    if n <= 1 then k 1
    else go (n - 1) (fun result -> k (n * result))
  in go n k

(* Fibonacci in CPS *)
let fibonacci_cps n k =
  let rec go n k =
    if n <= 1 then k n
    else
      go (n - 1) (fun a ->
        go (n - 2) (fun b ->
          k (a + b)))
  in go n k

(* Early exit: CPS allows "throwing" to a continuation *)
let find_cps pred lst found not_found =
  let rec go = function
    | []     -> not_found ()
    | x :: xs -> if pred x then found x else go xs
  in go lst

let () =
  Printf.printf "5! = %d\n" (factorial_direct 5);
  factorial_cps 5 (Printf.printf "CPS 5! = %d\n");
  fibonacci_cps 10 (Printf.printf "CPS fib(10) = %d\n");

  let lst = [1; 3; 5; 8; 9; 12] in
  find_cps (fun x -> x mod 2 = 0) lst
    (fun x -> Printf.printf "First even: %d\n" x)
    (fun () -> Printf.printf "No evens\n")
