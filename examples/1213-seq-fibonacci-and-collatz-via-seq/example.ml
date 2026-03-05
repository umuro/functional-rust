(* Seq — Fibonacci and Collatz via Seq *)
(* Classic sequences with Seq module *)

let collatz n =
  Seq.unfold (fun n ->
    if n = 1 then None
    else let next = if n mod 2 = 0 then n / 2 else 3 * n + 1 in
      Some (n, next)
  ) n

let () =
  Printf.printf "Collatz(27): ";
  let seq = collatz 27 in
  Seq.iter (fun x -> Printf.printf "%d " x) seq;
  Printf.printf "1\n";
  Printf.printf "Length: %d\n" (Seq.length (collatz 27) + 1)
