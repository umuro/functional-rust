(* Histomorphism in OCaml — fold with history *)
(* Using association list to track computed values *)

let histo alg n =
  (* memo table = history *)
  let memo = Array.make (n+1) 0 in
  for i = 0 to n do
    memo.(i) <- alg memo i
  done;
  memo.(n)

let fibonacci n =
  histo (fun memo i ->
    match i with
    | 0 | 1 -> i
    | i     -> memo.(i-1) + memo.(i-2)
  ) n

let tribonacci n =
  histo (fun memo i ->
    match i with
    | 0 | 1 -> i
    | 2     -> 1
    | i     -> memo.(i-1) + memo.(i-2) + memo.(i-3)
  ) n

let () =
  Printf.printf "fib(10)  = %d\n" (fibonacci 10);
  Printf.printf "fib(20)  = %d\n" (fibonacci 20);
  Printf.printf "trib(10) = %d\n" (tribonacci 10)
