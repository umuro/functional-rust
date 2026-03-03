let naturals = Seq.ints 0

let fibs =
  Seq.unfold (fun (a, b) -> Some (a, (b, a + b))) (0, 1)

let primes =
  let is_prime n =
    n >= 2 && Seq.ints 2
    |> Seq.take_while (fun i -> i * i <= n)
    |> Seq.for_all (fun i -> n mod i <> 0)
  in
  Seq.ints 2 |> Seq.filter is_prime

let () =
  Printf.printf "Naturals: ";
  Seq.take 5 naturals |> Seq.iter (Printf.printf "%d ");
  Printf.printf "\nFibs: ";
  Seq.take 10 fibs |> Seq.iter (Printf.printf "%d ");
  Printf.printf "\nPrimes: ";
  Seq.take 10 primes |> Seq.iter (Printf.printf "%d ");
  print_newline ()
