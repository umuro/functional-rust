let rec sieve = function
  | []      -> []
  | p :: rest ->
    p :: sieve (List.filter (fun n -> n mod p <> 0) rest)

let primes_up_to n =
  if n < 2 then []
  else sieve (List.init (n - 1) (fun i -> i + 2))

let () =
  let ps = primes_up_to 50 in
  assert (ps = [2; 3; 5; 7; 11; 13; 17; 19; 23; 29; 31; 37; 41; 43; 47]);
  assert (List.length (primes_up_to 100) = 25);
  print_endline "All assertions passed."
