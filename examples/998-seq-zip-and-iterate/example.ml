(* Seq — Zip and Iterate *)
(* Combine sequences and create repeated applications *)

let letters = List.to_seq ['a'; 'b'; 'c'; 'd']
let numbers = List.to_seq [1; 2; 3; 4]
let pairs = Seq.zip letters numbers |> List.of_seq
let () = List.iter (fun (c, n) -> Printf.printf "(%c, %d) " c n) pairs

(* Seq.iterate: repeated function application *)
let collatz n = if n mod 2 = 0 then n / 2 else 3 * n + 1
let seq = Seq.iterate collatz 27 |> Seq.take 20 |> List.of_seq
let () = print_newline ();
  List.iter (fun x -> Printf.printf "%d " x) seq
