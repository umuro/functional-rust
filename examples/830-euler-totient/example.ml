(* Euler's Totient Function in OCaml *)

(* Single value: φ(n) = n × ∏(1 - 1/p) over distinct prime factors *)
let totient (n : int) : int =
  let result = ref n in
  let n' = ref n in
  let p = ref 2 in
  while !p * !p <= !n' do
    if !n' mod !p = 0 then begin
      (* p is a prime factor — multiply result by (p-1)/p *)
      while !n' mod !p = 0 do n' := !n' / !p done;
      result := !result - !result / !p
    end;
    incr p
  done;
  (* Remaining prime factor > sqrt(n) *)
  if !n' > 1 then
    result := !result - !result / !n';
  !result

(* Totient sieve: compute φ(i) for all i in [0, n] in O(n log log n) *)
let totient_sieve (n : int) : int array =
  let phi = Array.init (n + 1) (fun i -> i) in
  for i = 2 to n do
    (* phi[i] = i means i hasn't been processed as prime factor yet *)
    if phi.(i) = i then begin
      (* i is prime *)
      let j = ref i in
      while !j <= n do
        phi.(!j) <- phi.(!j) - phi.(!j) / i;
        j := !j + i
      done
    end
  done;
  phi

(* Sum of totients: ∑φ(k) for k=1..n = (n*(n+1)/2 + 1)/... — test property *)
(* Property: ∑_{d|n} φ(d) = n *)
let sum_divisor_totients (n : int) : int =
  let sieve = totient_sieve n in
  (* Divisors of n *)
  let total = ref 0 in
  for d = 1 to n do
    if n mod d = 0 then total := !total + sieve.(d)
  done;
  !total

let () =
  let values = [1; 2; 6; 9; 10; 12; 36; 100] in
  List.iter (fun n ->
    Printf.printf "φ(%d) = %d\n" n (totient n)
  ) values;

  Printf.printf "\nTotient sieve up to 12:\n";
  let phi = totient_sieve 12 in
  Array.iteri (fun i v -> if i > 0 then Printf.printf "  φ(%d) = %d\n" i v) phi;

  Printf.printf "\nProperty ∑_{d|n} φ(d) = n:\n";
  List.iter (fun n ->
    Printf.printf "  n=%d: sum = %d (should be %d)\n" n (sum_divisor_totients n) n
  ) [6; 12; 30]
