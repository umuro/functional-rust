(* Sieve of Eratosthenes in OCaml *)

(* Return all primes up to and including limit *)
let sieve (limit : int) : int list =
  if limit < 2 then []
  else begin
    let is_prime = Array.make (limit + 1) true in
    is_prime.(0) <- false;
    is_prime.(1) <- false;
    let sqrt_limit = int_of_float (sqrt (float_of_int limit)) in
    for p = 2 to sqrt_limit do
      if is_prime.(p) then begin
        let j = ref (p * p) in
        while !j <= limit do
          is_prime.(!j) <- false;
          j := !j + p
        done
      end
    done;
    (* Collect all primes functionally *)
    Array.to_seq is_prime
    |> Seq.filter_mapi (fun i b -> if b then Some i else None)
    |> List.of_seq
  end

(* Count primes up to n (prime-counting function π(n)) *)
let prime_pi (n : int) : int =
  List.length (sieve n)

(* Segmented sieve for large ranges — sieve [lo, hi] *)
let segmented_sieve (lo : int) (hi : int) : int list =
  let base_primes = sieve (int_of_float (sqrt (float_of_int hi)) + 1) in
  let size = hi - lo + 1 in
  let is_prime = Array.make size true in
  (* 0 and 1 are not prime *)
  if lo <= 1 then
    for i = lo to min 1 hi do is_prime.(i - lo) <- false done;
  List.iter (fun p ->
    let start = max (p * p) (((lo + p - 1) / p) * p) in
    let j = ref start in
    while !j <= hi do
      is_prime.(!j - lo) <- false;
      j := !j + p
    done
  ) base_primes;
  Array.to_seq is_prime
  |> Seq.filter_mapi (fun i b -> if b then Some (lo + i) else None)
  |> List.of_seq

let () =
  let primes_30 = sieve 30 in
  Printf.printf "Primes up to 30: [%s]\n"
    (String.concat "; " (List.map string_of_int primes_30));
  Printf.printf "π(100) = %d (expected 25)\n" (prime_pi 100);
  Printf.printf "Primes in [10, 50]: [%s]\n"
    (String.concat "; " (List.map string_of_int (segmented_sieve 10 50)))
