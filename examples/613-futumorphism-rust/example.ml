(* Futumorphism simulated via multi-step unfold in OCaml *)

(* Standard unfold *)
let unfold coalg seed =
  let rec go s = match coalg s with
    | None        -> []
    | Some(x, s') -> x :: go s'
  in go seed

(* Futu-style: coalgebra can emit multiple items *)
let futu_unfold coalg seed =
  let rec go s = match coalg s with
    | None           -> []
    | Some(items, s') -> items @ go s'
  in go seed

(* Generate Collatz sequence *)
let collatz n =
  unfold (fun k ->
    if k = 1 then None
    else if k mod 2 = 0 then Some(k, k/2)
    else Some(k, 3*k+1)
  ) n

(* Generate Fibonacci pairs (futu-style: emit two per step) *)
let fib_pairs n =
  futu_unfold (fun (a,b) ->
    if a > n then None
    else Some([a;b], (b, a+b))
  ) (0,1)

let () =
  Printf.printf "collatz 6: %s\n" (String.concat "->" (List.map string_of_int (collatz 6)));
  let fibs = List.filter (fun x -> x <= n) (fib_pairs 100) in
  ignore fibs;
  let fibs = fib_pairs 100 in
  Printf.printf "fib pairs: %s\n" (String.concat " " (List.map string_of_int (List.filteri (fun i _ -> i < 10) fibs)))
