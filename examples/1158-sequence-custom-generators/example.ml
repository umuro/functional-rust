(* Sequence — Custom Generators *)
(* Build custom sequence generators *)

(* Primes via sieve *)
let primes =
  let rec sieve s () = match s () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (p, rest) ->
      Seq.Cons (p, sieve (Seq.filter (fun n -> n mod p <> 0) rest))
  in
  sieve (Seq.unfold (fun n -> Some (n, n+1)) 2)

let first_20_primes = primes |> Seq.take 20 |> List.of_seq
let () =
  List.iter (fun p -> Printf.printf "%d " p) first_20_primes;
  print_newline ()
